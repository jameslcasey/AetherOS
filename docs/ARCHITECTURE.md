# AetherOS Architecture (v2.0)

This is the living specification. The prototype in this repository implements the boot path and data structures defined here.

## 5-Layer Architecture

| Layer | Subsystem | Status |
| :--- | :--- | :--- |
| 4 | AOT Wasm + Legacy Bridge | Planned |
| 3 | AIOS Agent Runtime | Stubs |
| 2 | LithOS Accelerator Multiplexer | Stubs |
| 1 | AetherFS Merkle Tensor Store | Planned |
| 0 | Verified Rust Exokernel | Bootable skeleton |

## Capability Types (Implemented in Kernel)

- `MemoryFrame`
- `IpcEndpoint`
- `DeviceMmio`
- `AetherFsObject`
- `AcceleratorTpcSlice`
- `KvCachePool`
- `AgentToolEndpoint`

## IPC Rings

Lockless shared-memory Submission/Completion rings. Single-producer prototype in kernel; multi-agent extension next.

## Next Engineering Focus

1. Bitmap + buddy frame allocator.
2. Higher-half page tables (x86_64 + aarch64).
3. Real C-Space radix tree.
4. User-space virtio-blk driver.
5. Minimal Blake3 object store.
6. TPC mask enforcement stub + atomization timer.
7. Agent context snapshot/restore path.
8. QEMU exit-on-success tests.

Full original design notes are preserved in conversation history and will be expanded here.
