# EFF-2101 Implementation Report

## Scope

This milestone completes the RFC-0006 experimental v0.2 Effect core data
model in `crates/ling-effects/src/v2.rs`. It is intentionally separate from
the v0.0.1 Seed `EffectRow` so existing compiler, evaluator, bytecode, and
protocol behavior remains unchanged.

Milestone commits: `4a3a33b` (`feat(effects): add RFC-0006 core row model`) and
`61f68a93844431265eca725ee66aab894394c982` (`feat(effects): complete EFF-2101
model projection`).

本里程碑在 `crates/ling-effects/src/v2.rs` 中实现 RFC-0006 的实验性 v0.2
Effect 核心数据模型，并与 v0.0.1 Seed `EffectRow` 隔离，保持现有编译器、
求值器、字节码和协议行为不变。

## Implemented

- NFC/Unicode-17 XID `EffectId` validation with path-free canonical spelling.
- Canonical Typed-Core `EffectTypeRef` and parameterized `EffectLabel`.
- Sorted duplicate-free `EffectRowModel` with `Closed` and `Variable` tails.
- Deterministic row union, explicit distinct-tail constraint error, residual
  removal, canonical names, and length-delimited canonical bytes.
- `EffectOperation` with ordered inputs, output, and `Never`/`Once`/`Many`.
- `HandlerClause` and `HandlerContract` with owner matching, duplicate-clause
  rejection, lexical elimination, residual-row verification, and nested-tail
  preservation.
- `EffectGraphProjection` with the versioned `ling.effect/0.1` in-process
  schema shape, canonical ordering, and length-delimited graph-input bytes.
- Reserved constructors for Clock, Random, Console.Write, State, Task, and
  ActorSend labels.

## Evidence

- `cargo test -p ling-effects --offline` — 16 tests passed.
- `cargo check --workspace --all-targets --offline` — passed.
- `cargo clippy --workspace --all-targets --offline -- -D warnings` — passed.
- Tests cover pure/closed/open rows, duplicate/order determinism, NFC and path
  rejection, parameter identity, resume modes, nested residual handlers,
  reserved labels, polymorphic caller rows, graph projection, and canonical
  bytes.

## Handoff to later targets

Row inference, unification/occurs-check, source handler syntax, Checked-Core
lowering, solver diagnostics, runtime execution, interpreter/VM equivalence,
public graph/protocol adapters, and Task/Actor/Replay/Remote semantics remain
governed by EFF-2102+ and separate Accepted RFCs.
