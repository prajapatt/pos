# POS

POS is a Rust-based operating system project focused on building a real x86_64 kernel foundation, including boot flow, memory management, scheduling, process modeling, interrupts, and startup services.

## Project status

This repository is currently a kernel prototype and active engineering project, not a complete consumer operating system.

The codebase includes a real boot-first architecture foundation with:

- x86_64 boot and interrupt primitives
- kernel entry and panic reporting
- memory page and frame allocation logic
- virtual memory mapping support
- fixed-capacity round-robin scheduler
- task, thread, and CPU state models
- process lifecycle and process table logic
- timer and kernel tick integration
- serial output for low-level diagnostics
- startup service registration and kernel runtime states

## Current direction

The project is being developed in a boot-first sequence:

1. firmware / bootloader initialization
2. kernel entry and serial output
3. CPU feature detection
4. memory and paging setup
5. interrupts and timer infrastructure
6. scheduler and task execution
7. process lifecycle and userland preparation
8. service startup and higher-level OS runtime

## Design philosophy

POS aims to remain grounded in real kernel engineering instead of mock desktop-shell behavior or placeholder system code. Each subsystem is intended to map to a real operating system capability.

## Repository note

This project is suitable for a growing kernel and boot chain, but it is not yet a complete bootable operating system ready for a final production release claim.

It should be treated as an active prototype with real architecture foundations and gradual runtime maturation.

## Verification status

The repository does not yet claim full OS completion. The next milestones are:

- successful build verification
- passing unit and integration tests
- QEMU boot validation
- serial startup confirmation
- final bootable artifact generation

## PR / contribution guidance

This repository is best used as a work-in-progress kernel project or draft PR topic until the runtime validation milestones above are complete.
