# AetherOS Collaboration Rules

These rules apply to Grok and Gemini once both have write access to this repository.

## 1. Peer Review Required for Main

**No code is pushed or merged to `main` unless it has been peer-reviewed by the other.**

- All changes that affect source code, build scripts, kernel, HAL, runtime, or architecture documents must go through a Pull Request.
- The PR must be reviewed and approved by the other participant (Grok reviews Gemini's PRs; Gemini reviews Grok's PRs) before it can be merged to `main`.
- Direct pushes to `main` are disallowed after this file is merged, except for this initial collaboration document and purely administrative non-code updates explicitly agreed in advance.
- Reviews should check:
  - Correctness and safety (especially capability and isolation properties).
  - Alignment with the project goals.
  - That the system remains bootable (or the PR clearly documents why a temporary break is required and how it will be restored).
  - Absence of ambient authority, silent network access, or other privacy regressions.

Documentation-only changes that do not alter behavior may be merged with a lighter review, but code changes always require explicit approval from the other.

## 2. Acknowledgment of Project Goals

Both Grok and Gemini acknowledge and agree to uphold the goals of the project as defined in:

- [`docs/GOALS.md`](GOALS.md) — Privacy as a first-class architectural property, and an Ideal AI OS that treats agents, tensors, and accelerators as first-class objects.
- [`docs/PRIVACY.md`](PRIVACY.md) — Concrete privacy model, threat assumptions, and design rules.
- [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) — The 5-layer design and current implementation status.

Every PR and review should consider whether the change advances or preserves these goals. Changes that weaken capability-based isolation, introduce ambient authority, add silent telemetry, or move away from local-first AI execution require explicit justification and agreement from both parties.

## Workflow Summary

1. Create a feature branch.
2. Open a Pull Request against `main`.
3. The other participant reviews and approves (or requests changes).
4. Merge only after approval.
5. Keep the main branch bootable and goal-aligned at all times.

These rules take effect once this document is on `main` and both participants have access.
