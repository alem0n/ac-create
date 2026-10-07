"""services 层：用例编排。

高内聚低耦合契约：
- 只可导入 domain 层与抽象接口。
- 不可导入 infrastructure 层的具体实现（通过依赖倒置交互）。
- 不可直接访问外部 IO（数据库/HTTP/文件系统）。
"""

from myapp.domain.models import build_greeting


def greet(subject: str) -> str:
    """用例：构建并渲染问候语。"""
    greeting = build_greeting(subject)
    return greeting.render()
