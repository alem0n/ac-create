//! 模板目录定位：便携分发约定。
//!
//! 查找顺序（前者命中即返回）：
//! 1. 可执行文件同级的 `templates/`：便携分发包布局（etc/build-dist.sh 组装）。
//! 2. 当前工作目录的 `templates/`：开发态 `cargo run` / `cargo test`。
//!
//! 依据：Rust 二进制无脚本路径概念，以可执行文件位置替代 Python 版的
//! 「脚本同级」定位。Windows / macOS / Linux 分发包均按同一约定布置。

use std::env;
use std::path::PathBuf;

/// 按便携分发约定定位模板目录。未找到返回 None（由组合层转为错误）。
pub fn locate_templates() -> Option<PathBuf> {
    exe_dir_templates().or_else(cwd_templates)
}

fn exe_dir_templates() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    let dir = exe.parent()?;
    let candidate = dir.join("templates");
    candidate.is_dir().then_some(candidate)
}

fn cwd_templates() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    let candidate = cwd.join("templates");
    candidate.is_dir().then_some(candidate)
}
