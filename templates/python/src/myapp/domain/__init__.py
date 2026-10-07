"""domain 层：领域模型与核心规则。

高内聚低耦合契约：
- 本层不可导入 services / infrastructure。
- 本层不可访问任何外部 IO。
- 领域规则只依赖自身与其他 domain 模块。
- 本层被 services 层依赖，但不依赖任何上层。
"""

from myapp.domain.models import Greeting

__all__ = ["Greeting"]
