# DEC-0188: Internal Node virtual-time runtime boundary evidence / Node 虚拟时间运行时边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0187` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `GAP-DETERMINISTIC-REPLAY-001` | `PROTO-REPLAY`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`NODE-5304-OBSERVATION`. It records provisional virtual-time and reference
runtime vocabulary for clocks, ticks, injected input, output traces,
overrun/Fault, replay, privacy, migration, diagnostics, and fixtures while
RFC-K502, replay, and the dependent Node/runtime authorities remain
unresolved.

本决定只授权 `NODE-5304-OBSERVATION` 使用 test-local 的虚拟时间与参考运行时边界清单；在 RFC-K502、
replay、Node/runtime 等依赖权威尚未解决时，只记录临时的 clock、tick、injected input、output trace、
overrun/Fault、replay、privacy、migration、diagnostic 与 fixture 词汇。

## Question

NODE-5304 proposes a conformance runtime with a deterministic virtual clock,
exact ticks, injected input, output traces, overrun simulation, Fault/fallback,
and replay integration. Which vocabulary can be retained as bounded evidence
without choosing time units/advancement, trace identity, simulated versus
target time, replay equivalence, or privacy/migration semantics?

## Decision

1. `crates/ling-types/tests/node_virtual_time_runtime_evidence.rs` keeps a
   test-local inventory of sixty provisional virtual-time/runtime categories,
   clock/tick/input/output/trace relations, overrun/replay obligations,
   diagnostics, and fixtures.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.node-virtual-time-runtime-observation/0`. These
   bytes are evidence only; they are not a clock, runtime, trace, replay
   schema, Fault, diagnostic, protocol, or support claim.
3. No virtual-clock type, Node runtime, trace/replay schema, dependency,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `NODE-5304` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:254-264` is
  non-normative; it defines no clock type, epoch/units/advancement,
  input/output trace, overrun/Fault, fallback, or replay relation.
- `docs/SEMANTICS.md:1380-1425` sketches Node ticks/deadlines and
  `:1914-1931` reserves Node; `docs/LANGUAGE.md:857-866` is a surface example.
- Accepted RFC-0019 covers interpreter/VM logical event equivalence and
  RFC-0020 covers Experimental VM host cancellation/resource evidence; neither
  defines Node virtual time, injected input, or replay logs.
- `PROTO-REPLAY` is Future and `GAP-DETERMINISTIC-REPLAY-001` leaves event
  order, effects, privacy, corruption, divergence, and migration open.
- `docs/status/NODE-5304-AUTHORITY-AUDIT.md` records the missing runtime,
  replay, target, and transaction authority.

## Conformance plan

- Assert all sixty virtual-time/runtime boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer clock/tick runtime, input/output traces, overrun/Fault, replay,
  diagnostics, CLI/LSP, migration, and protocol behavior until accepted
  authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. RFC-0019/0020 events and host cancellation are not reinterpreted as
Node timing or replay semantics; only test-local evidence is added.

## Unresolved alternatives

Reference versus production runtime; checked Node Core input; epoch, units,
advancement, overflow, tick/release/deadline and tie-break; injected input,
output trace, port sampling/state commit, event identity, bounds and canonical
serialization; overrun/missed tick/Fault/fallback/cancellation/restart/recovery;
Effect/input/output records; replay equivalence/order/privacy/redaction,
corruption/truncation/divergence/migration; Critical Profile, Task/Actor,
Native/ABI, Kernel/Device and target/runtime relations; unknown/unsupported
traces; bilingual diagnostics and facts; clock/input/state/overrun/replay/
corruption/migration/determinism/Unicode/differential fixtures; protocol
inventory and public status remain open under NODE-5304, NODE-5303,
GAP-CRITICAL-PROFILE-001, GAP-DETERMINISTIC-REPLAY-001, Future PROTO-REPLAY,
and missing RFC-K502 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
