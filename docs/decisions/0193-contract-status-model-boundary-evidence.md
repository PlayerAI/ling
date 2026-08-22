# DEC-0193: Internal Contract status-model boundary evidence / Contract 状态模型边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0192` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5402-OBSERVATION`. It records provisional Contract status vocabulary
for obligation identity, proof/check evidence, lifecycle transitions,
provenance, projections, diagnostics, and fixtures while RFC-K503 and the
dependent Critical, proof, model-check, runtime, identity, and evidence
authorities remain unresolved.

本决定只授权 `CTR-5402-OBSERVATION` 使用 test-local 的 Contract 状态模型边界清单；在 RFC-K503 及
Critical、proof、model-check、runtime、identity、evidence 等依赖权威尚未解决时，只记录临时的 obligation
identity、proof/check evidence、lifecycle transition、provenance、projection、diagnostic 与 fixture 词汇。

## Question

CTR-5402 names `Proved`, `RuntimeChecked`, `Assumed`, `Unknown`, `Failed`,
and `NotApplicable`, and asks for propagation into Audit, Graph, and
Evidence. Which vocabulary can be retained as bounded evidence without
choosing the conflicting status set, legal transitions, aggregation,
provenance, trust, identity, or public schema?

## Decision

1. `crates/ling-types/tests/contract_status_model_evidence.rs` keeps a
   test-local inventory of sixty provisional Contract status, identity,
   evidence, lifecycle, projection, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-status-model-observation/0`. These
   bytes are evidence only; they are not a status enum, transition table,
   aggregation policy, evidence schema, renderer, diagnostic, protocol, or
   support claim.
3. No Contract status field, Graph/Audit/Evidence propagation, schema,
   proof/runtime adapter, diagnostic allocation, CLI/LSP route, protocol,
   support claim, or placeholder API is added. Public `CTR-5402` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:318-328` is a
  non-normative checklist. It names states and destinations but defines no
  state meanings, transition graph, precedence, aggregation, or schema.
- `docs/status/CTR-5402-AUTHORITY-AUDIT.md` records the unresolved conflict
  between the plan vocabulary and the Draft `SEMANTICS.md` sketch, as well
  as missing identity, provenance, trust, and evidence rules.
- `docs/SEMANTICS.md:1214-1238` is Draft status metadata and does not
  authorize a versioned lifecycle or public propagation. Contract proof is
  reserved for later work at `docs/SEMANTICS.md:1914-1931`.
- `GAP-CRITICAL-PROFILE-001` remains open for the Critical Core,
  Contract proof/runtime boundary, model-checking claims, and evidence
  schema. RFC-K503 and any replacement are not Accepted.
- Accepted Seed RFCs 0014–0020 and DEC-0012 define bytecode/VM and semantic
  identity evidence, not Contract obligation status semantics.

## Conformance plan

- Assert all sixty Contract status-model categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer the status enum, transition/aggregation rules, evidence schema,
  Graph/Audit projections, UI/diagnostics, CLI/LSP, and protocol behavior
  until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing implementation status and VM evidence are not
reinterpreted as Contract obligation status; only test-local evidence is
added.

## Unresolved alternatives

Status vocabulary/version and meanings; obligation/Contract/Definition/
Semantic identity and canonical bytes; proof, runtime-check, model-check,
test, assumption, unknown, failed, and not-applicable composition; trust,
provenance, solver/checker identity, bounds, timeout/cancellation,
staleness/corruption/revocation; transition, terminality, precedence,
aggregation, invalidation, migration and compatibility; Audit/Graph/Evidence
projection, UI text/accessibility, profiles/optimization, effect isolation
and Fault; bilingual diagnostics/facts; positive/negative/transition/stale/
corruption/migration/Unicode/differential fixtures; protocol inventory and
public status remain open under CTR-5402, CTR-5401, RFC-K503,
GAP-CRITICAL-PROFILE-001, and missing Contract authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
