# DEC-0204: Internal Model-Check Report boundary evidence / 模型检查报告边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0203` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`MC-5603-OBSERVATION`. It records provisional model-check result, bound,
assumption, counterexample, provenance, diagnostic, and fixture vocabulary
while RFC-K506/RFC-K507 and the report/evidence authorities remain
unresolved.

本决定只授权 `MC-5603-OBSERVATION` 使用 test-local 的 model-check result、bound、assumption、
counterexample、provenance、diagnostic 与 fixture 边界清单；在 RFC-K506/RFC-K507 及 report/evidence 等
权威尚未解决时，只记录临时词汇，不实现报告协议。

## Question

MC-5603 proposes `CounterexampleFound`,
`NoCounterexampleWithinBounds`, `Inconclusive`, and `InvalidModel`, and
prohibits treating bounded absence as proof. Which vocabulary can be retained
as bounded evidence without choosing a report schema, result validity rules,
counterexample payload, exit behavior, or public evidence protocol?

## Decision

1. `crates/ling-concurrency/tests/model_check_report_evidence.rs` keeps a
   test-local inventory of sixty provisional result, identity, bound,
   resource, provenance, counterexample, diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.model-check-report-observation/0`. These bytes
   are observation evidence only; they are not a report enum, schema,
   counterexample payload, exit-code contract, diagnostic, protocol, or
   support claim.
3. `NoCounterexampleWithinBounds`, `BoundedNonProof`, and
   `SafetyClaimProhibited` are recorded together to preserve the plan's
   non-claim boundary. This does not define any result semantics.
4. No report enum/schema, counterexample payload, dependency, diagnostic
   allocation, CLI/LSP route, public protocol, support claim, or placeholder
   API is added. Public `MC-5603` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:458-469` is a
  non-normative result checklist. It names four labels and one prohibition
  but defines no fields, canonical encoding, validity, exit, or evidence
  semantics.
- `docs/status/MC-5603-AUTHORITY-AUDIT.md` records the absent report,
  counterexample, proof, replay, and evidence authorities.
- RFC-K501/K502/K504/K505/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted compiler/runtime diagnostics and RFC-0019 differential outcomes
  are not model-check report authorities.
- Draft `SEMANTICS.md`/`LANGUAGE.md` `ModelChecked` sketches do not authorize
  report states or bounded-proof claims.

## Conformance plan

- Assert all sixty model-check report categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Assert the bounded-absence observation includes the non-proof and
  safety-claim-prohibition categories.
- Defer report/result/counterexample schemas, exit semantics, diagnostics,
  protocols, and public support until accepted authority and offline fixtures
  exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing diagnostics and differential reports are not
reinterpreted as model-check reports; only test-local evidence is added.

## Unresolved alternatives

Versioned report schema and canonical bytes; result validity for
CounterexampleFound/NoCounterexampleWithinBounds/Inconclusive/InvalidModel,
invalid property, timeout/memory/resource exhaustion, unknown/malformed/
corrupt/unsupported-version and fail-closed behavior; model/property/bound/
assumption/counterexample/replay/Semantic IDs and source spans; scheduler/time
configuration, explored counts and resource disclosure; tool identity,
provenance/checksum/signature/redaction, unknown fields/migration and
deterministic ordering; counterexample/replay/proof/evidence linkage;
independent verification, diagnostics/exit codes, positive/negative and every
result-state/bound/counterexample/malformed/Unicode/differential fixture;
protocol inventory and public support remain open under MC-5603, MC-5602,
MC-5604, RFC-K506, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and
missing report authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
