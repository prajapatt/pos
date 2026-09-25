#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if command -v rustfmt >/dev/null 2>&1; then
    echo "[format] rustfmt"
    find . -type f \( -name '*.rs' -o -name '*.toml' \) -not -path './.git/*' -print0 | xargs -0 -r rustfmt --edition 2021
else
    echo "[format] rustfmt not installed; skipping Rust formatting" >&2
fi

if command -v shfmt >/dev/null 2>&1; then
    echo "[format] shfmt"
    find . -type f -name '*.sh' -not -path './.git/*' -print0 | xargs -0 -r shfmt -w
else
    echo "[format] shfmt not installed; skipping shell formatting" >&2
fi

echo "[format] formatting finished"
