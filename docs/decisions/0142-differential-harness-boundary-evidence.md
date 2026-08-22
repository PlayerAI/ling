# DEC-0142: Internal differential-harness boundary evidence / 内部 Differential Harness 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: runtime-quality
> 相关规范/缺口：`DEC-0141` | `DEC-0140` | `DEC-0139` | `DEC-0138` | `DEC-0137` | `DEC-0109` | `DEC-0009` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001` | `GAP-OWNERSHIP-MODEL-001` | `GAP-SEMANTIC-HASH-LIFECYCLE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DIFF-3701-OBSERVATION` interpreter/VM/Native differential boundary. It records
vocabulary and deterministic ordering while Native execution, FFI, ABI,
ownership, replay, and equivalence authorities remain unresolved.

本决定只授权 `DIFF-3701-OBSERVATION` 使用 test-local 的拟议 interpreter/VM/Native
differential 边界清单。在 Native execution、FFI、ABI、ownership、replay 与 equivalence
权威尚未解决时，只记录词汇和确定性顺序。

## Question

DIFF-3701 proposes a three-engine harness comparing Interpreter, VM, and
Native outcomes over checked inputs, values, Faults, effects, resources,
actors/scheduling, traces, normalization, corpus cases, replay, and allowed
differences. Which planning vocabulary can be retained as bounded evidence
without creating a Native backend or claiming cross-engine equivalence?

DIFF-3701 计划对 Interpreter、VM、Native 三种 engine 在 checked input、value、Fault、
effects、resource、actor/scheduling、trace、normalization、corpus、replay 与 allowed
differences 上进行比较。在不创建 Native backend、不声明跨 engine 等价的前提下，哪些
规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/differential_harness_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering source/
   Checked Core/bytecode and engine identity, snapshots/entry/arguments/results/
   Faults/diagnostics/spans/Semantic IDs, effects/capabilities/resources/
   actors/scheduling/mailboxes/cancellation, step/instruction/event/heap
   observations, host-output exclusions, value/error/Fault/text/numeric/
   aggregate/closure/console/host-failure normalization, unsupported/malformed/
   limit cases, Seed positive/negative/property/replay corpus, canonical and
   deterministic/cross-process/target/compiler runs, differential classification
   and allowed differences, bilingual diagnostics/Unicode, schema migration, and
   public-protocol exclusion.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.differential-observation/0`. These bytes are not an execution result,
   engine trace, equivalence proof, allowed-difference registry, replay record,
   diagnostic, Semantic ID, or public protocol.
3. The child adds no differential harness, Native backend, engine adapter,
   trace schema, normalizer, corpus, replay tool, allowed-difference registry,
   dependency, toolchain, diagnostic, protocol, or placeholder API. Public
   `DIFF-3701` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its differential checklist cannot
  define engine inputs, scheduling, observation points, value/Fault semantics,
  normalization, equivalence, or allowed differences.
- Accepted Seed interpreter/VM decisions define only current Seed behavior;
  they do not authorize a Native engine, cross-engine contract, or replay
  equivalence claim.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-SEMANTIC-HASH-LIFECYCLE-001` remain Open. RFC-N305, RFC-0007,
  RFC-0011, and the future differential/protocol authorities are not Accepted.
- Accepted `DEC-0141` through `DEC-0137` authorize only test-local Native/FFI
  vocabulary; they do not supply engine equivalence semantics.

## Conformance plan

- Assert all sixty provisional differential boundaries and their test-local
  order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep engine adapters, Native execution, trace/normalization schemas,
  equivalence and allowed-difference policy, replay, corpus, diagnostics,
  migration, and cross-target/compiler behavior deferred until authorities are
  Accepted.

## Compatibility impact

- Accepted Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No differential result, equivalence,
  Native, diagnostic, dependency, protocol, or support claim is registered.

## Unresolved alternatives

Engine/input identity; observation points and event scheduling; value/Fault/
effect/resource/actor/mailbox/cancellation traces; host-output exclusions;
normalization of text/numeric/aggregate/closure/console/failures; unsupported,
malformed, limits; corpus/property/replay; canonical/deterministic/cross-process/
target/compiler runs; classification/allowed differences; diagnostics, Unicode,
schema migration, and public protocol rules remain open under DIFF-3701,
DIFF-3702, FFI-3605, GAP-NATIVE-BACKEND-ABI-001,
GAP-OWNERSHIP-MODEL-001, GAP-SEMANTIC-HASH-LIFECYCLE-001, and missing
Native/differential authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
