# AGENTS.md — Rust 项目模板

> 给开发者和 AI coding agent 的共同指令文件（AGENTS.md 开放标准）。
> 本文件为规范唯一入口。各规范的**可执行细节**落在对应配置文件中。按需查阅，不必通读。

## 1. 项目概览

- Rust 项目骨架：单 crate，`src/main.rs`（组装）+ `src/lib.rs`（公共 API 有意识导出）+ 按职责划分的模块。大型化时拆分为 Cargo workspace（保留扩展路径）。
- 分层架构：`domain`（领域模型，无外部依赖）→ `services`（用例编排）→ `infrastructure`（外部交互），依赖方向单向指向 `domain`。
- `main.rs` 只做组装（配置/日志初始化/依赖注入），不承载业务逻辑。

## 2. 环境与命令（本节命令会被 agent 主动执行）

> 统一门禁入口：`make check`。以下所有检查均收口于该命令。CI 引用同一入口。提交前自动检查点运行其快速子集（见下条）。

| 操作 | 命令 |
|---|---|
| 构建 | `cargo build` |
| 开发模式运行 | `cargo run` |
| 运行测试 | `cargo test` |
| 运行单个测试 | `cargo test <test_name>` |
| 格式化 | `cargo fmt`（检查：`cargo fmt --check`） |
| Lint（含惯用法与复杂度） | `cargo clippy --all-targets -- -D warnings` |
| 依赖漏洞审计 | `cargo audit`（外部子命令，见下） |
| **完整质量门禁** | `make check`（fmt 检查 + clippy + test + audit，任一失败即终止） |

`make check` 等价于：

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test && cargo audit
```

- **cargo-audit 自举**：`cargo audit` 不是 cargo 内建命令。须一次性安装：`cargo install cargo-audit`（bootstrap 说明已在 Makefile 注释中）。首次运行 `make check` 前执行一次。
- 依赖锁定：提交 `Cargo.lock`（可复现构建）。新增依赖时同步更新锁定。
- 五层质量门禁栈中，模板覆盖前四层。覆盖率工具 cargo-tarpaulin 为外部可选工具，作为附加 target（`make coverage`，须先 `cargo install cargo-tarpaulin`）。
- Rust 版本以 `rust-toolchain.toml`（如使用）声明的稳定版为准。
- 提交检查点：首次克隆后执行一次 `git config core.hooksPath .githooks`。此后 `git commit` 前自动运行 `make precommit`（快速子集。依赖审计留 CI。）。未通过则提交被拦截。
- 自动化优先以脚本/CLI 组合完成（Makefile 与配置文件），不为本项目引入额外 MCP 工具或自定义工具定义。确定性脚本优先于 LLM 判断。
- 提交规范（Conventional Commits）：<type>(scope)?: <description>。type ∈ feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert。`feat` 对应 SemVer MINOR，`fix` 对应 PATCH，`!` 或 `BREAKING CHANGE` 对应 MAJOR。commit-msg 钩子强制校验。
- 分支策略：默认 GitHub Flow。main 永远可部署。feature 分支短命化（≤2 天），命名 `feat/xxx`。多版本并存或受监管行业时演进 GitFlow。
- CI：`.github/workflows/ci.yml` 执行 `make check`。

## 3. 代码风格

- 格式化：`cargo fmt`（配置可选见 `rustfmt.toml`）。
- Clippy 零警告（`-D warnings`）。复杂度类 lint（如 `cognitive_complexity`、`module_inception`）纳入配置。
- 命名：函数/变量/模块使用 `snake_case`。类型/Trait 使用 `PascalCase`。
- 公共 API 须有文档注释（`///`）。`cargo doc` 不应产生警告。
- 注释默认不写。仅当 WHY 非显然时写注释（隐藏约束、微妙不变量、特定 bug 的变通、令读者意外的行为）。模块头 docstring 例外（记录分层契约与职责，见第 6 节）。

## 4. 日志规范

- **框架**：`tracing` + `tracing-subscriber`（tokio 生态事实标准，支持结构化字段与 span）。
- **强制规则**（可机器校验）：

  1. 禁止在业务代码中使用 `println!`/`print!`/`dbg!`。由 clippy restriction lint `print_stdout`、`dbg_macro` 拦截。在 `src/lib.rs` 顶部以 `#![warn(...)]` 启用。
  2. 结构化字段：日志必须附带领域键（操作名、实体 id、错误上下文）。示例：`tracing::info!(name = %c.name, "app.start")`。
  3. 日志订阅者（subscriber）只在入口 `main.rs` 经 `logger.rs` 初始化一次。业务模块使用 `tracing` 宏。禁止自行初始化 subscriber。
  4. 错误须以 `error!` 记录且携带上下文字段（与第 7 节错误处理配合）。

- 参考实现：`src/logger.rs`。

## 5. 测试框架规范

- **框架**：内建 `#[test]` + `cargo test`。
- 布局约定：单元测试与源码同文件（`#[cfg(test)] mod tests`）。集成测试置于 `tests/` 目录。
- 变更纪律：**任何代码变更必须附带或更新对应测试**。覆盖率闭合依赖可选的 cargo-tarpaulin（见第 2 节）。集成测试至少覆盖一个公共 API 的端到端路径。
- 规范同步：非平凡变更先在 `docs/SPEC.md` 记录意图（what/why）。实现产生的新决策必须回写 SPEC（实现改进规范）。
- 测试须覆盖 `docs/SPEC.md` 中的需求，而不仅是代码行覆盖率。每个公共行为至少一个端到端测试。
- 测试断言公共行为契约，不测内部实现细节。重建或重实现时行为测试不变。
- 测试必须独立可运行。禁止依赖执行顺序与共享可变状态（并发测试须用独立数据）。
- 外部交互（IO/网络）一律在 `infrastructure` 层做 trait 抽象，单测用假实现替换。

## 6. 高内聚低耦合要求

- 分层目录（模板默认结构，详见各模块文件头注释）：

  - `src/domain/`：领域模型与核心规则。**不得**导入 services/infrastructure 或任何 IO 库。
  - `src/services/`：用例编排，只依赖 `domain` 与抽象 trait。
  - `src/infrastructure/`：外部交互（数据库/HTTP/文件系统），实现抽象 trait。
  - `src/main.rs` / `config.rs` / `logger.rs`：组合层，负责依赖注入与初始化，不承载业务逻辑。

- **可机器校验（Rust 独有的编译期约束）**：
  - Rust 的模块可见性即依赖约束：`pub` 项只经 `lib.rs` 有意识地收口导出，模块内部的实现细节默认私有。跨层耦合在编译期即被限制。
  - 模块可见性 + clippy 复杂度类 lint 构成自动校验。分层目录约定本身为审查检查项。
  - 升级到 Cargo workspace 后，各层拆为独立 crate。`pub use` 边界即层级契约（编译器强制执行）。
- 禁止 `utils.rs` 式无内聚杂物模块。函数按职责归入对应层。
- **并行任务边界**：分层边界同时是多 agent 并行开发的任务边界。单个 agent 任务不应跨层。必须跨层的变更须显式声明并人工协调。

## 7. 错误处理

- 错误类型分层：`domain` 定义领域错误类型（推荐 `thiserror` 派生枚举）。`infrastructure` 将外部错误（IO/解析等）转译为领域错误。禁止把原始库错误 `?` 直接泄漏到 `services`/`domain`。
- 禁止吞错误：`Result` 必须显式处理。clippy restriction lint `unwrap_used` 拦截生产路径上的 `unwrap()`/`expect()`（测试代码豁免，见 `lib.rs` 中的 `cfg_attr` 配置说明）。
- 错误在层级边界向上传播时须附上下文（`map_err` + `thiserror` 的 `#[context]` 或 `Context` 习惯用法）。出错即记日志。

## 8. 安全注意事项

- 密钥/环境变量**不得硬编码**。开发环境统一走 `.env`。CI/生产环境走环境变量注入。`.env` 已在 `.gitignore` 中，禁止提交。
- `.env.example` 仅放占位符。`.env.example` 不得含真实凭据。
- 依赖漏洞扫描由 `cargo audit` 提供。该扫描纳入 `make check`。
- 密钥扫描：gitleaks。安装后由 pre-commit 自动调用（未安装则跳过并提示）。安装方式见 gitleaks 官方仓库。误提交的密钥必须轮换。清理历史不能替代轮换。
- 禁止 `unsafe` 块。除非有充分文档说明的安全论证，不得使用 `unsafe`（代码审查重点项）。

## 9. 人在环路与审查纪律

- 实现与其测试**不得仅由同一 agent 会话自证通过**：必须通过 `make check` 独立确定性门禁全绿，或经人工复核。
- 触发阈值：变更触及 3 个及以上文件、或涉及 infrastructure 层、或变更公共 API 时，报告完成前必须经过独立对抗性验证（另一 agent 会话或人工复核）。
- 不可逆变更（删除数据/迁移/发布）须经人类批准后方可执行。
- AI 生成的测试可能与实现共享同一误解。重点审查测试是否真正断言了行为，而非复述实现。

## 10. 提交前检查清单

- [ ] `make check` 全绿（含 `cargo fmt --check`、`cargo clippy -- -D warnings`）
- [ ] 变更已附带或更新测试（同文件 `#[cfg(test)]` 与 `tests/`）
- [ ] 无 `println!`/`dbg!`/`unwrap()` 残留
- [ ] 无硬编码密钥
- [ ] 新模块按第 6 节分层放置
- [ ] 可见性经 `lib.rs` 收口
- [ ] `Cargo.lock` 已同步更新
- [ ] AGENTS.md 本文件与新引入的约定保持同步（它是活文档，人工维护，见 §12）
- [ ] AI 生成的注释/日志串/文档遵守 §13 受控写作纪律（机械项抽检：句长与单句单指令。语义项人工审查。）
- [ ] 提交信息符合 Conventional Commits
- [ ] CI（`.github/workflows/ci.yml`）全绿
- [ ] 涉及需求或行为变化时，`docs/SPEC.md` 与 `CHANGELOG.md` 已同步
- [ ] 首次使用时已替换 `LICENSE` 占位与 `SECURITY.md` 联系方式

## 11. 按需深入（渐进式披露）

本文件不重复配置细节。需要时查阅：

- `Cargo.toml`：依赖、版本科目、clippy 配置
- `src/lib.rs`：公共 API 导出与编译期 lint 声明
- `src/logger.rs`：tracing 初始化参考实现
- `Makefile`：check 命令组装与外部工具自举说明
- `rustfmt.toml`：格式化规则
- `docs/SPEC.md`：意图与需求原子清单（SDD 三角的 spec 面）
- `.gitattributes` / `.editorconfig`：行尾归一化与编辑器一致性
- `LICENSE` / `CHANGELOG.md` / `SECURITY.md`：许可、变更记录与安全策略
- `.github/`：CI 工作流与 PR/issue 模板
- `.githooks/`：pre-commit（门禁+密钥扫描）与 commit-msg（提交规范）
- `.gitignore` / `.env.example`：安全边界
- 上级 `README.md`：调研依据与跨语言设计原则

**设计决策（刻意不做）**：本模板刻意不做的选择及理由：

- STE 受控写作纪律不设机器门禁（避免伪门槛，见 §13）。
- 不引入 MCP 工具或自定义工具定义（工具面最小化，见 §2）。
- 依赖审计不进 pre-commit（提交速度，留 CI）。

## 12. AGENTS.md 维护纪律

- **人工撰写、保持精简**：本文件由人类维护，只写 agent 无法自行推断的内容（构建/测试命令、约定、禁区）。公开实测显示：LLM 自动生成的上下文文件使 agent 任务成功率平均下降约 3%，人工撰写的才有成功率增益（+4%，但推理成本亦上升约两成）。
- **分层引用、不堆砌**：规则细节落在配置文件（`Cargo.toml`、`lib.rs` lint 声明等，见 §11），本文件只列入口与指引。膨胀的规范文件只会推高推理成本而不提升结果。
- **禁止 LLM 自生成规范文件自证**：本文件的变更不得由 agent 会话自行生成并自行验证通过。变更须经人工复核（与 §9 人在环路同一纪律）。
- **体量预算**：本文件保持 200 行以内。每季度审计：不挣得其位置的条款移入按需引用文档（见 §11）。新增条款奉行一进一出。
- **自动化卫生**：新增工作流自动化（slash command、skill、hook）前须已手动执行该流程三次以上。停止使用的自动化立即删除。
- **模型无关**：禁止累积针对特定模型的补丁式指令（prompt debt）。条款须模型无关。模型大版本升级后复核本文件，删除不再适用的条款。

## 13. 受控写作纪律（ASD-STE100 启发）

约束仓库内 **AI 生成的文本**（注释/docstring、人类可读的日志与错误消息字符串、文档、提交信息与 PR 描述）。代码标识符与结构化日志键不适用。依据与争议处置见 `RESEARCH_FINDINGS.md` §5。**本纪律是「受控写作纪律（ASD-STE100 启发）」，不是、也不得表述为「符合 ASD-STE100 规范」**（官方词表不可再分发且部分采用不构成合规）。

**严格档**（错误消息/指令性文档/工具描述）：主动语态；祈使句；仅简单时态；每句一条指令；不省略句子成分；复合名词 ≤3 词；英语每句 ≤20 词；项目内专有词一词一义（维护术语表）。

**平实档**（README/说明性散文）：短句；每句一义；一段一主题；每段 ≤6 句；不锁固定词表。

**中文内容**：采用结构原则的等价翻译（短句、每句一条指令、一段一主题、一词一义 + 术语表、祈使语气、不省略成分）。

**校验方式**：机械项（句长、单句单指令、段句数上限）列入代码审查抽检。语义项（一词一义、最平实用词）仅人工/agent 审查。不设机器门禁。
