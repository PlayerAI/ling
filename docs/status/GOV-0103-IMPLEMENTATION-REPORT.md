# GOV-0103 Implementation Report / 实施报告

> Status: **Done; committed locally**
> Verification date: 2026-08-20
> Verified base: `main@327bdef57ee476b2330e9f51ae9241346d4affde`
> Implementation commit: `4876a0328d994121fb32c10b3f2a25e3ce11e5ff`
> Task source: [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) `GOV-0103`

## Outcome

GOV-0103 now has one machine-readable lifecycle registry, one deterministic human report, checked RFC/decision templates, a pull-request evidence template, and an offline checker exposed as:

```text
cargo xtask governance check-lifecycle
```

The checker enforces `Open → Draft → Proposed → Accepted / Rejected → Superseded`, prevents Draft/Proposed records from becoming Stable implementation bases, requires acceptance evidence, validates replacement links, reconciles every RFC/decision tuple with the authority index, and rejects template or generated-report drift.

## Delivered artifacts

- [`lifecycle.toml`](../governance/lifecycle.toml): authoritative lifecycle state, dates, history, Stable-basis flag, acceptance evidence, compatibility impact, unresolved alternatives, and supersession relation for all 17 current RFC/decision records.
- [`lifecycle.md`](../governance/lifecycle.md): deterministic state-machine, migration-boundary, merge-policy, and template report.
- [`RFC.md`](../governance/templates/RFC.md) and [`DECISION.md`](../governance/templates/DECISION.md): required sections for every future non-legacy specification record.
- [`pull_request_template.md`](../../.github/pull_request_template.md): Accepted IDs, normative clauses, gaps/conflicts, tests, compatibility, diagnostic/schema/Semantic ID, determinism/Unicode, and deferred-work evidence.
- `tools/xtask/src/lifecycle.rs`: schema, state/history, evidence, supersession, authority parity, template, and report-drift validation.
- `.github/workflows/ci.yml`: the lifecycle gate runs with the existing authority and gap gates.

## Authority and clauses covered

- `GOV-0103`: the declared lifecycle, Stable-basis restriction, Accepted evidence, supersession, experimental-marker policy, semantic-PR authority citation, templates, and CI section checks.
- Root `AGENTS.md` Authority, Implementation boundaries, Pull-request evidence, and Execution-plan governance: Draft/gap material is not semantic authorization; experimental implementation must be traceable; semantic changes must cite Accepted authority.
- `docs/governance/authority.toml`: all one RFC and 16 decision records are reconciled by ID, kind, lifecycle state, and repository-relative source path.

This task adds governance controls only. It does not create or alter Ling language semantics.

## Specification gaps or conflicts

`RFC-0001.md` explicitly remains Draft, so it is indexed as non-Stable and cannot authorize implementation. `GAP-GOV-RFC-STATUS-001` remains Open; GOV-0103 does not promote, reject, or supersede the RFC.

RFC-0001 and DEC-0001 through DEC-0016 predate the new section templates. Rewriting those accepted records in bulk could blur the historical decision text, so they form a closed, checker-enforced legacy set. Required Accepted evidence is recorded in `lifecycle.toml`; every later RFC/decision must use the checked headings, and the checker rejects new legacy exemptions.

The evidence audit also found that the previous DEC-0003 authority metadata described generic serialization/snapshot coverage not present in the decision. Its `covers` entry now matches the source: CLI parsing, the scriptable REPL baseline, the conformance runner, and dependency discipline. The Accepted decision text and behavior are unchanged.

## Tests and verification

Executed locally on 2026-08-20:

- `cargo xtask governance check-lifecycle` — passed: 17 records, 16 Accepted, 17 legacy-format migrations.
- `cargo xtask governance check-authority` — passed: 35 documents, 16 Accepted.
- `cargo xtask governance check-gaps` — passed: 25 Open gaps, six gates.
- `cargo test --package xtask --locked --offline` — 34 passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 172 passed.
- Local Markdown path check across changed Markdown documents — 95 targets resolved.

Fixtures cover the declared transition matrix, deterministic rendering, repository-wide RFC/decision coverage, valid legacy Accepted evidence, valid supersession, dangling-successor rejection, Draft Stable-basis rejection, incomplete Accepted evidence, new legacy-exemption rejection, and missing required headings in non-legacy documents.

The CI workflow was edited and inspected locally; no remote GitHub Actions result is claimed for this unpushed worktree.

## Compatibility impact

- Diagnostics: no Ling diagnostic code, bilingual message, span, or payload changed. `GOV-LIFE-*` messages are internal repository-governance failures.
- Schema: adds internal governance schema `lifecycle` version `1`; no Diagnostic JSON, Semantic Graph, Audit Source, Canonical Bytes, Semantic ID, bytecode, package, LSP, or runtime schema changed.
- Contributor workflow: future RFCs/decisions must use the registered templates and lifecycle history; semantic PRs must provide Accepted authority and compatibility evidence.
- Language behavior and public runtime/CLI behavior: unchanged.

## Determinism and Unicode

The report sorts first by record kind and then by ID. Registry relations use ordered maps/sets, validation errors are sorted and deduplicated, paths are forward-slash repository-relative values, and report parity normalizes only CRLF to LF. No directory enumeration, Rust hash-map iteration, host path, or debug output becomes governance meaning.

Unicode source handling and all Unicode 17.0.0 tables are unchanged.

## Intentionally deferred

- Resolution of `GAP-GOV-RFC-STATUS-001`: requires an explicit reviewed lifecycle decision.
- Retrofitting the 17 historical documents to the new headings: unnecessary for this closed migration and deliberately avoided to preserve their published text.
- Full public protocol inventory and reader/writer compatibility rules: `GOV-0104`.
- Aggregate G0 release-state and compatibility policy: later G0 tasks.
- Remote CI evidence: available only after an authorized push.
