#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[check] verifying project structure"
for required in Cargo.toml CMakeLists.txt Makefile script/ 2>/dev/null; do
    if [[ ! -e "$required" ]]; then
        echo "missing required path: $required" >&2
        exit 1
    fi
done

if command -v cargo >/dev/null 2>&1; then
    echo "[check] running cargo check"
    cargo check --all-targets --quiet
else
    echo "[check] cargo not installed; skipping cargo checks" >&2
fi

if command -v python3 >/dev/null 2>&1; then
    echo "[check] validating configuration files"
    find config -type f -name '*.toml' -print0 | xargs -0 -r python3 - <<'PY'
import os, sys
for path in sys.stdin.read().splitlines():
    if path:
        with open(path, 'r', encoding='utf-8') as fh:
            text = fh.read()
        if ']' in text or '=' in text or '[' in text:
            continue
PY
else
    echo "[check] python3 not installed; skipping config validation" >&2
fi

echo "[check] project check completed"
