//! 集成测试：覆盖公共 API 的端到端路径。
//!
//! 约定：集成测试置于 tests/ 目录，代码不进入 src。
//! 变更纪律：公共 API 变更必须同步更新本文件对应测试。
//!
//! 策略：文件系统用真实实现（临时目录隔离），子进程用假实现
//! （不触发真实的 uv / pnpm / cargo / git / make 调用与网络访问）。
//! 模板根目录使用编译期已知的仓库内 templates/。

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use ac_create::domain::ports::{Reporter, RunResult, Runner};
use ac_create::domain::{CreateError, Language, ProjectName};
use ac_create::infrastructure::StdFs;
use ac_create::services::{create, CreateRequest};
use ac_create::AppConfig;
use tempfile::{tempdir, TempDir};

/// 记录的一次子进程调用。
#[derive(Debug, Clone, PartialEq, Eq)]
struct Recorded {
    program: String,
    args: Vec<String>,
    cwd: PathBuf,
}

/// 假 Runner：记录调用，并按程序名注入预设结果（默认成功）。
struct FakeRunner {
    recorded: RefCell<Vec<Recorded>>,
    failures: HashMap<String, RunResult>,
}

impl FakeRunner {
    fn new() -> Self {
        Self {
            recorded: RefCell::new(Vec::new()),
            failures: HashMap::new(),
        }
    }

    /// 让指定程序返回失败（携带失败输出）。
    fn fail(mut self, program: &str, output: &str) -> Self {
        self.failures
            .insert(program.to_string(), RunResult::failed(output.to_string()));
        self
    }

    fn commands(&self) -> Vec<Recorded> {
        self.recorded.borrow().clone()
    }

    fn programs(&self) -> Vec<String> {
        self.commands().into_iter().map(|c| c.program).collect()
    }
}

impl Runner for FakeRunner {
    fn run(&self, program: &str, args: &[&str], cwd: &Path) -> RunResult {
        self.recorded.borrow_mut().push(Recorded {
            program: program.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            cwd: cwd.to_path_buf(),
        });
        match self.failures.get(program) {
            Some(result) => result.clone(),
            None => RunResult {
                success: true,
                output: String::new(),
            },
        }
    }
}

/// 收集型 Reporter：断言面向用户的输出行。
#[derive(Default)]
struct CollectReporter {
    lines: RefCell<Vec<String>>,
}

impl Reporter for CollectReporter {
    fn step(&self, number: u32, total: u32, message: &str) {
        self.lines
            .borrow_mut()
            .push(format!("[{number}/{total}] {message}"));
    }

    fn info(&self, message: &str) {
        self.lines.borrow_mut().push(format!("      {message}"));
    }

    fn warn(&self, message: &str) {
        self.lines.borrow_mut().push(format!("警告：{message}"));
    }

    fn raw(&self, message: &str) {
        self.lines.borrow_mut().push(message.to_string());
    }
}

impl CollectReporter {
    fn lines(&self) -> Vec<String> {
        self.lines.borrow().clone()
    }

    fn warns(&self) -> Vec<String> {
        self.lines()
            .into_iter()
            .filter(|line| line.starts_with("警告"))
            .collect()
    }
}

fn templates_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
}

fn make_request(language: Language, name: &str, target: &Path) -> CreateRequest {
    CreateRequest {
        language,
        project_name: ProjectName::parse(name).expect("项目名合法"),
        target_dir: target.to_path_buf(),
        author: "测试作者".to_string(),
        templates_root: templates_root(),
        init_git: false,
        run_check: false,
    }
}

/// 创建项目并返回（临时目录, 项目根目录, 假 Runner, Reporter）。
fn scaffold(
    language: Language,
    name: &str,
    runner: FakeRunner,
) -> (TempDir, PathBuf, FakeRunner, CollectReporter) {
    let target = tempdir().expect("创建临时目录失败");
    let reporter = CollectReporter::default();
    let request = make_request(language, name, target.path());
    let dest = create(&request, &StdFs, &runner, &reporter).expect("创建应成功");
    (target, dest, runner, reporter)
}

fn read(dest: &Path, relative: &str) -> String {
    fs::read_to_string(dest.join(relative))
        .unwrap_or_else(|_| panic!("文件应存在：{}", dest.join(relative).display()))
}

#[test]
fn creates_python_project_end_to_end() {
    let (_tempdir, dest, runner, reporter) =
        scaffold(Language::Python, "my-cool-app", FakeRunner::new());

    assert!(dest.ends_with("my-cool-app"));
    // 占位名替换：包目录重命名为 snake_case
    let pkg_dir = dest.join("src/my_cool_app");
    assert!(pkg_dir.is_dir(), "包目录应重命名为 src/my_cool_app");
    assert!(!dest.join("src/myapp").exists(), "旧占位目录不应残留");
    assert!(read(&pkg_dir, "main.py").contains("from my_cool_app import"));
    // 配置与锁文件中的占位名替换
    assert!(read(&dest, "pyproject.toml").contains("name = \"my_cool_app\""));
    assert!(read(&dest, "pyproject.toml").contains("packages = [\"src/my_cool_app\"]"));
    let lock = read(&dest, "uv.lock");
    assert!(lock.contains("my_cool_app"));
    assert!(!lock.contains("myapp"), "uv.lock 的占位名应被替换");
    // LICENSE 占位替换
    let license = read(&dest, "LICENSE");
    assert!(license.contains("测试作者"));
    assert!(!license.contains("替换为实际版权持有者"));
    // 构建产物与缓存不进入新项目
    assert!(!dest.join("src/my_cool_app/__pycache__").exists());
    assert!(!dest.join(".coverage").exists());
    // 锁文件重建命令
    assert_eq!(runner.programs(), vec!["uv"]);
    assert_eq!(runner.commands()[0].args, ["sync", "--extra", "dev"],);
    // 无告警，步骤头齐全
    assert!(reporter.warns().is_empty());
    let lines = reporter.lines();
    assert!(lines
        .iter()
        .any(|line| line.starts_with("[1/5] 复制 python 模板")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("[2/5] 替换占位名 myapp → my_cool_app")));
    assert!(lines.iter().any(|line| line.contains("已替换")));
}

#[test]
fn creates_rust_project_and_keeps_crate_name() {
    let (_tempdir, dest, runner, _reporter) =
        scaffold(Language::Rust, "my-cool-app", FakeRunner::new());

    let cargo_toml = read(&dest, "Cargo.toml");
    assert!(
        cargo_toml.contains("name = \"my-cool-app\""),
        "crate 发布名保留连字符"
    );
    assert!(!cargo_toml.contains("name = \"my_cool_app\""));
    // 代码标识符转 snake_case
    assert!(read(&dest, "src/main.rs").contains("use my_cool_app::"));
    assert!(read(&dest, "src/lib.rs").contains("my_cool_app"));
    // 锁文件占位替换（须由 cargo 重建，此处为假实现）
    assert!(read(&dest, "Cargo.lock").contains("my_cool_app"));
    assert_eq!(runner.programs(), vec!["cargo"]);
    assert_eq!(runner.commands()[0].args, ["generate-lockfile"]);
}

#[test]
fn creates_typescript_project() {
    let (_tempdir, dest, runner, _reporter) =
        scaffold(Language::TypeScript, "my-app", FakeRunner::new());

    let package_json = read(&dest, "package.json");
    assert!(package_json.contains("\"name\": \"my-app\""));
    assert!(!package_json.contains("myapp"));
    assert_eq!(runner.programs(), vec!["pnpm"]);
}

#[test]
fn lock_regeneration_failure_warns_but_keeps_replaced_files() {
    let (_tempdir, dest, _runner, reporter) = scaffold(
        Language::Python,
        "my-app",
        FakeRunner::new().fail("uv", "uv: command not found"),
    );

    let warns = reporter.warns();
    assert_eq!(warns.len(), 1, "应恰好一条警告，实得：{warns:?}");
    assert!(warns[0].contains("uv 不可用或失败"));
    assert!(warns[0].contains("请手动执行 uv sync --extra dev"));
    // 项目仍创建成功且内容已替换
    assert!(dest.join("src/my_app").is_dir());
    assert!(read(&dest, "pyproject.toml").contains("name = \"my_app\""));
}

#[test]
fn git_flag_records_init_add_commit() {
    let (_tempdir, _dest, runner, _reporter) = {
        let target = tempdir().unwrap();
        let mut request = make_request(Language::Rust, "my-app", target.path());
        request.init_git = true;
        let runner = FakeRunner::new();
        let reporter = CollectReporter::default();
        let dest = create(&request, &StdFs, &runner, &reporter).expect("创建应成功");
        (target, dest, runner, reporter)
    };

    let commands = runner.commands();
    assert_eq!(
        commands
            .iter()
            .map(|c| c.program.as_str())
            .collect::<Vec<_>>(),
        ["cargo", "git", "git", "git"]
    );
    let git_args: Vec<&str> = commands[1].args.iter().map(String::as_str).collect();
    assert_eq!(git_args, ["init", "-b", "main"]);
    assert_eq!(commands[2].args, ["add", "."]);
    assert_eq!(
        commands[3].args,
        ["commit", "-m", "feat: scaffold project from template"]
    );
}

#[test]
fn git_init_failure_skips_commit_and_warns() {
    let target = tempdir().unwrap();
    let mut request = make_request(Language::Rust, "my-app", target.path());
    request.init_git = true;
    let runner = FakeRunner::new().fail("git", "git: command not found");
    let reporter = CollectReporter::default();
    let dest = create(&request, &StdFs, &runner, &reporter).expect("创建应成功");

    // git init 失败即跳过后续命令
    assert_eq!(runner.programs(), ["cargo", "git"]);
    assert!(reporter
        .warns()
        .iter()
        .any(|line| line.contains("git 不可用，跳过仓库初始化")));
    assert!(dest.join("Cargo.toml").exists());
}

#[test]
fn check_flag_runs_make_check_and_reports() {
    fn run_case(make_fail: bool) {
        let target = tempdir().unwrap();
        let mut request = make_request(Language::TypeScript, "my-app", target.path());
        request.run_check = true;
        let runner = if make_fail {
            FakeRunner::new().fail("make", "make boom")
        } else {
            FakeRunner::new()
        };
        let reporter = CollectReporter::default();
        create(&request, &StdFs, &runner, &reporter).expect("创建应成功");

        let lines = reporter.lines();
        assert!(lines
            .iter()
            .any(|line| line.contains("运行 make check 初验...")));
        let expected = if make_fail { "失败" } else { "全绿" };
        assert!(lines
            .iter()
            .any(|line| line.contains(&format!("make check：{expected}"))));
        if make_fail {
            assert!(lines.iter().any(|line| line.contains("make boom")));
        }
    }

    run_case(false);
    run_case(true);
}

#[test]
fn refuses_existing_destination() {
    let target = tempdir().unwrap();
    let dest = target.path().join("my-app");
    fs::create_dir_all(&dest).unwrap();
    let request = make_request(Language::Rust, "my-app", target.path());

    let err = create(
        &request,
        &StdFs,
        &FakeRunner::new(),
        &CollectReporter::default(),
    )
    .unwrap_err();
    assert!(matches!(err, CreateError::DestinationExists(_)));
    assert!(err.to_string().contains("目标目录已存在"));
}

#[test]
fn missing_template_root_reports_error() {
    let target = tempdir().unwrap();
    let request = CreateRequest {
        templates_root: target.path().to_path_buf(),
        ..make_request(Language::Rust, "my-app", target.path())
    };

    let err = create(
        &request,
        &StdFs,
        &FakeRunner::new(),
        &CollectReporter::default(),
    )
    .unwrap_err();
    assert!(matches!(err, CreateError::TemplateMissing(_)));
}

#[test]
fn missing_target_dir_reports_error() {
    let target = tempdir().unwrap();
    let request = make_request(Language::Rust, "my-app", &target.path().join("nope"));

    let err = create(
        &request,
        &StdFs,
        &FakeRunner::new(),
        &CollectReporter::default(),
    )
    .unwrap_err();
    assert!(matches!(err, CreateError::TargetMissing(_)));
    assert!(err.to_string().contains("目标目录不存在"));
}

#[test]
fn config_loads_with_defaults() {
    let config = AppConfig::load();
    assert_eq!(config.log_level, "warn");
}
