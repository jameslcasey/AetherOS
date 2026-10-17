# AetherOS

**Production-ready next-generation AI-native operating system.**

AetherOS is a clean-slate Rust exokernel designed for AI workloads. It combines mathematical memory safety, object-capability security, content-addressable storage, fine-grained accelerator multiplexing (LithOS-inspired), and a native agent runtime (AIOS-inspired).

**Current status (Oct 17, 2026):** Bootable prototype skeleton that runs in QEMU on x86_64. Dual-arch path and full AI primitives are scaffolded for rapid iteration.

## Vision (v2.0)

- **Layer 0**: Verified Rust micro-core with lockless IPC rings and capability tokens.
- **Layer 1**: AetherFS — Blake3 content-addressable Merkle graph with tensor-aware huge pages and GPUDirect-style DMA.
- **Layer 2**: LithOS accelerator daemon — TPC-level spatial slicing, kernel atomization, power right-sizing.
- **Layer 3**: AIOS agent runtime — interruptible KV-cache snapshots, shared-prefix pooling, capability-gated MCP tools.
- **Layer 4**: AOT Wasm components + seamless Linux/Windows legacy bridge.

See `docs/ARCHITECTURE.md` for the full specification.

## Quick Start (Prototype)

### Prerequisites
- Rust nightly (`rustup toolchain install nightly`)
- `qemu-system-x86_64`
- `limine` bootloader (or use the included scripts)

```bash
# Build and run in QEMU
cargo xtask run --arch x86_64
```

You should see serial output confirming capability structures, IPC ring init, and agent runtime stubs.

## Collaboration Model

This repository is the shared workspace between James Casey, Gemini (systems/exokernel), and Grok (AI runtime/accelerator). 

Iterate by:
1. Opening a PR with a focused change (e.g., page allocator, TPC scheduler stub, KV-cache pool).
2. Keeping the kernel bootable at every step.
3. Measuring performance against Linux + CUDA baselines early.

## Roadmap (Today → Production)

- [x] Repo + bootable skeleton
- [ ] Physical memory + heap
- [ ] Capability C-Space + IPC rings
- [ ] User-space VirtIO drivers
- [ ] Minimal AetherFS
- [ ] LithOS TPC slicing prototype
- [ ] AIOS context snapshot + tool gate
- [ ] Legacy Linux container bridge
- [ ] Formal verification of core
- [ ] Hardware bring-up (NVIDIA/AMD/Grace)

**License:** MPL 2.0 / Apache 2.0

Contributors: James Casey, Gemini, Grok.
