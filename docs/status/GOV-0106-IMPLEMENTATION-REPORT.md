# GOV-0106 schema lifecycle and golden corpus implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `35ba0da126be6b2df8ee3fe2aa9fd3ca27ebbdec`
> Verified baseline: `main@262b7356d124cc2c9d1e87594b22b1a21080f843`

## Outcome

GOV-0106 establishes a versioned, executable compatibility corpus for the three implemented public JSON protocols already recorded by GOV-0104: Diagnostic JSON `ling.diagnostic/0.1`, Semantic Graph JSON `ling.semantic/0.1`, and REPL event JSON `ling.repl/0.1`.

The work is intentionally split into three auditable slices:

- Slice A, commit `475bb87212826a75e92ed30598f82b7cb5bd1a18`, defines the Draft engineering policy in [`SCHEMA-LIFECYCLE.md`](../governance/SCHEMA-LIFECYCLE.md).
- Slice B, commit `cad38897c8f8310b8a6436797eda2a1b081971e1`, adds [`schemas/registry.toml`](../../schemas/registry.toml), three Draft 2020-12 schemas, valid/invalid expectation fixtures, a Semantic Graph canonical byte golden, and direct writer/reader tests.
- Slice C, commit `b1ffecac970cae41ac90bd65043a1a0d8f184fb3`, adds the offline validator and the three required `cargo xtask schema` gates, then integrates them with authority, protocol, traceability, CI, and the bilingual README.

Commit `35ba0da126be6b2df8ee3fe2aa9fd3ca27ebbdec` additionally replaces one pre-existing unstable let-chain in the lifecycle checker with equivalent Rust 1.85-compatible control flow after the declared MSRV gate exposed it. No lifecycle behavior changed.

The corpus describes existing bytes and reader behavior; it does not introduce a new Ling command, evaluator input path, language semantic, protocol marker, or stability promotion. Audit Source remains an explicit canonical non-JSON boundary.

## Normative clauses and decisions covered

- [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) GOV-0106: version meaning, current-only writers, reader ranges, unknown/missing-field policy, canonical encoding, hash scheme IDs, versioned manifests/fixtures, compatibility validation, and deterministic corrupt-input checks.
- Accepted DEC-0001 and DEC-0002: Diagnostic JSON continues to carry registered bilingual diagnostics and original UTF-8 byte spans.
- Accepted DEC-0012: ordinary JSON member order is not a Semantic ID input; Semantic IDs retain domain-separated canonical binary encodings and explicit hash-scheme IDs.
- Accepted DEC-0016: REPL JSON remains a line-oriented writer protocol with no fabricated standalone reader.
- [`protocol-inventory.toml`](../governance/protocol-inventory.toml) remains authoritative for public schema, stability, reader/writer, canonicality, and non-JSON boundary claims.
- The Draft schema lifecycle policy and Active schema registry are engineering controls. Neither is treated as Accepted language semantics or a Stable compatibility basis.

## Implementation

- [`schemas/registry.toml`](../../schemas/registry.toml) strictly records each schema ID, protocol, marker, version kind, stability, writer/reader range, previous-version state, field policies, reader defaults, canonical encoding, hash schemes, paths, adapters, and implementation evidence.
- Each package follows `schemas/<name>/<version>/{schema.json,valid,invalid,canonical}`. Invalid fixtures have strict TOML sidecars classifying `InvalidJson`, `SchemaViolation`, or `ReaderViolation`.
- [`schema.rs`](../../tools/xtask/src/schema.rs) implements an audited offline subset of JSON Schema Draft 2020-12. Unreviewed keywords, non-local references, unsafe paths, unregistered files, protocol drift, unsupported adapters, and false compatibility claims fail closed.
- Valid Semantic Graph fixtures invoke the real `ling_semantic::read_json` isolated reader. Diagnostic and REPL packages do not pretend that public readers exist.
- Canonical Semantic Graph bytes must be BOM-free UTF-8 compact JSON, contain no CR or insignificant whitespace, end in exactly one LF, match the corresponding writer fixture byte-for-byte, satisfy the schema, and pass the real reader.
- The `.gitattributes` rule `schemas/**/canonical/*.bin -text` prevents checkout-time line-ending conversion on every host.
- Corruption validation deterministically exercises truncation, trailing data, wrong marker, missing required fields, unknown core fields, accepted `x-*` extensions, the registered Semantic Graph reader default, and missing canonical LF.
- Protocol records expose category, public-schema, and canonical flags to other `xtask` validators; the support-matrix comparison remains unchanged.
- The schema validator depends only on the in-workspace `ling-semantic` crate. No external package was added or updated.
- CI runs all three schema gates after locked dependencies are fetched; the bilingual README documents the same commands and first-version boundary.

The implementation follows KISS and YAGNI by supporting only the schema keywords and reader adapter required by current Seed protocols. It follows DRY by using one registry for all commands and one recursive instance validator for valid, invalid, canonical, and corrupted inputs. Protocol and reader behavior are consumed through existing abstractions rather than duplicated.

## Specification gaps or conflicts

- There is no previous public version for any of the three schemas. The registry therefore says `NoPreviousVersion`; `cargo xtask schema compatibility --from N-1 --to N` reports zero verified edges and must not be presented as N-1 reader support.
- Diagnostic JSON and REPL JSON have writers but no public standalone readers. Their corpus verifies emitted shape and exact implementation writer output without inventing a decoder API.
- Semantic Graph JSON has an isolated exact-version reader. It returns data only and cannot create checked Typed Core or evaluator input.
- Audit Source is an Accepted canonical text grammar rather than JSON. The registry names it as `CanonicalText` evidence instead of fabricating a JSON schema.
- Package, lockfile, build metadata, bytecode, replay, ABI, evidence, and Semantic Transaction protocols remain Future or separately scoped. No placeholder schema package was created.
- The JSON Schema checker is deliberately a reviewed subset, not a claim of general Draft 2020-12 implementation. A future keyword requires an explicit code/test review.
- The schema lifecycle policy remains Draft because no Accepted RFC establishes a general cross-version policy. Existing Accepted per-protocol decisions continue to outrank it.
- The declared Rust 1.85 check initially failed on a pre-existing lifecycle-checker let-chain. The equivalent nested form was committed and the full MSRV check then passed; no specification choice was involved.

No unresolved specification conflict was decided through code or snapshots, and no new gap record was necessary for this bounded first-version corpus.

## Tests and verification

Executed locally on 2026-08-20 against the final verified implementation commit:

- `cargo xtask schema validate-all` — passed: three schemas, four valid fixtures, six classified invalid fixtures, and one canonical byte fixture.
- `cargo xtask schema compatibility --from N-1 --to N` — passed: zero verified N-1 edges and three explicit `NoPreviousVersion` records.
- `cargo xtask schema corrupt-inputs` — passed: 23 deterministic mutations produced their declared acceptance or rejection outcomes.
- `cargo xtask governance check-authority` — passed: 40 documents, 16 Accepted; `SCHEMA-REGISTRY` is Active and not a Stable basis.
- `cargo xtask governance check-protocols` — passed: 18 records; nine public, one Internal, and eight Future; current public stability remains three Experimental, six Preview, zero Stable.
- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 47 evidence records, and seven deferred differential paths.
- Support-matrix and implementation-status drift checks passed without changing feature/Profile/target claims.
- `cargo test --package xtask --locked --offline` — 85 tests passed, including seven schema registry, dialect, compatibility, corruption, canonicality, and repository-current tests.
- Focused writer/reader checks passed: one Diagnostic corpus test, one Semantic reader corpus test, and ten CLI conformance tests.
- `cargo test --workspace --all-features --locked --offline` — 228 tests passed, including doc-test harnesses.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo +1.85 check --workspace --all-features --locked --offline` — passed with the declared MSRV.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Execution-plan `SHA256SUMS.txt` — all 27 entries passed after the backlog status update.
- Active local Markdown target audit — 577 targets resolved, zero missing; the frozen execution-plan baseline was excluded.
- `git diff --check` — passed.

Negative coverage rejects duplicate or drifting registry records, unreviewed schema keywords, unresolved/non-local references, malformed JSON, schema-invalid data, real-reader-invalid data, missing sidecars, orphan sidecars, false previous-version claims, unsafe or missing paths, unexpected fixture files, writer-evidence drift, noncanonical bytes, and corrupt inputs with incorrect outcomes.

The CI workflow was edited and inspected locally. No remote GitHub Actions result is claimed before an authorized push and completed remote run.

## Compatibility impact

- Public schemas: versioned schema artifacts now formally describe the existing `ling.diagnostic/0.1`, `ling.semantic/0.1`, and `ling.repl/0.1` bytes. No marker, field, reader behavior, canonical output, or stability level changed.
- Diagnostics: no Ling diagnostic code, meaning, severity, bilingual message, Fact, Repair, or UTF-8 span behavior changed. `GOV-SCHEMA-*` messages are maintainer-tool failures, not public compiler diagnostics.
- CLI: no `ling` command, option, output, or exit code changed. Three maintainer commands were added under the existing locked/offline `cargo xtask` alias.
- Semantic IDs: hash algorithms, prefixes, domain encodings, normalization, and identity inputs are unchanged. The registry records existing IDs and checks that implementation evidence contains each one.
- Schema registry: schema version 1 is new. It explicitly has no previous registry or public-protocol migration edge.
- Dependencies: `xtask` gains one local path dependency on `ling-semantic`; no registry dependency or version changed.

## Determinism and Unicode

Registry comparisons use ordered maps/sets; errors and discovered paths are sorted; recursive fixture traversal is deterministic; corruptions are fixed transformations with no randomness; generated governance and traceability reports remain byte-drift checked. Canonical fixtures are protected from Git text conversion and compared as raw bytes. No timestamp, absolute host path, allocation identity, Rust debug representation, arena index, or hash-map iteration order enters a public artifact.

Unicode remains pinned to 17.0.0. No generated table, normalization, XID/security behavior, source decoding, or original UTF-8 byte-span mapping changed. The schemas describe existing JSON strings and byte offsets without redefining Unicode semantics.

## Intentionally deferred

- Any real N-1 reader, migration adapter, compatibility fixture, or previous-version package.
- Promotion of Experimental or Preview protocols to Stable.
- Public Diagnostic or REPL standalone readers.
- General-purpose JSON Schema support beyond the audited Seed keyword subset.
- Schemas for package/lock/build metadata, bytecode, replay, ABI, evidence, transactions, Profiles, Native targets, or devices.
- Any JSON-to-Typed-Core or JSON-to-evaluator path.
- Release publication, tags, signed artifacts, hosted documentation deployment, and remote CI evidence.
