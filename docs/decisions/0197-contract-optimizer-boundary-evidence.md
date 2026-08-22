# DEC-0197: Internal Contract optimizer boundary evidence / Contract 优化器边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0196` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5406-OBSERVATION`. It records provisional optimizer admission,
transformation, preservation, invalidation, proof/evidence, diagnostic, and
fixture vocabulary while RFC-K503/K505 and the dependent Critical and
evidence authorities remain unresolved.

本决定只授权 `CTR-5406-OBSERVATION` 使用 test-local 的 Contract 优化器边界清单；在 RFC-K503/K505 及
Critical、evidence 等依赖权威尚未解决时，只记录临时的 admission、transformation、preservation、
invalidation、proof/evidence、diagnostic 与 fixture 词汇。

## Question

CTR-5406 says only sufficiently trusted `Proved` facts may drive
semantics-changing optimization, while runtime-checked, assumed, or unknown
facts cannot justify removing safety checks. Which vocabulary can be retained
as bounded evidence without choosing a status trust algebra, transformation
catalogue, preservation obligations, or optimization protocol?

## Decision

1. `crates/ling-types/tests/contract_optimizer_evidence.rs` keeps a
   test-local inventory of sixty provisional optimizer status/admission,
   transformation/preservation, invalidation, proof/evidence, diagnostic,
   and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-optimizer-observation/0`. These bytes
   are evidence only; they are not an optimizer pass, status trust model,
   proof reader, transformation contract, schema, diagnostic, protocol, or
   support claim.
3. No optimizer pass, safety-check elimination, proof/assumption schema,
   diagnostic allocation, CLI/LSP route, protocol, support claim, or
   placeholder API is added. Public `CTR-5406` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:371-373` is a
  non-normative checklist. It defines no trust ordering, transformation set,
  preservation relation, or stale/unknown behavior.
- `docs/status/CTR-5406-AUTHORITY-AUDIT.md` records missing Contract/proof/
  profile/evidence authority and the absence of optimizer implementation.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` are Draft and use status sets
  that conflict with the plan; they cannot authorize a semantics-changing
  optimizer.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future;
  RFC-K503/K505 and an optimization replacement are not Accepted.
- Accepted Seed RFC-0014–0020 define checked lowering, VM safety, Effects,
  Faults, differential events, and cancellation, not source Contract proofs
  or optimization passes.

## Conformance plan

- Assert all sixty Contract optimizer categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer optimizer passes, admission/trust, preservation/invalidation,
  proof/evidence readers, diagnostics, CLI/LSP, and protocol behavior until
  accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Seed lowering and VM verification are not reinterpreted as
Contract-aware optimization; only test-local evidence is added.

## Unresolved alternatives

Status trust and admission; Proved/runtime/model/test/assumed/unknown/stale/
corrupt/unverifiable semantics; profile/Critical non-weakening; transformation
catalogue and pre/postconditions; constant folding/check elimination/dead
code/inlining; Effect/Capability, evaluation order/short-circuit,
Fault/cleanup, ownership/resource/timing/Node/Task/Actor, numeric/FFI/ABI,
stack/debug mappings, Semantic IDs/spans; invalidation on dependency/source
changes; proof/assumption/evidence links; diagnostics, profiles, positive/
negative/rejection/stale/corrupt/unknown/effect-Fault/Unicode/differential/
optimization fixtures; protocol inventory and public status remain open under
CTR-5406, CTR-5405, RFC-K503, RFC-K505, GAP-CRITICAL-PROFILE-001,
PROTO-EVIDENCE, and missing optimization authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
