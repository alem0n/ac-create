//! 集成测试：覆盖公共 API 的端到端路径。
//!
//! 约定：集成测试置于 tests/ 目录，代码不进入 src。
//! 变更纪律：公共 API 变更必须同步更新本文件对应测试。

use ac_create::{greet, AppConfig, Greeting};

#[test]
fn public_api_greet_end_to_end() {
    // 断言行为而非复述实现
    assert_eq!(greet("world").unwrap(), "Hello, world!");
    assert!(greet("").is_err());
}

#[test]
fn services_output_matches_domain_rule() {
    // 跨层一致性：services 输出等于 domain 规则结果
    assert_eq!(
        greet("agent").unwrap(),
        Greeting::new("agent").unwrap().render()
    );
}

#[test]
fn config_loads_with_defaults() {
    let config = AppConfig::load();
    assert_eq!(config.name, "world");
    assert_eq!(config.log_level, "info");
}
