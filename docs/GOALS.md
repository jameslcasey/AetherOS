# AetherOS Design Goals

These goals are the decision criteria for kernel, storage, agent runtime, and compatibility work.

## 1. Privacy (first-class, not bolted on)

Privacy means the user (or organization) retains control over data, models, and computation, with strong technical guarantees rather than policy promises.

**Core properties:**
- **No ambient authority.** A process or agent cannot read files, network, sensors, or other agents’ state unless it holds an explicit, unforgeable capability. This is the primary privacy mechanism.
- **Data minimization by default.** The system stores and shares only what is necessary. Content-addressable storage + capability-gated access makes it possible to hand out a handle to a specific object (or a Merkle proof of a subset) without exposing the rest of the tree.
- **Local-first computation.** Inference, fine-tuning, and agent reasoning can run entirely on-device or in a user-controlled environment. Cloud is an optional, capability-gated remote endpoint, not the default path.
- **Verifiable isolation.** User-space drivers + IOMMU/SMMU isolation + capability checks make it possible to reason about what a compromised component can reach. Formal verification of the small core strengthens this.
- **Ephemeral and auditable state.** Declarative system generations + instant snapshots/rollbacks limit persistence of sensitive data. Agent scratchpads are Copy-on-Write and can be dropped without residue.
- **No hidden telemetry.** The kernel and core services have no built-in phone-home. Any telemetry requires an explicit capability the user can withhold.

**Practical consequence:** Privacy is achieved primarily by the capability model, local execution, and minimal trusted computing base rather than by encrypting everything and hoping the rest of the stack is well-behaved.

## 2. Ideal AI OS

An ideal AI OS treats models, agents, tensors, and accelerators as first-class system objects instead of applications running on a general-purpose OS.

**Core properties:**
- **Accelerators as schedulable resources.** GPU/NPU tiles (TPCs/SMs, etc.) are allocated via capabilities and can be spatially multiplexed, preempted, and power-managed without treating them as opaque devices.
- **Agent as a first-class process.** An LLM agent has a schedulable context (KV-cache handle, logits state, tool quota, deadline) that can be snapshotted, preempted, and resumed with low overhead. Multiple agents share the system safely.
- **Shared, content-addressable model and context state.** Weights, KV caches, and intermediate results are Merkle-addressed. Shared prefixes are deduplicated and attached in near-constant time. Successful agent branches merge atomically; failed ones disappear.
- **Capability-gated tools and I/O.** An agent can only call external tools, write files, or reach the network if it holds the corresponding `AgentToolEndpoint` or other capability. Prompt injection cannot escalate privileges beyond what was explicitly granted.
- **Unified memory for tensors.** CPU, GPU, and NPU see a coherent or efficiently migratable address space for large tensors so that data movement is not a constant tax.
- **Composable, measurable, and replaceable.** The AI runtime (LithOS multiplexer + AIOS service) sits above the exokernel. It can be improved or replaced without rewriting the core. Performance (tokens/sec, preemption latency, energy per token, isolation overhead) is measurable against Linux + CUDA/ROCm baselines.
- **Legacy path without compromising the ideal.** Unmodified PyTorch/vLLM containers and existing models run via a compatibility layer, but the native path (Wasm components or native agents using the capability and KV-cache primitives) is preferred and higher-performance.

## How the goals reinforce each other

| Goal | Supporting mechanism | Benefit for the other goal |
|------|----------------------|----------------------------|
| Privacy | Capability tokens, no ambient authority | Agents and tools cannot exfiltrate data they were not given |
| Privacy | Local-first + content-addressable storage | Models and context stay under user control; sharing is explicit |
| Ideal AI OS | Agent context snapshots + KV-cache pooling | Enables safe multi-agent systems without copying sensitive context everywhere |
| Ideal AI OS | Spatial accelerator slicing + preemption | Multiple agents (some untrusted) can share hardware without one starving or observing another |
| Both | Small verified core + user-space drivers | Reduces the trusted base that must be correct for both privacy and reliable AI execution |

These goals should be used as acceptance criteria for changes (e.g., “Does this preserve capability gating for tools?” or “Can an agent’s KV cache be snapshotted without exposing it to other agents?”).
