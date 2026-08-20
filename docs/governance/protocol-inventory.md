# Ling 公开接口与协议清单 / Public Protocol Inventory

> 状态：由 `protocol-inventory.toml` 确定性生成
> 更新日期：2026-08-21
> 本清单记录当前兼容边界，不新增语言语义或协议承诺。

## Summary

- 20 records: 12 current public, 1 internal, 7 Future.
- Current public stability: 6 Experimental, 6 Preview, 0 Stable.
- `Stable` means the ROADMAP-1.0 1.x commitment. No current Seed protocol has passed that gate; stable diagnostic codes remain a documented compatibility subset inside the Preview Diagnostic protocol.

## Inventory

| ID | Visibility | Category | Current version | Stability | Public schema | Canonical | Fixtures |
| --- | --- | --- | --- | --- | --- | --- | ---: |
| `PROTO-CLI` | Public | CLI | `0.0.1-dev` | `Preview` | no | no | 2 |
| `PROTO-CLI-EXIT` | Public | CLI | `0.0.1-dev` | `Preview` | no | yes | 3 |
| `PROTO-HUMAN-OUTPUT` | Public | Human output | `0.0.1-dev` | `Preview` | no | no | 2 |
| `PROTO-DIAGNOSTIC-JSON` | Public | JSON | `ling.diagnostic/0.1` | `Preview` | yes | no | 8 |
| `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.2` | `Experimental` | yes | yes | 6 |
| `PROTO-REPL-JSON` | Public | JSON | `ling.repl/0.1` | `Preview` | yes | no | 5 |
| `PROTO-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.1` | `Experimental` | yes | yes | 6 |
| `PROTO-CANONICAL-BYTES` | Public | Canonical identity | `file-mode v1 and package-aware v2 domain encodings` | `Experimental` | no | yes | 2 |
| `PROTO-PACKAGE-IDENTITY` | Public | Canonical identity | `v1 domain encodings` | `Experimental` | no | yes | 4 |
| `PROTO-SEMANTIC-ID` | Public | Canonical identity | `experimental:blake3:` | `Experimental` | no | yes | 4 |
| `PROTO-AUDIT-SOURCE` | Public | Text protocol | `ling.audit/0.1` | `Preview` | yes | yes | 2 |
| `PROTO-PACKAGE-MANIFEST` | Public | Package metadata | `ling.manifest/1` | `Experimental` | no | no | 13 |
| `PROTO-INTERNAL-INCIDENT` | Internal | Incident | `ling.internal-incident/0.1` | `Internal` | no | no | 1 |
| `PROTO-SEMANTIC-TRANSACTION` | Planned public | Transaction | — | `Future` | no | no | 0 |
| `PROTO-BUILD-METADATA` | Planned public | Package metadata | — | `Future` | no | no | 0 |
| `PROTO-LOCKFILE` | Planned public | Package metadata | — | `Future` | no | no | 0 |
| `PROTO-BYTECODE` | Planned public | Bytecode | — | `Future` | no | no | 0 |
| `PROTO-REPLAY` | Planned public | Replay | — | `Future` | no | no | 0 |
| `PROTO-ABI` | Planned public | ABI | — | `Future` | no | no | 0 |
| `PROTO-EVIDENCE` | Planned public | Evidence | — | `Future` | no | no | 0 |

## Reader, writer, and migration policies

### `PROTO-CLI` — Ling command and option surface

- Producer: ling executable
- Consumer: humans; shell scripts; editor and build integrations
- Reader policy: The hand-written parser accepts --help/-h, --version/-V, run, check, semantic, audit, repl, --format human|json, and the REPL-only --capability Console.Write; unknown commands/options and invalid arity are rejected with exit 2.
- Writer policy: Help and version output describe only implemented commands; commands route through the shared checked compiler pipeline, and no placeholder command is advertised.
- Unknown-field policy: Not field-based: unknown commands, options, formats, and capabilities are rejected.
- Migration tool: None; incompatible command or option changes require an accepted specification and release migration notes.
- Authority: `DEC-0003`, `DEC-0013`, `DEC-0015`, `DEC-0016`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: The compiler package version is the current CLI version; no independent CLI schema identifier exists.

### `PROTO-CLI-EXIT` — Ling process exit-code mapping

- Producer: ling process
- Consumer: shells; CI jobs; editor and build integrations
- Reader policy: Interpret 0 as success, 1 as compile/check failure, 2 as invalid usage, 4 as runtime or host fault, 5 as internal compiler error, and 6 as semantic snapshot mismatch; 3 is reserved and unreachable in Seed.
- Writer policy: Human versus JSON rendering never changes the exit class; run and scripted REPL preserve the accepted compile/runtime distinction.
- Unknown-field policy: Not field-based: unassigned exit values have no compatibility meaning.
- Migration tool: None; changing an assigned meaning requires an accepted decision and explicit compatibility guidance.
- Authority: `DEC-0013`, `DEC-0016`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`tests/conformance/p7-hello-run/expect.toml`](../../tests/conformance/p7-hello-run/expect.toml), [`tests/conformance/p12-text-format-fault/expect.toml`](../../tests/conformance/p12-text-format-fault/expect.toml)
- Notes: Exit 3 remains reserved for a future accepted Result-returning main and is not current behavior.

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
- Authority: `DEC-0012`
- Sources: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`schemas/semantic/0.1/valid`](../../schemas/semantic/0.1/valid), [`schemas/semantic/0.1/invalid`](../../schemas/semantic/0.1/invalid), [`schemas/semantic/0.1/canonical`](../../schemas/semantic/0.1/canonical)
- Notes: GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001 keeps Stable versus Experimental fields and cross-version migration open.

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
- Consumer: ling-project package graph; ling-resolve and ling-semantic package-aware identity; future lockfile writer and build planner
- Reader policy: PackageSourceId and PackageGraphId are opaque, distinct Rust types; no general byte-stream decoder is exposed. Text identities emitted by the resolver use exactly sha256: plus 64 lowercase hexadecimal digits.
- Writer policy: Hash RFC-0002's exact unsigned-64-bit big-endian length-prefixed streams with SHA-256 under the separate ling.package-content/1 and ling.package-graph/1 domains; sort every declared collection by its specified canonical key and exclude host paths, cosmetic manifest text, dependency locators, permissions, timestamps, and unordered iteration.
- Unknown-field policy: Closed binary projection: changing included fields, framing, ordering, normalization, or algorithms requires a new domain version and migration evidence.
- Migration tool: None; incompatible identity evolution requires new content and graph domains and must not reuse existing v1 text identities.
- Authority: `RFC-0002`
- Sources: [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`docs/RFC-0002.md`](../RFC-0002.md)
- Fixtures: [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/valid-transitive/ling.toml`](../../tests/projects/dependency-v1/valid-transitive/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml)
- Notes: PRJ-1104 implements recursive local path resolution and freezes independent content/graph vectors; PRJ-1103 consumes those identities for cross-package resolution and ling.semantic/0.2. Lockfiles, CLI project selection, registry/network sources, and publication remain deferred.

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
- Sources: [`crates/ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: [`crates/ling-project/tests/manifest_fixtures.rs`](../../crates/ling-project/tests/manifest_fixtures.rs), [`crates/ling-project/tests/discovery_fixtures.rs`](../../crates/ling-project/tests/discovery_fixtures.rs), [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`tests/projects/manifest-v1/valid-minimal/ling.toml`](../../tests/projects/manifest-v1/valid-minimal/ling.toml), [`tests/projects/manifest-v1/valid-unicode/ling.toml`](../../tests/projects/manifest-v1/valid-unicode/ling.toml), [`tests/projects/discovery-v1/valid-multi-root/ling.toml`](../../tests/projects/discovery-v1/valid-multi-root/ling.toml), [`tests/projects/discovery-v1/import-cycle/ling.toml`](../../tests/projects/discovery-v1/import-cycle/ling.toml), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml), [`tests/projects/manifest-v1/duplicate-field/ling.toml`](../../tests/projects/manifest-v1/duplicate-field/ling.toml), [`tests/projects/manifest-v1/path-traversal/ling.toml`](../../tests/projects/manifest-v1/path-traversal/ling.toml), [`tests/projects/manifest-v1/unsupported-language/ling.toml`](../../tests/projects/manifest-v1/unsupported-language/ling.toml), [`fuzz/corpus/manifest_bytes/minimal`](../../fuzz/corpus/manifest_bytes/minimal)
- Notes: PRJ-1101/1102/1104 implement the isolated reader/model, explicit-root source discovery, deterministic module/import graphs, recursive vendored dependency traversal, and content/package-graph identities; PRJ-1103 adds exported-module visibility and checked package-aware resolution. Manifest writing, ambient or CLI project selection, locks, and build integration remain later PRJ tasks.

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

### `PROTO-LOCKFILE` — Ling dependency lockfile

- Producer: Future Ling dependency resolver
- Consumer: Future offline package and build tooling
- Reader policy: RFC-0002 defines exact ling.lock/1 validation and rejects unknown fields, noncanonical bytes, dangling identities, cycles, and incompatible formats; no reader is implemented yet.
- Writer policy: A future PRJ-1105 writer emits RFC-0002 canonical compact JSON atomically after complete graph validation; no writer is implemented yet.
- Unknown-field policy: ling.lock/1 rejects every unknown field.
- Migration tool: An incompatible lock change uses a new format value and explicit migration; no legacy Ling lock exists.
- Authority: `RFC-0002`, `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: The protocol design is Accepted; visibility remains Planned public/Future until PRJ-1105 supplies the reader, writer, and canonical corpus.

### `PROTO-BYTECODE` — Portable bytecode and verifier format

- Producer: Future checked Typed Core to bytecode compiler
- Consumer: Future bytecode verifier and VM
- Reader policy: Not defined; no decoder or verifier exists.
- Writer policy: Not defined; no bytecode magic, version, instruction set, Fault mapping, or encoding is accepted.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: Blocked by GAP-BYTECODE-SEMANTICS-001.

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
