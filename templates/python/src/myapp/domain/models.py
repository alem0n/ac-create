"""领域模型示例（模板占位，替换为实际领域概念）。"""


class Greeting:
    """问候值对象：封装问候语的构成规则。"""

    def __init__(self, subject: str) -> None:
        self._subject = subject

    def render(self) -> str:
        """渲染为最终问候文本。"""
        return f"Hello, {self._subject}!"


def build_greeting(subject: str) -> Greeting:
    """工厂函数：领域对象的构造入口。"""
    return Greeting(subject)
