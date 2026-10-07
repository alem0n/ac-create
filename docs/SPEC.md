# SPEC — ac_create

> 记录项目意图（what/why）与需求原子清单。非平凡变更先在此记录意图。实现产生的新决策必须回写本文件。
> 本工具是 create_project.py 的 Rust 完全移植：按语言与项目名创建新项目并自动替换占位名。

## 意图

提供项目意图的单一来源。spec 定义需求。测试验证需求。代码实现需求。三者随演进保持同步（SDD 三角）。

模板负载存于仓库 `templates/` 目录（python / typescript / rust 三套），随二进制一同分发。

## 需求清单

### REQ-001 项目名校验

- 项目名匹配 `[a-z][a-z0-9_-]*`。否则 `CreateError::InvalidProjectName`，退出码 1。
- 首字符限定为小写字母：三语言派生的标识符均须合法（避免 1app 类名字）。
- 校验在 `ProjectName::parse` 中完成（CLI 解析时即拒绝非法输入）。

### REQ-002 语言与包名映射

- 支持语言：python | typescript | rust。其他输入返回 `CreateError::UnsupportedLanguage`。
- `package_name`：python / rust 将连字符转下划线（标识符合法性）；typescript 保留用户输入。

### REQ-003 模板定位（便携分发约定）

- 分发包布局：二进制与 `templates/` 同级（`etc/build-dist.sh` 每次构建复制 `templates/`）。
- 定位顺序：可执行文件同级 `templates/` → 当前工作目录 `templates/`。均未找到返回 `TemplatesNotFound`。

### REQ-004 模板复制

- 递归复制 `templates/<语言>/` 到 `<target>/<项目名>/`。
- 按条目名跳过构建产物与缓存（.venv / node_modules / target / __pycache__ / 各类缓存 / .git / .coverage 等），不进入新项目。
- 目标目录已存在时返回 `DestinationExists`；--target 指定目录不存在时返回 `TargetMissing`；模板目录缺失时返回 `TemplateMissing`。

### REQ-005 占位名替换

- 全树替换 `myapp` → 包名：覆盖所有「判定为文本」的文件（二进制后缀、超 5 MB、含 NUL 字节的文件静默跳过；非 UTF-8 文件跳过并警告）。
- 报告已替换文件数。
- Python：替换前先重命名 `src/myapp/` → `src/<包名>/`；缺失时返回 `PackageDirMissing`。
- Rust：另将 `Cargo.toml` 的 `name = "<蛇形名>"` 改回 `name = "<用户输入>"`。crate 发布名保留连字符，与代码标识符解耦。
- TypeScript：仅做全树替换（覆盖 package.json 的 name）。

### REQ-006 LICENSE 占位替换

- 存在 `LICENSE` 时，将 `<替换为实际版权持有者>` 替换为 `--author`；未提供时使用项目名。
- LICENSE 非 UTF-8 时跳过并警告，不失败。

### REQ-007 锁文件重建

- python：`uv sync --extra dev`；typescript：`pnpm install`；rust：`cargo generate-lockfile`。在占位替换后的项目目录中执行。
- 命令缺失或失败时仅警告（提示手动重建），保留文本替换后的旧锁，整体不失败。
- 子进程默认 600 秒超时，超时终止子进程并提示。

### REQ-008 可选 git 初始化

- `--git`：`git init -b main` → `git add .` → `git commit -m "feat: scaffold project from template"`。
- `git init` 失败（如 git 不可用）时警告并跳过后续命令；提交失败时警告并携带命令输出。

### REQ-009 可选 make check 初验

- `--check`：在新项目目录运行 `make check`，输出 `全绿` / `失败`；失败时原样印出命令输出。
- 结果只作初验提示，不影响创建成功与退出码。

### REQ-010 面向用户的输出

- 进度按 5 步流水线输出：`[n/5] ...`；警告前缀 `警告：`；错误输出到 stderr 前缀 `错误：`，退出码 1。
- 成功后输出完成路径与后续步骤（cd、启用 git hooks、按语言的同步命令、替换 SECURITY.md 联系方式）。
- lib 代码禁止 println!（编译期 lint 强制）；用户可读输出经表现端口 `Reporter`，实现在组合层 main.rs。

### REQ-011 配置默认值

- 环境变量 `RUST_LOG` 未设置时，`log_level` 默认为 `warn`。CLI 的诊断日志须经 `RUST_LOG` 显式开启（info / debug）。

### REQ-012 分层与可测试性

- domain（值对象 / 领域错误 / 端口定义）→ services（用例编排，只经端口依赖外部）→ infrastructure（fs / 子进程 / 模板定位实现）。
- 子进程在测试中以假实现替换：不触发真实的 uv / pnpm / cargo / git / make 与网络访问。

## 设计决策

- 从 Python 脚本移植为 Rust 二进制（clap CLI）。脚本同级的模板定位改为可执行文件同级 + 当前目录两级回退。
- 便携分发：`templates/` 每次构建复制到分发目录（`etc/build-dist.sh`），归档为 tar.gz，三平台一致解压布局 `ac-create/{二进制, templates/}`。
- CI：Linux 承担完整质量门禁（fmt + clippy + test + audit）与测试编译；Linux / Windows / macOS 三平台各自打包。tag（v*）触发 GitHub Release 汇聚三平台归档。
- 依赖倒置：端口定义在 domain 层，避免 services 绑定具体 IO 实现，单测以假 Runner 与真实文件系统（临时目录）覆盖端到端路径。
- 锁文件重建失败不阻断创建：与 Python 版一致，工具不可用时回退为文本替换并警告。
