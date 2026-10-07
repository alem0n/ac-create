//! 用例依赖的抽象端口（依赖倒置）。
//!
//! 端口定义在 domain 层；实现位于 infrastructure 层；注入由组合层（main.rs）完成。
//! services 层只经端口与外部交互，不绑定具体实现，便于单测替换。

use std::path::{Path, PathBuf};

use crate::domain::error::CreateError;

/// 全树文本替换的可测量结果。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplaceOutcome {
    /// 已发生替换的文件数。
    pub replaced_files: usize,
    /// 内容非 UTF-8 而被跳过的文件（须向用户提示）。
    pub skipped_non_utf8: Vec<PathBuf>,
}

/// 单文件文本替换的结果状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplaceStatus {
    /// 已写入替换后的内容。
    Replaced,
    /// 文件内无匹配内容。
    NotFound,
    /// 内容非 UTF-8，跳过替换。
    SkippedNonUtf8,
    /// 判定为二进制文件（后缀 / 超大 / 含 NUL 字节），不尝试替换。
    SkippedBinary,
}

/// 子进程运行结果（领域级表示，不泄漏 std::process 细节）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    /// 退出状态是否成功（退出码 0）。
    pub success: bool,
    /// stdout 与 stderr 合并后的文本（已 trim）。
    pub output: String,
}

impl RunResult {
    /// 构造携带提示文本的失败结果。
    pub fn failed(output: impl Into<String>) -> Self {
        Self {
            success: false,
            output: output.into(),
        }
    }
}

/// 表现端口：进度与提示上报（面向用户的输出由组合层实现）。
pub trait Reporter {
    /// 流水线步骤头：印为 `[{number}/{total}] {message}`。
    fn step(&self, number: u32, total: u32, message: &str);
    /// 缩进的补充信息。
    fn info(&self, message: &str);
    /// 用户可读警告。
    fn warn(&self, message: &str);
    /// 原样输出（如子进程输出）。
    fn raw(&self, message: &str);
}

/// 基础设施端口：文件系统操作（模板复制与文本替换）。
pub trait FileSystem {
    fn exists(&self, path: &Path) -> bool;
    fn is_dir(&self, path: &Path) -> bool;
    /// 递归复制目录树。跳过构建产物与缓存条目。
    fn copy_tree(&self, src: &Path, dest: &Path) -> Result<(), CreateError>;
    /// 重命名或移动（目录或文件）。
    fn rename(&self, from: &Path, to: &Path) -> Result<(), CreateError>;
    /// 全树文本替换：替换所有判定为文本的文件内匹配项。
    fn replace_all(&self, root: &Path, old: &str, new: &str)
        -> Result<ReplaceOutcome, CreateError>;
    /// 单文件文本替换（不做二进制判定）。
    fn replace_in_file(
        &self,
        path: &Path,
        old: &str,
        new: &str,
    ) -> Result<ReplaceStatus, CreateError>;
}

/// 基础设施端口：子进程执行（超时与缺失命令由实现处理）。
pub trait Runner {
    fn run(&self, program: &str, args: &[&str], cwd: &Path) -> RunResult;
}
