# GOV-0108 Ling 1.0 support-matrix draft implementation report

> Status: **Done**
> Completed: 2026-08-20
> Implementation commit: `258e2d1e46c286a9e7e937b0bb65f3d19ed5e8d3`
> Verified baseline: `main@272583392ca70f03afbd5b899ac449afac697917`

## Outcome

GOV-0108 now has one machine-readable source at [`docs/governance/support-matrix.toml`](../governance/support-matrix.toml), a deterministic bilingual generated report, two deterministic internal JSON fixtures, an offline validator with negative tests, and a required CI drift gate.

The draft records seven current Seed feature states, three unavailable Profiles, three CI host platforms, one unsupported Native target boundary, six backend/device paths, one built-in standard package surface, all 18 protocol inventory records, and nine explicitly unsupported capability groups. It separates current evidence from candidate 1.0 scope and does not convert roadmap intent into an implementation claim.

This task does not implement `ling version --format json` or `ling support --format json`. Their checked-in `ling.governance.*` JSON files have `implemented: false`, are explicitly internal fixtures, and are not public compatibility contracts.

## Normative clauses and decisions covered

- `docs/ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md` GOV-0108: machine-readable feature/Profile stability, host and target tiers, backend/device tiers, standard-package stability, protocol versions, explicit unsupported scope, generated user documentation, and pre-CLI fixtures.
- `docs/ROADMAP-1.0.md` §§1, 2.3, and G0.4: unsupported capabilities must not be implied by placeholder APIs; Experimental/Preview/Stable are distinct; the draft must cover Profiles, hosts, targets, backends, standard packages, protocols, and deferred scope.
- `docs/SEMANTICS.md` §23 and `docs/LANGUAGE.md` §13 are treated as Draft descriptions of Explore, Native, and Critical. They are not used to claim that Profile selection or enforcement exists.
- Accepted DEC-0003 governs the actual `ling` CLI name and offline tooling baseline. Accepted DEC-0011 and DEC-0014 support the current built-in `Ling.Prelude` record. The protocol rows are copied from and continuously checked against the GOV-0104 protocol inventory.
- Root repository governance remains unchanged: only the v0.0.1 Seed implementation is current; checked Typed Core remains the evaluator boundary; UTF-8 byte spans, bilingual stable diagnostics, deterministic behavior, and Unicode 17.0.0 remain intact.

## Implementation

- [`support-matrix.toml`](../governance/support-matrix.toml) is the sole active support source. It records matrix metadata, tier policy, current and candidate claims, evidence paths, blockers from the gap registry, and explicit unsupported records.
- [`support-matrix.md`](../governance/support-matrix.md) is generated deterministically for human review. It prominently labels the matrix `Draft` and the compiler/language version `0.0.1-dev`.
- [`version.governance.json`](../../tests/fixtures/support/version.governance.json) and [`support.governance.json`](../../tests/fixtures/support/support.governance.json) are deterministic internal fixtures for later CLI integration. Their schema namespace and `implemented: false` marker prevent them from masquerading as current public commands.
- [`tools/xtask/src/support.rs`](../../tools/xtask/src/support.rs) parses the strict TOML schema and cross-checks compiler/language/Unicode versions, traceability features, authority records, protocol records, registered blockers, required Profile/host/backend categories, evidence paths, and generated-file drift.
- `protocols`, `traceability`, and `gaps` expose read-only typed record APIs so the support checker reuses their canonical parsers instead of duplicating registry parsing.
- `cargo xtask support verify` is required by CI. Three render commands produce the Markdown report and the two internal fixtures without writing files implicitly.
- The diagnostic-code scanner now requires a token boundary, preventing registered `GAP-*` identifiers in JSON from being misclassified as malformed public `L-*` diagnostic codes.

Validation uses ordered maps/sets and explicit sorting. It reads only repository-relative declared paths, does not execute registry content, does not access the network, and rejects unknown relations or unsupported records that overclaim implementation/tier status.

## Specification gaps or conflicts

- `docs/ROADMAP-1.0.md` G0.4 says the support-matrix draft must undergo RFC review before the broader G0 exit. No Accepted support-matrix RFC exists. Therefore this artifact remains `Draft`, is not a Stable implementation basis, and does not claim that the full G0.4 exit has passed.
- Explore, Native, and Critical are described in Draft specifications, but the Seed CLI has no Profile selector or enforcement pass. All three are recorded as unavailable and non-selectable; no allowed Effect, memory, or runtime claim is fabricated.
- Only the checked interpreter exists. VM, Native/AOT, FFI, Kernel CPU, GPU, accelerator, package distribution, concurrency/replay, Critical verification, and LSP/editor integration remain linked to existing open gap IDs.
- The execution-plan examples use the obsolete `zero` executable name. This implementation follows the repository's accepted `ling` name and creates no alias.
- The three CI host runners prove source CI coverage, not published binary support or minimum host/toolchain contracts. They are conservatively Tier 2, with no Tier 1 claim.

No semantic option was selected, no specification was edited, and no new gap entry was necessary.

## Tests and verification

Executed locally on 2026-08-20 against the implementation commit:

- `cargo xtask support verify` — passed: seven features, three Profiles, three hosts, one Native-target record, six backends, one standard package, 18 protocols, and nine explicit unsupported records.
- `cargo xtask governance check-authority` — passed: 38 documents, 16 Accepted; `SUPPORT-MATRIX` is Draft and not a Stable basis.
- `cargo xtask governance check-gaps` — passed: 25 Open gaps and six gates.
- `cargo xtask governance check-lifecycle` — passed: 17 records, 16 Accepted, 17 legacy-format records.
- `cargo xtask governance check-protocols` — passed: 18 records; nine public, one Internal, eight Future.
- `cargo xtask governance check-error-codes` — passed: 55 active, one retired, 13 domains, 55 Rust constants.
- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 44 evidence records, seven deferred differential paths.
- `cargo test --package xtask --locked --offline` — 72 tests passed, including seven support-matrix positive/current/negative/determinism tests.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 212 tests passed, including all doc-test harnesses.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Active local Markdown target audit — 527 targets resolved; the immutable execution-plan baseline snapshot was excluded and retains two known historical broken targets.
- `git diff --check` — passed.

Negative tests reject an unavailable Profile that claims runtime support, an unimplemented backend assigned a supported tier, protocol omission/version drift, unknown gap blockers, and public-looking fixture schemas. Determinism tests render both JSON fixtures twice and compare exact bytes. Repository validation also rejects stale generated Markdown/JSON, mismatched compiler/language/Unicode versions, unknown evidence paths, missing required categories, and disagreement with the canonical traceability/protocol/gap registries.

The CI workflow was edited and inspected locally. No remote GitHub Actions result is claimed before an authorized push and completed remote run.

## Compatibility impact

- Diagnostics: no public Ling code, meaning, message, severity, Fact, Repair, span, or localization behavior changed. `GOV-SUPPORT-*` strings are maintainer-tool validation errors. The scanner correction only prevents false matches inside longer `GAP-*` tokens.
- Schema: adds internal governance schema version `1` for the support TOML and two explicitly non-contract `ling.governance.*` fixtures. No `ling.diagnostic/0.1`, `ling.semantic/0.1`, `ling.audit/0.1`, `ling.repl/0.1`, Semantic ID, canonical-byte, package, bytecode, ABI, or runtime schema changed.
- CLI: no `ling` command, argument, output, or exit behavior changed. Four maintainer commands were added under the existing `cargo xtask` alias.
- Semantic IDs and language behavior: unchanged. Support metadata observes existing artifacts and cannot affect parsing, checking, lowering, or evaluation.
- Dependencies: no package version was added or updated. Existing workspace `serde_json` was promoted to a direct `xtask` dependency; `Cargo.lock` changed only to record that dependency edge.

## Determinism and Unicode

All rendered arrays and report sections are sorted by stable IDs. Registry cross-checks use ordered collections, generated-file comparison normalizes only CRLF to LF, and no timestamp, host path, allocation identity, hash-map order, Rust debug output, or environment-specific value enters the artifacts.

The support matrix records Unicode `17.0.0` and verifies it against `ling-unicode`; it does not regenerate or modify Unicode tables. Source decoding, normalization, XID/security behavior, and original UTF-8 byte spans are unchanged.

## Intentionally deferred

- RFC review and any promotion of the support matrix beyond Draft.
- Public `ling version --format json` and `ling support --format json` commands and schemas; GOV-0109/G1 CLI work must define their lifecycle before exposure.
- Machine-readable per-feature implemented/tested/documented completion percentages and dependency state: GOV-0109.
- Schema lifecycle, N-1 readers, migrations, corruption tests, and golden corpus: GOV-0106.
- Tier 1 platform promises, published binary artifacts, minimum host/runtime contracts, and hardware/device evidence.
- All VM, package, Native/FFI, concurrency/replay, heterogeneous, Critical, and editor capabilities named by the explicit unsupported rows.
- Remote CI evidence, which is available only after an authorized push and GitHub Actions completion.
