//! ac_create：项目模板 CLI 库根。
//!
//! 分层架构（依赖方向单向指向 domain，见上级 AGENTS.md 第 6 节）：
//!
//! ```text
//! domain          领域模型与核心规则（无外部依赖；含用例端口定义）
//!   ^
//! services        用例编排（只依赖 domain 与抽象端口）
//!   ^
//! infrastructure  外部交互实现（文件系统 / 子进程 / 模板定位）
//! ```
//!
//! 组合层（main / config / logger）负责依赖注入与初始化，不承载业务逻辑。

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
pub use domain::{
    CreateError, FileSystem, Language, ProjectName, ReplaceOutcome, ReplaceStatus, Reporter,
    RunResult, Runner, LICENSE_PLACEHOLDER, PLACEHOLDER,
};
pub use infrastructure::{locate_templates, StdFs, StdRunner};
pub use services::{create, CreateRequest};
