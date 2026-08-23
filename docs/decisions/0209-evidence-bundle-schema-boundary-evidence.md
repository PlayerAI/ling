# DEC-0209: Internal Evidence Bundle Schema boundary evidence / Evidence Bundle Schema 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0208` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`EVD-5801-OBSERVATION`. It records provisional bundle content, identity,
evidence polarity, provenance, privacy, trust, failure, and fixture vocabulary
while RFC-K507 and the Evidence Bundle protocol remain unresolved.

本决定只授权 `EVD-5801-OBSERVATION` 使用 test-local 的 bundle content、identity、
evidence polarity、provenance、privacy、trust、failure 与 fixture 边界清单；在 RFC-K507
和 Evidence Bundle protocol 尚未解决时，只记录临时词汇，不实现 bundle schema。

## Question

EVD-5801 proposes a cross-feature Evidence Bundle containing identities,
authority versions, build inputs, Audit Source, tests, proofs, model checking,
replay, timing, memory, FFI, provenance, assumptions, reviews, and artifact
hashes. Which vocabulary can be retained as bounded evidence without choosing
a schema, canonical container, trust model, verifier, or public protocol?

## Decision

1. `crates/ling-types/tests/evidence_bundle_schema_evidence.rs` keeps a
   test-local inventory of sixty provisional content, identity, producer,
   polarity, provenance, privacy, trust, failure, diagnostic, and fixture
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.evidence-bundle-schema-observation/0`. These bytes
   are observation evidence only; they are not a bundle manifest, schema,
   container, canonical encoding, reader/writer, verifier input/output,
   signature policy, diagnostic, protocol, or support claim.
3. `NonClaim`, `OfflineVerification`, and `NoCodeExecution` remain distinct
   local categories. Their presence does not define verification semantics;
   it prevents the observation from being cited as proof authority or as
   permission to execute bundled artifacts.
4. No bundle schema, manifest, reader/writer, verifier, trust root, signing
   dependency, evidence diagnostic, CLI/LSP route, support claim, or
   placeholder API is added. Public `EVD-5801` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:518-541` is a
  non-normative inventory. It defines no required fields, encoding, digest
  domains, evidence polarity, privacy, trust, verification, or migration.
- `docs/status/EVD-5801-AUTHORITY-AUDIT.md` records the absent RFC-K507,
  schema, canonical form, reader/writer, verifier, signature/trust model,
  privacy policy, migration rules, and executable fixtures.
- `PROTO-EVIDENCE` is Planned public/Future and `GAP-CRITICAL-PROFILE-001`
  remains open; neither is implementation authority.
- Accepted bytecode/VM evidence, Audit Source, project manifests, governance
  checks, and internal incident reports keep their own scopes and are not a
  cross-feature Critical or release bundle.
- `DEC-0208` and its predecessors authorize only test-local boundary
  vocabulary; they do not provide evidence producers or accepted claims.

## Conformance plan

- Assert all sixty Evidence Bundle categories and local order; compare
  forward/reverse opaque bytes; reject duplicates; retain non-claim, offline-
  verification, and no-code-execution categories together.
- Defer bundle/schema/verifier implementation, identity/polarity/trust/privacy
  semantics, diagnostics, protocols, and public support until Accepted
  authority and offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing manifests, Audit Source, tests, reports, proofs, traces,
hashes, and internal evidence are not reinterpreted as a public bundle; only
test-local evidence is added.

## Unresolved alternatives

Versioned canonical container and manifest; required/optional fields,
ordering, size limits, digest domains, references, unknown fields and
migration; Program/Source/Semantic, authority, dependency/lock/build,
Profile/target/toolchain/TCB and artifact identities; Audit Source,
conformance/property/fuzz, Contract/proof, model-check/counterexample/replay,
timing/memory, FFI/Target Package and assumption linkage; passed/failed/
skipped/unavailable/assumed/unknown/bounded/non-claim polarity; source spans
and cross-references; AI provenance, human review, privacy/redaction/retention;
signatures, trust roots, independent offline verification and no-code-
execution behavior; malformed/corrupt/unsupported-version and fail-closed
outcomes; diagnostics, positive/negative/Unicode/determinism fixtures,
protocol inventory, and public support remain open under EVD-5801, EVD-5802,
EVD-5803, EVD-5804, RFC-K507, GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, and
missing evidence authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
