# AI Agent 生成软件工程 · 项目模板

本目录提供三套可直接复用的项目模板，每套含完整目录结构与 `AGENTS.md` 规范文件。

> 本模板基于 2026 年公开的「AI Agent 生成软件工程最佳实践」调研生成，调研结论见 [`RESEARCH_FINDINGS.md`](./RESEARCH_FINDINGS.md)（含 X 社区案例补充轮。已经多轮 subagent 迭代审查至无异议。）。

## 模板入口（渐进式披露：先读 AGENTS.md，再按需深入配置文件）

| 语言 | 入口 | 核心 check 命令 |
|---|---|---|
| Python | [`templates/python/AGENTS.md`](./templates/python/AGENTS.md) | `make check` |
| TypeScript | [`templates/typescript/AGENTS.md`](./templates/typescript/AGENTS.md) | `make check` |
| Rust | [`templates/rust/AGENTS.md`](./templates/rust/AGENTS.md) | `make check` |

## 使用方式

1. 从 `templates/` 复制对应语言目录到新项目根目录（或用本仓库的 `ac-create` 二进制自动完成）。将其中的 `myapp` 占位名替换为实际项目名。
2. 阅读该目录下的 `AGENTS.md`。它是给开发者与 AI coding agent 的共同指令文件（AGENTS.md 开放标准）。
3. 按 AGENTS.md「环境与命令」一节安装依赖并运行 `make check`。确认全绿后开始开发。
4. 执行一次 `git config core.hooksPath .githooks`。此后提交前自动运行质量门禁快速子集与密钥扫描，提交信息自动按 Conventional Commits 校验。

## 快速创建项目（CLI）

本仓库附带 `ac-create` 二进制（Rust 实现，移植自 create_project.py）。指定语言与项目名即可拉取模板并自动替换占位名。

```bash
ac-create <python|typescript|rust> <项目名> [--author <名字>] [--target <目录>] [--git] [--check]

# 示例
ac-create python my-cool-app --git --check
ac-create rust my-cool-app --author "作者名"
```

命名映射自动化：
- Python：包名转 snake_case（`my-cool-app` → `my_cool_app`），重命名 `src/myapp/` 并替换全部导入与配置。
- Rust：crate 名保留连字符形式，代码标识符转 snake_case（Cargo 自动映射），重建 `Cargo.lock`。
- TypeScript：替换 `package.json` 的 `name`，重建 `pnpm-lock.yaml`。

锁文件由对应包管理器重建（uv / pnpm / cargo）。工具不可用时回退为文本替换并警告。`--git` 初始化仓库并作首次 `feat:` 提交（须先配置 git 身份）。`--check` 创建后自动运行 `make check` 初验。

### 构建与便携分发

从源码构建：

```bash
make dist          # 输出 dist/ac-create-<版本>-<平台>.tar.gz
# 或手动：cargo build --release
```

CI（`.github/workflows/ci.yml`）在推送与 PR 时为 Linux / Windows / macOS 三平台各打一份便携包；Linux 另跑完整质量门禁（`make check`）。打 tag（`v*`）时发布 GitHub Release 汇聚三平台归档。

便携安装：解压归档后得到 `ac-create/{二进制, templates/}`。程序按「可执行文件同级 `templates/`」定位模板。请勿将二进制单独挪走而遗漏 `templates/`；若从仓库目录运行（开发态），也会使用当前目录下的 `templates/`。

## 文档分层（Getting Started / Diving Deeper / Reference）

- **Getting Started**：本 README 与各模板 AGENTS.md 的 §1–§2（概览与命令）。
- **Diving Deeper**：AGENTS.md 其余各节（按主题的规范与设计决策）。
- **Reference**：配置文件（`pyproject.toml` / `package.json` / `Cargo.toml` 等，AGENTS.md §11 列出）。

人类文档建立心智模型与设计决策。agent 文档（AGENTS.md）提供可执行指令。两者经 AGENTS.md §11 链接。

## 设计原则（三套通用）

- **AGENTS.md 为唯一规范入口**：构建/测试/lint 命令、代码风格、日志/测试/分层/错误/安全规范集中于此。agent 会主动执行其中列出的程序化检查。
- **确定性质量门禁**：`make check` 统一收口 lint + 类型检查 + 格式化 + 测试（含覆盖率阈值）+ 依赖审计。提交前必须全绿。
- **高内聚低耦合**：每套模板按 `domain → services → infrastructure` 分层。依赖方向单向指向 domain。依赖方向由工具机器校验。
- **渐进式文件披露**：README/AGENTS.md 只列入口与按需深入路径。具体规则细节落在对应配置文件中，不在单一文件堆砌。
