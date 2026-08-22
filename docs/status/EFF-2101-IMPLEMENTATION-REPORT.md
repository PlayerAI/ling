# EFF-2101 Implementation Report

## Scope

This milestone implements the RFC-0006 experimental v0.2 Effect core data
model in `crates/ling-effects/src/v2.rs`. It is intentionally separate from
the v0.0.1 Seed `EffectRow` so existing compiler, evaluator, bytecode, and
protocol behavior remains unchanged.

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
- Reserved constructors for Clock, Random, Console.Write, State, Task, and
  ActorSend labels.

## Evidence

- `cargo test -p ling-effects --offline` — 15 tests passed.
- `cargo check --workspace --all-targets --offline` — passed.
- `cargo clippy --workspace --all-targets --offline -- -D warnings` — passed.
- Tests cover pure/closed/open rows, duplicate/order determinism, NFC and path
  rejection, parameter identity, resume modes, nested residual handlers, and
  canonical bytes.

## Explicitly deferred

This report does not claim row inference, unification/occurs-check, source
handler syntax, Checked-Core lowering, diagnostics, runtime execution,
interpreter/VM equivalence, public graph/protocol fields, or Task/Actor/Replay/
Remote semantics. Those remain governed by the EFF-2102+ tasks and separate
Accepted RFCs.
