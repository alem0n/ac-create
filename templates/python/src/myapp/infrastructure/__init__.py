"""infrastructure 层：外部交互实现。

高内聚低耦合契约：
- 封装数据库/HTTP/文件系统等外部依赖。
- 对外暴露抽象接口供 services 层依赖（依赖倒置）。
- 将外部异常转译为领域异常。禁止原始库异常泄漏到上层。
"""

# 模板占位：实际项目在此定义 Repository / Gateway 等接口与实现。
# 在 main.py 中完成对 services 层的注入。
