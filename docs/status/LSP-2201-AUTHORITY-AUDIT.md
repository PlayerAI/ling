# LSP-2201 Authority Audit: Compiler diagnostic adapter

## Outcome

`LSP-2201` is correctly recorded as `BlockedSpec`. The execution plan proposes
mapping Ling stable codes, bilingual messages, byte spans, related labels,
severity/tags, and versioned fix data into LSP diagnostics. Ling's diagnostic
registry and JSON writer are already a Preview protocol, but no accepted LSP
adapter defines position conversion, field stability, or fix-data ownership.

Accepted DEC-0034 closes only the bounded `LSP-2201-ORDERING` child: an
internal canonical key over logical names, original UTF-8 byte spans, stable
code text, and a local tie-breaker. No LSP diagnostic adapter, range mapper,
related-information policy, severity translation, tag mapping, fix-data field,
or new diagnostic code was added. Existing diagnostic JSON and CLI rendering
remain unchanged.

## Normative traceability

- `docs/SEMANTICS.md` §26 and `docs/ERROR-CODES.md` require stable bilingual
  codes, root-cause ordering, original UTF-8 byte spans, structured Facts and
  repairs, and explicit localization. These are compiler/diagnostic
  authorities, not an LSP wire schema.
- `PROTO-DIAGNOSTIC-JSON` is a Preview writer with exact schema gating; it does
  not define LSP `Diagnostic` fields or a reader/adapter.
- Accepted DEC-0002 requires LSP positions to be an explicitly labeled
  SourceMap projection and forbids changing Span identity.
- Accepted DEC-0034 fixes only a path-free diagnostic ordering key; it does not
  authorize LSP position conversion, field mapping, publication, or caps.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves position/snapshot/version and
  Workspace Edit fields open; `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves
  public semantic protocol lifecycle open.
- RFC-0005 explicitly allocates no Trait diagnostics and forbids a Trait LSP
  claim without independent fixtures; future adapters cannot invent those
  codes.

## Current interface evidence

The current repository confirms the split boundary:

- `ling-diagnostics` renders bilingual human/JSON diagnostics with stable codes
  and byte spans, but it has no LSP range, related-information, severity/tag,
  or document-version model.
- The ordering child is deliberately not wired to `ling-diagnostics` or
  `LspServer`; it cannot claim adapter, publication, severity, or suppression
  behavior.
- `ling-cli` and REPL consume the diagnostic writer; there is no LSP server or
  adapter that can associate a diagnostic with an open-document snapshot.
- No protocol inventory entry defines an LSP diagnostic schema, fix-data
  stability, or mapping for package/semantic IDs.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. mapping of codes, bilingual messages, severity, tags, byte spans, related
   labels, Facts, repairs, and Semantic IDs to LSP fields;
2. negotiated position encoding and document/snapshot version association,
   stale-result behavior, URI/path policy, and range conversion failures;
3. Stable versus Experimental fields, fix-data schema/ownership, code links,
   localization, and unknown/retired diagnostic behavior;
4. publication/clearance lifecycle, debounce/cancellation, root-cause/error
   storm policy, and stdout/stderr/JSON-RPC transport boundaries; and
5. positive, negative, multi-span, related-info, severity/tag, localization,
   stale-version, Unicode/CRLF/BOM, deterministic, and migration fixtures.

Until those decisions and fixtures are Accepted, an adapter could misreport a
byte span, freeze experimental repair data, or expose an unregistered Trait
diagnostic as editor behavior.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/ERROR-CODES.md`,
`docs/decisions/0002-source-position-units.md`, `docs/ROADMAP-1.0.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
`crates/ling-diagnostics`, `crates/ling-cli`, and the current error-code
registry.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, runtime, bytecode, VM, or Unicode 17.0.0 claim is
made.

## Intentionally deferred

The parent `LSP-2201` can begin after LSP lifecycle/position/snapshot decisions
and an adapter schema are Accepted. The implementation must reuse registered
Ling diagnostics, derive ranges from SourceMap, preserve byte-span truth, and
keep experimental fix data clearly versioned. The `LSP-2201-ORDERING` child is
complete only for DEC-0034's internal key.
