//! 子进程端口实现：捕获输出、超时终止、缺失命令转译。
//!
//! 行为对齐 Python 版 subprocess.run：
//! - 命令缺失（PATH 未找到）→ 失败结果并提示「命令不存在：{program}」。
//! - 超时（默认 600 秒）→ 终止子进程并提示「超时：{command}」。
//! - 结果合并 stdout 与 stderr（trim 后供上层展示）。
//! - Windows 额外探测 .cmd / .bat：CreateProcess 不查 PATHEXT，
//!   npm 类脚本分发为 .cmd，需显式回退才能与 Python 行为一致。

use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::domain::ports::{RunResult, Runner};

/// 默认超时时长（与 Python 版一致）。
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(600);

/// 等待子进程退出时的轮询间隔。
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// 子进程端口的标准实现（std::process）。
pub struct StdRunner {
    timeout: Duration,
}

impl Default for StdRunner {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl StdRunner {
    /// 以指定超时时长构造（测试用）。
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl Runner for StdRunner {
    fn run(&self, program: &str, args: &[&str], cwd: &Path) -> RunResult {
        let mut child = match build_command(program, args, cwd).spawn() {
            Ok(child) => child,
            Err(_) => match spawn_with_extensions(program, args, cwd) {
                Some(child) => child,
                None => return RunResult::failed(format!("命令不存在：{program}")),
            },
        };

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let stdout_handle = thread::spawn(move || drain(stdout));
        let stderr_handle = thread::spawn(move || drain(stderr));

        let started = Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {
                    if started.elapsed() >= self.timeout {
                        let _ = child.kill();
                        let _ = child.wait();
                        let display = if args.is_empty() {
                            program.to_string()
                        } else {
                            format!("{} {}", program, args.join(" "))
                        };
                        return RunResult::failed(format!("超时：{display}"));
                    }
                    thread::sleep(POLL_INTERVAL);
                }
                Err(_) => return RunResult::failed(format!("命令运行失败：{program}")),
            }
        };

        let stdout_bytes = stdout_handle.join().unwrap_or_default();
        let stderr_bytes = stderr_handle.join().unwrap_or_default();
        let output = format!(
            "{}{}",
            String::from_utf8_lossy(&stdout_bytes),
            String::from_utf8_lossy(&stderr_bytes)
        );
        RunResult {
            success: status.success(),
            output: output.trim().to_string(),
        }
    }
}

fn build_command(program: &str, args: &[&str], cwd: &Path) -> Command {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

/// 原始名 spawn 失败时，在 Windows 上探测 .cmd / .bat（CreateProcess 忽略 PATHEXT）。
#[cfg(windows)]
fn spawn_with_extensions(program: &str, args: &[&str], cwd: &Path) -> Option<Child> {
    for extension in ["cmd", "bat"] {
        let probed = format!("{program}.{extension}");
        if let Ok(child) = build_command(&probed, args, cwd).spawn() {
            return Some(child);
        }
    }
    None
}

#[cfg(not(windows))]
fn spawn_with_extensions(_program: &str, _args: &[&str], _cwd: &Path) -> Option<Child> {
    None
}

fn drain(mut reader: Option<impl Read>) -> Vec<u8> {
    let mut bytes = Vec::new();
    if let Some(handle) = reader.as_mut() {
        let _ = handle.read_to_end(&mut bytes);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_command_reports_not_found() {
        let runner = StdRunner::default();
        let result = runner.run("ac-create-no-such-binary", &[], Path::new("."));
        assert!(!result.success);
        assert!(result.output.contains("命令不存在"));
    }

    #[test]
    fn success_returns_combined_output() {
        let runner = StdRunner::default();
        let result = runner.run("git", &["--version"], Path::new("."));
        assert!(
            result.success,
            "git --version 应成功，输出：{}",
            result.output
        );
        assert!(result.output.contains("git version"));
    }

    #[test]
    #[cfg(unix)]
    fn timeout_kills_long_running_command() {
        let runner = StdRunner::new(Duration::from_millis(100));
        let result = runner.run("sleep", &["30"], Path::new("."));
        assert!(!result.success);
        assert!(result.output.contains("超时"));
    }
}
