//! 项目名值对象：封装命名规则校验。
//!
//! 规则：首字符为小写字母；其余为小写字母、数字、连字符或下划线。
//! 依据：三语言派生的标识符（Python 包名、Rust crate 名、npm 包名）均须合法。
//! 首字符限定为字母可避免 1app 类名字。

use std::fmt;
use std::str::FromStr;

use crate::domain::error::CreateError;

/// 模板内统一使用的占位名。创建时整体替换为实际包名。
pub const PLACEHOLDER: &str = "myapp";

/// LICENSE 中的版权持有者占位文本。
pub const LICENSE_PLACEHOLDER: &str = "<替换为实际版权持有者>";

/// 已通过校验的项目名。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    /// 解析并校验项目名。非法输入返回领域错误，不 panic。
    pub fn parse(raw: &str) -> Result<Self, CreateError> {
        validate(raw)?;
        Ok(Self(raw.to_string()))
    }

    /// 原始输入（已通过校验）。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ProjectName {
    type Err = CreateError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::parse(raw)
    }
}

fn validate(raw: &str) -> Result<(), CreateError> {
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return Err(CreateError::InvalidProjectName);
    };
    let rest_valid =
        chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    if !first.is_ascii_lowercase() || !rest_valid {
        return Err(CreateError::InvalidProjectName);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_names() {
        for raw in ["my-app", "my_app", "myapp2", "a", "a1_2-3"] {
            let name = ProjectName::parse(raw).expect("合法名应被接受");
            assert_eq!(name.as_str(), raw);
        }
    }

    #[test]
    fn rejects_invalid_names() {
        for raw in [
            "", "1app", "-app", "_app", "MyApp", "my app", "my.app", "中文", "myapp\n",
        ] {
            assert!(
                matches!(
                    ProjectName::parse(raw),
                    Err(CreateError::InvalidProjectName)
                ),
                "非法名应被拒绝：{raw}"
            );
        }
    }
}
