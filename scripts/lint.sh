#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if command -v cargo >/dev/null 2>&1; then
    echo "[lint] cargo clippy check"
    cargo clippy --all-targets --all-features -- -D warnings || {
        echo "[lint] clippy warnings treated as errors" >&2
        exit 1
    }
else
    echo "[lint] cargo not installed; skipping clippy" >&2
fi

find . -type f \( -name '*.sh' -o -name '*.bash' \) -not -path './.git/*' -print0 | while IFS= read -r -d '' script; do
    if ! bash -n "$script" >/dev/null 2>&1; then
        echo "[lint] invalid shell syntax: $script" >&2
        exit 1
    fi
done

echo "[lint] lint passed"
