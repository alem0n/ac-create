//! 入口点：只做组装与初始化，不承载业务逻辑。
//!
//! 职责契约（高内聚低耦合）：
//! - 读取配置。
//! - 初始化日志（tracing subscriber）。
//! - 调用 services 层。
//! - 禁止在此编写业务规则。业务规则属于 services 层。

// 入口点显式豁免 print_stdout：仅允许入口向 stdout 输出面向用户的结果。
// 业务模块（lib crate）一律禁止 println!/dbg!。见 lib.rs 声明。
#![allow(clippy::print_stdout)]

use myapp::logger;
use myapp::{greet, AppConfig};

fn main() {
    let config = AppConfig::load();
    logger::init(&config);

    tracing::info!(log_level = %config.log_level, "app.start");

    match greet(&config.name) {
        Ok(greeting) => {
            tracing::info!(greeting = %greeting, "app.done");
            println!("{greeting}");
        }
        Err(err) => {
            tracing::error!(error = %err, "app.failed");
            std::process::exit(1);
        }
    }
}
