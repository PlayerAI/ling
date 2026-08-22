# EFF-2102 Implementation Report

## Scope

This milestone implements the Accepted RFC-0006/DEC-0062 Experimental v0.2
Effect-row solver in `crates/ling-effects/src/solver.rs`. It is an in-process
component over canonical checked rows; it does not change the v0.0.1 Seed
checker, source syntax, Typed-Core lowering, runtime, or public protocols.

Milestone commit: `e1827e7ceee6ffba9d40d882119a949d4af65a00`

本里程碑在 `crates/ling-effects/src/solver.rs` 中实现已接受的
RFC-0006/DEC-0062 实验性 v0.2 Effect Row 求解器。它只处理 canonical checked
row，不改变 v0.0.1 Seed 检查器、源代码语法、Typed Core lowering、运行时或公共协议。

## Implemented

- `Equal` and `Requires` constraints with stable ordinals and optional original
  UTF-8 byte spans.
- Canonical constraint sorting/deduplication independent of insertion order.
- Deterministic substitutions, closed/open row unification, distinct-tail
  residual variables, and an explicit occurs-check boundary.
- Deterministic required-label insertion that preserves an unknown open tail.
- Value-restriction generalization and caller-seeded instantiation with sorted
  quantified row variables.
- Handler subtraction through the accepted `HandlerContract`, preserving tails
  and keeping Capability facts separate.
- Minimal tracked conflict origins and bilingual `L-EFFECT-0001` row-conflict
  and `L-EFFECT-0002` occurs-check diagnostics with structured facts.
- Canonical substitution bytes suitable for deterministic comparison; no path,
  host state, allocation identity, hash-map order, or debug formatting enters
  row identity.

## Evidence

- `cargo test -p ling-effects --all-targets --offline` — 22 tests passed.
- `cargo clippy -p ling-effects --all-targets --offline -- -D warnings` — passed.
- Tests cover reordered constraints, closed/open residuals, distinct tails,
  required labels, occurs-check and row-conflict diagnostics, value restriction,
  deterministic instantiation, source-span evidence, and the existing Seed and
  RFC-0006 model suites.
- Registry evidence: `cargo xtask governance check-error-codes` reports 86
  active codes across 17 domains; the two EFF-2102 codes are in the single
  handwritten registry and generated lock.

## Compatibility and determinism

- Seed source acceptance, existing diagnostics, Semantic IDs, schemas, CLI,
  LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0 data remain unchanged.
- The new API is explicitly Experimental v0.2 and is not a public wire or CLI
  protocol. Original source byte spans are evidence only and never affect row
  canonical bytes.

## Handoff

EFF-2103 still owns source handler syntax and Checked-Core lowering. Runtime
handler execution, Task/Actor lifecycle, Replay, Remote, Native, GPU, FFI, and
Stable 1.0 compatibility remain deferred to their separate authorities.
