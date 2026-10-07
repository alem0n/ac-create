# SPEC — myapp

> 记录项目意图（what/why）与需求原子清单。非平凡变更先在此记录意图。实现产生的新决策必须回写本文件。
> 以下问候示例为模板占位。替换为实际领域概念时同步重写本文件。

## 意图

提供项目意图的单一来源。spec 定义需求。测试验证需求。代码实现需求。三者随演进保持同步（SDD 三角）。

## 需求清单

### REQ-001 问候语渲染

- 输入非空 subject，`greet(subject)` 返回 `Hello, {subject}!`。
- services 层输出与 domain 层 `Greeting.render()` 规则一致。

### REQ-002 问候值对象构造

- `buildGreeting(subject)` 构造 `Greeting`。
- `Greeting.render()` 返回 `Hello, {subject}!`。

### REQ-003 配置默认值

- 环境变量 `APP_NAME` 未设置时，配置 `name` 默认为 `world`。
- 环境变量 `LOG_LEVEL` 未设置时，配置 `logLevel` 默认为 `info`。

### REQ-004 应用入口

- `main()` 配置日志后生成问候语。
- `main()` 返回退出码 0。

## 设计决策

- 问候逻辑分层在 domain（规则）与 services（编排）。详见 AGENTS.md 第 6 节。
- 配置缺省值显式声明，不依赖环境变量隐式行为。
