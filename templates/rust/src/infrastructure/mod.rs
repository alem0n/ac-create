//! infrastructure 层：外部交互实现。
//!
//! 高内聚低耦合契约：
//! - 封装数据库/HTTP/文件系统等外部依赖。
//! - 对外暴露抽象 trait 供 services 层依赖（依赖倒置）。
//! - 将外部错误（IO/解析等）转译为领域错误。禁止原始库错误泄漏到上层。
//!
//! 模板占位：实际项目在此定义 Repository / Gateway 等 trait 与实现。在 main.rs 中完成对 services 层的注入。

// 模板占位：随项目演进出 trait 与具体实现。
