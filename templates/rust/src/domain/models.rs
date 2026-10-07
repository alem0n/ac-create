//! 领域模型示例（模板占位，替换为实际领域概念）。

use thiserror::Error;

/// 领域错误类型（错误处理分层：domain 层的错误语义从这里定义）。
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("The subject must not be empty.")]
    EmptySubject,
}

/// 问候值对象：封装问候语的构成规则。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeting {
    subject: String,
}

impl Greeting {
    /// 构造函数：校验不变量。失败时返回领域错误，不 panic。
    pub fn new(subject: &str) -> Result<Self, DomainError> {
        if subject.trim().is_empty() {
            return Err(DomainError::EmptySubject);
        }
        Ok(Self {
            subject: subject.trim().to_string(),
        })
    }

    /// 渲染为最终问候文本。
    pub fn render(&self) -> String {
        format!("Hello, {}!", self.subject)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_greeting() {
        let greeting = Greeting::new("world").unwrap();
        assert_eq!(greeting.render(), "Hello, world!");
    }

    #[test]
    fn rejects_empty_subject() {
        let err = Greeting::new("").unwrap_err();
        assert_eq!(err.to_string(), "The subject must not be empty.");
        assert!(Greeting::new("   ").is_err());
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let greeting = Greeting::new(" world ").unwrap();
        assert_eq!(greeting.subject, "world");
    }
}
