# PRJ-1104 content-identified dependency-graph implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `66a64c9b57c8bb327599a7463345c9d2fbe77a51`
> Verified baseline: `main@66a64c9b57c8bb327599a7463345c9d2fbe77a51`

## Outcome

PRJ-1104 adds deterministic recursive resolution for explicit local path dependencies and the canonical package-content and resolved-graph identities defined by Accepted RFC-0002. `resolve_package_graph` accepts only a caller-selected root, loads exact vendored `ling.toml` manifests below their containing packages, snapshots all manifest and source bytes, validates the entire dependency graph, and publishes an immutable sorted `PackageGraph` only after every check succeeds.

`PackageSourceId` is SHA-256 over the exact versioned, length-prefixed package-content stream. `PackageGraphId` is a separate SHA-256 digest over the root identity and canonical edge set. Physical roots, dependency locator spelling, manifest presentation, timestamps, permissions, directory enumeration, and dependency contents are excluded from a referring package's source identity. Host paths are never exposed in the public graph or identity types.

The resolver performs no parent discovery, registry or network access, environment lookup, process or shell execution, build-script execution, lockfile operation, or source write. Manifest and graph failures precede Ling source parsing, and the same retained source snapshot is used for identity hashing and later module parsing.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §3: exact relative dependency paths, package names and versions, case-sensitive logical matching, symlink-aware containment, and bounded inputs.
- RFC-0002 §4: recursive local path resolution, direct dependency namespaces, exact dependency-name matching, deterministic traversal, package-cycle rejection, identity collision rejection, and no ambient registry fallback.
- RFC-0002 §5: distinct `PackageIdentity`, `PackageSourceId`, and `PackageGraphId` types; exact SHA-256 domains and length framing; canonical source/module/dependency ordering; defined exclusions; and graph-edge sensitivity.
- RFC-0002 §7: graph validation before source parsing, failure atomicity, registered bilingual diagnostics, bounded structured Facts, and original UTF-8 byte spans for manifest-local failures.
- RFC-0002 conformance plan: vendored multi-package fixtures, changed-locator invariance, changed-source sensitivity, symlink escape, offline/no-execution guards, and frozen content/graph vectors.
- Accepted [`DEC-0002`](../decisions/0002-source-position-units.md): dependency-manifest diagnostics retain original UTF-8 byte offsets through CRLF and multi-byte prefixes.
- Accepted [`DEC-0007`](../decisions/0007-module-and-file-boundaries.md): package source discovery retains exact logical module/file mapping and deterministic import validation.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1104: local path dependencies, content identity, no registry auto-networking, cycle/collision/version failures, read-only dependency sources, canonical graph order, and path-free identity.

## Implementation

- [`package_graph.rs`](../../crates/ling-project/src/package_graph.rs) owns recursive dependency loading, filesystem confinement, immutable package snapshots, collision/cycle checks, content hashing, graph hashing, canonical ordering, resource limits, and deterministic failure selection.
- [`discovery.rs`](../../crates/ling-project/src/discovery.rs) separates source preparation from parsing. It bounds discovery and reads each source once into retained bytes so graph validation can finish before parsing and hashing cannot observe different bytes from later module discovery.
- [`ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs) exports focused package identity and graph types without exposing physical roots or placeholder lock/registry APIs.
- Each dependency component is matched exactly by directory enumeration. The resolved directory and exact lowercase `ling.toml` must remain below the containing package after canonicalization; symlink aliases or escapes cannot authorize another package.
- One physical package is snapshotted once even when reached by direct and transitive edges. Logical graph edges may share that immutable identity without rereading mutable host state.
- Package-content hashing uses domain `ling.package-content/1`; graph hashing uses `ling.package-graph/1`. Both use unsigned 64-bit big-endian length/count framing and canonical UTF-8 byte ordering.
- The implementation bounds unique identities, physical package instances, retained source bytes, discovered paths, source files per package, source bytes per package, dependency-directory entries, and manifest reads. Those implementation safety ceilings prevent resource exhaustion without changing the Accepted manifest-v1 field limits.
- The implementation follows SRP by keeping package resolution in one project-layer module, DRY by reusing manifest/discovery validation, KISS by returning one complete immutable graph or one deterministic failure, and YAGNI by omitting lock, registry, Git, publication, CLI, and item-visibility placeholders.

## Specification gaps or conflicts

- No unresolved Accepted-specification conflict blocks PRJ-1104. RFC-0002 defines the local graph, identity framing, failure order, and prohibited ambient behavior precisely enough to implement the task.
- The execution backlog originally ordered PRJ-1103 before PRJ-1104. RFC-0002 §5 requires `PackageIdentity` before package-aware Semantic IDs, so commit `44b2399` corrected the technical order without changing language semantics.
- RFC-0002 specifies manifest-v1 limits but does not prescribe every implementation-wide traversal ceiling. The additional deterministic ceilings are private denial-of-service protections: they do not add fields, widen accepted syntax, or claim to be portable protocol maxima.
- Root-manifest selection and parent-directory discovery remain PRJ-1107 concerns. PRJ-1104 accepts an explicit root and therefore does not choose projects or change file-oriented CLI behavior.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `66a64c9b57c8bb327599a7463345c9d2fbe77a51` and the completion metadata derived from it:

- `cargo test --locked --offline -p ling-project` — 13 unit tests, 33 integration tests, and documentation tests pass.
- Seven checked `dependency-v1` fixture trees cover valid direct and transitive graphs, missing/exact-case manifests, exact-case paths, dependency-name mismatch, and package cycles.
- Fourteen package-graph integration tests cover frozen package/content/graph vectors; canonical ordering; direct/transitive physical-package sharing; path, root, presentation, CRLF, creation-order, and locator invariance; source/dependency sensitivity; graph-before-source failure precedence; contextual UTF-8 spans; manifest limits; content/version collisions; read-only sources; symlink escape; and static no-network/no-process guards.
- Frozen vectors include root package source `sha256:9784dc68f2c10713f5945024e5c6085e34b7735be86acc21e27d523e31a918f1`, dependency source `sha256:76c6c29d652bbd86f607a472a6091c5df95c7656d68fcc8d5c14f23517b65ba3`, and graph `sha256:ac20007193def9b78cc55bc082dbc6cd27abb9ad42720091d38f540a9f3fb2e8`.
- `cargo xtask governance check-all` reports 45 authority documents, 26 gaps, 20 lifecycle records, 19 protocol records, and 70 active plus one retired diagnostic allocation across 14 domains.
- Schema validation/compatibility/corrupt-input gates, traceability, support, CI-contract, Seed reproduction, status verification, and execution-plan checksum verification pass.
- `cargo test --workspace --all-features --locked --offline`, full workspace Clippy with warnings denied, workspace check, documentation build, release build, formatting, Unicode regeneration idempotence, and `git diff --check` pass.

No remote CI result, lockfile reader/writer, registry denial at an HTTP boundary, project CLI behavior, sanitizer campaign, or non-Windows local platform result is claimed.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** adds `L-PROJECT-0014` and `L-PROJECT-0015`; host I/O failures reuse `L-IO-0001`. All meanings, bilingual templates, typed Facts, and severities are registered. Existing diagnostic meanings and wire fields are unchanged, and manifest-local spans remain original UTF-8 byte offsets.
- **Schemas and protocols:** adds the Public/Experimental package-identity protocol with versioned content and graph domains. `ling.manifest/1` remains Public/Experimental. Diagnostic JSON remains `ling.diagnostic/0.1`; no lock schema is implemented.
- **CLI and language behavior:** existing file-oriented commands are unchanged because `ling-cli` does not consume `ling-project`. Evaluation still consumes checked Typed Core only.
- **Semantic IDs and canonical bytes:** current Seed `DefinitionId`, `BodyId`, `ProgramId`, Semantic Graph `ling.semantic/0.1`, and Audit bytes are unchanged. Package IDs are distinct types. PRJ-1103 must version the Semantic Schema before adding `PackageIdentity` to semantic identity.
- **Determinism:** identities, packages, edges, diagnostics, and cycle witnesses use exact canonical values and stable ordering. Absolute paths, manifest presentation, locator spelling, enumeration order, timestamps, permissions, Rust hash seeds, and parser allocation are excluded as specified.
- **Unicode:** remains pinned to 17.0.0. Technical package names, versions, dependency keys, and digests are ASCII; source/module validation continues to use generated NFC/XID/security tables, while original source bytes remain package-content inputs.
- **Dependencies:** `ling-project` uses the already locked workspace `sha2` version. No third-party version changed.

## Intentionally deferred work

- PRJ-1103: package-aware import selection, dependency-export visibility, cross-package HIR/name resolution, versioned Semantic Graph package/module/definition identity, and cross-file definition evidence.
- PRJ-1105: canonical `ling.lock/1` reader/writer, corruption corpus, digest verification, `--locked`, and atomic replacement.
- PRJ-1106/1107: complete project conformance fixtures, shared project service/API, explicit CLI manifest selection, and file-mode migration evidence.
- PRJ-1108: property tests and long-running sanitizer-backed fuzz campaigns for graph, path, cycle, identity, and lock invariants.
- Item-level visibility syntax, selective/glob imports, module re-exports, multiple package versions, registry/Git/network dependencies, workspaces, package installation, publication, signatures, and arbitrary build scripts remain outside the Accepted version-1 boundary.
