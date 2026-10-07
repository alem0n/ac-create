# 全面调研：项目模板的基础设施、Git 全链与脚手架生态

> 本轮针对【项目模板】做全面调研，纠正上轮「只关注高级/热点内容」的片面性，优先补齐基础与核心项（尤其 Git）。
> 每部分给出：关键结论、依据/来源、示例、适用场景；末尾做覆盖度自查与当前模板差距分析。

---

## 一、基础概念：项目模板与脚手架

**关键结论**
- 项目模板（脚手架，scaffolding）= 从预置结构生成新项目骨架的机制，核心价值是**一致性、合规前置、启动速度**。
- 脚手架已分化为两代：**一次性生成器**（cookiecutter、yeoman、create-\*）与**代码生命周期管理工具**（copier：生成 + 模板更新 + 迁移）。
- 模板的三种失效模式：① 模板腐化（与真实项目脱节、长期不更新）；② 无法同步（生成的项目无法吸收模板修订）；③ boilerplate 陷阱（生成物含大量本项目不需要的代码）。

**依据/来源**：copier 官方对比页（"Although Copier was born as a code scaffolding tool, it is today a code lifecycle management tool"；"Template updates: Copier Yes / Cookiecutter No / Yeoman No"）；Wikipedia「Scaffold (programming)」；cookiecutter 替代清单一文。

**示例**：`cookiecutter gh:user/repo` 生成项目；`copier copy gh:user/template ./myproject` 后续可用 `copier update` 拉取模板修订（依赖 git tag 做版本差分）。

**适用场景**：团队多项目起步收敛结构；个人快速启动；开源社区降低贡献门槛（如 create-react-app 之于 React 生态）。

---

## 二、必备工具/基础设施

### 2.1 Git——定义、核心概念与常识

**关键结论**
- 定义：Linus Torvalds 2005 年创建的**分布式版本控制系统**（DVCS），设计上极端灵活（分支可任意创建）。
- 三个核心分区模型：**工作区 → 暂存区（index）→ 仓库**；提交是快照而非差异。
- 核心概念清单（模板须文档化的最小集）：仓库/远程（remote）、分支（branch）、标签（tag）、提交（commit）、暂存（staging）、合并（merge）、变基（rebase）、HEAD、分离 HEAD（detached HEAD）、干净/脏（clean/dirty）。

**依据/来源**：codewithmukesh 工作流指南（"When Git was created by Linus Torvalds in 2005, it was designed to be extremely flexible"）；git-scm 官方文档（gitattributes 等）。

### 2.2 Git 常用命令（模板须提供速查的最小集）

| 分类 | 命令 |
|---|---|
| 日常 | `clone` `status` `add` `commit` `push` `pull` `fetch` |
| 分支 | `branch` `switch`/`checkout` `merge` `rebase` |
| 历史 | `log` `diff` `blame` `show` |
| 修复 | `revert` `reset` `cherry-pick` `reflog` |
| 远程/发布 | `remote` `tag` `push --tags` |

**示例**：`git switch -c feat/login && git commit -m "feat(auth): add login" && git push -u origin feat/login`（遵守 Conventional Commits 的日常流）。

### 2.3 三大工作流对比

**关键结论**
- **GitFlow**（Vincent Driessen, 2010）：main/develop/feature/release/hotfix 五种长命分支。适合**版本化发布、多版本并存、受监管行业**（桌面/移动应用、医疗金融）；对持续部署的 web 应用是过度设计。JetBrains 2023 调查显示使用率约 22%（转述数据）。
- **GitHub Flow**：单一规则——**main 永远可部署** + 短命 feature 分支 + PR。适合持续部署的 SaaS/web 应用、中小团队、高频合并。
- **Trunk-Based Development（TBD）**：所有人每天至少一次提交到主干 + feature flags 隔离未完成工作。DORA 数据显示精英团队部署频率可达低效团队的 182 倍，但**前提是强测试自动化（建议 70%+ 覆盖率）与成熟 CI/CD**；测试薄弱或团队不成熟时不应采用。
- **模板默认推荐**：GitHub Flow（简单、与模板「main 可部署 + make check 门禁」语义一致）；随团队成熟向 TBD 演进。

**依据/来源**：codewithmukesh（含 nvie 原始文章引用、GitHub 官方 flow 文档引用、DORA 数据）；Atlassian TBD 词条；GitKraken 2026 决策指南。

**示例/适用场景**：GitFlow 的热修场景——从 v2.3 标签切 `hotfix/xxx`，修复合并回 main 与 develop，发 2.3.1。web 应用团队应直接 GitHub Flow：`feat/xxx` → PR → CI 全绿 → 合并 main → 自动部署。

### 2.4 Git 生态工具

**关键结论**
- **钩子管理**：`husky`（JS 生态管理 git hooks，设 `core.hooksPath`）+ `lint-staged`（**只检查暂存文件**，保证 pre-commit 速度）+ `commitlint`（校验提交信息格式）。Python 生态用 `pre-commit` 框架（多语言钩子）。原则：**pre-commit 做 staged-only 的快速检查；提交信息校验放 commit-msg 钩子**。
- **大文件**：
  - GitHub 硬限制：单文件 50 MiB 警告、100 MiB 阻止、仓库建议 ≤1 GB；LFS 单文件上限 5 GB（codenote 转述，落地前对照当前 GitHub 文档复核；常见口径有 2 GB/5 GB 两种说法）。
  - **Git LFS**：指针 + clean/smudge 过滤器，字节存 LFS 服务器；forge 集成最好；**不 diff 二进制**（改一字节存全量新对象）；对已入库文件启用需 `git lfs migrate` 重写历史；计费按存储 + 带宽，100 GB 量级月成本可达两位数美元、1 TB 量级三位数。
  - **git-annex**：内容寻址、分布式、location tracking（记录哪份内容在哪个远程），即时克隆；学习曲线陡、symlink 模型、Windows 支持弱。
  - **DVC**：ML 导向（metafile + 远程存储 + 可复现管线）。
  - 替代/省钱路线：GitHub Releases（免费带宽、tag 粒度、资产不入工作树；>2 GiB 需 split）、Cloudflare R2 自托管 LFS 代理（1 TB 月成本 <$15）、Hugging Face Hub（公开 ML 资产免费）。
  - 决策线：数据单机放得下且在 forge 上 → LFS；跨盘/跨地归档 → annex；ML 数据集+管线 → DVC；两条都符合选更简单的。
- **密钥扫描**：`gitleaks`（即时、全离线、pre-commit 友好，常见默认选择）vs `trufflehog`（**主动验证**凭据有效性以减少误报，需联网）。泄露密钥进历史是最常见的初始访问向量之一；CI 侧需 `git lfs pull` 类似的显式拉取与缓存注意。

**依据/来源**：bigiron.cc「git-lfs vs git-annex vs DVC」（含三工具对照表）；codenote.net（2026-07 对齐 GitHub 官方文档与定价；含 10/100/1000 GB 成本表）；rehansaeed（2019/2020，.gitattributes 与 LFS 配置）；husky/lint-staged/commitlint 官方与 betterstack/stevekinney 指南；gitleaks 与 trufflehog 官方文档 + rafter.so 对比。

**示例**：`.gitattributes` 中 `*.png filter=lfs diff=lfs merge=lfs -text`；gitleaks/pre-commit 阻止提交含 `AKIA...` 的行。

### 2.5 与同类工具对比（SVN / Mercurial / Perforce）

**关键结论**
- **Git vs SVN**：Git 分布式（本地完整历史、离线可提交、快）；SVN 集中式（需服务器、断网受限、**细粒度目录级权限控制是真实优势**，部分企业因此保留）。
- **Perforce（Helix Core）**：集中式，**大规模二进制资产与文件锁定语义强**（游戏/影视 Unreal/Unity 集成好）；1 TB 级二进制场景是离开 Git 生态的现实理由。
- **Mercurial**：与 Git 同代 DVCS，设计与命令更一致，但生态与社区收缩（Bitbucket 弃用后进一步边缘化）。
- 结论：**项目模板默认 Git**（生态与工具链事实标准）；仅当「大二进制 + 锁定语义」或「目录级 ACL 合规」时才考虑 Perforce/SVN，并在模板选型中显式声明。

**依据/来源**：nulab、perforce.com、gitkraken、stackoverflow.blog「Beyond Git」（2023）、rhodecode 2025 VCS 流行度（厂商博客，谨慎采信）。

### 2.6 其他必备基础设施

- **锁文件**：`uv.lock` / `pnpm-lock.yaml` / `Cargo.lock`（可复现构建，模板必须提交）——当前模板已具备。
- **`.editorconfig`**：跨编辑器一致的缩进/行尾/字符集；**即使有 Prettier/Ruff 仍有价值**（覆盖非源码文件与不跑格式化器的场景）。
- **`.gitattributes`**：行尾归一化（`* text=auto`）+ 脚本强制行尾（`*.sh eol=lf`、`*.bat/.cmd eol=crlf`）+ LFS 规则。
- **CI**：`make check` 的云端执行（GitHub Actions 等）。
- **仓库标准文件**（GitHub 官方 best practices + 社区共识）：`README.md`、`LICENSE`、`CHANGELOG.md`、`SECURITY.md`、`CONTRIBUTING.md`、`.github/`（PR/issue 模板、CI 工作流）。

**依据/来源**：GitHub Docs「Best practices for repositories」（SECURITY.md 等）；editorconfig.org 与「Why .editorconfig still matters even with Prettier around」；rehansaeed。

---

## 三、核心机制：脚手架如何工作

**关键结论**
- **变量替换**：Jinja2（cookiecutter/copier）或 EJS（yeoman）渲染文件内容与文件名。
- **模板更新**是分水岭能力：copier 用**模板 git tag 做版本差分**，向已生成项目推送修订；cookiecutter 无此能力（需 cruft 补足）；yeoman 无。
- **迁移（migrations）**：copier 支持跨模板版本的数据迁移脚本；cookiecutter/yeoman 均不支持。
- **循环生成文件结构**：copier 支持（按数据生成目录树），cookiecutter/yeoman 不支持。
- **任务钩子**：三者都支持生成前后任务（如初始化 git、安装依赖）。

**依据/来源**：copier readthedocs 官方对比表（逐项 feature 矩阵，含模板更新/migrations/循环结构）。

**示例**：copier 模板含 `copier.yml`（单 YAML 配置，无需手写 JSON）+ `*.jinja` 文件；`copier update` 比对模板 tag 生成 diff 并应用。

**适用场景**：需要长期同步模板修订的团队 → copier；一次性快速起步 → cookiecutter/degit；JS 生态且需编程逻辑 → yeoman/plop。

---

## 四、常见方案：各语言脚手架矩阵

| 语言 | 主流方案 | 特点 |
|---|---|---|
| Python | **cookiecutter**（Jinja、JSON 配置、生态最大）；**copier**（YAML+Jinja、模板更新/迁移，正成为首选）；pyscaffold；cruft（补 cookiecutter 更新） | 2025+ 社区趋势：copier + uv + just（Medium 2025 文章标题即「From Cookiecutter to Copier, uv, and Just」） |
| JS/TS | **create-\* 官方脚手架**（create-react-app 等）；**yeoman**（JS 编程式、npm 包模板）；**plop**（轻量代码生成器）；**degit**（直接拉取仓库、不做模板渲染） | yeoman 需编程、需单独安装模板；degit 零渲染最简单 |
| Rust | **cargo-generate**（以 git 仓库为模板，类 yeoman 的 Rust 实现） | 与 cargo 生态深度集成 |
| 跨语言/声明式 | **projen**（声明式项目定义，AWS 系风格，适合团队级统一治理） | 模板即代码 |

**依据/来源**：copier 官方对比表；cookiecutter README；cargo-generate 官方文档；safjan.com「Cookiecutter alternatives」（yeoman/hygen/plop/slush/sao/jolt/boilr 清单）；Medium「From Cookiecutter to Copier」；LinkedIn 讨论（projen 用于团队数据科学模板）。

**示例**：`cargo generate gh:rust-github/template-rust`；`degit gh:user/template myapp`（无变量替换）。

---

## 五、典型场景

| 场景 | 方案组合 |
|---|---|
| 新应用/库 | 脚手架 + LICENSE + CHANGELOG + CI + make check 门禁 |
| 持续部署 web 应用 | GitHub Flow + TBD 倾向 + feature flags + 强 CI |
| 版本化发布产品（桌面/移动/企业软件） | GitFlow + tag + release automation（release-please/semantic-release） |
| monorepo | pnpm workspaces（TS）/ cargo workspace（Rust）/ 嵌套 AGENTS.md（agent 场景）——当前模板已保留扩展路径 |
| 大二进制资产（游戏/媒体） | LFS（<100 GB）→ R2 自托管代理（更大）或 Perforce（锁定语义） |
| ML 数据集/模型 | DVC + 远程对象存储；或 Hugging Face Hub（公开免费） |
| 多 agent 并行开发 | 分层边界即任务边界（当前模板 §6 既有） |

**依据/来源**：codewithmukesh 场景段；bigiron/codenote 场景决策线；既有 RESEARCH_FINDINGS（monorepo/嵌套 AGENTS.md、多 agent 边界）。

---

## 六、最佳实践（项目模板落点）

**关键结论**
1. **仓库必备文件**：README、LICENSE、CHANGELOG、SECURITY.md、CONTRIBUTING、.gitignore（用 github/gitignore 官方模板）、.gitattributes、.editorconfig。
2. **行尾纪律**：`.gitattributes` 首行 `* text=auto` 是几乎每个仓库都该有的默认；不依赖个人的 `core.autocrlf` 配置（配置错误会静默污染历史且 PR 不可见）。
3. **提交规范**：Conventional Commits（`<type>[scope]: <description>`；`feat`→SemVer MINOR、`fix`→PATCH、`BREAKING CHANGE:`/`!`→MAJOR），commitlint 强制。
4. **Changelog**：Keep a Changelog 2.0.0——为人类写、每版本有条目、最新在前、Unreleased 节、六类（Added/Changed/Deprecated/Removed/Fixed/Security）、ISO 日期、`## [x.y.z] - YYYY-MM-DD`。**明确反对**：把 git log 当 changelog；把 changelog 编辑做成强制检查（会制造噪声）；可由 git-cliff/release-please/changesets 从 Conventional Commits 自动生成草稿，**但「notable 与否的判断留给人**。
5. **提交检查点**：husky + lint-staged（staged-only 快速检查）+ commit-msg 钩子；检查点必须 commit-fail（"skill 是建议，工具必须是检查点"——既有 Plumb 结论）。
6. **密钥预防**：gitleaks pre-commit 扫描 + `.gitignore` 覆盖 `.env` + `.env.example` 只放占位符。
7. **锁文件提交**、**小而频繁的提交**、**分支短命化**（>2–3 天的 feature 分支冲突经验上高发）。

**依据/来源**：conventionalcommits.org 1.0.0（规范全文）；keepachangelog.com 2.0.0（含 LLM 章节）；rehansaeed .gitattributes；github/gitignore 仓库；betterstack/stevekinney husky+lint-staged；gitleaks 文档；既有 RESEARCH_FINDINGS §1.3 独立质量门禁 + Plumb commit-fail。

**示例（Keep a Changelog 骨架）**：
```
# Changelog
本项目遵守 [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) 与 [Semantic Versioning](https://semver.org/)。

## [Unreleased]
### Added
- 新增 X

## [0.1.0] - 2026-10-07
### Fixed
- 修复 Y
```

---

## 七、常见问题与陷阱

| 问题 | 原因 | 对策 |
|---|---|---|
| 跨平台「幽灵 diff」（无可见改动却提示修改） | CRLF/LF 不一致 | `.gitattributes`：`* text=auto`；不依赖个人 `autocrlf` |
| 长命 feature 分支合并冲突 | 分支存活 >2–3 天（经验上高发） | 拆小任务、feature flags、每日合并主干 |
| 密钥泄露进历史 | `.env`/硬编码误提交 | gitleaks pre-commit；已泄露需 BFG/`git filter-repo` 清理 + **轮换密钥**（清理不能替代轮换） |
| 仓库被大文件撑大 | 二进制进库即全量历史 | LFS（新项目）/ `git lfs migrate`（旧项目，需重写历史）；评估 R2/Releases 替代 |
| 模板与生成项目漂移 | 一次性生成器无更新机制 | copier（模板更新+migrations）；cruft for cookiecutter |
| CHANGELOG 变噪声 | 强制每变都写一行 | 只记 notable；自动化草稿 + 人工判断 |
| `git log` 当 changelog | 偷懒 | 一条提交记录代码步骤；changelog 记**用户体验到的差异** |
| CI 拉不到 LFS/DVC 内容 | 普通 clone 只得指针 | CI 显式 `git lfs pull`/`dvc pull` + 缓存 |
| Trunk-Based 失败 | 测试覆盖弱、无 CI 文化 | 先 GitHub Flow，覆盖率到 70%+ 再演进 |

**依据/来源**：stackoverflow CRLF 高票问题；rehansaeed；codewithmukesh troubleshooting 段；keepachangelog「What makes a changelog worse」「Don't make it a required check」；bigiron（CI 需 lfs pull + 缓存）；gitleaks/trufflehog 实践文。

---

## 八、竞品/替代方案对比（汇总矩阵）

**版本控制系统**

| | Git | SVN | Mercurial | Perforce |
|---|---|---|---|---|
| 模型 | 分布式 | 集中式 | 分布式 | 集中式 |
| 优势 | 生态/速度/分支廉价 | 目录级 ACL | 命令一致性好 | 大二进制+锁定语义 |
| 劣势 | 二进制弱、ACL 弱 | 离线受限 | 生态收缩 | 离开 Git 生态、商业许可 |
| 适合 | 默认全部 | 特定合规遗留 | 历史项目 | 游戏影视 |

**工作流**：GitFlow（版本化/监管）/ GitHub Flow（web/SaaS/中小团队）/ TBD（精英团队+强测试）

**大文件**：LFS（团队默认）/ git-annex（分布式归档）/ DVC（ML）/ Releases（分发）/ R2/S3（对象存储）

**密钥扫描**：gitleaks（默认/离线）/ trufflehog（主动验证）/ GitHub 原生 secret scanning（平台层）

**脚手架**：cookiecutter（生态）/ copier（更新+迁移，推荐）/ yeoman（JS 编程式）/ degit（零渲染）/ cargo-generate（Rust）/ projen（声明式）

**发布自动化**：semantic-release（JS 全自动）/ changesets（monorepo 友好）/ release-it / release-please（Google 系，Conventional Commits → CHANGELOG+release PR）

**规范文件标准**（agent 场景，既有结论）：AGENTS.md 开放标准（nearest wins、60k+ 仓库）vs CLAUDE.md vs .cursorrules（厂商私有，分层优先级机制不同）

**依据/来源**：各小节来源汇总（nulab/perforce/stackoverflow.blog/rhodecode；codewithmukesh；bigiron/codenote；rafter.so/appsecsanta；copier 官方表；pkgpulse 发布工具对比；既有 RESEARCH §1.1）。

---

## 九、发展趋势

1. **SDD/spec-driven 成为分支**：spec 作为 source of truth，agent 起草/验证/演进规范；工具化（OpenSpec、BMAD-METHOD 等 6 工具清单）；既有结论（SDD 三角：实现反过来改进 spec）得到更多工具支撑。
2. **脚手架 → 生命周期管理**：copier 界定自己不再是 scaffolder，而是 template updates + migrations（模板可随项目成长，缓解模板腐化）。
3. **changelog 与 LLM 结合**：Keep a Changelog 2.0.0 专门新增「Changelogs, automation, and LLMs」——一致性格式天然可被 LLM/工具解析；LLM 起草 + 人工判断 notable；明确「不要做成强制检查」。
4. **AI 生成脚手架/skills 化**：skills 即「给 agent 看的文档 + 可复用工作流」（既有 dbreunig 结论）；模板内容本身可由 agent 按规范文件生成并维持一致。
5. **模板即代码**：projen 类声明式定义项目结构，适合团队级治理。
6. **行尾/平台边界依旧**：跨平台开发（尤其 Windows/WSL 混合环境）依旧依赖 `.gitattributes` 这类「无聊但必需」的基础设施——并未因 AI 时代改变。

**依据/来源**：augmentcode SDD 工具清单；keepachangelog 2.0.0；copier docs；既有 BLOG_RESEARCH（skills/文档三层）；projen 社区讨论。

---

## 十、来源、时效性与不确定点

| 来源 | 时效性 | 不确定点 |
|---|---|---|
| conventionalcommits.org 1.0.0 | 稳定规范 | — |
| keepachangelog.com 2.0.0 | 当前版（含 LLM 章节） | — |
| copier readthedocs（stable） | 持续更新 | 对照表由 copier 维护，有立场倾向（文档自述可能 biased） |
| rehansaeed .gitattributes（2019-07/2020-08） | 基础机制稳定 | 日期较旧；corefx 链接为 master 分支可能漂移 |
| codewithmukesh 工作流指南 | 2025/2026 实践 | JetBrains「22% 用 GitFlow」为博客转述，未核对原始调查；DORA「182x」同 |
| bigiron.cc / codenote.net LFS 对比 | 对齐 2026-07 厂商文档与定价 | 计费数字随厂商变动，仅作量级参考；codenote 为个人博客 |
| GitHub Docs best-practices | 持续更新 | — |
| husky/lint-staged/commitlint/gitleaks/trufflehog 文档 | 当前 | — |
| rhodecode VCS 流行度 2025 | 厂商博客 | 自有利场（卖 Mercurial/GitLab 类方案），谨慎采信 |
| cargo-generate / yeoman / plop / degit 文档与社区文 | 当前 | 部分为 Reddit/ Medium 讨论，非官方 |
| augmentcode SDD 工具清单 | 2026 | 厂商内容，工具入选有商业立场 |

---

## 十一、覆盖度自查

| 维度 | 覆盖情况 | 遗漏/原因/补全 |
|---|---|---|
| 基础概念（模板/脚手架） | ✅ 定义、价值、失效模式 | 无重大遗漏 |
| Git 定义/核心概念/命令 | ✅ 本轮补齐 | 命令速查为最小集；fork 工作流未展开（模板场景不需要） |
| Git 工作流 | ✅ 三模型 + 决策 | GitLab Flow 未单列（混合模型，非必需）；「fork + PR」开源模式可后续补 |
| Git 生态（hooks/LFS/扫描） | ✅ | git submodules vs subtree 仅在搜索结果中出现标题、未深读——**标记为轻度覆盖**；如需可补 |
| 与 SVN/Perforce/Mercurial 对比 | ✅ | Bazaar/Fossil 未覆盖（已边缘化，非必需） |
| 其他基础设施（editorconfig/锁/CI/标准文件） | ✅ | CODE_OF_CONDUCT 未展开（开源场景才需要，模板可选） |
| 核心机制/常见方案/场景 | ✅ | 无重大遗漏；monorepo 专用工具（nx/turbo/lerna）未展开（超出三语言模板范围，见下） |
| 最佳实践/常见问题 | ✅ | 无重大遗漏 |
| 竞品/替代对比 | ✅ 矩阵化 | 发布自动化四个工具为标题级对比（pkgpulse），未逐个精读 |
| 发展趋势 | ✅ | 无重大遗漏 |
| **上轮偏差纠正** | ✅ 本轮 Git 等基础项为主轴 | 上轮只覆盖 AI/agent 高级主题，本轮补齐 |

**自查结论**：本轮已从基础（Git 命令/概念/行尾）到高级（SDD/LLM changelog）全链覆盖，主要残留为 git submodules/subtree 深度对比与 monorepo 工具链两个「可按需深入」分支，不影响模板决策。

---

## 十二、对当前模板的差距分析（可落地清单）

对照第二章与第六章的必备项，当前 `python/` `typescript/` `rust/` 三套模板存在以下**基础缺口**（按严重度排序）：

| # | 缺口 | 严重度 | 依据 |
|---|---|---|---|
| 1 | **无 CI 配置**：AGENTS.md §2 反复引用「CI 引用同一入口」但**没有任何 CI 文件**（.github/workflows） | 高 | §2.6 仓库标准文件；GitHub best practices |
| 2 | **无 `.gitattributes`**：行尾未归一化。模板用于 Windows/WSL 混合环境时将面临 CRLF 幽灵 diff 风险（仓库位于 `/mnt/c` 边界） | 高 | §2.6、§6.2、§7；rehansaeed |
| 3 | **无 `LICENSE`**：模板无许可占位 | 高 | §2.6 |
| 4 | **无提交规范文件**：Conventional Commits 未约定、无 commitmsg 钩子 | 中高 | §6.3；commitlint |
| 5 | **无 `CHANGELOG.md`**：Keep a Changelog 骨架缺失（且与 SemVer 绑定） | 中高 | §6.4 |
| 6 | **无密钥扫描**：`.env` 被 .gitignore 覆盖但无 gitleaks 主动扫描 | 中 | §6.6 |
| 7 | **无 `.editorconfig`**：跨编辑器一致性缺失（ruff/prettier 之外文件无约束） | 中 | §2.6 |
| 8 | **无 `SECURITY.md`**：安全报告渠道缺失（AGENTS.md §8 只有规范无文件） | 中 | §2.6；GitHub best practices |
| 9 | **无 `.github/`（PR/issue 模板）** | 中 | §2.6 |
| 10 | **无分支策略默认约定**：AGENTS.md 未声明 GitHub Flow 默认 | 中 | §2.3 |

另有一个已闭合的对照点：**提交检查点**（.githooks/pre-commit + make precommit）已在上一轮落地，方向与 §6.5 一致，仅需补 commit-msg 钩子（#4）。

**结论**：当前模板在「AI/agent 规范」维度较完备，但在「仓库基础设施」维度（CI、许可、行尾、提交/变更记录规范、密钥扫描）系统性缺失。建议按上表 1–5 优先落地——这正是本轮全面调研纠正上轮片面的直接产出。
