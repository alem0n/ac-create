"""myapp：项目根包。

分层架构（依赖方向单向指向 domain，详见上级 AGENTS.md 第 6 节）：

    domain          领域模型与核心规则（无外部依赖）
      ↑
    services        用例编排（只依赖 domain 与抽象接口）
      ↑
    infrastructure  外部交互实现（数据库/HTTP/文件系统）

组合层（main / config / logging_config）负责依赖注入与初始化。
"""

__version__ = "0.1.0"
