# DEC-0203: Internal Exploration Engine boundary evidence / 探索引擎边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0202` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`MC-5602-OBSERVATION`. It records provisional exploration, state identity,
traversal, reduction, resource, result, counterexample, and fixture
vocabulary while RFC-K506 and the projected-model/evidence authorities
remain unresolved.

本决定只授权 `MC-5602-OBSERVATION` 使用 test-local 的 exploration、state identity、traversal、reduction、
resource、result、counterexample 与 fixture 边界清单；在 RFC-K506 及 projected-model/evidence 等权威尚未
解决时，只记录临时词汇，不实现探索引擎。

## Question

MC-5602 suggests BFS/DFS, later partial-order reduction, state hashing,
bounded depth, counterexample traces, timeout/memory bounds, and deterministic
search. Which vocabulary can be retained as bounded evidence without choosing
state identity, transition ordering, reduction soundness, resource charging,
result semantics, or a counterexample protocol?

## Decision

1. `crates/ling-concurrency/tests/exploration_engine_evidence.rs` keeps a
   test-local inventory of sixty provisional exploration, state/hash,
   traversal/reduction, bound/resource, result, provenance, diagnostic, and
   fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.exploration-engine-observation/0`. These bytes
   are observation evidence only; they are not an exploration engine, state
   hash, work queue, reduction rule, result/counterexample schema,
   diagnostic, protocol, or support claim.
3. No exploration engine, state hash, partial-order reduction, result or
   counterexample schema, dependency, diagnostic allocation, CLI/LSP route,
   public protocol, support claim, or placeholder API is added. Public
   `MC-5602` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:446-456` is a
  non-normative algorithm checklist. It defines no projected model,
  transition order, state identity, reduction independence, resource
  accounting, or result semantics.
- `docs/status/MC-5602-AUTHORITY-AUDIT.md` records the absent projected-model,
  exploration, counterexample, proof, and evidence authorities.
- RFC-K501/K502/K504/K505/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted project graph traversals and interpreter/VM differential runs are
  not concurrent finite-state exploration authorities.
- Draft `SEMANTICS.md`/`LANGUAGE.md` model-check sketches do not authorize
  state hashing, traversal, reduction, or result schemas.

## Conformance plan

- Assert all sixty exploration-engine categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer engine/state-hash implementation, traversal/reduction semantics,
  resource/result/counterexample schemas, diagnostics, protocols, and public
  support until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing graph traversals and differential tests are not
reinterpreted as model checking; only test-local evidence is added.

## Unresolved alternatives

Projected-model and canonical state/event bytes; state-hash algorithm,
version, collision behavior, deduplication and visited-set identity;
transition/event ordering, BFS/DFS work queues and tie-breaking;
partial-order reduction independence/soundness and reduction-off behavior;
depth/step/state/queue/time/memory bounds, cancellation, timeout/resource
exhaustion and incomplete/unknown/invalid/malformed/corrupt behavior;
result states, counterexample/replay linkage, stable IDs/spans, provenance/
checksum/redaction, bounded non-proof wording, assumptions/profile admission,
diagnostics, positive/negative/interleaving/reduction/hash-collision/
bound-edge/timeout/memory/determinism/replay/Unicode/differential fixtures,
protocol inventory, and public support remain open under MC-5602, MC-5601,
MC-5603, MC-5604, RFC-K506, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and
missing exploration authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
