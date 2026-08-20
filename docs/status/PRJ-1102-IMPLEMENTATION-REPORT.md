# PRJ-1102 module-discovery implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `f76f4953070b9ae555fce24c7dcc2fbf08a36f7a`
> Verified baseline: `main@f76f4953070b9ae555fce24c7dcc2fbf08a36f7a`

## Outcome

PRJ-1102 adds deterministic, explicit-root source discovery to `ling-project`. `discover_modules` consumes a caller-selected project root and a validated `ling.manifest/1` model, discovers exact lowercase-extension `.ling` files below declared source roots, validates their path/module boundaries and imports, and publishes an atomic `ModuleGraph` only after every project-graph check succeeds.

The graph contains sorted package-local module nodes and local or direct-dependency import edges. It deliberately contains no host path, file handle, parser arena, Rust map order, dependency contents, visibility result, package digest, lock state, or CLI selection state. Discovery performs no parent search, environment lookup, dependency traversal, network request, shell execution, or lockfile operation.

Six registered bilingual `L-PROJECT-*` diagnostics distinguish invalid source roots, invalid discovered paths, invalid module declarations, duplicate modules, missing modules, and invalid import graphs. Source-local diagnostics preserve original UTF-8 byte spans; all user-controlled diagnostic Facts are bounded and failures are sorted deterministically before publication.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §§1 and 3: explicit project-root operation, exact logical paths, Unicode module names, symlink-aware confinement, non-overlapping resolved source roots, and bounded manifest-controlled inputs.
- RFC-0002 §4: direct dependency names select an explicit import namespace, local top-level module collisions are rejected, and dependency contents or transitive namespaces are not guessed during this task.
- RFC-0002 §§5 and 7: discovered source ordering is independent of directory enumeration and physical paths, entry/export existence is checked, project-graph errors are bilingual and structured, and no partial graph is returned.
- Accepted [`DEC-0002`](../decisions/0002-source-position-units.md): declaration and import diagnostics retain original UTF-8 byte offsets through CRLF and multi-byte prefixes.
- Accepted [`DEC-0007`](../decisions/0007-module-and-file-boundaries.md): exact-case path/module mapping, first and unique module declarations, implicit `Main` only at the entry boundary, import placement, missing-module rejection, cycle rejection, and normalized module-order traversal.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1102: deterministic `.ling` discovery, path-to-module mapping, explicit declarations, duplicate/case/path diagnostics, and stable graph nodes and edges.

## Implementation

- [`discovery.rs`](../../crates/ling-project/src/discovery.rs) owns filesystem confinement, sorted traversal, source candidate construction, module/import validation, graph construction, cycle detection, and deterministic error ordering.
- [`ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs) retains private manifest-source locations for later project diagnostics. These locations are excluded from semantic equality and `Debug`, so caller paths cannot become manifest model behavior.
- Every declared root component is matched exactly through directory enumeration before canonicalization. Resolved roots must remain below the canonical project root and must not overlap after symlink resolution.
- Traversal is iterative and sorted by UTF-8 bytes. Canonical directory identities reject symlink escape, active cycles, and alias re-entry without recursive amplification. Non-UTF-8 components, non-NFC components, invalid Unicode 17 XID module segments, uppercase/mixed-case `.ling` extensions, non-regular `.ling` paths, and oversized sources fail before graph publication.
- A source path `<root>/Game/Math.ling` maps to `Game.Math`. Non-entry modules require one matching `module` declaration as the first non-comment declaration. The only omitted declaration accepted by DEC-0007 is the `Main` entry module.
- Imports must precede ordinary declarations. Local targets must exist; a first segment matching a declared direct dependency produces a typed dependency edge without reading that dependency. Local top-level namespaces cannot collide with dependency names.
- Local import cycles are found with an iterative, ordered depth-first traversal. Cycle Facts use the lexicographically smallest rotation, while graph nodes and edges use canonical sorted collections.
- The implementation follows SRP by keeping project discovery in `ling-project`, DRY by reusing `ling-source`, `ling-syntax`, `ling-ast`, `ling-unicode`, and `ling-diagnostics`, KISS by returning one immutable graph or one ordered error set, and YAGNI by omitting resolver, lock, hash, CLI, and writer placeholders.

## Specification gaps or conflicts

- No unresolved Accepted-specification conflict blocks PRJ-1102. RFC-0002 supplies the project boundary and DEC-0007 supplies source module/import behavior.
- The lower-authority plan asks for generic “unportable path” diagnostics but does not define a broader portable filename profile. The implementation enforces only Accepted, testable boundaries: UTF-8, NFC, Unicode 17 XID module segments, `/` logical separators, exact case, lowercase `.ling`, and symlink containment. It does not invent Windows-reserved-name or host-specific filename semantics.
- RFC-0002 requires cross-package export visibility, recursive local dependency resolution, content identity, and locks, but assigns their implementation evidence across PRJ-1103 through PRJ-1105. PRJ-1102 classifies only direct dependency import edges and makes no visibility or existence claim about dependency modules.
- The existing file-oriented CLI has its own DEC-0007 loader. Replacing it before PRJ-1107 would change public selection behavior, so this task leaves CLI orchestration untouched.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `f76f4953070b9ae555fce24c7dcc2fbf08a36f7a` and the completion metadata derived from it:

- `cargo test --locked --offline -p ling-project` — 9 unit tests, 19 integration tests, and documentation tests pass.
- Fifteen checked `discovery-v1` fixture directories cover valid multi-root and direct-dependency graphs plus source-root case mismatch, invalid source paths, extension case, declaration mismatch/absence, implicit non-`Main`, duplicate modules, missing entry/export/import, two- and three-node cycles, and dependency namespace collision.
- Programmatic tests cover creation-order and physical-root invariance, Unicode paths, aliases, emoji and CRLF byte spans, invalid UTF-8, duplicate/misplaced declarations, late imports, bounded long import Facts, graph path privacy, and manifest origin equality/debug privacy.
- Symlink tests cover root escape, directory cycles, and directory alias re-entry when the host grants symlink creation; Windows permission denial is the only explicit local skip condition.
- `cargo xtask governance check-error-codes` reports 68 active and 1 retired code across 14 domains, with 68 Rust constants.
- `cargo xtask governance check-protocols` reports 18 records and verifies the updated `PROTO-PACKAGE-MANIFEST` reader, sources, fixtures, and deferred scope.
- `cargo xtask governance check-all`, Schema validation/compatibility/corrupt-input gates, traceability, support, CI-contract, Seed reproduction, and status verification pass.
- `cargo test --workspace --all-features --locked --offline`, full workspace Clippy with warnings denied, workspace check, documentation build, release build, formatting, Unicode regeneration idempotence, execution-plan checksums, and `git diff --check` pass.

No remote CI result, dependency package load, export-visibility decision, package hash, lockfile operation, project CLI behavior, network denial instrumentation, sanitizer run, or non-Windows local platform result is claimed.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** adds `L-PROJECT-0008` through `L-PROJECT-0013` with bilingual meanings and typed optional Facts. Existing codes, severities, meanings, and fields are unchanged. Source declaration/import spans remain original UTF-8 byte offsets.
- **Schemas and protocols:** `ling.manifest/1` remains Public/Experimental; its documented reader now includes explicit-root source discovery. No protocol version or public JSON Schema changes. Diagnostic JSON remains `ling.diagnostic/0.1`.
- **CLI and language behavior:** existing file-oriented `ling check/run/semantic/audit` behavior is unchanged because `ling-cli` does not consume `ling-project`. The accepted module declaration and import rules are reused rather than redefined.
- **Semantic IDs and canonical bytes:** current DefinitionId, BodyId, ProgramId, Semantic Graph, and Audit bytes are unchanged. `ModuleGraph` is not a Semantic Graph and does not enter evaluation. Package content and graph hashes remain unimplemented.
- **Determinism:** roots, candidates, nodes, edges, diagnostics, adjacency, and cycle witnesses use exact logical values and canonical ordering. Directory creation/enumeration order, physical project root, symlink spelling, Rust hash seeds, and parser allocation are excluded from successful graph equality.
- **Unicode:** remains pinned to 17.0.0. Module/path validation reuses the existing generated NFC/XID/security implementation; regenerating compiler and Tree-sitter tables is byte-idempotent.
- **Dependencies:** no third-party dependency version changed. `ling-project` adds only existing workspace crates for source decoding, parsing, and AST lowering.

## Intentionally deferred work

- PRJ-1103: cross-package import selection against loaded dependency graphs, exported-module visibility, HIR/name-resolution integration, stable package/module/definition identity, and cross-file definition evidence.
- PRJ-1104: recursive vendored dependency loading, package/content and graph identities, package cycles/collisions, hash vectors, and explicit offline/no-execution guards.
- PRJ-1105: canonical `ling.lock/1` reader/writer, corruption corpus, digest verification, `--locked`, and atomic replacement.
- PRJ-1106/1107: complete multi-project conformance fixtures, a shared project service/API, explicit CLI manifest selection, and unchanged file-mode migration evidence.
- PRJ-1108: property tests and long-running sanitizer-backed fuzz campaigns for graph, path, cycle, and lock invariants.
- Item-level visibility syntax, selective imports, glob/open imports, module re-exports, registry/Git/network dependencies, workspaces, package installation, and publication remain outside the Accepted version-1 project boundary.
