# CTR-5407 Authority Audit

Task: `CTR-5407` — Contract LSP/Zed
Release: G5
Status: `BlockedSpec`

## Outcome

`CTR-5407` is not implementable from the current accepted authority. The
execution plan lists Contract hover, counterexample diagnostics, proof/evidence
code lenses, gutter status, Contract-aware rename, and Audit expansion of
implicit conditions. These are editor requirements, not a versioned LSP
protocol or a definition of the Contract facts that the editor would expose.

The upstream Contract and proof tasks (`CTR-5401` through `CTR-5406`) are all
`BlockedSpec`. RFC-K503 and RFC-K505 are absent, the Contract status vocabulary
still conflicts between the plan and Draft documents, `GAP-CRITICAL-PROFILE-001`
is open, and `PROTO-EVIDENCE` is Future without a schema. The general LSP
foundation is also blocked by `GAP-LSP-TRANSACTION-PROTOCOL-001` and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`. No LSP server, Contract projection,
counterexample/evidence schema, rename transaction, Zed extension, diagnostic
allocation, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:375-382` is a
  non-normative checklist. It does not define request/response methods,
  capability negotiation, versioning, stale snapshots, ranges, status
  payloads, evidence links, or rename atomicity.
- `docs/ROADMAP-1.0.md:243-249` plans LSP sequencing, Semantic Transaction
  use, and UTF-8-to-LSP position projection, but the roadmap cannot authorize
  a Contract field or an editor protocol.
- `docs/decisions/0002-source-position-units.md:9-18` makes original UTF-8
  byte spans authoritative and permits an explicitly labeled future LSP
  UTF-16 projection; it does not define an LSP transport or Contract range
  model.
- `docs/decisions/0019-incremental-query-boundary.md:69-74,98-121` explicitly
  authorizes only an internal query boundary. It adds no LSP request, position
  encoding, JSON schema, or editor field, and leaves LSP cancellation and
  Semantic Transaction decisions separate.
- `LSP-2101` through `LSP-2205` and the IDE tasks are `BlockedSpec` on the
  registered LSP/semantic-protocol gaps. `PROTO-DIAGNOSTIC-JSON` is a Preview
  compiler/CLI diagnostic writer, not an LSP diagnostic or Contract evidence
  schema.
- `docs/governance/gap-register.toml` records open
  `GAP-LSP-TRANSACTION-PROTOCOL-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`,
  and `GAP-CRITICAL-PROFILE-001`. The latter leaves Contract proof/runtime,
  boundedness, model-checking, and evidence claims unaccepted.
- `PROTO-EVIDENCE` in `docs/governance/protocol-inventory.toml` is Planned
  public/Future with no version, schema, canonical form, reader/writer policy,
  migration tool, or fixtures. It cannot back a code lens, hover link, or
  counterexample claim.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` are Draft. Their Contract
  sketches cannot authorize a public LSP status, proof, Audit, or rename
  projection, and their status sets do not match the execution-plan
  `Unknown` terminology.

## Repository evidence

The repository has no JSON-RPC LSP server, Contract AST/Checked Core, proof or
counterexample reader, evidence bundle reader, Contract rename transaction, or
Zed extension. `editors/tree-sitter-ling` provides syntax parsing and
highlighting fixtures only; it does not produce checked Contract facts,
diagnostics, proof/evidence links, or editor edits. Existing compiler/CLI
diagnostics remain bilingual, registered, and original-byte-span based, while
the Preview diagnostic JSON writer has no LSP reader or Contract semantics.

## Required authority before implementation

Accepted Contract, proof/evidence, LSP, and Semantic Transaction decisions must
define at least:

1. Versioned LSP capabilities and request/response schemas for Contract
   hover, status, diagnostics, counterexamples, proof/evidence links,
   implicit-condition Audit, gutter data, and rename; define unknown fields,
   migrations, result ordering, redaction, and client fallback.
2. A checked Contract/Proof/Evidence source of truth with stable IDs,
   provenance, proof status, counterexample identity, invalidation, profile
   admission, and fail-closed handling for unknown, stale, corrupt, or
   unverifiable data. Draft status names must not be promoted by an editor.
3. Snapshot/version and Semantic Transaction rules for rename and every
   workspace edit, including UTF-8 byte-span to negotiated LSP position
   conversion, CRLF/BOM/Unicode 17.0.0 behavior, conflict detection, and
   atomic rejection of stale edits.
4. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts/repairs
   for Contract failures, with clear separation between compiler diagnostics,
   LSP transport fields, and future evidence/proof protocols. Define whether
   counterexamples may contain values, paths, or redacted host data.
5. Offline positive/negative, malformed, stale-version, Unicode/multibyte,
   CRLF/BOM, incremental, rename-conflict, proof/counterexample/evidence-link,
   deterministic ordering, and client capability fixtures before any LSP or
   Zed support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, diagnostics, schema, CLI, LSP, Zed extension,
dependency, Semantic ID, or public protocol. It preserves the checked-only
execution boundary, accepted Seed semantics, original UTF-8 byte spans,
Unicode 17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no editor, Contract,
proof, evidence, or rename claim.

Implementation remains deferred until RFC-K503/RFC-K505 or accepted
replacements, LSP/Semantic Transaction authorities, Critical/evidence
decisions, and executable fixtures establish the public boundary. Do not add
an LSP method, Contract status field, proof/evidence link, counterexample
schema, rename edit, diagnostic allocation, CLI route, Zed package, public
protocol, support claim, or placeholder API while those authorities remain
unresolved.
