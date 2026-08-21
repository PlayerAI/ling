# LSP-2205 Authority Audit: Diagnostic fixtures

## Outcome

`LSP-2205` is correctly recorded as `BlockedSpec`. The execution plan asks for
LSP diagnostic fixtures covering stable codes, spans, related information,
stale results, and publication behavior. Existing compiler/CLI diagnostic
fixtures exercise Ling diagnostics and JSON, not an accepted LSP schema.

No LSP fixture corpus, expected JSON-RPC messages, position snapshots, result
IDs, stale-result vectors, or placeholder editor protocol was added.

## Normative traceability

- `docs/ERROR-CODES.md`, `docs/SEMANTICS.md`, and existing diagnostic fixtures
  define compiler-level bilingual codes, byte spans, Facts, repairs, and
  deterministic ordering; they do not define LSP request/response fixtures.
- `PROTO-DIAGNOSTIC-JSON` is a Preview writer and cannot serve as an LSP
  fixture schema. No LSP protocol inventory entry exists.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave positions, snapshots, versions,
  publication, and protocol lifecycle open.
- LSP-2201 through LSP-2204 are prerequisite adapter/publication/policy
  boundaries; fixtures cannot be normative before those decisions are accepted.

## Current interface evidence

The current repository confirms the missing boundary:

- `crates/ling-diagnostics` and `crates/ling-cli/tests` cover human/JSON
  rendering, stable codes, byte spans, and CLI/REPL behavior.
- No test harness consumes JSON-RPC messages, LSP diagnostics, negotiated
  positions, document versions, push/pull result IDs, or error-storm metadata.
- No fixture metadata records protocol version, Stable/Experimental fields,
  workspace snapshot identity, or migration vectors for an LSP adapter.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. LSP protocol/schema versions, request/notification transport, field
   stability, and fixture metadata/loader rules;
2. diagnostic mapping, positions, related information, severity/tags,
   experimental data, result IDs, snapshot/version, and stale behavior;
3. push/pull publication, cancellation, error-storm caps, clear/recovery,
   workspace/related-file scope, and deterministic ordering;
4. cross-platform stdout/stderr/stdio behavior, localization, offline/project
   setup, and migration/compatibility policy; and
5. positive, negative, malformed, Unicode/CRLF/BOM, stale/cancelled,
   cross-file, deterministic, and schema-migration fixtures.

Until those decisions and fixtures are Accepted, writing expected LSP bytes
would freeze an unregistered protocol and could turn implementation details
into compatibility commitments.

## Evidence and compatibility

This audit was checked against `docs/ERROR-CODES.md`, `docs/SEMANTICS.md`,
`docs/ROADMAP-1.0.md`, `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-diagnostics`, `crates/ling-cli/tests`, and the LSP-2201–2204
authority audits.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

`LSP-2205` can begin after the LSP diagnostic schema and publication/error-storm
decisions are Accepted. The fixture corpus must be generated from those
contracts, include both polarities and migration evidence, and remain separate
from the compiler diagnostic authority.
