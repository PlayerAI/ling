# PRJ-1103 package-aware import and visibility implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `8e98d32f54301dee3f198273cfae3146bbf2846b`
> Verified baseline: `main@8e98d32f54301dee3f198273cfae3146bbf2846b`

## Outcome

PRJ-1103 completes the Accepted RFC-0002 package namespace boundary at the library level. A resolved package graph now retains the exact validated source-byte snapshot, rejects imports of missing or unexported dependency modules with distinct registered diagnostics, and feeds package-qualified HIR into name resolution. Module names are package-local, direct dependency names select only their declared package, transitive dependency namespaces remain invisible, and every definition in an exported module is available through the existing explicit module-alias model.

`ling-resolve::resolve_project` requires the supplied package and module set to match the validated `PackageGraph` exactly, canonicalizes caller ordering, preserves cross-file source names and original UTF-8 spans, and assigns package-aware v2 `DefinitionId` values. File-mode `resolve` remains on the frozen v1 identity path.

`ling-semantic::build_project` emits deterministic `ling.semantic/0.2` snapshots with path-free package, module, definition, node, and reference coordinates. The isolated 0.2 reader rejects missing required fields, invalid identities, unknown package modules, private imports, unimported cross-module references, coordinate mismatches, dangling ownership, and dangling references. Existing file-mode `ling.semantic/0.1` and `ling.audit/0.1` bytes remain unchanged.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §4: direct dependency namespace selection, transitive dependency invisibility, exported-module visibility, package-private unexported modules, package-local module identity, and the absence of item-level visibility or re-export semantics.
- RFC-0002 §5: package-aware semantic identity consumes the full path-free `PackageIdentity`; the Semantic Schema and canonical hash domains are versioned before those coordinates enter public IDs.
- RFC-0002 §7: dependency-module failures are atomic, deterministic, bilingual, code-stable, and carry bounded Facts plus original UTF-8 import spans.
- RFC-0002 conformance plan: valid cross-package resolution, duplicate module names in different packages, direct-only namespaces, missing/private distinction, content identity consumption, and cross-file definition evidence.
- Accepted [`DEC-0002`](../decisions/0002-source-position-units.md): retained sources and definition locations preserve original UTF-8 byte offsets.
- Accepted [`DEC-0007`](../decisions/0007-module-and-file-boundaries.md): explicit imports retain their last-segment/default or declared alias and resolve through canonical module coordinates.
- Accepted [`DEC-0012`](../decisions/0012-semantic-identity-and-canonical-bytes.md): v2 definition, body, program, and semantic-node domains use typed, domain-separated canonical bytes and exclude spans, paths, presentation, allocation order, and Rust debug state.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1103: HIR/resolve cross-module imports, coarse public/private visibility, stable root-cause diagnostics, package-aware Semantic Graph identity, and cross-file definition locations.

## Implementation

- [`package_graph.rs`](../../crates/ling-project/src/package_graph.rs) validates every direct dependency import after all packages have been loaded. `L-PROJECT-0016` identifies a missing dependency module, while `L-PROJECT-0017` identifies an existing module absent from `exports.modules`.
- [`discovery.rs`](../../crates/ling-project/src/discovery.rs) publishes `PackageSource`, containing only canonical logical coordinates and the exact source bytes already used for package identity and parsing. Its custom `Debug` output exposes byte length, never contents or a physical root.
- [`ling-resolve`](../../crates/ling-resolve/src/lib.rs) keys modules by `(PackageIdentity, module name)`, resolves imports against only the importing package or its direct dependency map, accepts duplicate module names across packages, and retains path-free project metadata for downstream identity generation.
- Project resolver inputs are sorted and checked for unexpected, duplicate, or missing packages/modules before resolution. This prevents vector order from changing module IDs, references, diagnostics, or snapshots.
- Package-aware `DefinitionId` uses `ling.definition-id/v2`; user definitions include their exact package identity and system definitions carry a separate system tag. File and REPL v1 domains are unchanged.
- [`ling-semantic`](../../crates/ling-semantic/src/lib.rs) provides a separate project builder/reader using `ling.semantic/0.2`, `ling.body-id/v2`, `ling.program-id/v2`, and `ling.semantic-node-id/v2`. Program identity includes the package-graph ID, root identity, canonical package metadata, and sorted definition/body pairs.
- The 0.2 reader enforces its registered `RejectRequired` policy even for Rust fields that retain Serde defaults for 0.1 compatibility. It also proves that cross-module definition references have a matching import and that cross-package imports target exported modules.
- [`schemas/semantic/0.2`](../../schemas/semantic/0.2) supplies Draft 2020-12 schema, valid/invalid corpus, compact-JSON-LF golden bytes, and the exact isolated reader adapter. Governance registries identify 0.2 as a distinct Experimental package-aware protocol rather than silently migrating file mode.
- The implementation follows SRP by retaining filesystem/package validation in `ling-project`, name resolution in `ling-resolve`, and checked graph projection in `ling-semantic`; it reuses existing graph identities and avoids lock, CLI, item-visibility, or Audit placeholders.

## Specification gaps or conflicts

- No unresolved conflict blocks the Accepted coarse module-visibility behavior implemented by PRJ-1103.
- RFC-0002 permits hyphenated technical package names, while current DEC-0007 import segments use Ling identifiers and define no escaping or dependency-alias syntax. PRJ-1103 does not invent a spelling: such package names remain valid graph identities but are not source-selectable until an Accepted syntax decision exists.
- The plan's generic “public/private” wording is narrower in Accepted RFC-0002: only `exports.modules` is semantic today. Item-level visibility, selective imports, glob/open imports, and module re-exports are deliberately not inferred from the plan summary.
- `ling.audit/0.1` has no package-coordinate model. A package-aware Audit projection would require an Accepted versioned protocol; PRJ-1103 therefore exposes only Semantic Graph 0.2.
- A compiler-host API that constructs HIR directly from retained graph sources and selects project mode for CLI commands belongs to PRJ-1107. The current library integration test exercises that exact pipeline without changing existing file-oriented commands.

## Tests and verification

Executed locally on Windows on 2026-08-21 against implementation commit `8e98d32f54301dee3f198273cfae3146bbf2846b` and the completion metadata derived from it:

- `cargo test --locked --offline -p ling-project` — 13 unit, 36 integration, and documentation tests pass.
- `cargo test --locked --offline -p ling-resolve` — 11 unit, three package-resolution integration, and documentation tests pass.
- `cargo test --locked --offline -p ling-semantic` — 12 unit, five package-snapshot integration, and documentation tests pass.
- Four `resolution-v1` fixture trees cover an exported direct dependency, a missing module, a private module, and an attempted transitive-only namespace.
- Cross-package evidence covers package-local duplicate `Main` modules, exact graph/HIR-set validation, input-order invariance, definition navigation to `package:math/src/Algebra.ling` at original bytes `20..26`, and a resolved reference to that definition.
- Frozen package-aware vectors include Program ID `experimental:blake3:6f3d67e85b5820959041b90cfc9feee4dbca260afe88198ce612fbf4713b2cda`, math `answer` Definition ID `experimental:blake3:40cdc27a3113254a05f5786cb25402b81dfc27e1f0cdce5958af04c7329de5eb`, and Body ID `experimental:blake3:c1f6796db11f7b12827f6d6b433f8c1c6adcdb215556c20001791246b4ae700c`.
- Schema validation reports four schemas, five valid fixtures, eight invalid fixtures, two canonical byte fixtures, four explicit `NoPreviousVersion` records, and 30 deterministic corruptions.
- `cargo xtask governance check-all` reports 45 authority documents, 26 gaps, 20 lifecycle records, 20 protocols, and 73 total diagnostic-code records; traceability, support, status, CI-contract, and Seed-reproduction gates pass.
- `cargo test --workspace --all-features --locked --offline`, full workspace Clippy with warnings denied, workspace check, documentation build, release build, formatting, Unicode regeneration idempotence, and `git diff --check` pass.

No remote CI result, package-aware CLI command, lockfile behavior, long-running fuzz campaign, sanitizer result, or non-Windows local result is claimed.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** adds `L-PROJECT-0016` and `L-PROJECT-0017` with bilingual messages, stable meanings, typed bounded Facts, deterministic selection, and logical source names. Existing error meanings and `ling.diagnostic/0.1` fields are unchanged.
- **Schemas and protocols:** adds Public/Experimental `ling.semantic/0.2` for package-aware snapshots. Its exact-version reader is isolated from file-mode 0.1, accepts only `x-*` extensions, and defaults no required field.
- **Semantic IDs and canonical bytes:** package mode uses v2 definition/body/program/node domains and full `PackageIdentity` coordinates. File-mode `ling.semantic/0.1`, v1 Semantic IDs, REPL identities, Seed output, and Audit bytes remain byte-identical.
- **Determinism:** package/module keys, HIR inputs, imports, definitions, nodes, references, packages, and exports use canonical ordered collections. Package graph identity carries dependency-edge sensitivity; host paths, source presentation, Rust hash seeds, allocation order, and input vector order do not affect snapshots.
- **Unicode:** remains pinned to 17.0.0. Module and definition names reuse the existing NFC/XID/security pipeline, and all locations remain original UTF-8 byte spans.
- **Dependencies:** only internal workspace dependency edges were added (`ling-resolve` and `ling-semantic` consume `ling-project`); no third-party package or locked version changed.
- **Evaluation:** decoded Semantic Graph data remains non-executable. Evaluation continues to consume checked Typed Core only.

## Intentionally deferred work

- PRJ-1105: canonical `ling.lock/1` reader/writer, exact graph comparison, corruption behavior, atomic replacement, and locked/offline policy integration.
- PRJ-1106: the broader named project fixture matrix and expected graph/lock artifacts.
- PRJ-1107: shared compiler-host project loading, manifest selection, CLI `check/run/test/build` integration, exits, and JSON behavior.
- PRJ-1108: property tests and long-running fuzz/sanitizer campaigns for paths, package graphs, visibility, lock round trips, and resource limits.
- Item-level visibility syntax, selective/glob/open imports, re-exports, dependency aliases, multiple package versions, registry/Git/network sources, workspaces, installation, publication, signatures, and arbitrary build scripts remain outside the Accepted version-1 boundary.
