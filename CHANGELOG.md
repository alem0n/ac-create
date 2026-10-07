# Changelog

本项目遵守 [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) 与 [Semantic Versioning](https://semver.org/spec/v2.0.0.html)。

## [Unreleased]

### Changed

- Windows 分发包归档格式由 tar.gz 改为 zip（Windows 系统原生可解，无需额外工具）。Linux / macOS 保持 tar.gz，解压布局不变。

### Added

- 将 `create_project.py` 完全移植为 Rust 二进制 `ac-create`（clap CLI）：分层架构 + 端口注入，行为与 Python 版一致。
- 便携分发约定：二进制与 `templates/` 同级布置；新增 `etc/build-dist.sh` 与 `make dist`，每次构建自动复制 `templates/` 到分发目录。
- CI 扩展为三平台打包（Linux / Windows / macOS），Linux 承担完整质量门禁；`v*` tag 触发 GitHub Release 汇聚三平台归档。

## [0.1.0] - 2026-10-07

### Added

- 项目模板首版：分层目录、统一质量门禁（make check）、AGENTS.md 规范。
<!-- 版本链接：替换为项目的 compare/release 地址后使用。 -->
