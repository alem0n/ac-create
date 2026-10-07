//! 文件系统端口实现：目录树复制与文本替换。
//!
//! 行为对齐 Python 版：
//! - 复制时按条目名跳过构建产物与缓存（不限层级与类型）。
//! - 二进制后缀、超过大小上限、含 NUL 字节的文件不做文本替换（静默跳过）。
//! - 非 UTF-8 文件跳过替换，经 ReplaceOutcome 上报后向用户提示。
//! - 外部 IO 错误一律附上下文转译为领域错误。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::domain::error::CreateError;
use crate::domain::ports::{FileSystem, ReplaceOutcome, ReplaceStatus};

/// 复制时跳过的构建产物与缓存（不进入新项目）。
const IGNORED_ENTRIES: [&str; 13] = [
    ".venv",
    "node_modules",
    "target",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".import_linter_cache",
    "coverage",
    "dist",
    "htmlcov",
    ".git",
    ".coverage",
];

/// 超过此大小的文件不做文本替换（5 MB，与 Python 版一致）。
const MAX_TEXT_FILE: u64 = 5 * 1024 * 1024;

/// 明确的二进制后缀（不含前导点）。
const BINARY_SUFFIXES: [&str; 9] = [
    "png", "jpg", "jpeg", "gif", "ico", "webp", "woff2", "pdf", "zip",
];

/// 文件系统端口的标准实现（std::fs）。
pub struct StdFs;

impl FileSystem for StdFs {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn copy_tree(&self, src: &Path, dest: &Path) -> Result<(), CreateError> {
        copy_tree_inner(src, dest)
            .map_err(|source| io_error(format!("复制模板 → {}", dest.display()), source))
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), CreateError> {
        fs::rename(from, to).map_err(|source| {
            io_error(
                format!("重命名 {} → {}", from.display(), to.display()),
                source,
            )
        })
    }

    fn replace_all(
        &self,
        root: &Path,
        old: &str,
        new: &str,
    ) -> Result<ReplaceOutcome, CreateError> {
        let mut outcome = ReplaceOutcome::default();
        let mut files = Vec::new();
        collect_files(root, &mut files)
            .map_err(|source| io_error(format!("遍历 {}", root.display()), source))?;
        for path in files {
            match replace_entry(&path, old, new)? {
                ReplaceStatus::Replaced => outcome.replaced_files += 1,
                ReplaceStatus::SkippedNonUtf8 => outcome.skipped_non_utf8.push(path),
                ReplaceStatus::NotFound | ReplaceStatus::SkippedBinary => {}
            }
        }
        Ok(outcome)
    }

    fn replace_in_file(
        &self,
        path: &Path,
        old: &str,
        new: &str,
    ) -> Result<ReplaceStatus, CreateError> {
        replace_text(path, old, new)
    }
}

fn io_error(context: impl Into<String>, source: io::Error) -> CreateError {
    CreateError::Io {
        context: context.into(),
        source,
    }
}

/// 递归复制目录树。符号链接折向目标内容（与 shutil.copytree 默认一致）。
fn copy_tree_inner(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    let mut entries = fs::read_dir(src)?.collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry.file_name();
        let is_ignored = name
            .to_str()
            .map(|n| IGNORED_ENTRIES.contains(&n))
            .unwrap_or(false);
        if is_ignored {
            continue;
        }
        let from = entry.path();
        let to = dest.join(&name);
        if entry.metadata()?.is_dir() {
            copy_tree_inner(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// 收集目录树下全部文件路径（按名字排序，保证可复现的遍历顺序）。
fn collect_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(root)?.collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

/// 先做二进制判定，再尝试替换（用于全树替换）。
fn replace_entry(path: &Path, old: &str, new: &str) -> Result<ReplaceStatus, CreateError> {
    let is_text = is_probably_text(path)
        .map_err(|source| io_error(format!("读取 {}", path.display()), source))?;
    if !is_text {
        return Ok(ReplaceStatus::SkippedBinary);
    }
    replace_text(path, old, new)
}

/// 读取并替换单文件内容（无二进制判定，调用方自行决定是否预判）。
fn replace_text(path: &Path, old: &str, new: &str) -> Result<ReplaceStatus, CreateError> {
    let bytes =
        fs::read(path).map_err(|source| io_error(format!("读取 {}", path.display()), source))?;
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => return Ok(ReplaceStatus::SkippedNonUtf8),
    };
    if !text.contains(old) {
        return Ok(ReplaceStatus::NotFound);
    }
    fs::write(path, text.replace(old, new))
        .map_err(|source| io_error(format!("写入 {}", path.display()), source))?;
    Ok(ReplaceStatus::Replaced)
}

/// 判定文件是否可能为文本：二进制后缀、超限大小、含 NUL 字节均判定为二进制。
fn is_probably_text(path: &Path) -> io::Result<bool> {
    if let Some(suffix) = path.extension().and_then(|s| s.to_str()) {
        if BINARY_SUFFIXES.contains(&suffix) {
            return Ok(false);
        }
    }
    if fs::metadata(path)?.len() > MAX_TEXT_FILE {
        return Ok(false);
    }
    let bytes = fs::read(path)?;
    Ok(!bytes.contains(&0u8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, content: impl AsRef<[u8]>) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn copies_tree_and_skips_ignored_entries() {
        let tmp = tempdir().unwrap();
        let src = tmp.path().join("src");
        write(&src.join("pkg/a.txt"), "myapp");
        write(&src.join("node_modules/leak.txt"), "myapp");
        write(&src.join(".coverage"), "myapp");
        write(&src.join("pkg/.git/the-rest"), "myapp");

        let dest = tmp.path().join("dest");
        StdFs.copy_tree(&src, &dest).unwrap();

        assert_eq!(fs::read_to_string(dest.join("pkg/a.txt")).unwrap(), "myapp");
        assert!(!dest.join("node_modules").exists());
        assert!(!dest.join(".coverage").exists());
        assert!(!dest.join("pkg/.git").exists());
    }

    #[test]
    fn replaces_all_text_and_counts() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();
        write(&root.join("a.txt"), "myapp and myapp");
        write(&root.join("b.md"), "no match");
        write(&root.join("c.bin"), [0u8, 1, 2, b'm', 0u8, b'a']);
        write(&root.join("d.txt"), [0xFFu8, b'X']);

        let outcome = StdFs.replace_all(root, "myapp", "newapp").unwrap();

        assert_eq!(outcome.replaced_files, 1);
        assert_eq!(
            fs::read_to_string(root.join("a.txt")).unwrap(),
            "newapp and newapp"
        );
        assert_eq!(fs::read_to_string(root.join("b.md")).unwrap(), "no match");
        assert_eq!(outcome.skipped_non_utf8, vec![root.join("d.txt")]);
    }

    #[test]
    fn replace_in_file_reports_status() {
        let tmp = tempdir().unwrap();
        let target = tmp.path().join("x.txt");
        write(&target, "name = \"myapp\"");

        assert_eq!(
            StdFs
                .replace_in_file(&target, "name = \"myapp\"", "name = \"my-app\"")
                .unwrap(),
            ReplaceStatus::Replaced
        );
        assert_eq!(fs::read_to_string(&target).unwrap(), "name = \"my-app\"");
        assert_eq!(
            StdFs.replace_in_file(&target, "absent", "x").unwrap(),
            ReplaceStatus::NotFound
        );
        // 未匹配时不产生写入：内容保持不变
        assert_eq!(fs::read_to_string(&target).unwrap(), "name = \"my-app\"");

        let binary = tmp.path().join("y.bin");
        write(&binary, [0xFFu8]);
        assert_eq!(
            StdFs.replace_in_file(&binary, "x", "y").unwrap(),
            ReplaceStatus::SkippedNonUtf8
        );
    }
}
