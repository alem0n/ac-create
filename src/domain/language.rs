//! 语言枚举：各语言的标识符映射、锁文件重建与回退提示规则。
//!
//! 规则来源（与 Python 版一致）：
//! - python / rust：代码标识符须为合法标识符，连字符转下划线；
//!   Rust crate 发布名另保留连字符形式（见 services::create）。
//! - typescript：包名保留用户输入形式。

use std::fmt;
use std::str::FromStr;

use crate::domain::error::CreateError;
use crate::domain::project_name::ProjectName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Python,
    TypeScript,
    Rust,
}

impl Language {
    /// 支持的语言名（CLI 参数校验与错误提示复用）。
    pub const ALL: [&'static str; 3] = ["python", "typescript", "rust"];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::Rust => "rust",
        }
    }

    /// 代码标识符名（导入 / 包路径用）。
    pub fn package_name(&self, project: &ProjectName) -> String {
        match self {
            Self::Python | Self::Rust => project.as_str().replace('-', "_"),
            Self::TypeScript => project.as_str().to_string(),
        }
    }

    /// 重建锁文件所需的包管理器命令。无需重建的语言返回 None。
    pub fn lockfile_command(&self) -> Option<(&'static str, &'static [&'static str])> {
        match self {
            Self::Python => Some(("uv", &["sync", "--extra", "dev"])),
            Self::TypeScript => Some(("pnpm", &["install"])),
            Self::Rust => Some(("cargo", &["generate-lockfile"])),
        }
    }

    /// 锁文件重建失败时的回退提示（工具不可用或命令失败）。
    pub fn lockfile_warning(&self) -> &'static str {
        match self {
            Self::Python => "uv 不可用或失败。uv.lock 为旧占位替换版本。请手动执行 uv sync --extra dev 重建。",
            Self::TypeScript => {
                "pnpm 不可用或失败。pnpm-lock.yaml 为旧占位替换版本。请手动执行 pnpm install 重建。"
            }
            Self::Rust => {
                "cargo 不可用或失败。Cargo.lock 为旧占位替换版本。请手动执行 cargo generate-lockfile 重建。"
            }
        }
    }

    /// 创建后建议执行的同步命令（后续步骤提示用）。
    pub fn sync_hint(&self) -> &'static str {
        match self {
            Self::Python => "uv sync --extra dev",
            Self::TypeScript => "pnpm install",
            Self::Rust => "cargo build",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Language {
    type Err = CreateError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "python" => Ok(Self::Python),
            "typescript" => Ok(Self::TypeScript),
            "rust" => Ok(Self::Rust),
            other => Err(CreateError::UnsupportedLanguage(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_languages() {
        assert_eq!("python".parse::<Language>().unwrap(), Language::Python);
        assert_eq!(
            "typescript".parse::<Language>().unwrap(),
            Language::TypeScript
        );
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
    }

    #[test]
    fn rejects_unsupported_language() {
        let err = "Ruby".parse::<Language>().unwrap_err();
        assert!(matches!(err, CreateError::UnsupportedLanguage(_)));
        assert_eq!(
            err.to_string(),
            "不支持的语言：Ruby（支持：python | typescript | rust）"
        );
    }

    #[test]
    fn maps_package_names() {
        let project = ProjectName::parse("my-cool-app").unwrap();
        assert_eq!(Language::Python.package_name(&project), "my_cool_app");
        assert_eq!(Language::Rust.package_name(&project), "my_cool_app");
        assert_eq!(Language::TypeScript.package_name(&project), "my-cool-app");
    }

    #[test]
    fn lockfile_commands_match_package_managers() {
        assert_eq!(
            Language::Python.lockfile_command(),
            Some(("uv", &["sync", "--extra", "dev"][..]))
        );
        assert_eq!(
            Language::TypeScript.lockfile_command(),
            Some(("pnpm", &["install"][..]))
        );
        assert_eq!(
            Language::Rust.lockfile_command(),
            Some(("cargo", &["generate-lockfile"][..]))
        );
    }

    #[test]
    fn lockfile_warnings_mention_manual_recovery() {
        assert!(Language::Python
            .lockfile_warning()
            .contains("请手动执行 uv sync"));
        assert!(Language::TypeScript
            .lockfile_warning()
            .contains("请手动执行 pnpm install"));
        assert!(Language::Rust
            .lockfile_warning()
            .contains("请手动执行 cargo generate-lockfile"));
    }
}
