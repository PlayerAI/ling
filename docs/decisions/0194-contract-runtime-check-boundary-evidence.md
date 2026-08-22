# DEC-0194: Internal Contract runtime-check boundary evidence / Contract 运行时检查边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0193` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5403-OBSERVATION`. It records provisional Contract runtime-check
vocabulary for checked inputs, assertion boundaries, effect isolation,
Fault/status projection, profiles, provenance, evidence, and fixtures while
RFC-K503 and the dependent Critical, proof, effect, runtime, identity, and
evidence authorities remain unresolved.

本决定只授权 `CTR-5403-OBSERVATION` 使用 test-local 的 Contract 运行时检查边界清单；在 RFC-K503 及
Critical、proof、effect、runtime、identity、evidence 等依赖权威尚未解决时，只记录临时的 checked input、
assertion boundary、effect isolation、Fault/status projection、profile、provenance、evidence 与 fixture 词汇。

## Question

CTR-5403 proposes reference checks at calls, returns, declared invariant
boundaries, and instance values, with source provenance, a Contract Fault,
and a profile switch for runtime-only checks. Which vocabulary can be kept as
bounded evidence without choosing checked Contract Core semantics, exact
timing/order, side-effect isolation, Fault/status behavior, or profile
authority?

## Decision

1. `crates/ling-types/tests/contract_runtime_check_evidence.rs` keeps a
   test-local inventory of sixty provisional Contract runtime-check input,
   boundary, effect, Fault/status, profile, evidence, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-runtime-check-observation/0`. These
   bytes are evidence only; they are not an evaluator, runtime hook, Fault,
   profile gate, evidence schema, diagnostic, protocol, or support claim.
3. No Contract evaluator, runtime hook, profile switch, Fault kind,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `CTR-5403` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:331-345` is a
  non-normative checklist. It names check locations and evidence fields but
  defines no Contract expression, timing, order, effect, Fault, or profile
  contract.
- `docs/status/CTR-5403-AUTHORITY-AUDIT.md` records the missing checked Core,
  evaluation order, isolation, Fault/status, profile, identity, and evidence
  authority.
- `docs/SEMANTICS.md:1185-1238` is a Draft Contract sketch and its
  `ContractViolation` text is not executable authority; proof/enforcement is
  reserved at `docs/SEMANTICS.md:1914-1931`.
- `GAP-CRITICAL-PROFILE-001` remains open for the Critical Core,
  Contract proof/runtime boundary, boundedness, model-checking claims, and
  evidence schema. RFC-K503 and any replacement are not Accepted.
- Accepted RFC-0018, RFC-0019, RFC-0020, and DEC-0013 cover Seed host
  Capability/Runtime Faults, interpreter–VM differential events, cancellation,
  and Main/Runtime diagnostics; they do not execute user Contract claims.

## Conformance plan

- Assert all sixty Contract runtime-check categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer the evaluator/runtime hook, check order and isolation, Fault/status,
  profile policy, evidence schema, diagnostics, CLI/LSP, and protocol
  behavior until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Seed Runtime Faults and VM differential events are not
reinterpreted as Contract checks; only test-local evidence is added.

## Unresolved alternatives

Checked Contract Core and binding; pre/post/invariant/instance semantics;
call/return/declared/suspension/Node/Actor/FFI/cleanup boundaries; order,
short-circuit, purity/totality/effects/capabilities/allocation/termination,
assume restrictions; unknown/malformed input; effect isolation, atomicity and
committed state; Fault/category/code/facts, status and provenance; captured
values, privacy/size limits; profile gates and Critical non-weakening;
reference/VM/Native equivalence, evidence, replay and migration; diagnostics,
positive/negative/boundary/isolation/profile/replay/Unicode fixtures;
protocol inventory and public status remain open under CTR-5403, CTR-5402,
RFC-K503, GAP-CRITICAL-PROFILE-001, and missing Contract authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
