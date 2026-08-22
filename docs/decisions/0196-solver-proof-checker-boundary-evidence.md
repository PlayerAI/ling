# DEC-0196: Internal Solver/Proof Checker boundary evidence / Solver/Proof Checker 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0195` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`CTR-5405-OBSERVATION`. It records provisional solver, query, certificate,
checker, trust/TCB, timeout/unknown, provenance, and fixture vocabulary while
RFC-K505/K506/K507 and the dependent Contract, Critical, and evidence
authorities remain unresolved.

本决定只授权 `CTR-5405-OBSERVATION` 使用 test-local 的 solver、query、certificate、checker、trust/TCB、
timeout/unknown、provenance 与 fixture 边界清单；在 RFC-K505/K506/K507 及 Contract、Critical、evidence 等
依赖权威尚未解决时，只记录临时词汇。

## Question

CTR-5405 allows an external solver only as an untrusted candidate and asks
for a proof certificate or replayable query, fixed solver/version/config,
timeout/unknown handling, an independent checker, and a TCB inventory. Which
vocabulary can be preserved as bounded evidence without choosing a proof/query
schema, checker soundness, trust model, or public evidence protocol?

## Decision

1. `crates/ling-types/tests/solver_proof_checker_evidence.rs` keeps a
   test-local inventory of sixty provisional solver/query/certificate,
   checker/trust, result, identity, provenance, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.solver-proof-checker-observation/0`. These bytes
   are evidence only; they are not a solver adapter, query/certificate
   schema, checker, soundness claim, TCB registry, evidence protocol,
   diagnostic, or support claim.
3. No solver dependency, proof checker, certificate/query format,
   assumption/TCB registry, diagnostic allocation, CLI/LSP route, protocol,
   support claim, or placeholder API is added. Public `CTR-5405` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:360-369` is a
  non-normative checklist. It defines no proof/query/certificate grammar,
  solver result set, checker soundness, or TCB semantics.
- `docs/status/CTR-5405-AUTHORITY-AUDIT.md` records absent RFC-K505/K506/K507,
  Future `PROTO-EVIDENCE`, and unresolved schema, trust, checker, and
  provenance rules.
- `GAP-CRITICAL-PROFILE-001` remains open for Contract proof/runtime,
  boundedness, model-checking, identity, and evidence claims.
- Accepted Seed RFC-0014–0020 cover bytecode/VM safety, host Faults,
  differential events, and cancellation; the internal Trait solver and
  bytecode verifier are not source-Contract proof authorities.
- Draft Contract/Proof sketches in `docs/SEMANTICS.md` and
  `docs/LANGUAGE.md` cannot authorize solver output or certification.

## Conformance plan

- Assert all sixty solver/proof-checker categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer solver/checker adapters, query/certificate schemas, TCB registry,
  timeout/unknown behavior, evidence protocol, diagnostics, CLI/LSP, and
  protocol behavior until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Trait solving and bytecode verification are not
reinterpreted as external proof checking; only test-local evidence is added.

## Unresolved alternatives

Proof/Query/Certificate versions and canonical forms; candidate solver
identity/version/configuration and replay; obligation/Contract/Semantic IDs
and spans; well-formedness, result statuses, timeout/unknown/malformed/
corrupt/invalid-model; checker/version/independence/soundness; assumptions,
TCB scope, bounds/cancellation/fail-closed limits; stdout trust, provenance,
checksum/signature/redaction/revocation/migration; counterexample/evidence
linkage; profiles/optimization; diagnostics, positive/negative/malformed,
timeout/unknown/corruption/migration/Unicode/differential fixtures; protocol
inventory and public status remain open under CTR-5405, CTR-5404, RFC-K505,
RFC-K506, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing
proof authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
