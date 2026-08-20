# PRJ-1101 project-manifest implementation report

> Status: **Done**
>
> Completed: 2026-08-21
>
> Final verified implementation commit: `80f46bcf3d175eeb6402bf6267085cb905a5dbcf`
> Verified baseline: `main@80f46bcf3d175eeb6402bf6267085cb905a5dbcf`

## Outcome

PRJ-1101 adds the `ling-project` crate and an isolated, deterministic reader for the Accepted `ling.manifest/1` protocol. `parse_manifest` accepts caller-supplied bytes and a diagnostic display name, performs no filesystem or network access, rejects invalid encoding and structure before semantic validation, and returns a typed model whose roots, exports, and dependencies have deterministic ordering.

The reader implements the complete RFC-0002 manifest-version 1 field model: graph-local ASCII package names, optional Unicode display names, restricted three-component versions, language version `0.1`, nonempty source roots, entry and export module names, and local dependency paths. The public types make invalid decoded states unconstructible outside the crate and deliberately expose no discovery, graph, resolver, hashing, lockfile, or CLI placeholder API.

Seven registered bilingual `L-PROJECT-*` diagnostics distinguish byte-boundary, TOML-structure, version, package, source-layout, export, and dependency failures. Every manifest-local failure retains its original UTF-8 byte span, including CRLF, Chinese text, combining characters, and emoji prefixes. Diagnostic Facts use stable reason tokens and bounded user-text summaries.

## Normative clauses covered

- Accepted [`RFC-0002`](../RFC-0002.md) §1: exact `ling.toml` decoding boundary, UTF-8 without BOM/NUL, 1,048,576-byte limit, TOML 1.0, required `manifest-version = 1`, duplicate/unknown-field rejection, and LF/CRLF equivalence.
- RFC-0002 §2: the complete version-1 package, source, exports, and dependencies shape with no environment-derived defaults.
- RFC-0002 §3: package/display/module/version/path validation plus the 32-root, 1,024-dependency, and 4,096-export limits.
- RFC-0002 §7: structured bilingual diagnostics, original byte spans, bounded decoding/validation, and no partial project-graph publication.
- Accepted [`DEC-0002`](../decisions/0002-source-position-units.md): original UTF-8 byte offsets remain the diagnostic position authority.
- Accepted [`DEC-0007`](../decisions/0007-module-and-file-boundaries.md): module-name segments reuse the compiler's pinned NFC and Unicode XID validation rather than inventing a project-only name rule.
- [`03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) PRJ-1101: valid/invalid manifests, Chinese display metadata, traversal rejection, duplicate fields, and CRLF evidence.

## Implementation

- [`ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs) owns the versioned manifest model, private Serde input model, validation rules, stable error classification, span conversion, and deterministic collection construction.
- `PackageName`, `DisplayName`, `PackageVersion`, `LanguageVersion`, `QualifiedModuleName`, and `LogicalPath` have private representation and read-only accessors. `Manifest` exposes sorted slices/maps rather than Rust insertion or hash-map order.
- The workspace uses the current `toml` crate, whose decoder recognizes TOML 1.1. A bounded lexical compatibility guard rejects the TOML 1.1 additions that could otherwise enter this string/table schema (`\e`, `\xHH`, multiline inline tables, and trailing inline-table commas); optional datetime seconds cannot enter any accepted manifest field type. This preserves RFC-0002's TOML 1.0 accepted-input set without adding a second TOML dependency.
- Display-name boundary whitespace uses a generated Unicode 17.0.0 `White_Space` table. Bidi controls and Default Ignorables reuse the same pinned generated-property database. No host `char` Unicode-version behavior defines the protocol.
- Logical paths reject empty/absolute/drive/URI/backslash/NUL/dot/parent/empty-segment and non-NFC forms. Source roots are checked for duplicate or ancestor overlap before sorting.
- TOML decoding preserves spans on scalar values, array elements, and dependency keys. Duplicate-field spans expand deterministically to the offending source line; unknown-field spans select the original key token.
- User-controlled Fact values are deterministically summarized after a bounded UTF-8 prefix so a valid-size but invalid manifest cannot force proportional public diagnostic output.
- [`manifest_bytes.rs`](../../fuzz/fuzz_targets/manifest_bytes.rs) sends arbitrary bytes through both the reader and Diagnostic JSON renderer; two seed corpus inputs cover minimal ASCII and multi-script manifests.
- The diagnostic compatibility checker now permits several monotonic codes to be introduced together in a previously unused domain when `0001` is present. A regression test preserves the original no-backfill and first-code rules.

The implementation follows KISS by keeping PRJ-1101 byte/model validation in one crate, SRP by leaving filesystem and graph operations to later tasks, DRY by reusing `ling-unicode` and `ling-diagnostics`, and YAGNI by omitting every future resolver/writer/CLI surface.

## Specification gaps or conflicts

- No unresolved semantic gap blocks PRJ-1101. Accepted RFC-0002 is the direct public-protocol authority and explicitly authorizes the `PROJECT` diagnostic domain.
- The lower-authority plan asks for Profile and target to be explicit inputs. RFC-0002 excludes Profile/target defaults and fields from manifest version 1; PRJ-1101 therefore does not add speculative fields or implicit environment inputs. Later build orchestration must supply those inputs separately.
- RFC-0002 requires TOML 1.0 while the already locked workspace dependency implements TOML 1.1. The compatibility guard described above closes that implementation mismatch and has direct rejection tests. No protocol version was silently broadened.
- RFC-0002's symlink-aware containment, source discovery, module existence, dependency recursion, package/content identity, and lock atomicity require filesystem or graph context. They are intentionally deferred to PRJ-1102 through PRJ-1105 rather than being guessed inside the isolated reader.
- The execution package still contains stale `zero` command references in later tasks. PRJ-1101 introduces no CLI surface, and no stale command spelling enters code, fixtures, protocols, or documentation claims.

## Tests and verification

Executed locally on 2026-08-21 against implementation commit `80f46bcf3d175eeb6402bf6267085cb905a5dbcf` and the completion metadata derived from it:

- `cargo test --locked -p ling-project` — 6 unit tests, 6 integration tests, and documentation tests pass.
- Eleven versioned fixture directories cover minimal and Unicode-positive cases plus duplicate fields, unknown fields, invalid package/dependency/module/path forms, non-NFC display text, overlapping roots, and unsupported language versions.
- Programmatic acceptance tests cover comments and field reordering, LF/CRLF model equality, exact BOM/NUL/invalid-UTF-8/oversize spans, Chinese/bidi/emoji-prefix byte offsets, collection/path/file-size boundaries, bounded diagnostic Facts, and deterministic deletion/invalid-byte mutations without panic.
- TOML 1.1-only escape, multiline-inline-table, and trailing-comma cases are rejected as `L-PROJECT-0002`.
- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline` builds all four fuzz targets, including `manifest_bytes`, on the Windows host without claiming an unavailable sanitizer run.
- `cargo xtask governance check-error-codes` reports 62 active and 1 retired code across 14 domains, with 62 Rust constants; the generated compatibility lock is current.
- `cargo xtask governance check-protocols` reports 18 records: 10 current public, 1 internal, and 7 Future; `PROTO-PACKAGE-MANIFEST` is Public/Experimental at `ling.manifest/1`.
- `cargo xtask governance check-all`, Schema validation/compatibility/corrupt-input gates, traceability, support, CI-contract, Seed reproduction, and status verification pass.
- `cargo test --workspace --all-features --locked --offline`, full Clippy with warnings denied, Rust 1.85 workspace check, documentation build, release build, formatting, Unicode regeneration idempotence, execution-plan checksums, Markdown links, and `git diff --check` pass.

No remote CI result, filesystem project load, source discovery, dependency resolution, lockfile operation, or CLI behavior is claimed.

## Compatibility, determinism, and Unicode impact

- **Diagnostics and spans:** adds `L-PROJECT-0001` through `L-PROJECT-0007` with registered bilingual meanings and typed Facts. Existing codes, severities, and payload fields are unchanged. Manifest-local spans are original UTF-8 bytes and include CR bytes.
- **Schemas and protocols:** promotes only `PROTO-PACKAGE-MANIFEST` from Future to Public/Experimental at `ling.manifest/1`. It is TOML and has no JSON Schema. Diagnostic JSON remains `ling.diagnostic/0.1`; its container schema is unchanged.
- **CLI and source compatibility:** existing file-oriented Seed commands and `.ling` parsing are unchanged because `ling-cli` does not depend on `ling-project` and no ambient project discovery exists.
- **Semantic IDs and canonical bytes:** unchanged. Manifest data does not enter current DefinitionId, BodyId, ProgramId, Semantic Graph, Audit, or execution. Package content/graph identities and canonical `ling.lock/1` remain unimplemented.
- **Determinism:** roots and exports are unique and sorted; dependencies use `BTreeMap`; field/comment/newline spelling has no model effect; errors have stable code/reason/span selection; no host path, current directory, directory enumeration, map seed, or debug output enters the model.
- **Unicode:** remains pinned to 17.0.0. One generated `White_Space` range table is added from the already pinned/checksummed `PropList.txt`; input checksums and Tree-sitter identifier tables remain unchanged. NFC/XID/security behavior continues to come from `ling-unicode`.
- **Dependencies:** no new third-party version is introduced. The existing locked `toml`, `serde`, and `unicode-normalization` dependencies gain a documented production use in `ling-project`; the fuzz lock gains only the corresponding already-reviewed transitive graph.

## Intentionally deferred work

- PRJ-1102: symlink-aware source-root resolution, deterministic `.ling` discovery, path/module mapping, exact-case portability checks, entry/export existence, duplicate modules, and `ModuleGraph`.
- PRJ-1103: cross-package import selection and exported-module visibility in resolution/HIR/Semantic Graph.
- PRJ-1104: recursive local dependency graph, content/package identities, cycles/collisions, offline guarantees, and hash vectors.
- PRJ-1105: canonical `ling.lock/1` reader/writer, corruption corpus, `--locked`, and atomic replacement.
- PRJ-1106/1107: complete multi-project fixtures, project service/API, explicit CLI manifest selection, and unchanged file-mode migration evidence.
- PRJ-1108: graph/path/lock property tests and long-running sanitizer-backed fuzz campaigns. PRJ-1101 supplies only the decoder target and deterministic smoke corpus.
- Manifest writer/formatter, registry/Git/network dependencies, version solving, workspaces, build scripts, package installation, and publication remain outside manifest version 1.
