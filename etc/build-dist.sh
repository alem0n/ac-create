#!/usr/bin/env bash
# 组装便携分发目录：release 二进制 + templates/。
#
# 便携约定：程序按「可执行文件同级 templates/」定位模板
# （见 src/infrastructure/templates.rs）。本脚本保证每次构建都把
# templates/ 复制到分发目录，防止程序找不到模板位置。
#
# 用法：etc/build-dist.sh [归档后缀]
#   后缀用于区分平台产物，默认 local。归档输出 dist/ac-create-<版本>-<后缀>.<归档格式>
#   归档格式按平台选择：Windows 为 zip（Windows 系统原生可解），Linux / macOS 为 tar.gz。
# 三平台（Linux / Windows / macOS）均可运行（Windows 需在 Git Bash 中执行）。

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
SUFFIX="${1:-local}"

BIN_NAME="ac-create"
ARCHIVE_EXT="tar.gz"
if [ -n "${OS:-}" ] && [ "${OS}" = "Windows_NT" ]; then
  BIN_NAME="ac-create.exe"
  ARCHIVE_EXT="zip"
fi

VERSION="$(grep -m1 '^version' "$ROOT/Cargo.toml" | sed -E 's/^version *= *"([^"]*)".*$/\1/')"
if [ -z "${VERSION}" ]; then
  echo "错误：无法从 Cargo.toml 解析版本号。" >&2
  exit 1
fi

cd "$ROOT"
cargo build --release --locked

DIST="$ROOT/dist"
rm -rf "$DIST"
mkdir -p "$DIST/ac-create"

cp "target/release/${BIN_NAME}" "$DIST/ac-create/"
cp -r "$ROOT/templates" "$DIST/ac-create/templates"

ARCHIVE="$DIST/ac-create-${VERSION}-${SUFFIX}.${ARCHIVE_EXT}"
rm -f "$ARCHIVE"
if [ "$ARCHIVE_EXT" = "zip" ]; then
  # Git Bash 不自带 zip 命令，改用 PowerShell 原生压缩。
  STAGE_WIN="$(cygpath -w "$DIST/ac-create")"
  ARCHIVE_WIN="$(cygpath -w "$ARCHIVE")"
  powershell.exe -NoProfile -Command "\$ErrorActionPreference='Stop'; Compress-Archive -Path '${STAGE_WIN}' -DestinationPath '${ARCHIVE_WIN}' -Force"
else
  tar -C "$DIST" -czf "$ARCHIVE" ac-create
fi
[ -f "$ARCHIVE" ] || { echo "错误：归档生成失败：${ARCHIVE}" >&2; exit 1; }

echo "分发包：${ARCHIVE}"
echo "便携布局：${ARCHIVE} 解压后为 ac-create/{${BIN_NAME}, templates/}"
