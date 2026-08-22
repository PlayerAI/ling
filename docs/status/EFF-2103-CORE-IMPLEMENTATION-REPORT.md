# EFF-2103-CORE Implementation Report

## Scope

This child implements the Accepted DEC-0063 first-order Handler Typed Core
projection in `crates/ling-effects/src/handler_core.rs`. It is a checked,
in-process data boundary only; it does not add source syntax, parser/HIR
lowering, runtime execution, bytecode, VM behavior, or public protocols.

本子切片在 `crates/ling-effects/src/handler_core.rs` 中实现已接受的 DEC-0063
一阶 Handler Typed Core 投影。它只提供 checked 进程内数据边界，不增加源语法、
parser/HIR lowering、运行时、字节码、VM 行为或公共协议。

## Implemented

- `HandlerCoreNodeId` rejects unresolved zero identities at publication.
- `HandlerCoreClause` records canonical clause/body identities and declared
  `ResumeUse`.
- `HandlerCore` validates operation owners/signatures and duplicate labels via
  `HandlerContract`, computes nested residual rows, preserves open tails, and
  retains source spans only as evidence.
- Never/Once/Many resume-use bounds are checked before Core publication.
- `require_closed` emits bilingual `L-EFFECT-0003` residual diagnostics with
  structured facts and original UTF-8 spans.
- Canonical bytes exclude source paths, spans, host state, allocation identity,
  and debug formatting.

## Evidence

- `cargo test -p ling-effects --all-targets --offline` — 25 tests passed.
- `cargo clippy -p ling-effects --all-targets --offline -- -D warnings` — passed.
- Tests cover nested residuals, insertion-independent canonical bytes, resume
  limits, unresolved bodies, bilingual human/JSON residual diagnostics, and
  source-span preservation.

## Compatibility and handoff

Seed syntax and behavior, existing diagnostics, Semantic IDs, schemas, CLI,
LSP, runtime, bytecode, VM, protocols, ABI, and Unicode 17.0.0 remain
unchanged. The EFF-2103 parent remains blocked for source grammar/full lowering;
EFF-2104/2105 own execution and differential evidence.
