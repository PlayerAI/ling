# DEC-0199: Internal Proof IR boundary evidence / Proof IR 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0198` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROOF-5501-OBSERVATION`. It records provisional Proof IR vocabulary while
the proof grammar, certificate boundary, kernel, soundness, assumptions,
Contract translation, and evidence protocol remain unresolved.

本决定只授权 `PROOF-5501-OBSERVATION` 使用 test-local 的 Proof IR 边界清单；在 proof grammar、
certificate、kernel、soundness、assumption、Contract translation 与 evidence protocol 等权威尚未解决时，
只记录临时词汇，不实现 Proof IR。

## Question

PROOF-5501 requests a Proof IR. Which vocabulary can be retained as bounded
evidence without choosing a versioned proof grammar, certificate/query
format, trusted kernel, soundness claim, assumption registry, or public
evidence protocol?

## Decision

1. `crates/ling-types/tests/proof_ir_evidence.rs` keeps a test-local inventory
   of sixty provisional Proof IR, term, theorem, axiom, provenance,
   Contract/Typed-Core, checking, evidence, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.proof-ir-observation/0`. These bytes are
   observation evidence only; they are not a Proof IR grammar, certificate,
   parser, kernel, checker, assumption registry, schema, diagnostic,
   protocol, or support claim.
3. No Proof IR, certificate/query format, proof kernel, assumption registry,
   parser, dependency, diagnostic allocation, CLI/LSP route, public
   protocol, support claim, or placeholder API is added. Public `PROOF-5501`
   remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:386-398` is a
  non-normative Proof IR checklist. It defines no grammar, canonical
  encoding, certificate semantics, kernel, or soundness boundary.
- `docs/status/PROOF-5501-AUTHORITY-AUDIT.md` records the absent proof IR,
  certificate, kernel, assumption, Contract translation, and evidence
  authorities.
- `RFC-K505`, `RFC-K504`, `RFC-K506`, and `RFC-K507` are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted Seed Trait, bytecode-verifier, Checked-Core, and VM decisions do
  not authorize a source Contract proof calculus or a proof trust boundary.
- Draft `SEMANTICS.md`/`LANGUAGE.md` Contract and verification sketches do
  not authorize a public Proof IR representation.

## Conformance plan

- Assert all sixty Proof IR categories and local order; compare forward and
  reverse opaque bytes; reject duplicates.
- Defer proof grammar, terms, certificates, kernel/checker, soundness,
  assumptions, Contract/Typed-Core translation, diagnostics, protocols, and
  public support until accepted authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing Trait, bytecode, Checked-Core, and VM evidence is not
reinterpreted as Proof IR authority; only test-local evidence is added.

## Unresolved alternatives

Versioned proof grammar, sorts/types/terms, variables/constants/applications,
hypotheses/theorems/goals/rules, arithmetic/memory/alias/effect/ownership
axioms, bounds/termination, Node/Task/Actor and FFI/ABI facts, assumptions,
provenance/spans/Semantic IDs/Proof IDs, canonical bytes and normalization,
well-formedness and Contract/Typed-Core translation, status separation,
runtime/test/model evidence, unknown/malformed/corrupt/fail-closed behavior,
TCB/kernel/soundness/independent checking/resource limits, deterministic
ordering/migration, diagnostics, positive/negative/malformed/adversarial/
rejection/Unicode/differential fixtures, protocol inventory, and public
support remain open under PROOF-5501, PROOF-5502, PROOF-5503,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and missing proof authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
