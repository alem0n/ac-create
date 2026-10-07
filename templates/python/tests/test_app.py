"""示例测试：覆盖各层占位行为。

变更纪律：新增/修改任何源码，必须同步新增/更新测试。
"""

import logging
from typing import Any

from myapp import config, logging_config, main
from myapp.domain.models import build_greeting
from myapp.services import greet


def test_config_load_defaults() -> None:
    app_config = config.load_config()
    assert app_config.name == "world"
    assert app_config.log_level == "INFO"


def test_domain_greeting_render() -> None:
    greeting = build_greeting("world")
    assert greeting.render() == "Hello, world!"


def test_service_greet() -> None:
    assert greet("world") == "Hello, world!"


def test_service_greet_uses_domain_rule() -> None:
    """断言行为而非复述实现：services 层输出应与 domain 规则一致。"""
    assert greet("agent") == build_greeting("agent").render()


def test_logging_config_sets_level() -> None:
    logging_config.configure_logging("DEBUG")
    assert logging.getLogger().level == logging.DEBUG


def test_main_returns_zero_and_greets(capsys: Any) -> None:
    assert main.main() == 0
    assert "Hello, world!" in capsys.readouterr().out
