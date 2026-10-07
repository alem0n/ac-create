//! 用例：从模板创建新项目。
//!
//! 流水线（5 步，与 Python 版一致）：
//! 复制模板 → 替换占位名 → 替换 LICENSE → 重建锁文件 → 后续处理（可选 git 与检查）。
//!
//! 锁文件在占位替换后由对应包管理器重建。工具不可用时保留文本替换后的旧锁并警告，
//! 提示用户事后手动同步，不视为失败。

use std::path::{Path, PathBuf};

use crate::domain::error::CreateError;
use crate::domain::language::Language;
use crate::domain::ports::{FileSystem, ReplaceStatus, Reporter, Runner};
use crate::domain::project_name::{ProjectName, LICENSE_PLACEHOLDER, PLACEHOLDER};

/// 创建请求。由组合层解析 CLI 参数后组装并注入。
pub struct CreateRequest {
    pub language: Language,
    pub project_name: ProjectName,
    /// 新项目的父目录（须存在）。
    pub target_dir: PathBuf,
    /// LICENSE 版权持有者。
    pub author: String,
    /// 模板根目录（含 python / typescript / rust 子目录）。
    pub templates_root: PathBuf,
    /// 初始化 git 仓库并作首次提交。
    pub init_git: bool,
    /// 创建后运行 make check 初验。
    pub run_check: bool,
}

/// 创建新项目。成功返回项目根目录。
pub fn create(
    request: &CreateRequest,
    fs: &dyn FileSystem,
    runner: &dyn Runner,
    reporter: &dyn Reporter,
) -> Result<PathBuf, CreateError> {
    let template = request.templates_root.join(request.language.as_str());
    if !fs.is_dir(&template) {
        return Err(CreateError::TemplateMissing(template));
    }
    if !fs.is_dir(&request.target_dir) {
        return Err(CreateError::TargetMissing(request.target_dir.clone()));
    }
    let dest = request.target_dir.join(request.project_name.as_str());
    if fs.exists(&dest) {
        return Err(CreateError::DestinationExists(dest));
    }

    tracing::info!(
        language = %request.language,
        name = %request.project_name,
        "create.start"
    );

    let pkg = request.language.package_name(&request.project_name);

    reporter.step(
        1,
        5,
        &format!("复制 {} 模板 → {}", request.language, dest.display()),
    );
    fs.copy_tree(&template, &dest)?;

    reporter.step(
        2,
        5,
        &format!("替换占位名 {PLACEHOLDER} → {pkg}（Python 含目录重命名）"),
    );
    replace_placeholder_names(request, &pkg, &dest, fs, reporter)?;

    reporter.step(3, 5, "替换 LICENSE 占位");
    replace_license(request, &dest, fs, reporter)?;

    reporter.step(4, 5, "重建锁文件");
    regenerate_lockfiles(&request.language, &dest, runner, reporter);

    reporter.step(5, 5, "后续处理");
    if request.init_git {
        init_git(&dest, runner, reporter);
    }
    if request.run_check {
        run_make_check(&dest, runner, reporter);
    }

    tracing::info!(dest = %dest.display(), package = pkg, "create.done");
    Ok(dest)
}

/// 替换占位名：Python 先重命名包目录，再做全树文本替换；Rust 另保留 crate 连字符名。
fn replace_placeholder_names(
    request: &CreateRequest,
    pkg: &str,
    dest: &Path,
    fs: &dyn FileSystem,
    reporter: &dyn Reporter,
) -> Result<(), CreateError> {
    if let Language::Python = request.language {
        let from = dest.join("src").join(PLACEHOLDER);
        if !fs.is_dir(&from) {
            return Err(CreateError::PackageDirMissing(from));
        }
        let to = dest.join("src").join(pkg);
        fs.rename(&from, &to)?;
    }

    let outcome = fs.replace_all(dest, PLACEHOLDER, pkg)?;
    for path in &outcome.skipped_non_utf8 {
        reporter.warn(&format!("非 UTF-8 文件，跳过替换：{}", path.display()));
    }
    reporter.info(&format!("已替换 {} 个文件", outcome.replaced_files));

    if let Language::Rust = request.language {
        // Rust：crate 名（发布名）保留用户输入的连字符形式，与代码标识符解耦
        let cargo_toml = dest.join("Cargo.toml");
        let status = fs.replace_in_file(
            &cargo_toml,
            &format!("name = \"{pkg}\""),
            &format!("name = \"{}\"", request.project_name),
        )?;
        warn_if_non_utf8(&cargo_toml, status, reporter);
    }
    Ok(())
}

fn replace_license(
    request: &CreateRequest,
    dest: &Path,
    fs: &dyn FileSystem,
    reporter: &dyn Reporter,
) -> Result<(), CreateError> {
    let license = dest.join("LICENSE");
    if !fs.exists(&license) {
        return Ok(());
    }
    let status = fs.replace_in_file(&license, LICENSE_PLACEHOLDER, &request.author)?;
    warn_if_non_utf8(&license, status, reporter);
    Ok(())
}

fn warn_if_non_utf8(path: &Path, status: ReplaceStatus, reporter: &dyn Reporter) {
    if status == ReplaceStatus::SkippedNonUtf8 {
        reporter.warn(&format!("非 UTF-8 文件，跳过替换：{}", path.display()));
    }
}

/// 重建锁文件。工具不可用或失败时仅警告，保留文本替换后的旧锁。
fn regenerate_lockfiles(
    language: &Language,
    dest: &Path,
    runner: &dyn Runner,
    reporter: &dyn Reporter,
) {
    let Some((program, args)) = language.lockfile_command() else {
        return;
    };
    let result = runner.run(program, args, dest);
    if result.success {
        tracing::debug!(command = program, "lockfile.regenerated");
    } else {
        reporter.warn(language.lockfile_warning());
    }
}

fn init_git(dest: &Path, runner: &dyn Runner, reporter: &dyn Reporter) {
    let init = runner.run("git", &["init", "-b", "main"], dest);
    if !init.success {
        reporter.warn("git 不可用，跳过仓库初始化。");
        return;
    }
    runner.run("git", &["add", "."], dest);
    let commit = runner.run(
        "git",
        &["commit", "-m", "feat: scaffold project from template"],
        dest,
    );
    if !commit.success {
        reporter.warn(&format!("初始提交失败。{}", commit.output));
    }
}

fn run_make_check(dest: &Path, runner: &dyn Runner, reporter: &dyn Reporter) {
    reporter.info("运行 make check 初验...");
    let result = runner.run("make", &["check"], dest);
    let label = if result.success { "全绿" } else { "失败" };
    reporter.info(&format!("make check：{label}"));
    if !result.success {
        reporter.raw(&result.output);
    }
}
