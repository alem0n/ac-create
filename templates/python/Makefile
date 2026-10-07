# Python 模板质量门禁：统一入口
# 使用：make check（提交前必跑，CI 引用同一入口）
# Python 操作一律经 uv 在虚拟环境中执行（见上级 AGENTS.md / 项目规范）

.PHONY: check precommit test lint typecheck fmt audit coverage clean

PYTHON := uv run

check: lint typecheck fmt test audit  ## 一次性质量门禁：lint + 类型 + 格式化 + 测试 + 审计
precommit: lint typecheck fmt test  ## pre-commit 快速子集（依赖审计留 CI，避免提交卡顿）
test:  ## 运行 pytest（覆盖率门禁经 pyproject.toml addopts 注入）
	$(PYTHON) pytest
lint:  ## ruff 规则检查（含 T20 禁 print、S 安全规则）
	$(PYTHON) ruff check .
typecheck:  ## mypy 严格类型检查
	$(PYTHON) mypy src/
fmt:  ## 格式化检查
	$(PYTHON) ruff format --check .
audit:  ## 依赖漏洞审计（pip-audit 为 dev 依赖）
	$(PYTHON) pip-audit
layer-contracts:  ## 分层依赖契约校验（import-linter）
	$(PYTHON) lint-imports
coverage:  ## 附加 target：生成本地覆盖率报告
	$(PYTHON) pytest --cov-report=html
clean:
	rm -rf .pytest_cache .mypy_cache .ruff_cache htmlcov .coverage
	find . -type d -name __pycache__ -exec rm -rf {} +
