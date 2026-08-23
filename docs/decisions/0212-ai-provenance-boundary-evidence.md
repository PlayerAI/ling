# DEC-0212: Internal AI Provenance boundary evidence / 内部 AI Provenance 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: critical-quality
> 相关规范/缺口：`DEC-0211` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001` | `PROTO-EVIDENCE` | `PROTO-SEMANTIC-GRAPH-JSON`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`EVD-5804-OBSERVATION`. It records provisional actor, semantic linkage,
change/verification, human-review, privacy, trust, failure, and fixture
vocabulary while RFC-K507, provenance, privacy, approval, and bundle-linkage
semantics remain unresolved.

本决定只授权 `EVD-5804-OBSERVATION` 使用 test-local 的 actor、semantic
linkage、change/verification、human-review、privacy、trust、failure 与 fixture
边界清单；在 RFC-K507、provenance、privacy、approval 和 bundle-linkage
semantics 尚未解决时，只记录临时词汇，不声明 AI 产物正确或已获人工批准。

## Question

EVD-5804 proposes recording agent/tool identity, an input semantic snapshot,
task/goal, changed semantic nodes, preserved contracts, new obligations,
verification commands, and human approval without requiring complete private
conversation disclosure. Which vocabulary can be retained as bounded evidence
without defining a provenance schema, privacy policy, approval authority, or
public protocol?

## Decision

1. `crates/ling-types/tests/ai_provenance_evidence.rs` keeps a test-local
   inventory of sixty provisional actor, semantic-linkage, change/
   verification, human-review, non-claim, privacy, trust, failure, diagnostic,
   and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.ai-provenance-observation/0`. These bytes are
   observation evidence only; they are not provenance records, approval,
   correctness evidence, proof, diagnostics, protocols, or support claims.
3. Automated and human actions remain distinct. `TraceabilityOnly`,
   `CorrectnessClaimProhibited`, `ProofClaimProhibited`, and
   `ApprovalInferenceProhibited` explicitly preserve the plan's non-proof
   boundary. A tool log cannot imply human approval.
4. Private conversations, secrets, credentials, and PII remain distinct
   exclusion categories. Their presence does not define redaction, retention,
   access-control, or disclosure policy and does not authorize capture of any
   sensitive content.
5. No provenance reader/writer, agent registry, approval/signoff verifier,
   privacy service, bundle field, signature dependency, CLI/LSP route,
   diagnostic allocation, public protocol, support claim, or placeholder API
   is added. Public `EVD-5804` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:566-581` is a
  non-normative checklist. It defines no field types, canonicalization,
  identities, privacy boundaries, approval semantics, retention, or bundle
  linkage.
- `docs/status/EVD-5804-AUTHORITY-AUDIT.md` records the absent RFC-K507,
  provenance schema, identity authority, privacy/redaction/retention policy,
  approval semantics, diagnostics, and executable fixtures.
- `PROTO-EVIDENCE` is Planned public/Future and
  `GAP-CRITICAL-PROFILE-001` remains open; neither is implementation authority.
- `PROTO-SEMANTIC-GRAPH-JSON` is Experimental. Its AI/editor consumer note
  does not authorize agent identity, conversation capture, approval, or
  provenance records.
- `DEC-0211` authorizes only test-local reproducible-build vocabulary; it
  defines no provenance bytes, identities, trust, privacy, or approval claims.

## Conformance plan

- Assert all sixty AI-provenance categories and local order; compare forward/
  reverse opaque bytes; reject duplicates; retain human approval, traceability-
  only, correctness/proof/approval-inference prohibitions, and private/
  sensitive-content exclusions together.
- Defer provenance schema, privacy and retention policy, approval and trust
  semantics, diagnostics, protocols, and public support until Accepted
  authority and synthetic offline fixtures exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Existing governance records, Semantic Graph snapshots, task status,
and test reports are not reinterpreted as AI provenance or human approval;
only test-local boundary evidence is added.

## Unresolved alternatives

Versioned provenance schema and canonical bytes; agent/tool/reviewer identity
and version authorities; semantic snapshot, changed-node, contract, obligation,
artifact, evidence, and review linkage; task/goal and verification-command/
result representation; automated versus human actions; approval decision,
authority, revocation, and contradiction; traceability/non-proof semantics;
prompt/source disclosure scopes; secret, credential, PII, private-conversation
redaction; retention, deletion, access control, export, and incident response;
tamper evidence, signatures, trust, and TCB; incomplete/contradictory/stale/
malformed/corrupt/unsupported/migration and fail-closed behavior; bilingual
stable diagnostics and exits; positive, negative, redaction, secret/PII,
private-conversation, approval, tamper, Unicode 17.0.0, BOM/CRLF, source-span,
and determinism fixtures; protocol inventory and public support remain open
under EVD-5804, EVD-5801, EVD-5802, EVD-5803, RFC-K507,
GAP-CRITICAL-PROFILE-001, PROTO-EVIDENCE, PROTO-SEMANTIC-GRAPH-JSON, and
missing provenance/privacy/approval authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
