# Rust 模板质量门禁：统一入口
# 使用：make check（提交前必跑，CI 引用同一入口）

.PHONY: check precommit build test fmt fmt-check clippy audit coverage clean

# 一次性自举：cargo audit 为外部子命令，非 cargo 内建
#   cargo install cargo-audit
# 附加可选：cargo tarpaulin（覆盖率，非默认链）
#   cargo install cargo-tarpaulin

check: fmt-check clippy test audit  ## 一次性质量门禁：fmt + clippy + test + audit
precommit: fmt-check clippy test  ## pre-commit 快速子集（依赖审计留 CI，避免提交卡顿）

build:  ## 构建
	cargo build

test:  ## 运行全部测试（同文件单元 + tests/ 集成）
	cargo test

fmt:  ## 格式化
	cargo fmt

fmt-check:  ## 格式化检查
	cargo fmt --check

clippy:  ## Clippy 零警告（-D warnings，含测试目标）
	cargo clippy --all-targets -- -D warnings

audit:  ## 依赖漏洞审计（须先 cargo install cargo-audit）
	cargo audit

coverage:  ## 附加 target：覆盖率（须先 cargo install cargo-tarpaulin）
	cargo tarpaulin --fail-under 80

clean:
	cargo clean
