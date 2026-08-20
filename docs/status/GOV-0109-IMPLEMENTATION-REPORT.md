# GOV-0109 machine-readable implementation status report

> Status: **Done**
> Completed: 2026-08-20
> Implementation commit: `695e40eb6310ba1dcb36580f5feb63b4301ab656`
> Verified baseline: `main@fbc2a9728e69994a771001bdbe3e03d86bb2c4ef`

## Outcome

GOV-0109 extends the existing [`implementation-status.toml`](implementation-status.toml) registry to schema version 2 instead of creating a competing state file. The registry now records task lifecycle evidence and seven public Seed feature states, including current state, stability, implemented/tested/documented flags, stabilization blockers, last verified commit, supported Profiles, supported targets, and evidence paths.

One validator generates and drift-checks three consumers from that source: the bilingual [`implementation-status.md`](implementation-status.md) documentation page, the [`release-status.md`](release-status.md) release-note input fragment, and an internal [`feature-state.governance.json`](../../tests/fixtures/status/feature-state.governance.json) CLI fixture. The fixture explicitly says `implemented: false` and `public_contract: false`; no public `ling` command or schema was added.

All seven Seed features are currently `Implemented` and `Experimental`, with implementation/test/documentation evidence. No feature claims a selectable Profile or supported Ling Native target, because neither capability exists.

## Normative clauses and decisions covered

- `docs/ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md` GOV-0109: machine-readable feature state, implemented/tested/documented flags, blockers, last verified commit, supported Profiles/targets, and generated documentation/CLI/release-note views.
- `docs/ROADMAP-1.0.md` §2.3: Experimental/Preview/Stable state must appear in user-visible documentation and machine-readable metadata instead of existing only in comments.
- `docs/ROADMAP-1.0.md` §§1 and G0.4: unimplemented Profile/target behavior must not be implied, and support claims remain bounded by the support matrix.
- `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md` STAB-6103 supplies the future lifecycle vocabulary `Experimental`, `Preview`, `Stable`, `Deprecated`, and `Removed`. GOV-0109 accepts that vocabulary in metadata but does not claim that G6 consumers or stabilization work exist.
- GOV-0102, GOV-0107, and GOV-0108 remain the canonical sources for blocker IDs, feature identities/evidence, and current Profile/target support. GOV-0109 cross-checks them instead of copying their semantic definitions.
- Root repository governance remains unchanged: only the v0.0.1 Seed subset is implemented; checked Typed Core remains the evaluation boundary; UTF-8 byte spans, bilingual stable diagnostics, deterministic behavior, and Unicode 17.0.0 remain intact.

## Implementation

- `docs/status/implementation-status.toml` schema v2 retains all existing `[[task]]` records and adds strict registry metadata plus seven `[[feature]]` records.
- Feature names, scope, stability evidence, and the historical release candidate commit are read from traceability. Current state/Profile support and real Native targets are read from the support matrix. Blocker IDs are resolved through the gap register.
- `cargo xtask status verify` validates registry versions, paths, task IDs/states/sizes/dependencies/cycles, Done-task commit evidence, feature parity, state/stability agreement, evidence claims, sorted unique relations, known blockers, supported Profile/target claims, and generated-file drift.
- `cargo xtask status render`, `render-release-notes`, and `render-cli-fixture` emit deterministic views without writing files implicitly.
- The generated status page presents both task and feature state. The generated release-status file is labeled as an input fragment, not a published announcement or compatibility promise.
- The generated JSON is an internal `ling.governance.feature-state-fixture/1` fixture associated with the future `ling support --format json` command already named by the support matrix.
- CI and the bilingual README now run and document the status verifier.

Validation is read-only, locked/offline, and repository-scoped. Registry text is never executed. Paths must be safe repository-relative paths; the one existing conformance wildcard form is expanded only as a single directory segment and must match a real artifact.

## Specification gaps or conflicts

- No Accepted RFC defines a public feature-status CLI schema. Consequently no `ling support`, `ling features`, or compatibility alias was exposed; the JSON remains an internal governance fixture.
- The G6 STAB-6103 plan names future build-manifest, LSP, package, and Zed consumers. Those systems do not exist in Seed and are not fabricated by this G0 task.
- The support matrix records all Profiles as unavailable and the only Native target as unsupported. Every feature therefore has empty `supported_profiles` and `supported_targets` arrays.
- Each feature's `last_verified_commit` is the published v0.0.1 candidate `652d19b9eaec2ab607edfe1a1e7ea742c861cf91`, obtained from traceability. Later governance-only local commits do not replace that historical multi-platform language evidence.
- All features retain `GAP-GOV-RFC-STATUS-001` as a stabilization blocker because RFC-0001 remains Draft. The Semantic Graph/Audit feature additionally names the registered Semantic Hash and protocol-lifecycle gaps.
- Generated release notes are explicitly a fragment. Release publication, signing, tagging, and remote evidence are outside this task.

No semantic option was selected, no specification was edited, and no new gap entry was necessary.

## Tests and verification

Executed locally on 2026-08-20 against the implementation commit:

- `cargo xtask status verify` — passed: eight tasks, all Done; seven features, all with explicit stabilization blockers.
- `cargo xtask support verify` — passed: seven features, three Profiles, three hosts, one Native-target record, six backends, one standard package, 18 protocols, and nine explicit unsupported records.
- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 44 evidence records, seven deferred differential paths.
- `cargo xtask governance check-authority` — passed: 38 documents, 16 Accepted; `TASK-STATUS` is registry schema v2 and not a Stable basis.
- All other governance registry checks passed: 25 Open gaps/six gates; 17 lifecycle records; 18 protocols; 55 active/one retired diagnostic codes.
- `cargo test --package xtask --locked --offline` — 78 tests passed, including six status validation/current-repository/determinism tests.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 218 tests passed, including all doc-test harnesses.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Active local Markdown target audit — 536 targets resolved; the immutable execution-plan baseline snapshot was excluded and retains two known historical broken targets.
- `git diff --check` — passed.

Negative tests reject Done tasks without full commit evidence, unknown stabilization blockers, unsupported target claims, omitted features, state drift from the support matrix, and public-looking/implemented CLI fixture claims. Determinism tests render all three consumers twice and compare exact bytes. Repository validation additionally rejects stale generated files, unsafe or missing evidence paths, task dependency cycles, unsorted/duplicate relations, traceability/support feature mismatch, and last-verified-commit drift.

The CI workflow was edited and inspected locally. No remote GitHub Actions result is claimed before an authorized push and completed remote run.

## Compatibility impact

- Diagnostics: no public Ling diagnostic code, meaning, message, severity, Fact, Repair, span, or localization behavior changed. `GOV-STATUS-*` strings are maintainer-tool validation errors.
- Schema: the internal task registry advances from schema 1 to 2 and gains feature schema 1. The generated JSON uses an explicitly internal, non-contract `ling.governance.*` schema. No public diagnostic, Semantic Graph, Audit, REPL, CLI, Semantic ID, canonical-byte, package, bytecode, ABI, or runtime schema changed.
- CLI: no `ling` command, option, output, or exit behavior changed. Four maintainer commands were added under the existing `cargo xtask` alias.
- Semantic IDs and language behavior: unchanged. Status metadata observes evidence and cannot affect parsing, checking, lowering, or evaluation.
- Dependencies: none added or updated; `Cargo.toml`, `Cargo.lock`, and the fuzz lockfile are unchanged.

## Determinism and Unicode

Tasks, features, blockers, Profiles, targets, errors, Markdown tables, release-note entries, and JSON arrays are ordered by stable identifiers. Generated-file comparison normalizes only CRLF to LF. No timestamp is generated at runtime; the reviewed registry date is the sole displayed date. No host path, allocation identity, hash-map order, Rust debug output, or environment-specific value enters an artifact.

The registry references Unicode `17.0.0` evidence through traceability/support validation but does not regenerate or modify Unicode tables. Source decoding, normalization, XID/security behavior, and original UTF-8 byte spans are unchanged.

## Intentionally deferred

- Public feature/status CLI commands and their versioned compatibility schemas.
- Build-manifest, LSP hover/completion, package metadata, and Zed compatibility-table consumers from G6 STAB-6103.
- Any promotion from Experimental to Preview/Stable, which requires accepted semantics, resolved blockers, and the applicable release evidence.
- Selectable Explore/Native/Critical Profiles and supported Native targets.
- Release publication, tags, signed artifacts, hosted documentation deployment, and remote CI evidence.
