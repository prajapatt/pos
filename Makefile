.PHONY: build test config-check bios qemu kernel-test clean

BUILD_DIR ?= build/host

build:
	cmake -S . -B $(BUILD_DIR)
	cmake --build $(BUILD_DIR)

kernel-test:
	cargo test --lib --quiet

test: build kernel-test
	ctest --test-dir $(BUILD_DIR) --output-on-failure

config-check:
	python tools/config/validate.py

bios:
	powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\build\build.ps1

qemu: bios
	powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\testing\run-qemu.ps1

clean:
	cmake --build $(BUILD_DIR) --target clean
	cargo clean || true
