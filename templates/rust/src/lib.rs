//! myapp：项目库根。
//!
//! 分层架构（依赖方向单向指向 domain，详见上级 AGENTS.md 第 6 节）：
//!
//! ```text
//! domain          领域模型与核心规则（无外部依赖）
//!   ^
//! services        用例编排（只依赖 domain 与抽象 trait）
//!   ^
//! infrastructure  外部交互实现（数据库/HTTP/文件系统）
//! ```
//!
//! 组合层（main / config / logger）负责依赖注入与初始化。

// 日志规范与错误处理规范的编译期强制（机器可校验）：
// - print_stdout / dbg_macro：禁止业务代码使用 println!/dbg!
// - unwrap_used：禁止生产路径 unwrap()/expect()（测试代码见下方 cfg(test) 豁免）
#![warn(clippy::print_stdout, clippy::dbg_macro, clippy::unwrap_used)]
// 测试代码豁免 unwrap_used：单测中 unwrap 是惯用法，生产路径仍禁止
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod logger;
pub mod services;

// 公共 API 有意识地收口导出（信息隐藏：模块内部实现细节默认私有）
pub use config::AppConfig;
pub use domain::{DomainError, Greeting};
pub use services::greet;
