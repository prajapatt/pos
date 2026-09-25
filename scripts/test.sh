#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[test] running Rust unit tests"
if command -v cargo >/dev/null 2>&1; then
    cargo test --all-targets --quiet
else
    echo "[test] cargo not installed; skipping Rust tests" >&2
fi

if command -v python3 >/dev/null 2>&1; then
    echo "[test] validating script syntax"
    find . -type f -name '*.sh' -not -path './.git/*' -print0 | while IFS= read -r -d '' script; do
        bash -n "$script"
    done
else
    echo "[test] python3 not installed; skipping script validation" >&2
fi

echo "[test] finished"
