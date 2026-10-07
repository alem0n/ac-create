# 调研结论：AI Agent 生成软件工程最佳实践（2026）

本文档汇总三轮多角度公开调研的结论，作为生成 Python / TypeScript / Rust 项目模板的依据。

---

## 一、AI Agent 协作规范（跨语言通用）

### 1.1 AGENTS.md 是事实上的开放标准
- 来源：agents.md 官网、InfoQ 2025-08 报道、aihero.dev 指南。本次调研已直接抓取 agents.md 官网全文核实。
- 结论：
  - 在仓库根目录放置 `AGENTS.md`，作为"给 agent 看的 README"，写明构建命令、测试命令、代码风格、测试要求、安全注意事项。
  - 采用规模（截至本次调研）：agents.md 官网宣称有 60,000+ 开源仓库采用（官网自述数字，本次已直接抓取官网核实；ETH Zurich 论文经 InfoQ 2026-03 报道时亦引用了同一量级——见 §4.2 与来源 33）；InfoQ 2025-08 报道时点为 20,000+。两个数字时点与来源性质不同，并存时须分别标注。
  - **分层优先级**：离被编辑文件最近的 AGENTS.md 优先；用户显式提示优先于一切。大型 monorepo 应在每个子包内放置嵌套 AGENTS.md（官网称 OpenAI 主仓库有数十个 AGENTS.md，具体数字未核实，模板中不引用）。
  - agent 会**主动执行** AGENTS.md 中列出的程序化检查命令，并在结束任务前尝试修复失败。因此列出确定性检查命令（lint/type-check/test）是强制行为约束的手段。
  - **净收益存争议**：§4.2 记录的苏黎世联邦理工实测（人工撰写 +4% 成功率、LLM 生成 -3%、推理成本 +20% 量级）表明 AGENTS.md 增益有限且有推理成本——处置方案为「保留标准 + 人工撰写保持精简 + 禁止 LLM 自生成规范文件自证」（详见 §4.2）。
  - AGENTS.md 现由 Linux Foundation 下的 Agentic AI Foundation 托管，得到 OpenAI Codex、Amp、Google Jules、Cursor、Factory 等共同支持，兼容多种工具（Aider、Gemini CLI 等可配置）。（以上均为 agents.md 官网自述，本次已直接抓取官网核实。）

### 1.2 规范驱动开发（Spec-Driven Development, SDD）
- 来源：nimbalyst.com SDD 实践指南、arXiv 2602.00180（SDD 综述）、augmentcode.com、exceeds.ai 2026 实践数据。
- 结论：
  - 工作流应为「意图 → 规范/计划 → 实现」，在实现前先写 spec（如 plan.md / spec 文件），并设置检查点。
  - 一份可行的规范标准：**规范描述的行为应详细到 agent 能实现它、测试能验证它**。
  - 以小的、模块化的步骤迭代（agentic loops），可降低错误积累、提升交付速度。
  - 方向性参考数据（厂商自述，单一来源，未经独立验证，仅作方向性参考）：exceeds.ai 宣称清晰的规范与 plan.md 检查点可带来约 71% 的生产力提升、减少约 41% 的需求/任务流失；该比例的分子分母未在原文严格定义，**模板中不引用具体数字**，仅采用「先规范后实现、设检查点」这一无争议的方法论结论。

### 1.3 确定性质量门禁（Independent Quality Gates）
- 来源：codacy.com（独立质量门禁）、augmentcode.com（AI 生成代码审查纪律）、getautonoma.com（五层质量门禁栈）、Krzysztof Wróbel 的静态指标门禁、testkube.io。
- 结论：
  - **不能只靠 AI 审查 AI 代码**：当 AI 为 AI 代码生成测试时，测试可能验证了已有 bug 而非捕获它（测试与实现共享同一误解）。
  - 必须建立独立、确定性的质量门禁栈，典型五层：**静态检查（lint）→ 类型检查 → 安全扫描 → 自动化测试 → 智能体测试**。
  - 静态分析最擅长确定性发现：安全问题、重复代码、复杂度、风格违规、策略合规。
  - 可对可维护性指标（复杂度、重复率等）设定硬性通过/失败阈值。
  - 提交前必须运行 lint 与全量测试，全部通过后方可合并。

### 1.4 人在环路（Human-in-the-Loop）与审查纪律
- 来源：arXiv 2603.15911（人机协同审查分析）、port.io、developerway.com、Dan Adler 演讲。
- 结论：
  - 单一研究（arXiv 2603.15911）报告：人类审查 AI 生成代码时的交流轮次比审查人写代码多 11.8%（该数字依赖特定样本与语境，相关性不应直接当作因果）；该研究据此建议对 AI 生成代码采用更严格的审查流程。模板中据此固化「审查流程更严格」的约定，不引用具体百分比。
  - AI agent 会在大代码库中制造"代码洪水"问题：重复代码、标准漂移。对策是在模板中预先固化目录约定与规范，让 agent 生成的代码"天然贴合"既有结构。
  - 代码库本身在"训练" agent：干净一致的代码库让 agent 产出贴合规范的代码。因此**模板本身就是规范的第一载体**。
  - 人机分工：agent 执行可实现的工作，但不可逆变更需人类批准；人类重点审查代码**行为**（而非仅审查代码文本）。
  - 经典工程实践依然成立且更重要：单元测试、先打通"黄金路径"。

### 1.5 模块设计：高内聚低耦合
- 来源：arXiv 2503.24260（动态需求下的可维护代码生成）、GeeksforGeeks、levelup.gitconnected。
- 结论：
  - 高内聚：模块内部元素紧密相关、共同完成单一明确职责（单一职责原则 SRP）。
  - 低耦合：模块间依赖最小化，变更一个模块对其他模块影响极小。
  - 研究表明专门化的 agent 可进行需求分析、模块分解、模式应用来强制高内聚/低耦合/SRP——因此模板应使"按职责分目录"成为默认结构。
  - 低耦合通常与高内聚相关；低内聚导致难以维护和测试。

---

## 二、语言生态惯例（模板结构依据）

### 2.1 Python
- 来源：pyOpenSci 包指南、Python Packaging User Guide（src-layout vs flat-layout）、pytest 官方 Good Integration Practices、Real Python best practices。
- 结论：
  - **目录结构**：采用 `src/` layout——`src/<package_name>/` 存放源码，`tests/` 独立于应用代码放在根目录。pyOpenSci 与 Packaging User Guide 均强烈建议 src layout（避免导入测试时的路径污染、防止意外导入未安装版本）。
  - **构建/依赖**：`pyproject.toml` 是依赖与项目元数据的唯一来源（结合本机 uv 规范，虚拟环境用 `.venv`，依赖锁定到 `uv.lock`）。
  - **测试框架**：pytest 为事实标准；约定 `tests/` 目录、文件名 `test_*.py`、测试函数 `test_*`；测试代码不放进应用包内部。
  - **日志**：标准库 `logging`（或 `structlog`）。规范要点：不使用 `print()` 调试、在模块顶层 `logging.getLogger(__name__)`、配置由入口点统一注入、日志包含结构化上下文。
  - **静态检查**：ruff（lint + 格式化，已取代 flake8/black 的生态位）+ mypy（类型检查）为当代默认组合。

### 2.2 TypeScript
- 来源：dev.to Real-World TypeScript Project Setup 2026、Vitest 官方文档、Sentry 2026 JS 日志库对比、hsb.horse TS monorepo 最佳实践、pnpm 对比。
- 结论：
  - **目录结构**：TS 项目倾向扁平结构，`src/` 下按功能组织，测试镜像源码树（`src/**/*.spec.ts` 或并行 `tests/`）。生态不强制约定，因此更需要在 AGENTS.md 中显式声明。
  - **包管理**：pnpm 是 2026 年推荐默认（pnpm 官方基准自述：显著节省磁盘与安装时间）；monorepo 用 pnpm workspaces。模板采用 pnpm 作为包管理器。
  - **测试框架**：Vitest 为 2026 年首选（基于 Vite、原生 ESM、内置 TS 处理）。具体 Node/Vite 最低版本以所选 Vitest 大版本在 package.json 中声明的 engines 与 peer dependency 为准（见 vitest.dev 文档）；模板锁定一组已验证的版本组合并写入 package.json，不沿用任何未核实的硬编码版本组合。
  - **日志**：Node 环境首选 Pino（性能最高、体积小）；Winston 适合需要多 transport/重度配置的场景。库代码可用 console 抽象封装。
  - **静态检查**：`tsc --noEmit` 严格模式类型检查 + ESLint + Prettier；`strict: true`。
  - 单仓库单包模板采用 `src/ + tests/`（或 co-located spec），构建产物输出到 `dist/`。

### 2.3 Rust
- 来源：LogRocket Rust web 服务结构、reintech 大型应用结构、Rust Project Primer（Organization）、Leapcell 大型项目组织。
- 结论：
  - **目录结构**：单 crate 用 `src/main.rs` + `src/lib.rs` + `src/<module>/` 按职责分模块；大型项目用 Cargo workspace 拆分多 crate。模板采用单 crate + 清晰模块划分（领域/基础设施分层），保留扩展为 workspace 的路径。
  - **模块组织**：crate 内部模块遵循"每个模块单一职责"，公共 API 通过 `lib.rs` 有意识地导出（信息隐藏）；`main.rs` 只做组装（依赖注入/初始化），不承载业务逻辑——这是高内聚低耦合在 Rust 中的直接体现。
  - **测试框架**：内建 `#[test]` + `cargo test` 为默认；单元测试与源码同文件（`#[cfg(test)] mod tests`），集成测试放 `tests/` 目录。
  - **日志**：`tracing` crate（与 tokio 生态集成）为事实标准，配合 `tracing-subscriber` 输出；优于传统 `log` crate，支持结构化字段与 span。
  - **静态检查**：`cargo fmt`（格式化）+ `cargo clippy`（lint，含大量惯用法与复杂度检查）为内建质量门禁。

---

## 三、模板必须落实的规范清单（审查重点）

每套模板（Python / TypeScript / Rust）的 AGENTS.md 与目录结构必须覆盖：

| 主题 | 规范要点 | 可校验性（落到具体工具与命令） |
|---|---|---|
| AGENTS.md 存在 | 根目录有 AGENTS.md，含构建/测试/lint 命令 | 文件存在性检查 |
| 日志 | 语言生态主流库（Py: logging/structlog；TS: Pino；Rs: tracing）；**禁止 print/console.log 进入业务代码**；模块级 logger 命名；结构化字段 | 机器可校验：Python 用 ruff 启用 `T20`（flake8-print，拦截 print/pprint）；TS 用 ESLint `no-console`（或 `@typescript-eslint/no-console`）并允许显式 exceptions 白名单；Rust 用 clippy restriction 集 `print_stdout`/`dbg_macro`；门禁命令统一纳入各语言 check 入口 |
| 测试框架 | Py: pytest（tests/, test_*.py）；TS: Vitest（*.spec.ts）；Rs: cargo test（#[cfg(test)] + tests/）；变更必须附带或更新测试 | 命令可执行：pytest / vitest / cargo test 全绿；并设 coverage 阈值闭合「变更未加测试」校验：Python `pytest --cov --cov-fail-under=<N>`（pytest-cov）；TS Vitest coverage thresholds（configuration.coverage.thresholds）；Rust 用 cargo-tarpaulin（`cargo tarpaulin --fail-under <N>`，外部工具，模板注释说明可选）。阈值在模板中给出初始默认值，随项目成熟度调整 |
| 质量门禁 | 确定性检查链：lint + 类型检查 + 测试 + 格式化 + SAST 安全规则，提交前全过。1.3 节五层门禁栈中，模板覆盖前四层（lint / 类型检查 / SAST 安全扫描 / 自动化测试）；「智能体测试」层属 CI 侧增量能力，不在模板范围，生成时显式声明 | 三语言统一以 Makefile `make check` 收口，并将「安全与密钥」行定义的依赖审计与「测试框架」行定义的覆盖率门禁一并纳入同一入口（Python: Makefile 封装 `ruff check && ruff format --check && mypy && pytest && uv run pip-audit`，其中覆盖率默认经 pyproject.toml 的 `[tool.pytest.ini_options] addopts` 注入 `--cov --cov-fail-under=<N>`（pytest-cov 列为 dev 依赖；附加 `make coverage` target 按需启用）；TS: package.json 定义 `check` script，内含 lint、`tsc --noEmit`、`prettier --check`、`vitest run --coverage`（coverage thresholds 生效）与 `pnpm audit`，Makefile 封装 `pnpm check`；Rust: Makefile 封装 `cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo audit`，cargo-audit 为需 `cargo install cargo-audit` 的外部子命令，模板在 bootstrap/Makefile 注释中给出一次性安装步骤（与 Python 侧自举方式对称），cargo-tarpaulin 同为外部工具按需作为附加 target）。模板随附 Makefile 本体，AGENTS.md 与 CI 引用同一入口，避免门禁散落 |
| 高内聚低耦合 | 按职责划分目录（领域/基础设施/接口分层）；模块单一职责；公共 API 显式导出；禁止跨层直接依赖、禁止 utils 大杂烩 | 机器 + 人工结合：Python 用 `import-linter` 配置分层依赖契约（lint-imports）；TS 用 `dependency-cruiser` 或 ESLint import 边界规则校验分层方向；Rust 用 crate/workspace 边界 + 模块可见性（pub 仅经 lib.rs 收口）做编译期约束，辅以 clippy 复杂度类 lint（如 cognitive_complexity、module_inception）；目录约定本身亦作为审查检查项 |
| 渐进式文件披露 | 文件分层组织，README/AGENTS.md 仅列入口与按需深入路径，不在单个文件堆砌全部内容 | 文档结构检查 |
| 人在环路 | 实现与其测试不得仅由同一 agent 会话自证通过：必须通过独立的确定性门禁（lint/类型/测试）全绿，或经人工复核；涉及不可逆变更须人类批准 | 门禁命令可执行（同「质量门禁」行）+ 人工审查；AGENTS.md 写明该流程约定 |
| 安全与密钥 | 密钥/环境变量不经代码硬编码，统一走 .env（不入库）与环境变量注入；依赖漏洞扫描纳入门禁 | .gitignore 覆盖 .env/密钥文件；Python 用 pip-audit（列为 dev 依赖，经 `uv run pip-audit` 审计项目环境，确保审计对象为项目依赖）；TS 用 pnpm audit；Rust 用 cargo audit（外部子命令，bootstrap 时一次性安装） |
| 错误处理约定 | 错误在各语言惯用层显式传播（Py: 异常分层 + 不吞异常；TS: Result/错误类型分层，禁止 any 吞错；Rs: Result/thiserror 分层，避免 unwrap 进入生产路径），错误与日志边界清晰 | lint 规则（如 clippy `unwrap_used`、ruff `BLE001`/`S110`、TS `no-unsafe-*`）+ 代码审查 |

---

## 四、X（Twitter）社区分享案例（补充轮）

> 方法与局限：X 调研经公开嵌入接口（cdn.syndication.twimg.com）读取单条推文正文与互动数据（点赞数/回复数作为社区认可度佐证）；较长推文仅可读取前约 280 字符，完整串文与 X 长文（Article）正文不可达，故以「推文要点 + 其链接的长文（已可访问）」组合取证。以下均为本次直接读取到的原文。

### 4.1 @bcherny（Boris Cherny，Claude Code 作者）— 2026-01-02
- 原帖：https://x.com/bcherny/status/2007179832300581177（54,225 次点赞）
- 原文要点：「我创建的 Claude Code……我的设置可能出乎意料地朴素（surprisingly vanilla）！Claude Code 开箱即用就很好，我个人没怎么做定制。没有唯一正确的使用方式……」
- 对模板的印证与警示：
  - 印证「代码库与约定本身是第一载体」：与 §1.4「干净一致的代码库让 agent 产出贴合规范的代码」一致。
  - 警示：**勿过度定制**。模板不应堆积花哨开关与重度配置；默认结构 + 精简规范优于技巧堆叠（与本模板「渐进式文件披露」原则直接吻合）。

### 4.2 @omarsar0（elvis，dair-ai）— 2026-02-26（争议条目，处置方式见下）
- 原帖：https://x.com/omarsar0/status/2027025932339278029（119 次点赞）
- 原文要点：「这篇热门论文测量 AGENTS.md 是否有助于编程 agent：人工撰写的有少许帮助（+4%），LLM 生成的有少许损害（-2%），且都会增加 20%+ 推理成本。agent 忠实遵循指令，但这并未转化为（更好的结果）……」
- 该帖所引论文为本轮新引入来源（见来源清单 32）：苏黎世联邦理工（ETH Zurich）的 AGENTbench 研究（arXiv 2602.11988，经 InfoQ 2026-03 报道，见来源 33）。论文实测：LLM 生成的上下文文件使任务成功率平均下降约 3%、步数增多、推理成本上升逾 20%；人工撰写的成功率平均 +4% 但成本最高 +19%（推文作者将 +4%/-3% 转述为「+4%/-2%」）；论文建议完全省略 LLM 生成的上下文文件，人工撰写部分仅保留**不可推断的细节**（如特定工具与自定义构建命令），架构总览类信息未见实质收益。
- **争议状态**：AGENTS.md 的净收益存在公开争议（效果增益小、有推理成本）。处置方案（已经 subagent 审查确认）：
  1. 保留 AGENTS.md（生态既成标准、60k+ 仓库采用、agent 会主动执行其中命令——行为收益不限于编码质量分）；
  2. 以「人工撰写、精简、渐进式披露」化解争议要点：模板 AGENTS.md 按层引用配置而非堆砌，限制上下文膨胀；
  3. 明确禁止「LLM 自动生成规范文件自证」——恰与论文实测 -3% 结果及 §三「人在环路」行呼应。

### 4.3 @pauliusztin_（Paul Iusztin）— 2026-01-09
- 原帖：https://x.com/pauliusztin_/status/2009637593873322116（9 次点赞——普通量级分享，非高传播帖；取其观点而非社区认可度）
- 原文要点：「若你 2026 年在构建 AI agent，需要更快地做架构决策……95% 的 agent 从未投产。原因是从第一天起就选错架构：需要 agent 时用了 workflow，需要简单 pipeline 时用了多 agent。」
- 对模板的印证：与 §1.5 高内聚低耦合、§2 各语言「从简单结构起步」一致——模板的「单包/单 crate + 分层模块」正是避免首日选错架构的默认起点；大型化时再演进为 workspace/monorepo（模板已保留扩展路径）。

### 4.4 @mvanhorn（Matt Van Horn）— 2026-06-02
- 原帖：https://x.com/mvanhorn/status/2061877533885473181（3,348 次点赞）；所链 X 长文标题《Every Agentic Engineering Hack I Know (June 2026)》，其预览文中称：三个月前发布的《Every Claude Code Hack I Know》获得 91.3 万次浏览，被问及用什么 IDE 时答「不用 IDE，只用 plan.md 文件与语音」。
- 对模板的印证：plan.md / 规范先行的强社区信号（高传播量），与 §1.2 SDD「意图 → 规范 → 实现」一致。

### 4.5 X 案例汇总对模板的影响
- 除 §4.2 处置方案第 3 条（见下）外无新增必须条款；四案例均为既有结论的社区佐证。
- §4.2 处置方案第 3 条「禁止 LLM 自动生成规范文件自证」实质是一条新增明确禁令（现有「人在环路」条款仅覆盖实现与测试的自证，未覆盖规范文件），将随维护纪律一并落为模板条款（见下条）。
- 新增一条已经审查确认的纪律：AGENTS.md「保持人工撰写、精简、分层引用」作为明确维护纪律写入三套模板（§12「AGENTS.md 维护纪律」），本次已落地。

## 五、ASD-STE100 受控语言规范（约束 AI 生成文本）

> 本轮目的：用户要求引入 ASD-STE100 限制 AI 生成的文档、注释、日志等落地文本。

### 5.1 规范本体（事实层）
- 来源：Wikipedia「Simplified Technical English」、asd-ste100.org 官网（Issue 9 规范 PDF 公开预览）、asd-europe.org。
- 事实：
  - 现行版本 Issue 9，2025 年 1 月发布，由**53 条写作规则** + 约 **900 个许可词**的词典构成；规则区分「程序性文本（procedures）」与「描述性文本（descriptions）」两类。
  - 核心结构规则（公开摘要，非穷尽）：仅按词典给出的词性与词义使用许可词；指令尽可能清晰具体；复合名词不超过 3 个词；动词仅用不定式/祈使句/一般现在时；句子简短；**不省略句子成分**（动词/主语/冠词）；复杂内容用垂直列表；**每句只写一条指令**；每段只写一个主题；**每段不超过 6 句**；安全指令以明确命令或条件开头。
  - 「一词一义」是词典的核心原则（如 `close` 仅可用于两个指定含义）。
  - 商业校验工具存在（HyperSTE、Congree、TechScribe、波音 BSEC）；Wikipedia 亦指出 LLM 被请求时可以按 ASD-STE100 风格检查与写作。

### 5.2 适用性与争议（须按边界条款经审查确认）
- **争议一：跨领域采用有限。** Wikipedia 明确记载：航空航天与军工之外的大多数组织不在其技术文档中使用 STE，原因在很大程度上是「它已不再服务于其最初目的」。
- **争议二：官方反对部分采用。** 官方规范文本声明：部分使用或偏离其写作规则与词表「会削弱 STE 的准确性并在用户间造成混乱」；并以「以规范本身为唯一参考」为写作前提。
- **争议三：词表不可移植。** 约 900 词的词典面向航空维修词汇（`propeller`、`ream`、`to drill` 等），软件领域的常用词（`middleware`、`payload`、`fixture` 等）不在其中；且规范可免费获取但**不可自由再分发**（Issue 9 再分发须经 ASD 书面授权）。
- **争议四：单语言。** STE100 仅为英语设计；本模板体系内容以中文为主。
- **社区与学术信号（正方）**：
  - @karpathy（Andrej Karpathy，54,175 次点赞，2026-10-02，X 原帖经公开嵌入接口直接读取）：「请你的 LLM 用 ASD-STE100 解释问题……它是一种受控语言规范，最初为[航空维修文档]而开发」。
  - 开源先例 danyuchn/asd-ste100-skill：将 STE100 规则重构为面向 **agent 可读输出**的 Claude Code 技能——理由是「解析另一 agent 输出的 LLM 与看不懂说明书的机修工处境惊人相似：没有反馈通道，无法追问『你是指 X 还是 Y』」。其设计要点：分 Strict（程序/错误消息/工具描述）与 STE-flavored（README/说明性散文）两档——后一档保留句子纪律但**不锁词表**；确定性 linter 只查机械规则（分号、长句、短语动词、现在完成时、悬挂连词等），词典相关规则仅标为建议性；明确声明**不**再分发官方词表，只采用「最平实的词、每次用法一致」的原则。
  - 学术：Nieminen 2025（哥德堡大学）研究技术写作中的 LLM 辅助、并将 ASD-STE100 用作生成式 AI 框架（相关工作，本轮仅摘要级核实，未采信其结论）。

### 5.3 处置方案（已经 subagent 审查确认并落地）

采「受控写作纪律」而非「STE100 合规」的路线，理由：官方禁止词表再分发且明确警示部分采用会削弱合规准确性，而软件领域无法使用其航空词表，故只能以规则子集 + 原则方式引入，并如实命名：

1. **适用范围**（约束仓库内 **AI 生成**的文本，含 AI 参与生成的文档片段）：注释/docstring、人类可读的日志与错误消息字符串、README/AGENTS.md 等文档、提交信息与 PR 描述。代码标识符与结构化日志键（如 `app.start`）不适用。人工撰写的 AGENTS.md 仍由 §4.2 处置与各模板 §12 维护纪律治理，不受本节约束。
2. **两档纪律**（沿用开源先例的分层）：
   - **严格档**（错误消息/指令性文档/工具描述）：一词一义；主动语态；祈使句；仅简单时态；每句一条指令；不省略句子成分；复合名词 ≤3 词；句子简短（英语 ≤20 词）。
   - **平实档**（README/AGENTS.md/说明性散文）：保留句子结构纪律（短句、一段一主题、一段 ≤6 句、每句一义），不锁固定词表。
3. **中文内容的对应处理**：STE100 为英语单语言设计；中文内容采用其**结构原则的等价翻译**（短句、每句一条指令、一段一主题、专有词在项目内一词一义并维护术语表、指令用祈使语气、不省略句子成分），不声称符合 ASD-STE100。
4. **可校验性**（诚实的分层）：
   - 机械可校验：句长上限、句子含分号连接多条指令、一段句数上限等结构规则可 lint/人工抽检；作为代码审查检查项。
   - 不可机械校验：一词一义、最平实用词——列为人工/agent 审查项，不承诺机器门禁（避免引入伪门槛）。
5. **声明纪律**：模板中一律表述为「受控写作纪律（ASD-STE100 启发）」，**不得**表述为「符合 ASD-STE100 规范」——官方明确部分采用不构成合规，且词表不可再分发。
6. **对既有模板的影响**：现有模板的注释与日志多为短句（如「领域模型示例（模板占位，替换为实际领域概念）」、`app.start`），基本已满足结构纪律；本纪律主要约束后续 AI 生成内容，不要求重写已验收文本。

## 六、来源清单（可追溯）

1. https://agents.md/ —— AGENTS.md 官方规范
2. https://github.com/agentsmd/agents.md —— 官方仓库
3. https://www.infoq.com/news/2025/08/agents-md/ —— InfoQ 报道（20k+ 仓库采用）
4. https://www.aihero.dev/a-complete-guide-to-agents-md —— 完整指南
5. https://blog.exceeds.ai/ai-software-development-best-practices/ —— 2026 AI 软件开发 7 大实践（71%/41% 为该厂商自述数据，未独立验证）
6. https://nimbalyst.com/blog/spec-driven-development-with-coding-agents/ —— SDD 实践（spec 可实现+可验证标准）
7. https://arxiv.org/html/2602.00180v1 —— SDD 学术综述
8. https://blog.codacy.com/why-coding-agents-need-independent-quality-gates —— 独立质量门禁
9. https://www.augmentcode.com/guides/reviewing-ai-generated-code —— AI 代码审查纪律（AI 审 AI 验证 bug 风险）
10. https://getautonoma.com/blog/quality-gate-vibe-coding —— 五层质量门禁栈
11. https://arxiv.org/html/2603.15911v1 —— 人机协同 agentic code review（+11.8% 轮次，单一研究）
12. https://www.developerway.com/posts/building-the-playground-for-ai-coders —— 代码库适配 AI
13. https://www.port.io/blog/human-in-the-loop-for-ai-coding-agents —— HITL 定义
14. https://arxiv.org/html/2503.24260v3 —— 动态需求下可维护代码生成（内聚/耦合/SRP）
15. https://pyopensci.org/python-package-guide/package-structure-code/python-package-structure.html —— Python 包结构（src layout）
16. https://packaging.python.org/en/latest/discussions/src-layout-vs-flat-layout/ —— 官方 src vs flat
17. https://docs.pytest.org/en/stable/explanation/goodpractices.html —— pytest 测试布局惯例
18. https://dev.to/gabrielanhaia/real-world-typescript-project-setup-for-2026-5an —— TS 2026 项目结构（扁平+测试镜像）
19. https://vitest.dev/guide/ —— Vitest 官方文档
20. https://blog.sentry.io/javascript-logging-library-definitive-guide/ —— 2026 JS 日志库对比（Pino/Winston）
21. https://hsb.horse/en/blog/typescript-monorepo-best-practice-2026/ —— TS monorepo（pnpm workspaces）
22. https://blog.logrocket.com/best-way-structure-rust-web-services/ —— Rust 服务结构
23. https://reintech.io/blog/rust-project-structure-best-practices-large-applications —— Rust 大型应用结构
24. https://rustprojectprimer.com/organization/index.html —— Rust 项目组织（crates/modules/workspaces）
25. https://leapcell.medium.com/mastering-large-project-organization-in-rust-a21d62fb1e8e —— Rust 大型项目组织
26. https://import-linter.readthedocs.io/ —— Python 分层依赖契约工具（补：高内聚低耦合可校验性）
27. https://github.com/sverweij/dependency-cruiser —— TS/JS 依赖方向校验工具（补：高内聚低耦合可校验性）
28. https://x.com/bcherny/status/2007179832300581177 —— X 案例：Claude Code 作者谈「朴素设置」（经公开嵌入接口直接读取）
29. https://x.com/omarsar0/status/2027025932339278029 —— X 案例：AGENTS.md 效果实测推文（同上）
30. https://x.com/pauliusztin_/status/2009637593873322116 —— X 案例：agent 架构决策推文（同上）
31. https://x.com/mvanhorn/status/2061877533885473181 —— X 案例：plan.md 实践推文（同上）
32. https://arxiv.org/abs/2602.11988 —— X 案例所引论文：ETH Zurich AGENTbench 研究（AGENTS.md 对 agent 任务成功率/成本的影响；经本次直接抓取 InfoQ 报道核实）
33. https://www.infoq.com/news/2026/03/agents-context-file-value-review/ —— InfoQ 2026-03 对上列论文的报道（直接抓取全文核实；报道亦佐证 60,000+ 仓库采用量级）
34. https://www.asd-ste100.org/about_STE.html —— ASD-STE100 官网规范说明
35. https://www.asd-ste100.org/assets/files/ASD-STE100_ISSUE9.pdf —— Issue 9 规范官方公开预览（再分发受限，本调研未复制其词表）
36. https://en.wikipedia.org/wiki/Simplified_Technical_English —— STE 概览（53 条规则 / 约 900 词 / 采用争议）
37. https://github.com/danyuchn/asd-ste100-skill —— 面向 agent 输出的 STE 技能（分层设计与许可再分发声明）
38. https://x.com/karpathy/status/2105819303471976479 —— X 案例：Karpathy 推荐 ASD-STE100 风格提示（经公开嵌入接口直接读取）
39. https://gupea.ub.gu.se/bitstreams/99118e8e-c4b5-4571-bd7a-3c1126fa6418/download —— Nieminen 2025：技术写作中的 LLM 辅助与 ASD-STE100（摘要级）
40. https://www.asd-europe.org/standards-specifications/simplified-technical-english/ —— ASD Europe 的 STE 说明页（摘要级二级来源，事实经来源 35/36 交叉核实）
