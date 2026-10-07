//! 配置层：统一加载环境变量，供组合层注入。
//!
//! 职责契约：仅做配置读取与默认值声明。不包含业务逻辑。不访问外部服务。

use std::env;

/// 应用配置（构造后不可变）。
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub log_level: String,
}

impl AppConfig {
    /// 从环境变量加载，缺省值须显式声明。
    /// 默认 warn：CLI 的用户可读输出由 Reporter 负责，诊断日志须经 RUST_LOG 显式开启。
    pub fn load() -> Self {
        Self {
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_defaults() {
        let config = AppConfig::load();
        assert_eq!(config.log_level, "warn");
    }
}
