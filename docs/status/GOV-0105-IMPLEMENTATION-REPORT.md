# GOV-0105 Implementation Report / 实施报告

> Status: **Done; committed locally**
> Verification date: 2026-08-20
> Verified base: `main@feed35e9c00a1e4d9e4e97224ffe787bd5e29c49`
> Implementation commit: `7f4452b9c5c629b02f2cfc810529e797dff805b6`
> Task source: [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) `GOV-0105`

## Outcome

GOV-0105 now has one handwritten public allocation source, one generated compatibility lock, and one deterministic offline checker:

```text
cargo xtask governance check-error-codes
```

[`ERROR-CODES.md`](../ERROR-CODES.md) inventories 55 active codes and one retired code across 13 currently allocated domains. Every row records the root-cause phase, protocol stability, error/warning classification, Chinese and English titles/templates, typed required/optional Facts, structured Repair schema, and first version. The current Rust constant set is exactly the 55 active allocations; retired `L-IMPL-0001` remains historical and has no constant or emitter.

## Delivered artifacts

- [`ERROR-CODES.md`](../ERROR-CODES.md): the sole handwritten registry, split into active and retired allocations with machine-checked columns and lifecycle/consumer rules.
- [`error-code-lock.toml`](../governance/error-code-lock.toml): generated SHA-256 root-cause fingerprints, typed Fact contracts, retired state, and per-domain high-water marks; it allocates nothing by itself.
- `tools/xtask/src/error_codes.rs`: parser, registry/constant/source/fixture parity checker, deterministic lock renderer, compatibility-evolution checks, and negative fixtures.
- `crates/ling-diagnostics/src/lib.rs`: focused JSON tests for required protocol fields, ordered Fact/Repair maps, structured `changes_semantics`, and original UTF-8 byte offsets without freezing localized punctuation.
- `.github/workflows/ci.yml`, `.github/pull_request_template.md`, root `AGENTS.md`, bilingual `README.md`, and protocol/authority inventories: contributor and CI integration.
- `tools/xtask/Cargo.toml`, root `Cargo.lock`, and [`DEPENDENCIES.md`](../DEPENDENCIES.md): maintainer-only use of the already locked `sha2` dependency for compatibility fingerprints; no dependency version changed and `sha2` is not linked into `ling`.

## Authority and clauses covered

- Accepted DEC-0001: `L-<DOMAIN>-<NUMBER>` root-cause allocation, independent monotonic domain numbering, non-reuse, retained deprecated records, stable code meaning and Fact types, and structured Repair `changes_semantics`.
- Accepted DEC-0002: public diagnostic spans remain offsets into original UTF-8 bytes; the new test covers a three-byte Chinese scalar.
- Accepted DEC-0013: runtime, internal, and snapshot failure classes retain their existing code/severity/exit boundaries; no CLI exit behavior changed.
- Accepted DEC-0015 and DEC-0016: Audit diagnostics and REPL-added optional `committed` Facts are inventoried without changing Audit or REPL schemas.
- `GOV-0105` and first-sprint Task D: a single registry, code/phase/stability/bilingual title/payload/first-version inventory, generated or validated Rust parity, duplicate/translation/unregistered-code CI rejection, deterministic ordering/JSON evidence, and UTF-8 span preservation.

This milestone adds governance and tests only. It does not create a new Ling diagnostic, change any emitted code, message, severity, Fact value, Repair, span, CLI command, language semantic, or executable path.

## Specification gaps or conflicts

- The lower-authority execution plan proposes `L0000/P0000/...` ranges. Accepted DEC-0001 and root repository governance require `L-<DOMAIN>-<NUMBER>` and the existing [`ERROR-CODES.md`](../ERROR-CODES.md), so no code was renumbered and no second registry was created.
- The plan uses the future-facing name `FixPlan`; accepted DEC-0001 and `ling.diagnostic/0.1` currently expose structured `Repair`. The checker therefore requires `kind:string, changes_semantics:boolean` for any non-empty Repair schema and does not invent a second public wire type.
- `ENTRY`, `AUDIT`, and `SNAPSHOT` were already published allocations outside DEC-0001's initial domain list. This task preserves those identities and inventories current accepted behavior; it does not retroactively redefine or merge their root causes.
- No LSP implementation exists. The registry records that a future adapter must copy the same `DiagnosticCode` string into LSP `Diagnostic.code`, but adds no placeholder crate, command, schema, or capability claim.
- Some codes have optional Facts because not every existing emitter path supplies the enrichment (for example REPL-only `committed` data). The registry describes current wire behavior instead of silently making fields mandatory.

No semantic option was selected and no new specification gap was required; the plan conflicts are resolved mechanically by the documented authority order.

## Tests and verification

Executed locally on 2026-08-20 against the implementation commit:

- `cargo xtask governance check-error-codes` — passed: 55 active, one retired, 13 domains, 55 canonical Rust constants.
- `cargo xtask governance check-authority` — passed: 36 documents, 16 Accepted.
- `cargo xtask governance check-gaps` — passed: 25 Open gaps, six gates.
- `cargo xtask governance check-lifecycle` — passed: 17 records, 16 Accepted, 17 legacy-format migrations.
- `cargo xtask governance check-protocols` — passed: 18 records; nine public, one Internal, eight Future.
- `cargo test --package xtask --package ling-diagnostics --locked --offline` — 56 xtask and four diagnostic tests passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 196 tests passed.
- Local Markdown path check across eight milestone Markdown documents — 141 targets resolved, zero missing.

Negative fixtures reject duplicate allocations, missing Chinese/English translations, mismatched bilingual template parameters, invalid phase/domain pairs, unstructured Repairs, unregistered public codes, retired-code use, changed root-cause fingerprints, number backfilling, and new required Facts. Positive fixtures cover deterministic lock ordering, compatible optional-Fact additions, live registry/constant parity, structured Repair JSON, deterministic BTreeMap serialization, and original UTF-8 byte offsets.

The CI workflow was edited and inspected locally. Remote GitHub Actions evidence is not claimed until the committed milestone is pushed and GitHub reports it.

## Compatibility impact

- Diagnostics: no emitted code, meaning, severity, bilingual message, Fact value/type, Repair, or span changed. Registry metadata now explicitly freezes 55 active identities and retains one retired identity.
- Schema: adds internal governance schema `ling.diagnostic-registry/0.1` and a generated lock schema version `1`; public `ling.diagnostic/0.1`, Semantic Graph, Audit, REPL, Semantic ID, and canonical-byte schemas are unchanged.
- CLI/LSP: CLI human/JSON and REPL continue serializing one `DiagnosticCode`; no LSP implementation or protocol claim was added.
- Dependencies: the xtask maintenance binary now directly uses the already locked `sha2` package; no package version or normal/offline build graph for the `ling` executable changed.
- Semantic IDs and language behavior: unchanged.

## Determinism and Unicode

Registry errors, code sets, Facts, domains, source paths, lock entries, and high-water marks use ordered collections. The generated lock sorts domains and codes lexically, fingerprints normalized immutable fields with length-prefixed SHA-256 input, emits repository-relative forward-slash paths in diagnostics, and normalizes only CRLF to LF for generated-file comparison. The checker recursively sorts scanned paths and sorts/deduplicates validation errors.

Localized message bytes remain deliberately outside the immutable fingerprint, while root-cause titles, phase, severity, first version, existing Fact requiredness/types, retired state, and domain high-water marks remain checked. No canonical language bytes, HashMap iteration behavior, source normalization, Unicode table, XID/NFC/security behavior, or pinned Unicode 17.0.0 version changed.

## Intentionally deferred

- Schema lifecycle, N-1 readers, golden corpora, and migration fixtures: `GOV-0106`.
- Cross-requirement traceability matrix: `GOV-0107`.
- LSP adapter and byte-span/position conversion: later accepted architecture/LSP tasks; this milestone only freezes code reuse.
- Actual Repair candidates: none are currently emitted; any future candidate must use the checked structured schema and behavior-specific tests.
- New diagnostic domains/codes and public schema changes: require their governing accepted specification and monotonic allocation workflow.
- Remote CI evidence: available only after push and GitHub Actions completion.
