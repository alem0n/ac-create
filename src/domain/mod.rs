//! domain 层：领域模型与核心规则。
//!
//! 高内聚低耦合契约：
//! - 本层不得导入 services / infrastructure。
//! - 本层不得访问任何外部 IO。
//! - 本层被 services 层依赖，但不依赖任何上层。

pub mod models;

pub use models::{DomainError, Greeting};
