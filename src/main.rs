//! 入口点：只做组装与初始化，不承载业务逻辑。
//!
//! 职责契约（高内聚低耦合）：
//! - 解析 CLI（clap）。
//! - 定位模板目录并组装 CreateRequest。
//! - 初始化日志（tracing subscriber）。
//! - 注入端口实现（StdFs / StdRunner / CliReporter）。
//! - 禁止在此编写业务规则。业务规则属于 services 与 domain 层。

// 入口点显式豁免 print_stdout：表现层向用户输出进度与结果。
// 业务模块（lib crate）一律禁止 println!/dbg!。见 lib.rs 声明。
#![allow(clippy::print_stdout)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

use ac_create::domain::ports::Reporter;
use ac_create::domain::{CreateError, Language, ProjectName};
use ac_create::infrastructure::{locate_templates, StdFs, StdRunner};
use ac_create::services::{create, CreateRequest};
use ac_create::{logger, AppConfig};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = AppConfig::load();
    logger::init(&config);

    tracing::info!(language = %cli.language, name = %cli.name, "app.start");

    match run(&cli) {
        Ok(dest) => {
            tracing::info!(dest = %dest.display(), "app.done");
            print_followup(&cli.language, &dest);
            ExitCode::SUCCESS
        }
        Err(err) => {
            tracing::error!(error = %err, "app.failed");
            eprintln!("错误：{err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<PathBuf, CreateError> {
    let templates_root = locate_templates().ok_or(CreateError::TemplatesNotFound)?;
    let request = CreateRequest {
        language: cli.language,
        project_name: cli.name.clone(),
        target_dir: resolve_target(&cli.target),
        author: match cli.author.as_ref() {
            Some(value) if !value.trim().is_empty() => value.clone(),
            _ => cli.name.to_string(),
        },
        templates_root,
        init_git: cli.git,
        run_check: cli.check,
    };
    create(&request, &StdFs, &StdRunner::default(), &CliReporter)
}

/// 相对路径基于当前工作目录解析（与 Python 版的 resolve() 后处理一致）。
fn resolve_target(target: &str) -> PathBuf {
    let path = Path::new(target);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(path),
        Err(_) => path.to_path_buf(),
    }
}

/// 控制台表现层：把用例上报的进度原样印出（用户可读输出，非日志）。
struct CliReporter;

impl Reporter for CliReporter {
    fn step(&self, number: u32, total: u32, message: &str) {
        println!("[{number}/{total}] {message}");
    }

    fn info(&self, message: &str) {
        println!("      {message}");
    }

    fn warn(&self, message: &str) {
        println!("警告：{message}");
    }

    fn raw(&self, message: &str) {
        println!("{message}");
    }
}

fn print_followup(language: &Language, dest: &Path) {
    let dir = dest
        .file_name()
        .map(|name| name.to_string_lossy().to_string());
    println!();
    println!("完成：{}", dest.display());
    println!("后续步骤：");
    if let Some(dir) = dir {
        println!("  cd {dir}");
    }
    println!("  git config core.hooksPath .githooks  # 启用提交检查点");
    println!("  {}  # 若未自动执行", language.sync_hint());
    println!("  替换 SECURITY.md 联系方式后投入使用。");
}

/// 从项目模板创建新项目并替换占位名。
#[derive(Parser, Debug)]
#[command(
    name = "ac-create",
    version,
    about = "从项目模板创建新项目并替换占位名。",
    after_help = EXAMPLES
)]
struct Cli {
    /// 语言：python | typescript | rust
    #[arg(value_parser = clap::value_parser!(Language))]
    language: Language,

    /// 项目名（小写字母/数字/连字符/下划线，如 my-cool-app）
    #[arg(value_parser = clap::value_parser!(ProjectName))]
    name: ProjectName,

    /// LICENSE 版权持有者（默认使用项目名）
    #[arg(long)]
    author: Option<String>,

    /// 目标目录（默认当前目录）
    #[arg(long, default_value = ".")]
    target: String,

    /// 初始化 git 仓库并作首次提交
    #[arg(long)]
    git: bool,

    /// 创建后运行 make check 初验
    #[arg(long)]
    check: bool,
}

const EXAMPLES: &str = "\
示例：
  ac-create python my-cool-app
  ac-create rust my-cool-app --author \"张三\" --git
  ac-create typescript my-app --target ./projects

模板定位：templates/ 与可执行文件同级（便携分发），或位于当前目录。";
