//! services 层：用例编排。
//!
//! 高内聚低耦合契约：
//! - 只可导入 domain 层与抽象 trait。
//! - 不得导入 infrastructure 层的具体实现（通过 trait 依赖倒置交互）。
//! - 不得直接访问外部 IO（数据库/HTTP/文件系统）。

use crate::domain::{DomainError, Greeting};

/// 用例：构建并渲染问候语。
pub fn greet(subject: &str) -> Result<String, DomainError> {
    let greeting = Greeting::new(subject)?;
    Ok(greeting.render())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greet("world").unwrap(), "Hello, world!");
    }

    #[test]
    fn propagates_domain_error() {
        assert!(greet("").is_err());
    }
}
