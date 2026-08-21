# EVD-5804 Authority Audit

- Task: `EVD-5804` — AI Provenance
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:566-581`
- Release: G5
- Status: `BlockedSpec`

## Decision

EVD-5804 is `BlockedSpec`. The plan proposes recording agent/tool identity,
input semantic snapshot, task/goal, changed semantic nodes, preserved
contracts, new obligations, verification commands, and human approval, while
not requiring disclosure of complete private conversations and explicitly
stating that AI provenance is traceability rather than correctness evidence.
It does not define a provenance schema, identity authority, privacy boundary,
approval semantics, retention policy, or bundle linkage.

No accepted RFC-K507 or replacement authorizes AI provenance as a public
protocol. EVD-5801 through EVD-5803 and `PROTO-EVIDENCE` remain Future/blocked.
Implementing provenance fields now would invent governance and privacy
semantics, risk recording secrets or private prompts, and could cause
traceability metadata to be mistaken for proof or human signoff.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:566-581` is a non-normative checklist. It does not
  define field types, canonicalization, agent/tool identity, semantic snapshot
  identity, changed-node selection, approval authority, retention, redaction,
  access control, or how provenance is linked to an Evidence Bundle.
- `PROTO-EVIDENCE` is Planned public/Future with no version, schema, canonical
  encoding, reader/writer, verification, privacy/redaction, migration policy,
  or fixtures. AI provenance cannot be implemented as a bundle extension before
  EVD-5801's schema and trust boundary are accepted.
- `PROTO-SEMANTIC-GRAPH-JSON` is an Experimental semantic snapshot protocol.
  Its AI/editor tooling consumer note does not authorize agent identity,
  conversation capture, human approval, or provenance records; semantic
  lifecycle and Stable/Experimental compatibility remain open.
- `GAP-CRITICAL-PROFILE-001` remains Open and requires evidence, provenance,
  independent checking, and reproducible-build decisions before Critical
  claims. No RFC-0012/RFC-K507 provenance authority is present or Accepted.
- `docs/ROADMAP-1.0.md:480-490` requires future evidence-bundle provenance and
  non-claims, while its later release sections call for provenance/SBOM and
  review artifacts. The roadmap is Planning authority, not a privacy or
  provenance contract.
- Existing governance lifecycle records, task status, Semantic Graph JSON, and
  VM/bytecode test reports are project/evidence records with separate scopes.
  They do not establish AI provenance identity or human approval semantics.

## Evidence in this repository

There is no AI provenance schema, bundle field definition, agent/tool identity
registry, prompt/input redaction policy, approval/signoff verifier, retention or
access-control implementation, provenance reader/writer, or provenance fixture
under `crates/`, `tests/`, or `schemas/`. Existing AI/editor references only
consume semantic snapshots and do not record conversations or approvals. No
`ling` CLI, LSP request, diagnostic, or public protocol claims EVD-5804 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. A versioned provenance schema and canonical linkage to bundle, source,
   Semantic/Program IDs, checked snapshots, changed semantic nodes, artifacts,
   evidence results, and review records. It must distinguish engineering
   traceability from correctness, proof, or approval.
2. Stable agent/tool identity and versioning, input/goal/task representation,
   command/tool evidence, preserved contracts, new obligations, and human
   reviewer identity/decision, including how automated and human actions are
   separated and audited.
3. Privacy and security rules: default redaction, secret/credential/PII and
   private conversation handling, prompt and source disclosure scopes,
   retention/deletion, access control, export, incident response, and explicit
   prohibition on placing sensitive text in Semantic IDs or canonical hashes.
4. Approval and trust semantics, reproducibility, tamper evidence, bundle
   signatures if any, verifier/TCB boundaries, unknown/migration behavior, and
   fail-closed handling for unverifiable, incomplete, contradictory, or stale
   provenance. Human approval must not be inferred from an agent or tool log.
5. Registered bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and process behavior
   for invalid identity, redaction failure, missing approval, privacy policy
   violation, hash/link mismatch, unsupported producer, and malformed data.
6. Offline positive, negative, redaction, secret/PII, private-conversation,
   approval, tamper, migration, unknown-field, Unicode 17.0.0, BOM/CRLF,
   source-span, repeated-run, and deterministic fixture suites using synthetic
   data only where real sensitive material is unnecessary.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no provenance schema,
agent registry, privacy/approval service, bundle field, signature dependency,
diagnostic, CLI command, or placeholder API, and it introduces no stale `zero`
names.

EVD-5804 remains deferred until EVD-5801/EVD-5802/EVD-5803, Critical Profile,
semantic lifecycle, reproducible-build, privacy, trust, provenance, and
evidence authorities are Accepted with executable fixtures. AI provenance must
remain traceability metadata only and must never be advertised as correctness
evidence.
