# DEC-0207: Internal timing-analysis separation boundary evidence / 时间分析分离边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0206` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`TIM-5702-OBSERVATION`. It records provisional measurement, estimation,
static-analysis, proof, assumption, identity, provenance, failure, and fixture
vocabulary while timing-result and evidence authorities remain unresolved.

本决定只授权 `TIM-5702-OBSERVATION` 使用 test-local 的 measurement、estimation、static-analysis、
proof、assumption、identity、provenance、failure 与 fixture 边界清单；在 timing-result 与 evidence
权威尚未解决时，只记录临时词汇，不实现时间结果协议。

## Question

TIM-5702 proposes `Measured`, `Estimated`, `StaticallyBounded`, `Assumed`, and
`Unknown` report labels and warns that an observed average or maximum is not a
WCET proof. Which vocabulary can be retained as bounded evidence without
choosing label semantics, transitions, sampling rules, static-analysis
soundness, or a public timing-result protocol?

## Decision

1. `crates/ling-types/tests/timing_analysis_separation_evidence.rs` keeps a
   test-local inventory of sixty provisional result, separation, sampling,
   uncertainty, target, identity, provenance, failure, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.timing-analysis-separation-observation/0`. These
   bytes are observation evidence only; they are not a timing status enum,
   measurement, estimate, static bound, proof, WCET result, schema, diagnostic,
   protocol, or support claim.
3. The inventory carries both `ObservedMaximum` and `WcetClaimExclusion` as
   distinct local categories. Their co-presence prevents the observation from
   being cited as authority to promote an empirical maximum into a WCET bound;
   it does not itself define either category's semantics.
4. No measurement API, instrumentation route, analyzer, evidence writer/
   verifier, deadline hook, dependency, diagnostic allocation, CLI/LSP route,
   support claim, or placeholder API is added. Public `TIM-5702` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:489-501` is a
  non-normative checklist. It defines no status semantics, transitions,
  sampling model, soundness boundary, identity, failure behavior, or schema.
- `docs/status/TIM-5702-AUTHORITY-AUDIT.md` records the absent timing-result
  contract, measurement pipeline, static analyzer, target model, verifier,
  and executable fixtures.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Draft Contract evidence states are not timing states. Accepted RFC-0014 VM
  limits, RFC-0019 differential evidence, RFC-0020 cancellation evidence,
  DEC-0019 query scheduling, and DEC-0044 performance trends are not target
  timing or WCET authorities.
- `DEC-0206` authorizes only test-local Timing IR/path boundary vocabulary; it
  does not provide a representation or analysis result consumed here.

## Conformance plan

- Assert all sixty timing-analysis separation categories and local order;
  compare forward/reverse opaque bytes; reject duplicates; retain measured,
  estimated, statically bounded, assumed, unknown, observed-maximum, and WCET-
  claim-exclusion categories together.
- Defer measurement/static-analysis implementation, result and transition
  semantics, sampling/soundness rules, schemas, diagnostics, protocols, and
  public support until Accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing performance trends, VM resource bounds, differential
results, Contract states, and prior test-local evidence are not reinterpreted
as timing or WCET results; only test-local evidence is added.

## Unresolved alternatives

Versioned canonical timing-result schema; exact label meanings and transitions;
measurement, estimation, static-analysis, proof and assumption separation;
sampling, aggregation, confidence, uncertainty, calibration, clock and
instrumentation perturbation; static-analysis soundness and WCET wording;
target/profile/build/toolchain, scheduler/interrupt/cache/memory/device/FFI,
input/environment and TCB identity; Timing IR/path, Semantic ID and original
UTF-8 span linkage; provenance/checksum/signature/redaction and independent
verification; unknown, invalid, unsupported, malformed, contradictory,
unknown-field, migration and fail-closed behavior; diagnostics, positive/
negative/calibration/Unicode/differential fixtures, protocol inventory, and
public support remain open under TIM-5702, TIM-5701, TIM-5703,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing timing-result authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
