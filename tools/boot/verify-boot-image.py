#!/usr/bin/env python3
"""Validate the CHUT-OS BIOS image layout without executing it."""

from __future__ import annotations

import argparse
from pathlib import Path


SECTOR_SIZE = 512
STAGE1_SIZE = SECTOR_SIZE
STAGE2_SIZE = 8 * SECTOR_SIZE
KERNEL_SIZE = 8 * SECTOR_SIZE
EXPECTED_SIZE = STAGE1_SIZE + STAGE2_SIZE + KERNEL_SIZE
KERNEL_MARKER = b"CHUT-OS: freestanding x86_64 kernel payload online"


def verify_image(image: Path) -> None:
	data = image.read_bytes()
	errors: list[str] = []

	if len(data) != EXPECTED_SIZE:
		errors.append(f"expected {EXPECTED_SIZE} bytes, got {len(data)}")
	if len(data) >= STAGE1_SIZE and data[510:512] != b"\x55\xaa":
		errors.append("missing BIOS boot signature")
	kernel_start = STAGE1_SIZE + STAGE2_SIZE
	kernel_end = kernel_start + KERNEL_SIZE
	if len(data) >= kernel_end and KERNEL_MARKER not in data[kernel_start:kernel_end]:
		errors.append("kernel payload marker missing from kernel region")

	if errors:
		raise SystemExit("boot image verification failed: " + "; ".join(errors))
	print(f"verified {image} ({len(data)} bytes)")


def main() -> None:
	parser = argparse.ArgumentParser(description=__doc__)
	parser.add_argument("image", type=Path)
	args = parser.parse_args()
	verify_image(args.image)


if __name__ == "__main__":
	main()
