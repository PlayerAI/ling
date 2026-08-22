# DEC-0200: Internal Independent Checker boundary evidence / 独立 Checker 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0199` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROOF-5502-OBSERVATION`. It records provisional independent-checker,
certificate, result, resource, TCB, provenance, and fixture vocabulary while
the Proof IR, certificate/query, kernel, soundness, and evidence authorities
remain unresolved.

本决定只授权 `PROOF-5502-OBSERVATION` 使用 test-local 的独立 Checker、certificate、result、resource、TCB、
provenance 与 fixture 边界清单；在 Proof IR、certificate/query、kernel、soundness 与 evidence 等权威尚未
解决时，只记录临时词汇，不实现 checker。

## Question

PROOF-5502 requests an independent checker with bounded offline inputs,
deterministic behavior, fuzzed decoding, explicit TCB disclosure, and
machine-readable results. Which vocabulary can be retained as bounded
evidence without choosing the proof representation, checker kernel,
certificate format, result schema, trust boundary, or public CLI contract?

## Decision

1. `crates/ling-types/tests/independent_checker_evidence.rs` keeps a
   test-local inventory of sixty provisional independent-checker, Proof IR,
   certificate, result, resource, TCB, provenance, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.independent-checker-observation/0`. These bytes
   are observation evidence only; they are not a checker, parser, kernel,
   certificate/query schema, result protocol, TCB registry, diagnostic,
   command, or support claim.
3. No proof checker crate/binary, `zero-proof-check` command, `ling` checker
   route, dependency, certificate/query schema, TCB registry, diagnostic
   allocation, public protocol, support claim, or placeholder API is added.
   Public `PROOF-5502` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:400-410` is a
  non-normative independent-checker checklist. It defines no proof language,
  certificate validation, kernel, result states, or CLI contract.
- `docs/status/PROOF-5502-AUTHORITY-AUDIT.md` records the absent Proof IR,
  certificate, trusted kernel, TCB, result, and evidence authorities.
- `RFC-K505` and `RFC-K507` are absent or unresolved; `RFC-K506` model-check
  semantics are also absent. `GAP-CRITICAL-PROFILE-001` remains open and
  `PROTO-EVIDENCE` is Future.
- Accepted Seed bytecode verification and VM evidence validate executable
  bytecode, not source Contract proofs; the internal Trait solver is not an
  independent checker authority.
- Draft `SEMANTICS.md`/`LANGUAGE.md` Contract and verification sketches do
  not authorize a checker schema or public command.

## Conformance plan

- Assert all sixty independent-checker categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer checker implementation, decoding, certificates, TCB/result schemas,
  diagnostics, protocols, CLI behavior, and public support until accepted
  authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing bytecode verification and Trait solving are not
reinterpreted as source proof checking; only test-local evidence is added.

## Unresolved alternatives

Proof IR/query/certificate versions and canonical forms; input envelope and
limits; checker identity/version/configuration and replay; obligation,
Contract, Semantic IDs and spans; result states including timeout, unknown,
malformed, corrupt, unsupported-version, invalid, and counterexample;
kernel/checker independence and soundness; assumptions and TCB scope;
cancellation, fail-closed/resource bounds, offline dependency policy,
provenance/checksum/signature/redaction/migration; machine-readable result
schema and exit codes; diagnostics; fuzz, positive/negative/malformed/deep/
timeout/unknown/corruption/Unicode/differential fixtures; protocol inventory
and public support remain open under PROOF-5502, PROOF-5501, PROOF-5503,
RFC-K505, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing
checker authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
