# SPEC — myapp

> 记录项目意图（what/why）与需求原子清单。非平凡变更先在此记录意图。实现产生的新决策必须回写本文件。
> 以下问候示例为模板占位。替换为实际领域概念时同步重写本文件。

## 意图

提供项目意图的单一来源。spec 定义需求。测试验证需求。代码实现需求。三者随演进保持同步（SDD 三角）。

## 需求清单

### REQ-001 问候语渲染

- 输入非空 subject，`greet(subject)` 返回 `Hello, {trimmed}!`。
- `Greeting::new` 去除 subject 前后空白后渲染。
- services 层输出与 domain 层 `Greeting::render()` 规则一致。

### REQ-002 问候值对象不变量

- `Greeting::new("")` 与 `Greeting::new("   ")` 返回 `Err(DomainError::EmptySubject)`。
- 错误信息为完整句子：`The subject must not be empty.`
- 构造失败不 panic。

### REQ-003 配置默认值

- 环境变量 `APP_NAME` 未设置时，配置 `name` 默认为 `world`。
- 环境变量 `RUST_LOG` 未设置时，配置 `log_level` 默认为 `info`。

## 设计决策

- 问候逻辑分层在 domain（规则）与 services（编排）。详见 AGENTS.md 第 6 节。
- 空主体是领域错误而非 panic。错误经 `Result` 显式传播。
- 配置缺省值显式声明，不依赖环境变量隐式行为。
- 二进制入口 `main.rs` 属组合层，不纳入需求清单。其行为由 AGENTS.md 第 6 节与模块文档约束。
