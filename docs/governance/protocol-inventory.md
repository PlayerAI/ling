# Ling 公开接口与协议清单 / Public Protocol Inventory

> 状态：由 `protocol-inventory.toml` 确定性生成
> 更新日期：2026-08-22
> 本清单记录当前兼容边界，不新增语言语义或协议承诺。

## Summary

- 24 records: 18 current public, 1 internal, 5 Future.
- Current public stability: 10 Experimental, 8 Preview, 0 Stable.
- `Stable` means the ROADMAP-1.0 1.x commitment. No current Seed protocol has passed that gate; stable diagnostic codes remain a documented compatibility subset inside the Preview Diagnostic protocol.

## Inventory

| ID | Visibility | Category | Current version | Stability | Public schema | Canonical | Fixtures |
| --- | --- | --- | --- | --- | --- | --- | ---: |
| `PROTO-CLI` | Public | CLI | `0.0.1-dev` | `Preview` | no | no | 2 |
| `PROTO-CLI-EXIT` | Public | CLI | `0.0.1-dev` | `Preview` | no | yes | 3 |
| `PROTO-LSP-LIFECYCLE` | Public | LSP | `ling.lsp.lifecycle/0.1` | `Preview` | no | no | 3 |
| `PROTO-LSP-OVERLAY` | Public | LSP | `ling.lsp.overlay/0.1` | `Experimental` | no | no | 2 |
| `PROTO-HUMAN-OUTPUT` | Public | Human output | `0.0.1-dev` | `Preview` | no | no | 2 |
| `PROTO-DIAGNOSTIC-JSON` | Public | JSON | `ling.diagnostic/0.1` | `Preview` | yes | no | 8 |
| `PROTO-FORMAT-CLI` | Public | JSON | `ling.format/0.1` | `Preview` | yes | no | 5 |
| `PROTO-LOCKFILE` | Public | JSON | `ling.lock/1` | `Experimental` | yes | yes | 8 |
| `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.2` | `Experimental` | yes | yes | 6 |
| `PROTO-REPL-JSON` | Public | JSON | `ling.repl/0.1` | `Preview` | yes | no | 5 |
| `PROTO-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.1` | `Experimental` | yes | yes | 6 |
| `PROTO-CANONICAL-BYTES` | Public | Canonical identity | `file-mode v1 and package-aware v2 domain encodings` | `Experimental` | no | yes | 2 |
| `PROTO-PACKAGE-IDENTITY` | Public | Canonical identity | `v1 domain encodings` | `Experimental` | no | yes | 9 |
| `PROTO-SEMANTIC-ID` | Public | Canonical identity | `experimental:blake3:` | `Experimental` | no | yes | 4 |
| `PROTO-AUDIT-SOURCE` | Public | Text protocol | `ling.audit/0.1` | `Preview` | yes | yes | 2 |
| `PROTO-PACKAGE-MANIFEST` | Public | Package metadata | `ling.manifest/1` | `Experimental` | no | no | 26 |
| `PROTO-BYTECODE` | Public | Bytecode | `ling.bytecode/1.2` | `Experimental` | no | no | 7 |
| `PROTO-VM-CONTROL` | Public | Runtime control | `ling.vm.control/0.1` | `Experimental` | no | no | 4 |
| `PROTO-INTERNAL-INCIDENT` | Internal | Incident | `ling.internal-incident/0.1` | `Internal` | no | no | 1 |
| `PROTO-SEMANTIC-TRANSACTION` | Planned public | Transaction | — | `Future` | no | no | 0 |
| `PROTO-BUILD-METADATA` | Planned public | Package metadata | — | `Future` | no | no | 0 |
| `PROTO-REPLAY` | Planned public | Replay | — | `Future` | no | no | 0 |
| `PROTO-ABI` | Planned public | ABI | — | `Future` | no | no | 0 |
| `PROTO-EVIDENCE` | Planned public | Evidence | — | `Future` | no | no | 0 |

## Reader, writer, and migration policies

### `PROTO-CLI` — Ling command and option surface

- Producer: ling executable
- Consumer: humans; shell scripts; editor and build integrations
- Reader policy: The hand-written parser accepts --help/-h, --version/-V, run, check, semantic, audit, repl, fmt, the Preview lsp --stdio launcher, --format human|json where applicable, and the REPL-only --capability Console.Write; unknown commands/options and invalid arity are rejected with exit 2.
- Writer policy: Help and version output describe only implemented commands; compiler commands route through the shared checked pipeline, lsp --stdio routes to the framed lifecycle server, and no placeholder command is advertised.
- Unknown-field policy: Not field-based: unknown commands, options, formats, and capabilities are rejected.
- Migration tool: None; incompatible command or option changes require an accepted specification and release migration notes.
- Authority: `DEC-0003`, `DEC-0013`, `DEC-0015`, `DEC-0016`, `RFC-0004`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: The compiler package version is the current CLI version; no independent CLI schema identifier exists. RFC-0004 adds only the explicitly gated Preview `ling lsp --stdio` launcher.

### `PROTO-CLI-EXIT` — Ling process exit-code mapping

- Producer: ling process
- Consumer: shells; CI jobs; editor and build integrations
- Reader policy: Interpret 0 as success, 1 as compile/check failure or an early LSP exit, 2 as invalid usage, 4 as runtime, host, or LSP transport fault, 5 as internal compiler error, and 6 as semantic snapshot mismatch; 3 is reserved and unreachable in Seed.
- Writer policy: Human versus JSON rendering never changes the exit class; run and scripted REPL preserve the accepted compile/runtime distinction, while the LSP lifecycle preserves shutdown-before-exit status.
- Unknown-field policy: Not field-based: unassigned exit values have no compatibility meaning.
- Migration tool: None; changing an assigned meaning requires an accepted decision and explicit compatibility guidance.
- Authority: `DEC-0013`, `DEC-0016`, `RFC-0004`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`tests/conformance/p7-hello-run/expect.toml`](../../tests/conformance/p7-hello-run/expect.toml), [`tests/conformance/p12-text-format-fault/expect.toml`](../../tests/conformance/p12-text-format-fault/expect.toml)
- Notes: Exit 3 remains reserved for a future accepted Result-returning main and is not current behavior.

### `PROTO-LSP-LIFECYCLE` — Ling LSP lifecycle and stdio transport

- Producer: ling lsp --stdio; ling-lsp lifecycle server
- Consumer: LSP clients; editor and integration test harnesses
- Reader policy: Accept one JSON-RPC 2.0 object per CRLF Content-Length frame; reject malformed framing, invalid IDs, batches, unsupported lifecycle state, and invalid initialize metadata without guessing or converting URIs to host paths.
- Writer policy: Emit only framed compact UTF-8 JSON-RPC responses for initialize and shutdown; preserve request IDs, flush each response, and write no unframed protocol bytes or human text to stdout.
- Unknown-field policy: Unknown JSON-RPC object fields and ASCII transport headers are ignored for this Preview; unknown methods are rejected only when they are requests, and future incompatible fields require a new protocol version.
- Migration tool: None; `ling.lsp.lifecycle/0.1` is current-writer-only and a future Stable/editor contract requires an accepted migration specification.
- Authority: `RFC-0004`, `DEC-0029`
- Sources: [`docs/RFC-0004.md`](../RFC-0004.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-lsp/tests/lifecycle.rs`](../../crates/ling-lsp/tests/lifecycle.rs), [`crates/ling-cli/tests/lsp.rs`](../../crates/ling-cli/tests/lsp.rs), [`tests/protocols/lsp-lifecycle/README.md`](../../tests/protocols/lsp-lifecycle/README.md)
- Notes: Preview lifecycle only: initialize/initialized/shutdown/exit, position-encoding negotiation, bounded opaque workspace folders, deterministic bilingual JSON-RPC errors, and stdio purity. Document synchronization, diagnostics, edits, snapshots, cancellation, and Semantic Transactions remain deferred.

### `PROTO-LSP-OVERLAY` — Ling LSP Preview document overlay

- Producer: ling lsp --stdio; ling-lsp overlay adapter
- Consumer: Preview LSP clients; editor and integration test harnesses
- Reader policy: Accept only the restricted ling://workspace, ling://dependency, and untitled://ling URI forms; require non-negative monotonic document versions and exactly one full-text change; reject invalid, stale, closed, ranged, oversized, or read-only edits without VFS mutation.
- Writer policy: Retain exact UTF-8 text in the session-local VFS, preserve overlay precedence over disk, reveal the latest disk layer on close, remove temporary untitled files, and expose no SourceId or host path on the wire.
- Unknown-field policy: Unknown JSON-RPC fields are ignored by the current Preview parser except range/rangeLength on full-sync changes, which are rejected; incompatible URI, version, or edit evolution requires a new protocol version.
- Migration tool: None; ling.lsp.overlay/0.1 is current-writer-only and remains Experimental.
- Authority: `RFC-0023`, `RFC-0004`, `DEC-0019`
- Sources: [`docs/RFC-0023.md`](../RFC-0023.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-source/src/vfs.rs`](../../crates/ling-source/src/vfs.rs)
- Fixtures: [`crates/ling-lsp/tests/overlay.rs`](../../crates/ling-lsp/tests/overlay.rs), [`tests/protocols/lsp-overlay/README.md`](../../tests/protocols/lsp-overlay/README.md)
- Notes: Full-text Preview synchronization only. Incremental ranges, diagnostics, compiler queries, snapshots, Workspace Edits, cancellation, and Semantic Transactions remain deferred.

### `PROTO-HUMAN-OUTPUT` — Human-readable CLI and diagnostic output

- Producer: ling CLI; ling-diagnostics human renderer; Ling REPL
- Consumer: humans
- Reader policy: Human output is not a machine-readable input protocol; automation must use the versioned JSON or Audit interfaces and process exit code.
- Writer policy: Public diagnostics remain bilingual and preserve stable codes and meanings, while wording, punctuation, layout, prompts, and optional context may improve.
- Unknown-field policy: Not applicable because human output has no field schema.
- Migration tool: Not applicable; no byte-for-byte compatibility is promised.
- Authority: `DEC-0001`, `DEC-0002`, `DEC-0013`, `DEC-0015`, `DEC-0016`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: Stable diagnostic code meanings are a compatibility subset; the surrounding human bytes are not Stable or canonical.

### `PROTO-DIAGNOSTIC-JSON` — Structured bilingual Diagnostic JSON

- Producer: ling-diagnostics JSON renderer; ling CLI --format json; Ling REPL JSON events
- Consumer: CLI integrations; future LSP/editor integrations; test harnesses
- Reader policy: The repository currently provides a writer but no public Diagnostic JSON reader; consumers must gate on the exact schema version.
- Writer policy: Emit schema, stable code, severity, Chinese and English messages, optional UTF-8 byte span and Semantic ID, ordered Facts, and structured Repairs.
- Unknown-field policy: Optional Facts and Repair candidates are compatible extensions; no compatibility promise currently exists for unknown top-level fields.
- Migration tool: None; breaking field changes require a new diagnostic schema with migration guidance, while changed code meaning requires a new code.
- Authority: `DEC-0001`, `DEC-0002`
- Sources: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/governance/error-code-lock.toml`](../governance/error-code-lock.toml), [`tools/xtask/src/error_codes.rs`](../../tools/xtask/src/error_codes.rs), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/diagnostic/0.1/schema.json`](../../schemas/diagnostic/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`tests/conformance/m2-invalid-number/expect.toml`](../../tests/conformance/m2-invalid-number/expect.toml), [`docs/governance/error-code-lock.toml`](../governance/error-code-lock.toml), [`tools/xtask/src/error_codes.rs`](../../tools/xtask/src/error_codes.rs), [`schemas/diagnostic/0.1/schema.json`](../../schemas/diagnostic/0.1/schema.json), [`schemas/diagnostic/0.1/valid`](../../schemas/diagnostic/0.1/valid), [`schemas/diagnostic/0.1/invalid`](../../schemas/diagnostic/0.1/invalid)
- Notes: Code meaning, error/warning classification, and existing Facts types are the documented stable subset; the 0.1 container remains Preview until 1.0 gates close; The Markdown registry is the sole handwritten allocation source; the generated lock and offline checker reject drift, retired reuse, and unregistered implementation/test codes.

### `PROTO-FORMAT-CLI` — Ling formatter CLI and report

- Producer: ling fmt
- Consumer: shell scripts; CI jobs; formatter integrations
- Reader policy: Consumers must gate on the exact ling.format/0.1 marker; no standalone reader is provided and unknown core fields are rejected by the schema.
- Writer policy: Emit exactly one report object in JSON mode with source, check, changed, disposition, and optional formatted text or diagnostics; human mode writes only formatted Author Source bytes when not checking.
- Unknown-field policy: Reject unknown core fields; no extension namespace or compatibility promise exists for this Preview schema.
- Migration tool: None; an incompatible report or write-in-place behavior requires a new schema and accepted decision.
- Authority: `DEC-0003`, `DEC-0023`, `DEC-0028`
- Sources: [`docs/decisions/0028-formatter-cli-contract.md`](../decisions/0028-formatter-cli-contract.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-format/src/author.rs`](../../crates/ling-format/src/author.rs), [`schemas/format/0.1/schema.json`](../../schemas/format/0.1/schema.json)
- Fixtures: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`schemas/format/0.1/schema.json`](../../schemas/format/0.1/schema.json), [`schemas/format/0.1/valid`](../../schemas/format/0.1/valid), [`schemas/format/0.1/invalid`](../../schemas/format/0.1/invalid)
- Notes: Preview, current-writer-only stdout contract; it does not claim in-place writing, range formatting, format-on-save, LSP Workspace Edits, or Semantic Transactions.

### `PROTO-LOCKFILE` — Ling dependency lockfile

- Producer: ling-project lock writer
- Consumer: Ling offline package and build tooling
- Reader policy: Accept only byte-valid canonical ling.lock/1 with exact required fields, identities, ordering, reachability, and acyclic references; reject unknown fields and every incompatible format without guessing.
- Writer policy: Emit compact UTF-8 JSON with ascending object keys, canonical package/dependency order, and exactly one trailing LF only after the complete local graph validates; unchanged locks are not rewritten.
- Unknown-field policy: ling.lock/1 rejects every unknown field.
- Migration tool: An incompatible lock change uses a new format value and explicit migration; no legacy Ling lock exists.
- Authority: `RFC-0002`
- Sources: [`crates/ling-project/src/lockfile.rs`](../../crates/ling-project/src/lockfile.rs), [`docs/RFC-0002.md`](../RFC-0002.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/lock/1/schema.json`](../../schemas/lock/1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`schemas/lock/1/valid/basic.json`](../../schemas/lock/1/valid/basic.json), [`schemas/lock/1/canonical/basic.bin`](../../schemas/lock/1/canonical/basic.bin), [`schemas/lock/1/invalid/whitespace.json`](../../schemas/lock/1/invalid/whitespace.json), [`crates/ling-project/tests/lockfile_fixtures.rs`](../../crates/ling-project/tests/lockfile_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/path-dependency/expected.ling.lock`](../../tests/projects/path-dependency/expected.ling.lock), [`tests/projects/offline-lock/ling.lock`](../../tests/projects/offline-lock/ling.lock)
- Notes: PRJ-1105 implements the library reader, writer, Update/Locked policy, local-only offline guarantee, and corruption corpus. PRJ-1106 adds end-to-end update, failure-atomicity, and checked-in offline-lock fixtures. PRJ-1108 adds generated model/canonical-byte round trips and enumeration-invariant lock evidence. CLI --locked/--offline selection remains owned by PRJ-1107.

### `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` — Package-aware Semantic Graph JSON

- Producer: ling-semantic package snapshot writer
- Consumer: ling-semantic package-aware isolated reader; future project IDE and build integrations
- Reader policy: Require the exact 0.2, language, and Unicode versions; validate package graph/root identities, package-local module coordinates, IDs, ownership, imports, and cross-package references; decoded data cannot enter evaluation.
- Writer policy: Emit deterministic package-aware JSON only from checked Typed Core produced by the exact resolved PackageGraph; include path-free PackageIdentity coordinates and use v2 Semantic ID domains without changing file-mode 0.1 bytes.
- Unknown-field policy: Accept x-* extension fields at checked object levels and reject unknown core fields.
- Migration tool: None; this is a context-specific package protocol, not a silent replacement or migration claim for file-mode ling.semantic/0.1.
- Authority: `RFC-0002`, `DEC-0012`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/semantic/0.2/schema.json`](../../schemas/semantic/0.2/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-semantic/tests/project_snapshot.rs`](../../crates/ling-semantic/tests/project_snapshot.rs), [`tests/projects/resolution-v1/valid-cross-package`](../../tests/projects/resolution-v1/valid-cross-package), [`schemas/semantic/0.2/schema.json`](../../schemas/semantic/0.2/schema.json), [`schemas/semantic/0.2/valid`](../../schemas/semantic/0.2/valid), [`schemas/semantic/0.2/invalid`](../../schemas/semantic/0.2/invalid), [`schemas/semantic/0.2/canonical`](../../schemas/semantic/0.2/canonical)
- Notes: File-oriented Seed commands remain on ling.semantic/0.1; PRJ-1107 must explicitly select project mode before any CLI can emit this protocol.; No package-aware Audit Source is claimed because accepted ling.audit/0.1 has no package coordinate model.

### `PROTO-REPL-JSON` — REPL submission event JSON

- Producer: ling repl --format json
- Consumer: scripted REPL clients; test harnesses
- Reader policy: The repository provides no standalone reader; consumers must gate on the exact schema and interpret each line as one submission or console event.
- Writer policy: Emit one JSON object per line with status, committed, submission, and status-specific value/type/effect/capability/diagnostic/console data; never mix raw Console text into JSON mode.
- Unknown-field policy: No unknown-field compatibility is promised for 0.1; consumers must not infer semantics from unrecognized fields.
- Migration tool: None; incompatible event changes require a new schema and migration notes.
- Authority: `DEC-0016`
- Sources: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`docs/decisions/0016-repl-session-semantics.md`](../decisions/0016-repl-session-semantics.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/repl/0.1/schema.json`](../../schemas/repl/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`schemas/repl/0.1/schema.json`](../../schemas/repl/0.1/schema.json), [`schemas/repl/0.1/valid`](../../schemas/repl/0.1/valid), [`schemas/repl/0.1/invalid`](../../schemas/repl/0.1/invalid)
- Notes: Deterministic scripted output is tested, but byte-canonical JSON and an N-1 reader are not claimed.

### `PROTO-SEMANTIC-GRAPH-JSON` — Semantic Graph JSON

- Producer: ling-semantic snapshot writer; ling semantic
- Consumer: ling-semantic isolated reader; ling audit projection; AI and editor tooling experiments
- Reader policy: Require the exact semantic, language, and Unicode versions; validate IDs, kinds, ownership, references, Prelude invariants, and ordering-independent structure; the returned graph is data only and cannot enter evaluation.
- Writer policy: Emit deterministic JSON from checked Typed Core with canonical ordering and no source paths, hash-map order, arena indices, allocation addresses, or Rust debug data in identity.
- Unknown-field policy: Accept x-* extension fields at checked object levels and reject unknown core fields.
- Migration tool: None; schema or identity changes require an explicit version upgrade, migration notes, and regenerated fixtures.
- Authority: `DEC-0012`, `RFC-0022`
- Sources: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/RFC-0022.md`](../RFC-0022.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`schemas/semantic/0.1/valid`](../../schemas/semantic/0.1/valid), [`schemas/semantic/0.1/invalid`](../../schemas/semantic/0.1/invalid), [`schemas/semantic/0.1/canonical`](../../schemas/semantic/0.1/canonical)
- Notes: GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001 keeps Stable versus Experimental fields and cross-version migration open.; RFC-0022 defines the optional Experimental x-ling-trait-ide witness/member projection; it does not add a core field or an LSP wire method.

### `PROTO-CANONICAL-BYTES` — Canonical bytes for semantic identities

- Producer: ling-resolve identity encoder; ling-semantic identity encoders
- Consumer: DefinitionId, REPL DefinitionId, BodyId, ProgramId, and semantic node ID hashers in file and project modes
- Reader policy: No general decoder is exposed; each identity class consumes only its own domain-separated, length-prefixed canonical input.
- Writer policy: Use distinct file-mode v1 or package-aware v2 ASCII domains, version inputs, normalized checked semantics, explicit lengths and values, and canonical collection ordering; v2 Definition/node/Program inputs include path-free package or graph identity while all modes exclude spans, host paths, comments, spelling, arena indices, and hash-map iteration.
- Unknown-field policy: Closed binary projection: unrecognized semantic inputs cannot be appended without a domain/schema version change.
- Migration tool: None; an encoding or normalization change requires a Semantic Schema or ID-prefix upgrade and migration explanation.
- Authority: `DEC-0012`, `RFC-0002`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md)
- Fixtures: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs)
- Notes: The domains are versioned separately in current code; no invented umbrella wire identifier is claimed.; The v2 domains are selected only for package-aware ling.semantic/0.2 snapshots; file mode remains byte-stable on v1.

### `PROTO-PACKAGE-IDENTITY` — Ling package content and dependency-graph identities

- Producer: ling-project local dependency resolver
- Consumer: ling-project package graph and lock writer; ling-resolve and ling-semantic package-aware identity; future build planner
- Reader policy: PackageSourceId and PackageGraphId are opaque, distinct Rust types; no general byte-stream decoder is exposed. Text identities emitted by the resolver use exactly sha256: plus 64 lowercase hexadecimal digits.
- Writer policy: Hash RFC-0002's exact unsigned-64-bit big-endian length-prefixed streams with SHA-256 under the separate ling.package-content/1 and ling.package-graph/1 domains; sort every declared collection by its specified canonical key and exclude host paths, cosmetic manifest text, dependency locators, permissions, timestamps, and unordered iteration.
- Unknown-field policy: Closed binary projection: changing included fields, framing, ordering, normalization, or algorithms requires a new domain version and migration evidence.
- Migration tool: None; incompatible identity evolution requires new content and graph domains and must not reuse existing v1 text identities.
- Authority: `RFC-0002`
- Sources: [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`docs/RFC-0002.md`](../RFC-0002.md)
- Fixtures: [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/valid-transitive/ling.toml`](../../tests/projects/dependency-v1/valid-transitive/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml), [`tests/projects/path-dependency/expected-graph.json`](../../tests/projects/path-dependency/expected-graph.json), [`tests/projects/cycle/expected-diagnostics.json`](../../tests/projects/cycle/expected-diagnostics.json), [`tests/projects/unicode-names/expected-graph.json`](../../tests/projects/unicode-names/expected-graph.json)
- Notes: PRJ-1104 implements recursive local path resolution and freezes independent content/graph vectors; PRJ-1103 consumes those identities for cross-package resolution and ling.semantic/0.2; PRJ-1105 projects them into canonical ling.lock/1 bytes; PRJ-1106 freezes the named end-to-end project graph and failure matrix; PRJ-1108 verifies generated cycle/oracle and filesystem-enumeration invariance properties. CLI project selection, registry/network sources, and publication remain deferred.

### `PROTO-SEMANTIC-ID` — Experimental semantic ID text form

- Producer: ling-resolve and ling-semantic BLAKE3 hashers
- Consumer: file-mode and package-aware Semantic Graphs; Audit Source; REPL events; snapshot validation
- Reader policy: Accept exactly the experimental:blake3: prefix followed by 64 lowercase hexadecimal digits in the identity positions allowed by the current schema.
- Writer policy: Hash the appropriate file-mode v1 or package-aware v2 domain-separated canonical bytes and emit lowercase BLAKE3 hexadecimal text with the experimental prefix.
- Unknown-field policy: Not field-based; unknown algorithms, prefixes, lengths, or non-hex text are rejected by current readers.
- Migration tool: None; algorithm, prefix, dependency propagation, or canonical-input changes require an explicit schema/ID upgrade and cannot silently reuse the current prefix.
- Authority: `DEC-0012`, `RFC-0002`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/RFC-0002.md`](../RFC-0002.md)
- Fixtures: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`crates/ling-semantic/tests/project_snapshot.rs`](../../crates/ling-semantic/tests/project_snapshot.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: GAP-SEMANTIC-HASH-UPGRADE-001 blocks stabilization and migration policy.

### `PROTO-AUDIT-SOURCE` — Canonical Audit Source

- Producer: ling-format Audit renderer; ling audit
- Consumer: ling-format isolated Audit parser; independent audit tooling
- Reader policy: Require the exact Audit version; parse into an isolated AuditModel, validate semantic/reference invariants, and never convert the result to CheckedProgram or evaluator input.
- Writer policy: Emit one BOM-free UTF-8/LF/two-space canonical document with fixed ordering, JSON string escaping, implemented Seed fields only, and exactly one final LF.
- Unknown-field policy: Accept and discard x-* extension fields, accept input field reordering, and reject unknown core fields.
- Migration tool: None; incompatible grammar/model changes upgrade ling.audit/* and must document compatibility with the referenced Semantic Schema.
- Authority: `DEC-0015`
- Sources: [`crates/ling-format/src/lib.rs`](../../crates/ling-format/src/lib.rs), [`docs/decisions/0015-audit-source-format.md`](../decisions/0015-audit-source-format.md)
- Fixtures: [`crates/ling-format/src/lib.rs`](../../crates/ling-format/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: The accepted 0.1 format is Preview rather than 1.0 Stable and embeds Experimental semantic identities.

### `PROTO-PACKAGE-MANIFEST` — Ling package/project manifest

- Producer: Ling project authors and future project tooling
- Consumer: ling-project manifest, module-discovery, and local dependency-graph readers; future build planner
- Reader policy: ling-project accepts exact UTF-8 ling.toml inputs using TOML 1.0, requires manifest-version = 1, validates the complete RFC-0002 model and limits, preserves original byte spans, discovers deterministic module/import graphs, and recursively resolves only explicitly declared vendored path dependencies beneath each referring package root; it performs no ambient project search.
- Writer policy: A future writer emits only the RFC-0002 version-1 model and never infers environment-dependent defaults; no writer is implemented yet.
- Unknown-field policy: Version 1 rejects every unknown top-level key, table, and field.
- Migration tool: No legacy Ling manifest exists; incompatible evolution requires a new manifest-version and explicit migration.
- Authority: `RFC-0002`, `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`crates/ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`fuzz/fuzz_targets/manifest_bytes.rs`](../../fuzz/fuzz_targets/manifest_bytes.rs), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: [`crates/ling-project/tests/manifest_fixtures.rs`](../../crates/ling-project/tests/manifest_fixtures.rs), [`crates/ling-project/tests/discovery_fixtures.rs`](../../crates/ling-project/tests/discovery_fixtures.rs), [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/README.md`](../../tests/projects/README.md), [`tests/projects/manifest-v1/valid-minimal/ling.toml`](../../tests/projects/manifest-v1/valid-minimal/ling.toml), [`tests/projects/manifest-v1/valid-unicode/ling.toml`](../../tests/projects/manifest-v1/valid-unicode/ling.toml), [`tests/projects/discovery-v1/valid-multi-root/ling.toml`](../../tests/projects/discovery-v1/valid-multi-root/ling.toml), [`tests/projects/discovery-v1/import-cycle/ling.toml`](../../tests/projects/discovery-v1/import-cycle/ling.toml), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml), [`tests/projects/single-package/ling.toml`](../../tests/projects/single-package/ling.toml), [`tests/projects/multi-module/ling.toml`](../../tests/projects/multi-module/ling.toml), [`tests/projects/path-dependency/ling.toml`](../../tests/projects/path-dependency/ling.toml), [`tests/projects/cycle/ling.toml`](../../tests/projects/cycle/ling.toml), [`tests/projects/visibility/ling.toml`](../../tests/projects/visibility/ling.toml), [`tests/projects/offline-lock/ling.toml`](../../tests/projects/offline-lock/ling.toml), [`tests/projects/unicode-names/ling.toml`](../../tests/projects/unicode-names/ling.toml), [`tests/projects/manifest-v1/duplicate-field/ling.toml`](../../tests/projects/manifest-v1/duplicate-field/ling.toml), [`tests/projects/manifest-v1/path-traversal/ling.toml`](../../tests/projects/manifest-v1/path-traversal/ling.toml), [`tests/projects/manifest-v1/unsupported-language/ling.toml`](../../tests/projects/manifest-v1/unsupported-language/ling.toml), [`fuzz/corpus/manifest_bytes/malformed`](../../fuzz/corpus/manifest_bytes/malformed), [`fuzz/corpus/manifest_bytes/minimal`](../../fuzz/corpus/manifest_bytes/minimal), [`fuzz/corpus/manifest_bytes/path-traversal`](../../fuzz/corpus/manifest_bytes/path-traversal), [`fuzz/corpus/manifest_bytes/unicode`](../../fuzz/corpus/manifest_bytes/unicode)
- Notes: PRJ-1101 through PRJ-1106 plus PRJ-1108 implement the isolated reader/model, explicit-root source discovery, deterministic module/import graphs, recursive vendored dependency traversal, content/package-graph identities, exported-module visibility, checked package-aware resolution, the canonical local lock protocol, the complete named project fixture matrix, generated path/cycle/order properties, and deterministic manifest fuzz coverage. Manifest writing, ambient or CLI project selection, and build integration remain later PRJ tasks.

### `PROTO-BYTECODE` — Portable bytecode and verifier format

- Producer: VM-1202 ling-bytecode checked Typed Core lowerer and deterministic writer; VM-1203 canonical VerifiedProgramV1 re-encoder; VM-1205 closure/recursion lowerer and ling.bytecode/1.1 writer; VM-1206 aggregate/match lowerer and ling.bytecode/1.2 writer; VM-1208 checked Effect/Capability metadata boundary; VM-1209 table-driven Interpreter–VM differential harness; VM-1210 decoder/resource/cancellation evidence
- Consumer: VM-1203 bounded independent decoder/verifier; VM-1204 verifier-gated ling-vm executor; VM-1205 closure/partial-application VM execution; VM-1206 aggregate/match VM execution; VM-1208 explicit host-capability preflight and Runtime Fault boundary; VM-1209 differential oracle over checked snapshots and verified bytecode; VM-1210 robustness and host-control test harness
- Reader policy: The 1.2 reader accepts valid format (1, 0), (1, 1), and (1, 2) artifacts, dispatches on the exact version before decoding version-specific tables or instructions, validates hard and caller artifact bounds before allocation, rejects unknown executable content, and produces only untrusted decoded models. The independent verifier is the sole constructor of VerifiedProgramV1, and ling-vm accepts only that verified state plus explicit limits and injected host Capabilities.
- Writer policy: The library-only writers accept checked-source lowering output or independently verified models. The 1.0 writer emits RFC-0014; the 1.1 writer emits RFC-0015 closure/recursion; the 1.2 writer emits RFC-0016 aggregate/match records and instructions with canonical type, field, case, update, and source-map order, zero reserved bytes, and path-free metadata under hard and caller-supplied limits. No CLI artifact contract is published.
- Unknown-field policy: Each revision rejects unknown tags, opcodes, flags, fields, nonzero reserved bytes, incompatible versions, and trailing bytes. A reader rejects every newer revision and accepts only the explicitly supported earlier records.
- Migration tool: No previous format exists. A future migration must decode and verify the old version before encoding the new version; no tool is implemented.
- Authority: `RFC-0014`, `RFC-0015`, `RFC-0016`, `RFC-0018`, `RFC-0019`, `RFC-0020`, `RFC-0021`
- Sources: [`docs/RFC-0014.md`](../RFC-0014.md), [`docs/RFC-0015.md`](../RFC-0015.md), [`docs/RFC-0016.md`](../RFC-0016.md), [`docs/RFC-0018.md`](../RFC-0018.md), [`docs/RFC-0019.md`](../RFC-0019.md), [`docs/RFC-0020.md`](../RFC-0020.md), [`docs/RFC-0021.md`](../RFC-0021.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml), [`crates/ling-bytecode/src/lib.rs`](../../crates/ling-bytecode/src/lib.rs), [`crates/ling-bytecode/src/lower/v1_1.rs`](../../crates/ling-bytecode/src/lower/v1_1.rs), [`crates/ling-vm/src/lib.rs`](../../crates/ling-vm/src/lib.rs), [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/src/fault.rs`](../../crates/ling-vm/src/fault.rs), [`fuzz/fuzz_targets/bytecode_bytes.rs`](../../fuzz/fuzz_targets/bytecode_bytes.rs), [`tests/bytecode/README.md`](../../tests/bytecode/README.md)
- Fixtures: [`tests/bytecode/v1/golden/hello.lbc.hex`](../../tests/bytecode/v1/golden/hello.lbc.hex), [`tests/bytecode/v1/golden/hello.dis`](../../tests/bytecode/v1/golden/hello.dis), [`tests/bytecode/v1/malformed-cases.tsv`](../../tests/bytecode/v1/malformed-cases.tsv), [`crates/ling-bytecode/tests/decode_verify.rs`](../../crates/ling-bytecode/tests/decode_verify.rs), [`crates/ling-bytecode/tests/lowering.rs`](../../crates/ling-bytecode/tests/lowering.rs), [`crates/ling-vm/tests/execution.rs`](../../crates/ling-vm/tests/execution.rs), [`crates/ling-vm/tests/differential.rs`](../../crates/ling-vm/tests/differential.rs)
- Notes: VM-1201 through VM-1204 implement the unverified data model, typed index/digest domains, fixed tags/opcodes/limits, checked-snapshot minimal lowering, deterministic writing, debug disassembly, bounded independent decoding, failure-atomic verification, canonical VerifiedProgramV1 re-encoding, verifier-gated execution, registered bilingual diagnostics, and valid/corrupt/fuzz/differential evidence. The protocol is Experimental, is not a DEC-0012 semantic canonical-byte format, and has no CLI artifact command, default backend, or N-1 compatibility claim.; Accepted RFC-0015 is implemented by VM-1205 as the backward-compatible ling.bytecode/1.1 closure/recursion extension. Both 1.0 and 1.1 remain Experimental; no Stable, CLI artifact, default-backend, or general N-1 release promise is implied.; Accepted RFC-0016 is implemented by VM-1206 as the backward-compatible ling.bytecode/1.2 aggregate and checked-match extension. All revisions remain Experimental; no Stable, CLI artifact, default-backend, or general N-1 release promise is implied.; Accepted RFC-0018 is implemented by VM-1208: Effect closure, explicit Capability preflight, source-mapped L-RUNTIME-0001 host Faults, and host-panic containment use the existing wire revisions. The protocol remains Experimental; VM-1209 differential corpus and VM-1210 fuzz/resource work remain separate.; Accepted RFC-0019 is implemented by VM-1209: the table-driven harness compares checked-interpreter and verifier-created VM logical events, Unit results, stable Fault projections, source spans, committed state, and deterministic ProgramId values.; Accepted RFC-0020 is implemented by VM-1210: the existing bytecode protocol gains bounded decoder/resource/cancellation evidence, while the experimental ling.vm.control/0.1 host API is inventoried separately and makes no wire or CLI promise.; Accepted RFC-0021 is implemented by the checked Trait member lowering slice: selected implementation DefinitionIds reuse existing direct-call instructions and do not add a wire revision or serialized dictionary table.

### `PROTO-VM-CONTROL` — Experimental VM host control API

- Producer: ling-vm execute_v1_with_cancellation
- Consumer: host orchestration and VM robustness tests
- Reader policy: No wire reader exists; host code links the explicit Rust API and owns token lifetime and cancellation requests.
- Writer policy: No serialized writer exists; cancellation is a host-memory request and is never inferred from source, Capability, wall clock, or thread state.
- Unknown-field policy: Not applicable because the API has no field-based wire schema.
- Migration tool: None; incompatible API changes require a new ling.vm.control version and an accepted specification.
- Authority: `RFC-0020`, `DEC-0013`
- Sources: [`docs/RFC-0020.md`](../RFC-0020.md), [`crates/ling-vm/src/lib.rs`](../../crates/ling-vm/src/lib.rs), [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/src/fault.rs`](../../crates/ling-vm/src/fault.rs)
- Fixtures: [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/tests/execution.rs`](../../crates/ling-vm/tests/execution.rs), [`fuzz/fuzz_targets/bytecode_bytes.rs`](../../fuzz/fuzz_targets/bytecode_bytes.rs)
- Notes: Experimental host control only: execute_v1 remains non-cancellable, cancellation is cooperative and source-mapped, committed effects remain visible, and structured Task/LSP cancellation is separately unresolved.

### `PROTO-INTERNAL-INCIDENT` — Local internal-incident reproduction report

- Producer: ling-cli internal incident capture
- Consumer: local compiler debugging and incident triage
- Reader policy: No public reader or compatibility contract exists; reports are local debugging data under the OS temporary directory.
- Writer policy: Write versioned pretty JSON containing the incident ID, compiler version, internal stage/detail, and bounded reproduction inputs; expose only a logical report label in public diagnostics.
- Unknown-field policy: Internal-only and unspecified; consumers must not treat fields as a Ling public protocol.
- Migration tool: None; internal reports may evolve with the compiler while keeping public L-INTERNAL facts within their documented compatibility boundary.
- Authority: `DEC-0001`, `DEC-0013`
- Sources: [`crates/ling-cli/src/incident.rs`](../../crates/ling-cli/src/incident.rs)
- Fixtures: [`crates/ling-cli/src/incident.rs`](../../crates/ling-cli/src/incident.rs)
- Notes: This record prevents a versioned implementation artifact from being mistaken for a public 1.x commitment; it is not the Future evidence-bundle protocol.

### `PROTO-SEMANTIC-TRANSACTION` — Semantic Transaction

- Producer: Future checked semantic-edit planner
- Consumer: Future compiler transaction verifier and AI/editor tooling
- Reader policy: Not defined; Draft SEMANTICS sketches input and atomicity but does not authorize a wire schema or reader.
- Writer policy: Not defined; no public writer or placeholder API exists.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `SEMANTICS`, `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/SEMANTICS.md`](../SEMANTICS.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: Blocked by GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001 and related edit/snapshot gaps.

### `PROTO-BUILD-METADATA` — Ling build graph and artifact metadata

- Producer: Future Ling build planner
- Consumer: Future cache, package, IDE, and release tooling
- Reader policy: Not defined; current Cargo target metadata is an implementation concern, not a Ling public schema.
- Writer policy: Not defined; toolchain identity, target/profile inputs, artifact identity, and cache boundaries require accepted specifications.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: No build metadata is exposed as a Ling compatibility surface in v0.0.1.

### `PROTO-REPLAY` — Deterministic replay log

- Producer: Future effect/runtime recorder
- Consumer: Future replay verifier and debugging tooling
- Reader policy: Not defined; no replay decoder or equivalence verifier exists.
- Writer policy: Not defined; recorded effects, ordering, redaction, corruption handling, and divergence semantics remain unresolved.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: Blocked by GAP-DETERMINISTIC-REPLAY-001.

### `PROTO-ABI` — Native/FFI binary ABI

- Producer: Future native backend and target packages
- Consumer: Future linker, foreign interfaces, and deployment tooling
- Reader policy: Not defined; no public ABI decoder, verifier, or compatibility checker exists.
- Writer policy: Not defined; layouts, calling convention, ownership transfer, exceptions/Faults, target identity, and symbol versioning require accepted RFCs.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `LANGUAGE`, `ROADMAP-1.0`
- Sources: [`docs/LANGUAGE.md`](../LANGUAGE.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md)
- Fixtures: —
- Notes: No Rust ABI, allocation detail, or host calling convention is exposed as Ling semantics.

### `PROTO-EVIDENCE` — Critical/release evidence bundle

- Producer: Future build, test, proof, and release evidence pipeline
- Consumer: Future independent evidence verifier and Critical/release tooling
- Reader policy: Not defined; existing Markdown release reports are project records, not a versioned Ling evidence bundle.
- Writer policy: Not defined; identity, provenance, checksums, signatures, proof/test linkage, redaction, and verification rules require accepted specifications.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: The versioned internal incident report is not this future public evidence-bundle protocol.

## Machine source

The machine-readable source is [`protocol-inventory.toml`](protocol-inventory.toml). Run `cargo xtask governance check-protocols` to reject duplicate or missing required records, unversioned implemented/public schemas, invalid stability claims, Preview/Stable protocols without Accepted authority, Stable protocols without fixtures, missing paths/version markers, and generated-report drift.
