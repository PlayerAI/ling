# GOV-0107 unified traceability implementation report

> Status: **Done**
> Completed: 2026-08-20
> Implementation commit: `dcfc75a78333bc7ed8e020985d5419b1c21bc789`
> Verified baseline: `main@5f03405f066e68004d2451bfa21d4580827d71fd`

## Outcome

GOV-0107 now has one machine-readable source at [`docs/traceability/registry.toml`](../traceability/registry.toml), a documented schema template, a deterministic bilingual `v0.0.1` evidence index, stable IDs embedded in every conformance fixture, an offline `xtask` validator, negative validator tests, and a required CI gate.

The registry covers seven public Seed feature chains and all 32 directories under `tests/conformance/`. Together with 12 named Rust tests, the generated release index exposes 44 evidence records. Each public feature maps Requirement/Spec → indexed authority → checked Core/schema → implementation symbol → positive and negative evidence → explicit differential state → release artifact.

This is governance and test metadata only. It does not add or alter a Ling language behavior, compiler stage, evaluator path, command, diagnostic, or public protocol.

## Normative clauses and decisions covered

- `docs/ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md` GOV-0107: stable fixture IDs, stable feature IDs, checked links, human-reviewable semantic coverage, and generated release evidence.
- RFC-0001 §14.5: differential testing remains conditional on a second evaluator or Core evaluator.
- RFC-0001 §§18.1–18.7 and `IMPLEMENTATION.md` §§5/10: the existing Seed release requirements are represented as seven feature chains.
- Accepted DEC-0001 through DEC-0016 are referenced where they govern current Seed evidence; their status and paths are resolved through the authority index rather than copied into the traceability registry.
- Root repository governance: checked Typed Core remains the evaluator boundary; original UTF-8 byte spans, bilingual stable diagnostics, deterministic output, Unicode 17.0.0, and locked/offline builds remain unchanged.

RFC-0001 and `SEMANTICS.md` remain recorded as Draft by the lifecycle/authority registries. Their clauses are displayed for traceability but are not relabeled Accepted.

## Implementation

- [`docs/traceability/registry.toml`](../traceability/registry.toml) is the sole active traceability source. It assigns immutable `FTR-*` IDs and records scope, conservative stability, requirement headings, authority IDs, concrete Core/implementation symbols, release artifacts, and differential disposition.
- [`docs/traceability/TEMPLATE.md`](../traceability/TEMPLATE.md) defines the record contract and contributor workflow.
- [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md) is generated deterministically from the registry and fixture metadata. The prior [`SEED-TRACEABILITY.md`](../SEED-TRACEABILITY.md) remains historical release evidence, not a second active registry.
- Every `tests/conformance/*/expect.toml` now owns an immutable `TEST-CONF-*` ID, explicit `Positive`/`Negative` polarity, and one or more `FTR-*` links. The CLI conformance harness rejects missing, malformed, duplicate, or semantically inconsistent metadata.
- `cargo xtask traceability verify --release v0.0.1` discovers every conformance directory, validates strict fixture metadata, cross-checks authority IDs, verifies Markdown clauses and source symbols, requires both evidence polarities for public features, enforces differential-state honesty, and rejects generated-report drift.
- The authority index now records the machine registry and points `SEED-TRACEABILITY` at the generated unified report.
- CI runs the traceability verifier after all governance registry checks. The repository alias keeps `cargo xtask` locked and offline.

The validator uses ordered maps/sets and sorted directory traversal. It does not scan arbitrary external paths, run fixture-supplied commands, access the network, or treat evidence metadata as executable input.

## Specification gaps or conflicts

- RFC-0001 is still Draft even though `v0.0.1` is published. The generated report labels it Draft and relies on the existing authority index; it does not resolve `GAP-GOV-RFC-STATUS-001` or infer acceptance from release history.
- RFC-0001 §14.5 says cross-engine differential tests begin only after a second evaluator/Core evaluator exists. Ling currently has one reference interpreter and no VM. All seven rows therefore say `Deferred`, link the G1 VM plan, and do not mislabel independent-process/canonical round-trips as engine differential coverage.
- The execution plan uses future `zero` examples elsewhere. This implementation uses the repository's actual `ling`/`cargo xtask` command surface and does not add a compatibility alias.
- The hand-written `docs/SEED-TRACEABILITY.md` predates GOV-0107. It is preserved as historical release evidence while the machine registry is the only active traceability source.

No semantic option was selected, no specification was edited, and no new spec-gap entry was necessary.

## Tests and verification

Executed locally on 2026-08-20 against the implementation commit:

- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 44 total evidence records, seven explicitly deferred differential paths.
- `cargo xtask governance check-authority` — passed: 37 documents, 16 Accepted.
- `cargo xtask governance check-gaps` — passed: 25 Open gaps and six gates.
- `cargo xtask governance check-lifecycle` — passed: 17 records, 16 Accepted, 17 legacy-format records.
- `cargo xtask governance check-protocols` — passed: 18 records; nine public, one Internal, eight Future.
- `cargo xtask governance check-error-codes` — passed: 55 active, one retired, 13 domains, 55 Rust constants.
- `cargo test --package xtask --locked --offline` — 65 tests passed, including nine traceability positive/negative/current-repository tests.
- `cargo test --package ling-cli --test conformance --locked --offline` — nine integration tests passed; the generic runner executed all 32 fixture manifests.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 205 tests passed, with all doc-test harnesses passing.
- `implementation-status.toml` parse — passed: seven task records; GOV-0107 is the final `Done` record with the exact implementation hash.
- Execution-plan SHA-256 verification — passed: all 27 manifest entries match.
- Local Markdown target validation across the changed reports and README — 314 targets resolved, zero missing.
- `git diff --check` — passed after removing a generated Markdown hard-break whitespace warning at the generator source.

Negative validator tests reject duplicate feature IDs, missing headings/symbols, unknown or non-Accepted stable authority bases, public features without both polarities, unsupported Covered differential claims, and malformed fixture IDs/polarity. Repository verification additionally rejects missing fixture files, duplicate evidence/test IDs, unknown feature links, unsafe repository paths, stale generated reports, and unregistered releases.

The CI workflow was edited and inspected locally. No remote GitHub Actions result is claimed before an authorized push and completed remote run.

## Compatibility impact

- Diagnostics: no public code, message, severity, Fact, Repair, span, or localization behavior changed. `GOV-TRACE-*` strings are maintainer-tool validation errors, not Ling public diagnostics.
- Schema: adds internal governance schema version `1` for `docs/traceability/registry.toml` and strict metadata fields to conformance-only `expect.toml` files. No `ling.diagnostic/0.1`, Semantic Graph, Audit, REPL, CLI, Semantic ID, canonical-byte, package, or runtime schema changed.
- CLI: no `ling` command or exit behavior changed. Two maintainer commands were added under the existing offline `cargo xtask` alias: `traceability verify` and `traceability render`.
- Semantic IDs and language behavior: unchanged. The registry links to existing checked Core and implementation symbols without becoming executable semantics.
- Dependencies: none added or updated; root and fuzz lockfiles are unchanged.

## Determinism and Unicode

Releases, features, fixture directories, evidence, validation errors, and generated table entries are sorted deterministically with ordered collections. Generated links use normalized repository-relative forward-slash paths, and generated-file comparison normalizes only CRLF to LF. No host path, allocation identity, hash-map order, Rust debug output, or current-time value enters the report.

The conformance harness still executes the same source bytes and expected outputs. Added metadata does not alter source decoding, line normalization, UTF-8 byte spans, XID/NFC/security rules, generated tables, or the pinned Unicode 17.0.0 version.

## Intentionally deferred

- Interpreter/VM differential evidence: `VM-1209`, after a checked bytecode/VM path exists under Accepted specifications.
- Schema lifecycle, N-1 readers, migration fixtures, and golden corpora: `GOV-0106`.
- The broader 1.0 support matrix and machine-readable implementation state: `GOV-0108` and `GOV-0109`.
- Traceability for future G1–G6 features: added only when their specifications and real implementation/test artifacts exist; no placeholder feature claims were created.
- Remote CI evidence: available only after push and GitHub Actions completion.
