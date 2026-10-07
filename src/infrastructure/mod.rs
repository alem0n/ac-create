//! infrastructure 层：外部交互实现。
//!
//! 职责契约：
//! - 封装文件系统、子进程与模板定位等外部依赖。
//! - 实现 domain 层定义的抽象端口（FileSystem / Runner）。
//! - 将外部错误（IO / 进程）附上下文转译为领域错误 CreateError。禁止原始库错误泄漏到上层。
//! - 模块内实现细节默认私有，公共项经 lib.rs 收口导出。

pub mod fs;
pub mod process;
pub mod templates;

pub use fs::StdFs;
pub use process::StdRunner;
pub use templates::locate_templates;
