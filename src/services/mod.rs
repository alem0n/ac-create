//! services 层：用例编排。
//!
//! 高内聚低耦合契约：
//! - 只依赖 domain 层与抽象端口（FileSystem / Runner / Reporter）。
//! - 不得导入 infrastructure 层的具体实现（通过端口依赖倒置交互）。
//! - 不直接访问外部 IO（数据库 / HTTP / 文件系统 / 子进程），一切经端口委托。

pub mod create;

pub use create::{create, CreateRequest};
