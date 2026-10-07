//! 日志模块：唯一允许初始化 tracing subscriber 的地方。
//!
//! 职责契约：业务模块一律使用 `tracing` 宏记录日志。
//! 禁止自行初始化 subscriber 或使用 println!/dbg!。

use tracing_subscriber::{fmt, EnvFilter};

use crate::config::AppConfig;

/// 初始化 tracing subscriber。仅在入口 main.rs 调用一次。
pub fn init(config: &AppConfig) {
    let filter = EnvFilter::try_new(&config.log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    fmt().with_env_filter(filter).with_target(false).init();
}
