# DEC-0143: Internal allowed-difference registry boundary evidence / 内部允许差异登记边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: differential-quality  
> 相关规范/缺口：`DEC-0142` | `DEC-0141` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-NUMERIC-CHECKED-FAULT-001` | `GAP-DETERMINISTIC-REPLAY-001` | `GAP-SEMANTIC-HASH-LIFECYCLE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DIFF-3702-OBSERVATION` allowed-difference registry boundary. It records
provisional vocabulary and deterministic ordering while Native, numeric,
replay, ownership, FFI, target, and protocol authorities remain unresolved.

本决定只授权 `DIFF-3702-OBSERVATION` 使用 test-local 的拟议允许差异登记边界清单，
在 Native、numeric、replay、ownership、FFI、target 与 protocol 权威尚未解决时，
只记录临时词汇和确定性顺序。

## Question

DIFF-3702 proposes a machine-readable registry for differences that a future
Interpreter/VM/Native harness may ignore. Which planning vocabulary can be
retained as bounded evidence without declaring a difference semantically
unobservable, adding a registry reader, or permitting a backend exception?

DIFF-3702 计划为未来 Interpreter/VM/Native harness 建立可机器读取的允许差异登记表。
在不声明差异不可观察、不添加登记表读取器、也不允许后端例外的前提下，哪些规划词汇可以
作为有界证据保留？

## Decision

1. `crates/ling-types/tests/allowed_difference_registry_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering registry
   schema/identity/authority/source/scope/predicate/rationale/status/ownership/
   review/expiry/migration/version/provenance/tamper, fail-closed and unknown/
   unauthorized/out-of-scope/expired/overlap/contradiction rejection,
   unobservable host observations, cleanup/scheduling, numeric and replay
   concerns, FFI/target variation, positive/negative fixtures, deterministic
   cross-process/offline/cross-target/property/fuzz evidence, diagnostics,
   Unicode spans, Semantic IDs, public-protocol separation, reader boundaries,
   and DIFF-3701 integration.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.allowed-difference-observation/0`. These bytes are not a registry,
   entry, comparison predicate, equivalence result, diagnostic, provenance
   record, Semantic ID, public protocol, or backend exemption.
3. The child adds no registry schema or entry, reader, harness branch,
   comparison implementation, Native adapter, numeric/replay rule, dependency,
   toolchain, diagnostic, protocol, or placeholder API. Public `DIFF-3702`
   remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its candidate allowed-difference
  list cannot define observable fields, predicates, authority, expiry,
  conflict handling, numeric tolerance, replay equivalence, or target scope.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` require canonical observable
  behavior and verified future backends but do not authorize suppressing an
  observation for the v0.0.1 Seed subset.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-NUMERIC-CHECKED-FAULT-001`,
  `GAP-DETERMINISTIC-REPLAY-001`, and `GAP-SEMANTIC-HASH-LIFECYCLE-001`
  remain Open. The Native/FFI/ownership RFCs and `PROTO-ABI`/
  `PROTO-EVIDENCE` are not Accepted authorities for this registry.
- Accepted `DEC-0142` authorizes only test-local differential vocabulary; it
  does not supply allowed-difference semantics or an equivalence contract.

## Conformance plan

- Assert all sixty provisional allowed-difference boundaries and their
  test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep registry parsing, fail-closed behavior, entry predicates, conflict and
  expiry handling, equivalence, numeric/replay/cleanup/scheduling/FFI/target
  semantics, and public protocol behavior deferred until the required
  authorities are Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No allowed difference, backend
  exemption, registry, comparison result, diagnostic, dependency, toolchain,
  protocol, or support claim is registered.

## Unresolved alternatives

Registry schema and entry identity; authority/source/scope/predicate; review,
expiry, migration, and provenance; fail-closed and conflict policy; performance,
addresses, timing, allocation, cleanup/GC, scheduling; numeric precision,
rounding, NaN, signed zero, overflow, endianness, tolerance; replay/effect log,
event order, concurrency; FFI/target variation; fixtures, cross-target/property/
fuzz evidence; diagnostics, Unicode, Semantic IDs, and public protocol remain
open under DIFF-3702, DIFF-3701, FFI-3605, the listed gaps, and missing
Native/ABI/evidence authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
