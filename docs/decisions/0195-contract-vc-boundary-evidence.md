# DEC-0195: Internal Contract VC boundary evidence / Contract VC 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0194` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5404-OBSERVATION`. It records provisional Proof IR/VC vocabulary for
identity, SSA/path conditions, arithmetic/memory/effect facts, assumptions,
certificates, boundedness, evidence, diagnostics, and fixtures while RFC-K505
and the dependent Contract, Critical, model-check, and evidence authorities
remain unresolved.

本决定只授权 `CTR-5404-OBSERVATION` 使用 test-local 的 Proof IR/VC 边界清单；在 RFC-K505 及 Contract、
Critical、model-check、evidence 等依赖权威尚未解决时，只记录临时的 identity、SSA/path condition、
arithmetic/memory/effect fact、assumption、certificate、boundedness、evidence、diagnostic 与 fixture 词汇。

## Question

CTR-5404 lists a versioned Proof IR/VC with SSA/path conditions,
pre/postconditions, loop invariants, arithmetic, memory/alias, Effect facts,
source mappings, and trusted assumptions. Which vocabulary can be retained as
bounded evidence without choosing a proof grammar, translation, soundness
claim, solver/checker boundary, or evidence protocol?

## Decision

1. `crates/ling-types/tests/contract_vc_evidence.rs` keeps a test-local
   inventory of sixty provisional Proof IR/VC identity, control-flow,
   arithmetic/memory/effect, assumption/certificate, boundedness, evidence,
   diagnostic, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.contract-vc-observation/0`. These bytes are
   evidence only; they are not Proof IR, VCs, a translator, a soundness
   claim, a solver/checker interface, an evidence schema, a diagnostic,
   protocol, or support claim.
3. No Proof IR, VC generator, obligation lowering, assumption registry,
   solver adapter, checker, diagnostic allocation, CLI/LSP route, protocol,
   support claim, or placeholder API is added. Public `CTR-5404` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:347-358` is a
  non-normative checklist. It defines no Proof IR/VC grammar, well-formedness,
  translation, soundness, or unknown/failure behavior.
- `docs/status/CTR-5404-AUTHORITY-AUDIT.md` records absent RFC-K505,
  Contract-to-VC mapping, arithmetic/alias/effect semantics, TCB and
  evidence authority.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and `docs/ROADMAP-1.0.md` are
  Draft/planning authorities for later Contract proof and model checking;
  they cannot authorize a proof-producing compiler path.
- `GAP-CRITICAL-PROFILE-001` remains open for Contract proof/runtime,
  boundedness, model-checking claims, and evidence schema. RFC-K505/RFC-K506
  are not Accepted.
- `PROTO-EVIDENCE` is Future, unversioned, and fixture-free; its writer,
  reader, identity, provenance, and verification rules remain unresolved.
- Accepted Seed RFCs 0014–0020 cover bytecode/VM safety, host Faults,
  differential events, and cancellation, not source Contract proof
  obligations or soundness.

## Conformance plan

- Assert all sixty Contract VC categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer Proof IR/VC, Contract-to-VC translation, assumption/TCB registry,
  solver/checker, evidence schema, diagnostics, CLI/LSP, and protocol
  behavior until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing bytecode verifier/lowering invariants and VM evidence are
not reinterpreted as source proof; only test-local evidence is added.

## Unresolved alternatives

Proof IR/VC grammar/version/well-formedness/canonical bytes; obligation,
Contract and Semantic IDs/spans; SSA/path, branches/loops/recursion/
termination; arithmetic/overflow/rounding, memory/alias/ownership,
Effect/Capability, timing/FFI and external assumptions; translation and
soundness/non-claims; bounded/unbounded reasoning; solver candidate,
certificate, timeout/unknown/invalid model/fail-closed behavior; limits,
evidence/counterexample/replay/provenance/redaction/revocation/migration;
diagnostics, profiles/optimization; positive/negative/malformed,
arithmetic, alias/effect, assumption, timeout/unknown, Unicode/differential
fixtures; protocol inventory and public status remain open under CTR-5404,
CTR-5403, RFC-K505, RFC-K506, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and
missing proof authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
