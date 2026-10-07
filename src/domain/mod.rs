//! domain 层：领域模型与核心规则。无外部依赖、无 IO。
//!
//! 职责契约：
//! - 定义值对象（ProjectName）、领域错误（CreateError）与语言规则（Language）。
//! - 定义用例依赖的抽象端口（FileSystem / Runner / Reporter）。
//! - 不得导入 services / infrastructure 或任何 IO 库。
//! - 端口定义放在本层（依赖倒置：基础设施层实现，组合层注入）。

pub mod error;
pub mod language;
pub mod ports;
pub mod project_name;

pub use error::CreateError;
pub use language::Language;
pub use ports::{FileSystem, ReplaceOutcome, ReplaceStatus, Reporter, RunResult, Runner};
pub use project_name::{ProjectName, LICENSE_PLACEHOLDER, PLACEHOLDER};
