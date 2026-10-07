"""日志配置：唯一允许配置 logging 的地方。

职责契约：集中管理 level/handler/format。业务模块一律通过
``logging.getLogger(__name__)`` 获取 logger。禁止自行 basicConfig。
"""

import logging
import sys


def configure_logging(level: str = "INFO") -> None:
    """初始化根 logger。仅在入口点调用一次。"""
    handlers = [logging.StreamHandler(sys.stderr)]

    logging.basicConfig(
        level=level.upper(),
        format='{"time": "%(asctime)s", "level": "%(levelname)s", '
        '"logger": "%(name)s", "message": "%(message)s"}',
        handlers=handlers,
        force=True,
    )
