# AetherOS Privacy Model

Privacy in AetherOS is an architectural property, not a policy or an add-on encryption layer. It is enforced by the absence of ambient authority, capability-based access control, local-first execution, and a minimal trusted computing base.

This document expands the privacy goal in `docs/GOALS.md` into concrete mechanisms, threat assumptions, and design rules.

## Threat Model

**In scope:**
- Compromised or malicious user-space processes, agents, and drivers.
- Prompt injection and tool misuse by LLM agents.
- Accidental or intentional data exfiltration via network, storage, or inter-process channels.
- Persistence of sensitive data across reboots or agent lifetimes.
- Side channels that can be closed by capability and isolation design (not full microarchitectural side-channel resistance).

**Out of scope (for the core OS):**
- Physical attacks on hardware.
- Fully malicious hypervisor or firmware below the exokernel (Secure Boot / measured boot are complementary).
- Cryptographic breaks of the underlying primitives (Blake3, capability tokens).
- User error in explicitly granting overly broad capabilities.

## Core Mechanisms

### 1. No Ambient Authority
Every resource access (file, network socket, device, another agent’s state, tool endpoint) requires an unforgeable capability token held in the caller’s C-Space. There is no global “user” or “root” that inherits rights. This eliminates the classic Unix model in which any process running as the user can read SSH keys, browser cookies, or other agents’ context.

### 2. Capability-Gated Data Sharing
- Objects in AetherFS are addressed by cryptographic hash.
- Access is granted by delegating a capability (optionally with reduced rights).
- Merkle proofs allow proving properties of data without revealing the full object.
- Agent KV-caches, logits state, and scratchpads are themselves capability-protected objects. One agent cannot read another’s cache unless explicitly granted the handle.

### 3. Local-First Execution
- Model weights, KV caches, and agent reasoning run on local accelerators by default.
- Remote inference or tool calls require an explicit network or endpoint capability.
- The system does not phone home. Telemetry, crash reporting, or model updates are opt-in capabilities the user must grant.

### 4. Isolation Boundaries
- Device drivers run in unprivileged user space behind IOMMU/SMMU.
- Agents and ordinary processes are isolated; the exokernel core mediates all cross-domain communication via lockless IPC rings and capability checks.
- The trusted computing base is intentionally small (target <15 kLoC for the verified core) so that privacy properties are feasible to audit and formally verify.

### 5. Ephemeral and Recoverable State
- Agent scratchpads use Copy-on-Write branches of the Merkle tree. A failed or abandoned agent can be discarded with no residual data.
- System state is declarative and atomic. Snapshots are instantaneous; rollbacks do not leave partial sensitive state.
- The root filesystem can be configured as ephemeral, resetting to a known-good generation on boot.

### 6. Tool and I/O Mediation for Agents
An LLM agent may only invoke external tools, write storage, or open network connections if its C-Space contains the corresponding `AgentToolEndpoint` or other capability. Prompt injection cannot create new capabilities; it can only exercise those already granted. This makes privilege escalation via indirect injection a capability violation rather than a policy failure.

## Design Rules

1. **Default deny.** New subsystems must not introduce ambient rights. All access paths go through capabilities.
2. **Least privilege for agents.** Tool and data capabilities should be scoped to the minimum required for a task and revocable.
3. **No silent network.** Any outbound connection requires an explicit capability. There are no hidden update or telemetry channels in the core.
4. **Data lifetime is explicit.** Sensitive objects (KV caches, intermediate tensors, user documents) should have clear ownership and be droppable without side effects.
5. **Compatibility must not weaken privacy.** The Linux/Windows legacy bridge runs with reduced privileges. It cannot grant capabilities the native path would withhold.
6. **Measurable isolation.** It should be possible to test that an agent without a given capability cannot observe or modify the corresponding resource.

## Relationship to Ideal AI OS Goals

Privacy and the ideal AI OS reinforce each other:
- Capability-gated tools prevent agents from exfiltrating context they were not given.
- Shared-prefix KV-cache pooling is safe because the cache pages remain capability-protected.
- Spatial accelerator slicing and preemption allow multiple (possibly untrusted) agents to share hardware without one observing another’s tensors or state.
- Local-first execution keeps model weights and conversation context under user control by default.

## Open Questions / Future Work

- Hardware-backed capability storage or sealing for higher assurance.
- Formal statement of non-interference properties between agents.
- User-facing capability management UI that makes grants and revocations understandable.
- Audit log of capability delegations (itself capability-protected).
- Constrained profile for phones/wearables that preserves the same privacy invariants with a smaller footprint.

Privacy bugs are treated as security bugs. Changes that introduce ambient authority, silent network access, or unmediated agent tool use should be rejected.
