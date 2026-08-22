# DEC-0201: Internal Assumption Registry boundary evidence / 假设注册表边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: critical-quality  
> 相关规范/缺口：`DEC-0200` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PROOF-5503-OBSERVATION`. It records provisional assumption identity,
lifecycle, review, risk, proof-effect, provenance, evidence, and fixture
vocabulary while the proof, TCB, and Evidence Bundle authorities remain
unresolved.

本决定只授权 `PROOF-5503-OBSERVATION` 使用 test-local 的 assumption identity、lifecycle、review、risk、
proof-effect、provenance、evidence 与 fixture 边界清单；在 proof、TCB 与 Evidence Bundle 等权威尚未解决时，
只记录临时词汇，不实现假设注册表。

## Question

PROOF-5503 lists assumption ID, description, source, scope, owner/reviewer,
expiry/version, risk class, and affected obligations. Which vocabulary can
be retained as bounded evidence without choosing identifier generation,
approval/revocation semantics, proof effect, TCB membership, canonical
schema, or Evidence Bundle protocol?

## Decision

1. `crates/ling-types/tests/assumption_registry_evidence.rs` keeps a
   test-local inventory of sixty provisional assumption record, lifecycle,
   review, proof-effect, provenance, diagnostic, evidence, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.assumption-registry-observation/0`. These bytes
   are observation evidence only; they are not an assumption record schema,
   registry, review workflow, proof rule, TCB entry, Evidence Bundle,
   diagnostic, protocol, or support claim.
3. No assumption registry, schema, reviewer/expiry workflow, TCB field,
   dependency, diagnostic allocation, CLI/LSP route, public protocol,
   support claim, or placeholder API is added. Public `PROOF-5503` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:412-427` is a
  non-normative checklist. It names fields but defines no identity,
  canonical bytes, lifecycle, proof effect, risk policy, or evidence schema.
- `docs/status/PROOF-5503-AUTHORITY-AUDIT.md` records the absent assumption,
  proof, TCB, review, expiry, and evidence authorities.
- RFC-K501/K503/K505/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted governance gap/lifecycle registries, bytecode verification,
  Trait solving, and compiler provenance are not proof assumption or TCB
  authorities.
- Draft `SEMANTICS.md`/`LANGUAGE.md` TCB and `Assumed` sketches do not
  authorize a registry schema or proof effect.

## Conformance plan

- Assert all sixty assumption-registry categories and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer registry/schema implementation, identity/lifecycle/review/risk/
  expiry/proof-effect rules, diagnostics, Evidence Bundle linkage,
  protocols, and public support until accepted authority and offline
  fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing governance records and compiler evidence are not
reinterpreted as proof assumptions; only test-local evidence is added.

## Unresolved alternatives

Assumption identity and canonical record version; description/source/digest/
scope/owner/reviewer/expiry/version/risk/affected-obligation fields; stable
Contract/Proof/Semantic IDs and spans; proposed/approved/revoked/expired/
stale/missing/duplicate/conflicting/out-of-scope/unreviewed/unverifiable/
malformed/corrupt semantics; unknown fields and migration; distinction among
assumptions, hypotheses, axioms, runtime checks, tests, model checks, solver
candidates, and proved facts; TCB/proof effect/optimizer/profile admission;
fail-closed policy, provenance/checksum/signature/redaction, Evidence Bundle,
diagnostics, positive/negative/expired/revoked/Unicode/differential fixtures,
protocol inventory, and public support remain open under PROOF-5503,
PROOF-5502, RFC-K505, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE,
and missing assumption authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
