# 博客调研报告：可落地的模板优化洞见

调研对象：dbreunig.com（Drew Breunig）与 lucumr.pocoo.org（Armin Ronacher）2025 年 1 月之后的文章及其延伸外链。
调研目标：提炼优化当前项目模板（`python/`、`typescript/`、`rust/` 三套模板 + 根 `README.md`）的可执行信息，不做泛泛摘要。

---

## 一、调研来源清单（全部经直接抓取阅读，日期均可确认）

| # | 标题 | 链接 | 发布日期 |
|---|---|---|---|
| 1 | 10 Lessons for Agentic Coding | https://www.dbreunig.com/2026/05/04/10-lessons-for-agentic-coding.html | 2026-05-04 |
| 2 | The Problem is Prompt Debt | https://www.dbreunig.com/2026/06/22/the-problem-is-prompt-debt.html | 2026-06-22 |
| 3 | What Do Humans Need From Docs? | https://www.dbreunig.com/2026/05/31/what-do-humans-need-from-docs.html | 2026-05-31 |
| 4 | How Long Contexts Fail | https://www.dbreunig.com/2025/06/22/how-contexts-fail-and-how-to-fix-them.html | 2025-06-22 |
| 5 | How to Fix Your Context | https://www.dbreunig.com/2025/06/26/how-to-fix-your-context.html | 2025-06-26 |
| 6 | Harnesses are Situated Agents | https://www.dbreunig.com/2026/08/14/harnesses-are-situated-agents.html | 2026-08-14 |
| 7 | Learnings from a No-Code Library: Keeping the SDD Triangle in Sync | https://www.dbreunig.com/2026/03/04/the-spec-driven-development-triangle.html | 2026-03-04 |
| 8 | How Claude Code Builds a System Prompt | https://www.dbreunig.com/2026/04/04/how-claude-code-builds-a-system-prompt.html | 2026-04-04 |
| 9 | What We Can Learn from Claude's Fable 5.1 System Prompt | https://www.dbreunig.com/2026/09/07/what-we-can-learn-from-claude-s-fable-5-1-system-prompt.html | 2026-09-07 |
| 10 | Agentic Coding Things That Didn't Work | https://lucumr.pocoo.org/2025/7/30/things-that-didnt-work/ | 2025-07-30 |
| 11 | Tools: Code Is All You Need | https://lucumr.pocoo.org/2025/7/3/tools/ | 2025-07-03 |
| 12 | Your MCP Doesn't Need 30 Tools: It Needs Code | https://lucumr.pocoo.org/2025/8/18/code-mcps/ | 2025-08-18 |
| 13 | Fast and Hard Code | https://lucumr.pocoo.org/2026/8/22/fast-hard-code/ | 2026-08-22 |
| 14 | What is Codemode | https://lucumr.pocoo.org/2026/10/6/codemode/ | 2026-10-06 |
| 15 | AI Changes Everything | https://lucumr.pocoo.org/2025/6/4/changes/ | 2025-06-04 |

**延伸外链（在正文中被引用，未独立打开核验，按二手引用对待）**：Anthropic 多代理研究系统博客、Anthropic "think tool" 博客、Berkeley Function-Calling Leaderboard、Databricks 长上下文 RAG 研究、Datadog State of AI Engineering、若干 arXiv 论文（2407.06866 / 2411.15399 / 2505.06120 / 2505.03275 / 2501.16214 / 2604.07709 / 2512.04123）、Gemini 2.5 技术报告、Vercel just-bash、Pydantic Monty、Anthropic C 编译器实验、Steve Yegge Gas Town、scaffold-docs 与 whenwords 仓库、Cloudflare Code Mode 博客。

---

## 二、关键洞见提炼（按主题，附来源）

### 主题 A：规范文件即上下文——每一 token 都在付费

- 上下文四种失效模式：Poisoning（错误进入上下文被反复引用）、Distraction（超长后模型重复历史动作而非推理；Gemini 实测显著超过 100k tokens 触发；Databricks 研究显示 Llama 3.1 405b 约 32k 后正确率下降）、Confusion（无关内容被纳入并影响输出；一项研究给 8b 模型 46 个工具失败、19 个成功——不是窗口限制而是混淆）、Clash（上下文自相矛盾；分片式多轮问答平均掉 39%）。（来源 4、5）
- 「放进上下文的东西，模型必须为它付出注意力」；核心检验标准只有一句：**「Is everything in this context earning its keep?」**（来源 5）
- Claude Code 的系统提示是动态装配的：大量组件**条件加载**（仅在相关功能启用时注入），并用缓存边界标记把全局可缓存内容与 session 特定内容分离。（来源 8）
- 与我们既有调研交叉印证：RESEARCH_FINDINGS.md §5.2 的 ETH Zurich 实测——上下文文件使推理成本上升 20% 量级。

### 主题 B：用测量而非散文约束行为

- 「用测量而非散文规范系统行为：评估、指标、类型化规格是可读、可共享的硬边界」；「最好的工程师把更多带宽花在测试上——测试不再是安全网，而是让模型放手干的前提」。（来源 2）
- 「验证必须更确定化。能用代码就不用模糊的 LLM 判断，LLM 调用是最后手段」。（来源 7，Plumb 的设计原则）
- 「我们热爱 linter 和格式化工具，因为它们不含歧义。能完全自动化的就自动化；把 LLM 用在不需要推理的任务上是错误做法」。（来源 10）

### 主题 C：检查点而非建议

- 「skill 只是建议，工具必须是检查点。提交即失败的拦截模式是关键的，否则会被无视」；「系统必须是强制的、不可选的——agent 会漂移」。（来源 7，Plumb 把决策提取装进 git pre-commit，有未审阅决策则提交失败）

### 主题 D：人在环路需要可判定阈值

- Claude Code 自身提示要求：非平凡实现（**3 个以上文件编辑、后端/API 变更、基础设施变更**）在报告完成前**必须经过独立对抗性验证**，无论实现者是谁。（来源 8）
- 模型生成代码的数量压倒了人工审查 throughput，正成为行业性问题。（来源 7 引用的当日新闻标题；与我们 RESEARCH_FINDINGS.md §1.4 的 +11.8% 审查轮次研究同向）

### 主题 E：注释纪律——默认不写

- Claude Code（Anthropic 内部规则）：**默认不写注释；仅当 WHY 非显然时才写**（隐藏约束、微妙不变量、特定 bug 的变通、令读者意外的行为）。（来源 8）

### 主题 F：SDD 三角——spec 是活文档而非前置产物

- SDD 不是单向方程而是反馈回路：**「实现代码会反过来改进 spec 与测试」**；三个节点（spec/测试/代码）随演进必须保持同步——「改进代码时必须改进 spec」。（来源 7）
- 同向结论：lesson 1「写代码会暴露未曾考虑的决策、让 spec 更好」、lesson 4「记录意图（why）」、lesson 5「spec 随代码与测试同步更新，否则捕获不到实现中的学习」。（来源 1）
- 关键缺口信号：**覆盖度工具只说明代码被测试，不能说明测试反映了 spec——测试必须覆盖规范**。（来源 7）
- 架构应允许并行开发：「让每个人（每个 agent）知道自己能做哪一块」的架构极具价值。（来源 7）
- 「测试是珍贵资产」——趋势是代码免费、测试付费（SQLite 模式）。（来源 7）

### 主题 G：文档为人 vs 为 agent——渐进式披露互相印证

- 「~95% 的 skill 就是 Markdown，按文件夹组织、支持渐进式披露」——skill 就是给 agent 看的文档，且可直接充当人类文档。（来源 3）
- 人类文档的正确职责：建立心智模型而非罗列细节；教会「可能的边界」；解释 why；记录设计决策（**包括刻意不做什么**）；不为完整性优化——目标是让读者能更好地指挥 agent。（来源 3）
- 三层文档结构：Getting Started（单一代表性用例的叙述教程）→ Diving Deeper（一主题一文件、围绕意图与设计决策）→ Reference（按模块的 API 查阅）。（来源 3）

### 主题 H：脚本/CLI 优先，工具面最小化

- MCP 两大缺陷：组合经由推理完成、上下文开销大——「用 `gh` CLI 完成 GitHub 任务比 GitHub MCP 更省上下文、更快」；自动化应优先用代码而非推理：**「review 公式而不是 review 计算结果」**。（来源 11）
- Playwright MCP 约 30 个工具定义可压缩为 1 个「ubertool」（直接执行代码）；减少工具数还能降低 context rot。（来源 12）
- 模型在训练中学习过 bash 与文件系统语义，故 CLI 组合对模型更自然；Codemode 让工具调用组合绕过 LLM 上下文。（来源 14）
- 工具面阈值（二手数据）：DeepSeek-v3 实验中超过 30 个工具描述开始重叠混淆，超过 100 个几乎必败；选出 <30 个相关工具可提升工具选择精度至 3 倍。（来源 5 引 RAG MCP 论文）

### 主题 I：自动化卫生与工具纪律的执行手段

- 失败的自动化必须删除：「用不上的 Claude 命令会堆积并污染工作区、混淆他人」；只自动化已重复执行多次的流程，用「同任务跑 3 次看方差」评估价值。（来源 10）
- hook 未能强制工具选择（uv 代替 python），最终方案是 **PATH 拦截器**：一个拒绝执行并提示「This project uses uv, please use 'uv run python' instead.」的 shim，启动前把 `.claude/interceptors` 注入 PATH。（来源 10）
- LLM 自动化的隐性风险：促进精神脱钩、高估 agent 能力；「LLM 降低重构成本但未降到零，回归很常见」。（来源 10）

### 主题 J：模型漂移与 prompt debt

- 手工调提示=技术债：每加一条补丁式指令就增加脆弱性与回归风险，并锁定到特定模型（Berkeley 研究显示企业因新模型破坏既有 agent 而停留在旧模型）。（来源 2）
- 系统提示的指令随模型代际被训练吸收或回归，产品提示里挤满 case 式补丁。（来源 9）
- 「模型不是干净版本化的软件——换权重即换行为」；Anthropic 自己警告为旧模型写的 skills 可能「degrade output quality」。（来源 2）

### 主题 K：语言选择变化（验证性洞见）

- 熟悉语言不再是门槛，人们按「营销/性能诉求」选语言；Rust/Zig 等「硬语言」因 fast & small 回潮；**「LLM 擅长在不回归行为的前提下优化代码」**——前提是有行为测试兜底（间接强化主题 B/F）。（来源 13）

---

## 三、模板优化建议（针对当前项目模板，逐条可执行）

> 以下建议落点均对应模板实际结构（三语言 AGENTS.md 的 §2/§3/§5/§6/§9/§11/§12/§13）。所有新增文案须遵守 §13 受控写作纪律（短句、单句单指令、句号收尾）。

### 建议 1（D）— 人在环路阈值具体化【高优先】

- **改动位置**：`python/AGENTS.md` §9、`typescript/AGENTS.md` §9、`rust/AGENTS.md` §9
- **改动内容**：在现有「实现与其测试不得仅由同一 agent 会话自证通过」后追加：
  「触发阈值：变更触及 3 个及以上文件、或涉及 infrastructure 层、或变更公共 API 时，报告完成前必须经过独立对抗性验证（另一 agent 会话或人工复核）。」
- **预期收益**：把目前定性的 HITL 纪律变成可判定的数值与范围门槛（来源 8 的 Claude Code 自身实践）；与 §13 机械抽检方式一致。
- **依据**：来源 8；来源 7（人工审查被代码量压倒）。

### 建议 2（E）— 注释默认不写，仅写非显然 WHY【高优先】

- **改动位置**：三份 AGENTS.md §3「代码风格」
- **改动内容**：新增一条：
  「注释默认不写。仅当 WHY 非显然时写注释（隐藏约束、微妙不变量、特定 bug 的变通、令读者意外的行为）。模块头 docstring 例外（其记录分层契约与职责，见第 6 节）。」
- **预期收益**：减少低信噪比 token；与主题 A 的「上下文付费」一致；docstring 例外保留我们已验证的分层契约注释。
- **依据**：来源 8（Claude Code/Anthropic 内部规则原文）。

### 建议 3（C）— 提交即失败的检查点【高优先，需权衡】

- **改动位置**：新增 `python/.githooks/pre-commit`、`typescript/.githooks/pre-commit`、`rust/.githooks/pre-commit`；三份 AGENTS.md §2 新增安装行。
- **改动内容**：pre-commit 脚本执行 `make check` 的快速子集（lint + 类型检查 + 测试；依赖审计保留在 CI，避免提交卡顿），失败即退出非零以阻止提交。AGENTS.md §2 增「首次克隆后执行 `git config core.hooksPath .githooks`」。
- **预期收益**：把「提交前必须全绿」从散文纪律变成强制检查点（「skill 是建议，工具必须是检查点」）；agent 漂移时仍被拦截。
- **不确定性与权衡（推断）**：pre-commit 的执行时长与开发者体验需实测；审计步骤是否进 pre-commit 本身有争议，来源未讨论 pre-commit 重量级问题，此为推断设计。

### 建议 4（F）— 引入 spec 面 + 需求级覆盖【高优先，结构性补齐】

- **改动位置**：三语言模板新增 `docs/SPEC.md`（模板占位）；三份 AGENTS.md §5「测试框架规范」新增条款。
- **改动内容**：
  1. `docs/SPEC.md` 记录意图（what/why）与需求原子清单，每条需求带 ID。
  2. §5 新增：「测试须覆盖 SPEC.md 中的需求，而不仅是代码行覆盖率。每个公共行为至少一个端到端测试。」
  3. §12 或 §5 新增「规范同步：非平凡变更先在 docs/SPEC.md 记录意图。实现产生的新决策必须回写 SPEC（实现改进规范）。」
- **预期收益**：补齐 SDD 三角中我们完全缺失的 spec 面；把覆盖度从「行覆盖」升级为「需求覆盖」——来源 7 明确指出这是当前工具盲区。
- **依据**：来源 7（三角与 Plumb 的全流程）、来源 1（lesson 1/4/5）。

### 建议 5（B/F）— 行为契约测试导向

- **改动位置**：三份 AGENTS.md §5 + `tests/` 示例
- **改动内容**：§5 新增「测试断言公共行为契约，不测内部实现细节。重建与重实现时行为测试不变。」
- **预期收益**：支持「代码廉价→经常重建」的工作流（测试是可重建性的保障）；与 lesson 3 一致。
- **依据**：来源 1 lesson 3；来源 2（measurements not prose）。
- **注**：现有模板示例测试已按公共 API 断言，改动主要是写明纪律。

### 建议 6（A）— AGENTS.md 体量预算

- **改动位置**：三份 AGENTS.md §12
- **改动内容**：§12 新增「本文件保持 200 行以内。每季度审计：不挣得其位置的条款移入按需引用文档（见 §11）。新增条款奉行一进一出。」
- **预期收益**：直接控制每次会话的上下文成本；与主题 A 的失效模式（Distraction/Confusion）及我们自己的 ETH 实测（+20% 推理成本）闭环。
- **不确定性（推断）**：200 行的具体阈值为推断建议值，来源未给出该数字；应随实测调整。

### 建议 7（H）— 固化「脚本优先、工具面最小化」

- **改动位置**：三份 AGENTS.md §2 或 §11
- **改动内容**：新增「自动化优先以脚本/CLI 组合完成（Makefile 与配置文件），不为本项目引入额外 MCP 工具或自定义工具定义。确定性脚本优先于 LLM 判断。」
- **预期收益**：防止模板使用中漂移到臃肿的 MCP/工具面路线；这是对现状（Makefile `make check` 路线）的确证与固化，有双向来源支撑（lucumr 反 MCP 立场 + dbreunig 工具面阈值数据）。
- **依据**：来源 11、12、14、5。

### 建议 8（I）— Python uv 拦截器（可选，harness 相关）

- **改动位置**：`python/` 新增 `interceptors/python`（shim 脚本）；`python/AGENTS.md` §2 说明
- **改动内容**：提供失败即提示的 shim（「This project uses uv, please use 'uv run python' instead.」）；AGENTS.md §2 说明启动 agent 前把 `interceptors/` 注入 PATH 的方式。
- **预期收益**：把「禁止使用系统 python/pip」从不可校验的散文变成可执行机制（ruff 无法拦截解释器选择）。
- **不确定性**：来源 10 的语境是 Claude Code yolo 模式，机制针对特定 harness；标注为可选 opt-in，非默认启用。

### 建议 9（I）— 自动化卫生条款

- **改动位置**：三份 AGENTS.md §12
- **改动内容**：新增「新增工作流自动化（slash command、skill、hook）前须已手动执行该流程三次以上。停止使用的自动化立即删除。」
- **预期收益**：防止仓库堆积无用指令污染工作区与上下文（与主题 A 同向）。
- **依据**：来源 10。

### 建议 10（G）— 三层文档结构映射与「刻意不做」记录

- **改动位置**：根 `README.md` + 三份 AGENTS.md §11
- **改动内容**：
  1. README 明确三层映射：README=Getting Started、AGENTS.md 各节=Diving Deeper、配置文件=Reference，说明该结构来自既有人类文档方法论。
  2. AGENTS.md §11 末尾新增「设计决策：刻意不做的选择及其理由」小节，例如：STE 纪律不设机器门禁（避免伪门槛）、不引入 MCP 工具（工具面最小化）、不把依赖审计放进 pre-commit（提交速度）。
- **预期收益**：文档同时服务于人类心智模型与 agent 检索（来源 3 的核心论点）；「刻意不做」是来源 3 点名的人类文档职责。
- **注**：现有 README 与 AGENTS.md 结构已近似三层，改动主要是显式化与补「刻意不做」。

### 建议 11（J）— 模型无关与定期复核

- **改动位置**：三份 AGENTS.md §12
- **改动内容**：新增「禁止累积针对特定模型的补丁式指令（prompt debt）。条款须模型无关。模型大版本升级后复核本文件，删除不再适用的条款。」
- **预期收益**：避免模型锁定与陈旧条款；与主题 J 的实证（企业停留在旧模型）闭环。
- **依据**：来源 2、9；来源 8（条件加载说明指令应随能力装配）。

### 建议 12（F）— 分层边界即并行任务边界

- **改动位置**：三份 AGENTS.md §6
- **改动内容**：新增「分层边界同时是多 agent 并行开发的任务边界。单个 agent 任务不应跨层。必须跨层的变更须显式声明并人工协调。」
- **预期收益**：把现有分层结构复用到多 agent 并行工作流（来源 7 点名的价值）；与现有 import-linter/dependency-cruiser 边界一致，无额外工具成本。

---

## 四、不确定性与例外

1. **未纳入主分析的文章（均在范围内但未读或不可用于模板）**：
   - dbreunig：Dr. Skill/loadout（2026-07-24）、AI 生态 Pace Layers（2026-07-03）、Cybersecurity as Proof of Work（2026-04-14）等——抓取主页时见标题，未展开阅读，不计入结论。
   - lucumr：Better Models: Worse Tools（2026-07-04，工具调用回归，与主题 J 高度相关）、The Coming Loop（2026-06-23）、Building Pi With Pi（2026-05-24）等——**建议作为下一轮跟进**，本轮未读。
   - 「AI Changes Everything」（来源 15，2025-06-04）：通论性乐观主义文章，无模板可执行点，仅作背景。
2. **二手数据未独立核验**：主题中所有具体数字（100k/32k token 阈值、46→19 工具、39% 掉分、90.2%、54%、44%、3 倍精度、50% GPT-4o 流量等）均转述自来源博客正文，未打开原始论文/报告核验。若用于宽松引用无妨；若写入模板条款（如建议 6 的阈值），应视为待实测的推断值。
3. **断链/墙**：未遇到失效链接；两博客正文内链均可达（如来源 5 由来源 4 正文外链跟进获得）。
4. **推断性内容标注**：
   - 建议 6 的「200 行」体量阈值：**推断**（来源仅给出「earn its keep」原则，无数值）。
   - 建议 3 的 pre-commit 子集划分（审计留 CI）：**推断**（来源的 commit-fail 机制未讨论重量级权衡）。
   - 建议 10 的三层结构与现有模板的映射：**推断映射**（来源 3 讨论的是文档方法论，模板结构是我们既有的）。
   - 主题 K 的「Rust 模板因此更有优势」：**推断**——来源 13 提供趋势观察，未评价模板选择。
5. **与模板无关而排除**：Fable 5.1 提示篇（来源 9）的大量内容是对话产品行为规范（emoji、危机应对、金融信息定义），仅取「指令随代际被训练吸收/回归」一点用于主题 J。

---

## 五、建议优先级与落地次序（实施参考）

| 优先级 | 建议 | 理由 |
|---|---|---|
| P0 | 4（spec 面 + 需求级覆盖）、2（注释纪律）、1（HITL 阈值） | 填补结构性缺口 / 引入可判定规则，均有直接来源规则原文 |
| P1 | 3（pre-commit 检查点）、6（体量预算）、11（模型无关） | 强制性与可持续性，但需实测与权衡 |
| P2 | 5（行为测试）、7（脚本优先固化）、12（并行边界） | 补强与固化既有设计 |
| P3 | 8（uv 拦截器，可选）、9（自动化卫生）、10（三层文档显式化） | 锦上添花或语境受限 |

所有改动应遵循既有迭代审查流程（写入 → subagent 审查 → 无异议后合入），并遵守 §12（人工维护、分层引用）与 §13（受控写作纪律）。
