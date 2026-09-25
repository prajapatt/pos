#!/usr/bin/env python3
"""Validate CHUT-OS TOML configuration invariants."""

from pathlib import Path
import sys
import tomllib


ROOT = Path(__file__).resolve().parents[2]
CONFIG = ROOT / "config"
REQUIRED = {
    "build.toml": ("build", "tools"),
    "kernel.toml": ("kernel", "memory", "scheduler"),
    "graphics.toml": ("display", "gpu"),
    "network.toml": ("network", "dns"),
    "audio.toml": ("audio",),
    "power.toml": ("power", "cpu"),
    "security.toml": ("security", "filesystem"),
    "gaming.toml": ("gaming", "policy"),
    "ai.toml": ("ai", "local", "openrouter"),
}


def main() -> int:
    errors: list[str] = []
    for filename, sections in REQUIRED.items():
        path = CONFIG / filename
        try:
            data = tomllib.loads(path.read_text(encoding="utf-8"))
        except (OSError, tomllib.TOMLDecodeError) as error:
            errors.append(f"{filename}: {error}")
            continue
        for section in sections:
            if section not in data or not isinstance(data[section], dict):
                errors.append(f"{filename}: missing [{section}]")

    kernel = tomllib.loads((CONFIG / "kernel.toml").read_text(encoding="utf-8"))
    if kernel["kernel"]["page_size"] != 4096:
        errors.append("kernel.toml: page_size must remain 4096")
    if kernel["scheduler"]["policy"] != "round_robin":
        errors.append("kernel.toml: unsupported scheduler policy")

    ai = tomllib.loads((CONFIG / "ai.toml").read_text(encoding="utf-8"))
    if not ai["openrouter"]["endpoint"].startswith("https://"):
        errors.append("ai.toml: OpenRouter endpoint must use HTTPS")

    if errors:
        for error in errors:
            print(f"config error: {error}", file=sys.stderr)
        return 1
    print(f"validated {len(REQUIRED)} CHUT-OS configuration files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())