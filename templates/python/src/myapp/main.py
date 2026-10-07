"""入口点：只做组装与初始化，不承载业务逻辑。

职责契约（高内聚低耦合）：
- 读取配置。
- 初始化日志。
- 完成依赖注入。
- 启动应用。
- 禁止在此编写业务规则。业务规则属于 services 层。
"""

import logging

from myapp import config, logging_config
from myapp.services import greet

logger = logging.getLogger(__name__)


def main() -> int:
    """应用入口：返回进程退出码。"""
    app_config = config.load_config()
    logging_config.configure_logging(app_config.log_level)

    logger.info("app.start", extra={"log_level": app_config.log_level})
    greeting = greet(app_config.name)
    logger.info("app.done", extra={"greeting": greeting})
    # 仅入口允许面向用户输出。业务模块一律走 logging。
    print(greeting)  # noqa: T201
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
