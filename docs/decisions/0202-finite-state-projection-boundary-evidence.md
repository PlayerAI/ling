# DEC-0202: Internal Finite-State Projection boundary evidence / 有限状态投影边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0201` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`MC-5601-OBSERVATION`. It records provisional finite-state projection,
concurrency, bounds, property, result, provenance, and fixture vocabulary
while RFC-K506 and the Node/Task/Actor, boundedness, proof, and evidence
authorities remain unresolved.

本决定只授权 `MC-5601-OBSERVATION` 使用 test-local 的有限状态投影、concurrency、bound、property、result、
provenance 与 fixture 边界清单；在 RFC-K506 及 Node/Task/Actor、boundedness、proof、evidence 等权威尚未
解决时，只记录临时词汇，不实现投影。

## Question

MC-5601 lists state variables, bounded mailboxes/queues, transitions,
scheduler choices, Fault/restart, time abstraction, properties, and explicit
bounds. Which vocabulary can be retained as bounded evidence without choosing
concurrency semantics, a projection relation, property language, state
identity, exploration meaning, or model-check protocol?

## Decision

1. `crates/ling-concurrency/tests/finite_state_projection_evidence.rs` keeps
   a test-local inventory of sixty provisional projection, Task/Actor/Node,
   state, transition, bound, property, result, provenance, diagnostic, and
   fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.finite-state-projection-observation/0`. These
   bytes are observation evidence only; they are not a finite-state IR,
   projection, property language, state hash, model checker, result schema,
   diagnostic, protocol, or support claim.
3. No projection IR, model checker, scheduler/mailbox semantics, report or
   counterexample schema, dependency, diagnostic allocation, CLI/LSP route,
   public protocol, support claim, or placeholder API is added. Public
   `MC-5601` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:431-444` is a
  non-normative projection checklist. It defines no source concurrency
  semantics, projection relation, bounds, fairness, time, property language,
  or claim semantics.
- `docs/status/MC-5601-AUTHORITY-AUDIT.md` records the absent projection,
  model-check, Node/Task/Actor, boundedness, proof, and evidence authorities.
- RFC-K501/K502/K504/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted Seed bytecode/VM behavior and internal concurrency observation
  structures do not authorize a language-level state projection.
- Draft `SEMANTICS.md`/`LANGUAGE.md` `ModelChecked` and concurrency sketches
  do not authorize a model-check representation or result.

## Conformance plan

- Assert all sixty finite-state projection categories and local order;
  compare forward/reverse opaque bytes; reject duplicates.
- Defer projection/model implementation, concurrency/fairness/time/property/
  bound semantics, exploration/result schemas, diagnostics, protocols, and
  public support until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing non-executable concurrency observations are not
reinterpreted as a finite-state model; only test-local evidence is added.

## Unresolved alternatives

Checked Task/Actor/Node source and Core model; state variables/types/values;
mailbox/queue semantics and bounds; transitions, scheduler choices/fairness,
Fault/restart identity, time abstraction, external inputs, and ownership;
property language and explicit depth/state/time/memory bounds; canonical
state identity, stable IDs/spans and state hashing; projection relation and
soundness/non-proof wording; ModelChecked/bounded-evidence status;
assumptions/profile admission; deterministic/resource/timeout/memory/
unknown/incomplete/malformed/corrupt/migration behavior; counterexample and
replay linkage; provenance/checksum/redaction; diagnostics; interleaving,
Fault/restart, Unicode and differential fixtures; protocol inventory and
public support remain open under MC-5601, MC-5602, MC-5603, MC-5604,
RFC-K506, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing model-check
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
