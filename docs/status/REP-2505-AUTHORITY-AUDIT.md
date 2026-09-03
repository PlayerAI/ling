# REP-2505 Authority Audit: Replay Privacy, Trimming, and Corruption

## Outcome

`REP-2505` is Done only for the private Experimental baseline authorized by
Accepted DEC-0283 and bound to implementation commit
`b31bd278df5f3482052c463186063b03e13329c9`. Its five-case matrix over existing
DEC-0267/DEC-0282 Task traces proves raw payload retention, strict prefix/gap
rejection, mutation refusal, and explicit-input hermetic reconstruction without
claiming a public privacy or integrity contract.

The public G2 plan still requires field-level redaction, exclusion of
secrets/PII from default logs, dependency-preserving trimming, chunk checksums,
corruption diagnostics, and offline Replay tools. No accepted public replay
schema, sensitivity taxonomy, redaction/retention policy, dependency graph,
checksum scope, corruption recovery taxonomy, or offline protocol exists.

No production redactor, log trimmer, checksum/chunk format, corruption
diagnostic, privacy metadata, key/retention component, offline Replay tool, or
placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its privacy checklist cannot
  authorize a data-classification model, redaction bytes, retention behavior,
  checksum algorithm, or diagnostic contract.
- REP-2501 through REP-2504 are Done only for private Experimental baselines.
  No Accepted RFC-C205 or replacement RFC-0010 exists;
  `GAP-DETERMINISTIC-REPLAY-001` remains Open and RFC-0001 remains a Draft
  baseline under DEC-0018.
- `docs/SEMANTICS.md` mentions replay logs, payload hashes, privacy boundaries,
  and Audit Source, but does not define sensitivity labels, secret/PII rules,
  redaction semantics, dependency-preserving trim closure, checksum/chunk
  canonical bytes, corruption recovery, or key management. v0.0.1 implements
  no Replay runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit privacy,
  versioning, and corruption behavior, but do not define a stable schema,
  offline command, or migration policy.
- Accepted DEC-0012 defines canonical bytes for existing Semantic IDs, not
  replay chunks or privacy. DEC-0010/DEC-0013 cover Seed authorization/Faults;
  RFC-0020 excludes Task/Actor replay. None authorizes this data policy.
- `GAP-DETERMINISTIC-REPLAY-001` explicitly requires privacy, corruption,
  migration, and cross-process evidence before replay tasks can graduate.
- Accepted DEC-0108 is vocabulary-only evidence. Accepted DEC-0283 permits
  exact executable evidence for raw retention and private validation refusal
  while leaving all public privacy, checksum, trimming, and recovery contracts
  deferred.

## Current implementation evidence

- The workspace has no public replay log, sensitivity metadata, redaction
  layer, trimmer, dependency graph, chunk checksum, corruption decoder, offline
  player, or privacy diagnostic. No CLI replay command exists.
- `replay_privacy_execution_evidence.rs` contains the exact DEC-0283 five-case
  matrix. The test-local scheduler assertion rejects every strict prefix and
  event gap with exact private structural reasons.
- The repository-local Unicode sentinel remains raw in Host events and private
  canonical trace bytes. This is negative privacy evidence: the bytes must not
  be described as redacted, persisted, or safe for disclosure.
- `ling-semantic` and `ling-bytecode` formats do not define replay payload
  labels, chunk boundaries, or redaction/checksum fields. `ling-eval`/`ling-vm`
  have no replay data sink or privacy boundary.
- No public protocol inventory entry, error-code allocation, compatibility lock,
  fixture, or threat-model evidence defines default logging of secret/PII,
  redaction failure, dependency closure, truncation, checksum failure, or
  offline enforcement.
- Existing source/Unicode/differential tests do not exercise sensitive replay
  data or corrupt/truncated logs.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. sensitivity classes and field-level labels, default allow/deny policy for
   secrets/PII/Capabilities/Resources, explicit opt-in and failure behavior,
   authorization, encryption/key handling, retention, and auditability;
2. redaction representation and canonical bytes, dependency graph/closure
   required for safe trimming, checkpoint/event references, sequence gaps, and
   migration/version behavior;
3. chunking, checksum/integrity algorithm and scope, truncation/corruption
   taxonomy, partial-log policy, diagnostics, recovery/refusal behavior,
   resource limits, and deterministic/offline operation;
4. replay/player interaction, privacy metadata, cross-process/remote boundary,
   Semantic Graph/Audit Source projection, public CLI/protocol stability, and
   data deletion/retention guarantees; and
5. executable positive/negative/migration/privacy/corruption/cross-process
   fixtures covering redaction, forbidden secret/PII, trim dependency closure,
   checksum mismatch, truncation, unknown fields, offline enforcement,
   Unicode/CRLF/BOM spans, deterministic output, and interpreter/VM/runtime
   behavior without unchecked-AST execution.

Until these decisions are Accepted, implementing privacy or corruption rules
would risk leaking sensitive data or silently accepting incomplete evidence.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0012, DEC-0010, DEC-0013,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, CLI, diagnostic, and schema crates.

No production compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor
protocol, Replay privacy/corruption behavior, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed. The only scheduler
addition is a complete `cfg(test)` prefix/gap rejection assertion.

## Intentionally deferred

REP-2505 is Done only for the private DEC-0283 Experimental baseline. Public
privacy, redaction, retention, trimming, checksums, corruption recovery, and
offline tooling still require Accepted RFC-0010 or replacement authority. See
`docs/status/REP-2505-IMPLEMENTATION-REPORT.md` for exact evidence and executed
verification.
