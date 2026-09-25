#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-dev}"
OUT_DIR="${2:-$ROOT_DIR/dist}"
mkdir -p "$OUT_DIR"

if command -v cargo >/dev/null 2>&1; then
    echo "[release] building kernel release artifacts"
    cargo build --release
fi

mkdir -p "$OUT_DIR/kernel" "$OUT_DIR/tools"
cp -f "$ROOT_DIR/Cargo.toml" "$OUT_DIR/"
find "$ROOT_DIR" -maxdepth 2 -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.sh' \) -exec cp -f {} "$OUT_DIR/kernel/" \; 2>/dev/null || true

cat > "$OUT_DIR/manifest.txt" <<EOF
Chut OS release manifest
version: ${VERSION}
root: ${ROOT_DIR}
EOF

echo "[release] artifacts prepared in $OUT_DIR"
