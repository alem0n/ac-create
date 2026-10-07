"""配置层：统一加载环境变量，供组合层注入。

职责契约：仅做配置读取与校验。不包含业务逻辑。不访问外部服务。
"""

import os
from dataclasses import dataclass


@dataclass(frozen=True)
class AppConfig:
    """应用配置（不可变，避免运行期被意外修改）。"""

    name: str
    log_level: str


def load_config() -> AppConfig:
    """从环境变量加载配置，缺省值需显式声明。"""
    return AppConfig(
        name=os.getenv("APP_NAME", "world"),
        log_level=os.getenv("LOG_LEVEL", "INFO"),
    )
