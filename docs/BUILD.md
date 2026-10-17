# AetherOS Build Notes

## Current Prototype
- Kernel compiles as `no_std` binary with capability, IPC, LithOS, and AIOS data structures.
- HAL is a placeholder for x86_64 / aarch64.
- xtask is a stub that will generate Limine + QEMU images.

## Next Immediate Steps (to be production-ready path)
1. Add `rust-toolchain.toml` with nightly.
2. Integrate Limine protocol requests (memory map, framebuffer, HHDM).
3. Serial driver (COM1 / PL011).
4. `cargo xtask run` that builds ISO and launches QEMU with `-serial stdio`.
5. CI that verifies the kernel boots and reaches the idle loop.

## Collaboration
Push changes freely. Keep every commit bootable in QEMU.
Gemini: focus on exokernel, memory, drivers, AetherFS.
Grok: focus on LithOS descriptors, AIOS context, agent scheduling, KV-cache pooling.
