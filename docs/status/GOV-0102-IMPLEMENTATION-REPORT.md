# GOV-0102 Implementation Report / 实施报告

> Status: **Done; committed locally**
> Verification date: 2026-08-20
> Verified base: `main@de5d256321506988f43b54ca15c9aa08e7215f7f`
> Implementation commit: `c147b5c02532b61e23df46f6cb25251d8c94dd7d`
> Task source: [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) `GOV-0102`

## Outcome

GOV-0102 now has one machine-readable specification-gap registry, one deterministic release-ordered report, and an offline checker exposed as:

```text
cargo xtask governance check-gaps
```

The registry contains 25 Open gaps and six explicit v0.1 specification-gate mappings. It imports every item from `SEMANTICS §31`, every RFC-0002 through RFC-0013 candidate topic listed by Draft RFC-0001, the v0.1 package/bytecode/Trait/incremental/formatter/LSP gates, and all current `experimental:` markers found in implementation source.

## Delivered artifacts

- [`gap-register.toml`](../governance/gap-register.toml): gap lifecycle, priority, blocked releases/tasks, observable behavior, authority, candidate RFCs, neutral options, irreversible consequences, required evidence, owner, next action, resolution, supersession, and implementation-marker mappings.
- [`gap-register.md`](../governance/gap-register.md): deterministic gate coverage and release/priority/status ordering.
- `tools/xtask/src/gaps.rs`: schema, lifecycle, relation, source-marker, TODO marker, and report-drift validation.
- `.github/workflows/ci.yml`: both governance checks now run in the normal three-platform CI job after locked dependency fetch.

## Authority and clauses covered

- `SEMANTICS.md §31`: all 11 unresolved questions are represented by source-item IDs `SEMANTICS-31.1` through `SEMANTICS-31.11`.
- Draft `RFC-0001 §22`: candidate topics RFC-0002 through RFC-0013 are indexed as candidates only; this task does not promote Draft text or create those RFCs.
- `ROADMAP-1.0 §4.2/G0.1` and §5.2: lifecycle fields, blockers, evidence, and all six v0.1 gates.
- `GOV-0102` and first-sprint Task C acceptance requirements.

## Specification gaps or conflicts

All 25 entries remain Open. This task deliberately selects no candidate option. Existing Accepted decisions are linked only for the scope they already cover; they do not silently close broader package, Trait, VM, schema, formatter, LSP, concurrency, ownership, backend, device, or Critical questions.

The RFC-0001 lifecycle mismatch remains Open as `GAP-GOV-RFC-STATUS-001`. Its earlier discovery document now points to the machine registry as the single status authority.

## Tests and verification

Executed locally on 2026-08-20:

- `cargo xtask governance check-gaps` — passed: 25 Open gaps, six gates.
- `cargo xtask governance check-authority` — passed: 34 documents, 16 Accepted.
- `cargo test --package xtask --locked --offline` — 24 passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 162 passed.
- Local Markdown path check across touched documents — 77 targets resolved.

Fixtures cover duplicate gap IDs, valid and invalid Accepted/Rejected lifecycle states, unknown statuses, dangling gate links, supersession cycles, stable sort order, registered/unregistered/unknown `TODO(spec)` markers, unmapped `experimental:` markers, required source coverage, and all six v0.1 gates.

The CI workflow was edited and inspected locally; no remote GitHub Actions result is claimed for this unpushed worktree.

## Compatibility impact

- Diagnostics: no Ling diagnostic code, message, span, or payload changed.
- Schema: adds internal governance schema `gap-register` version `1`; no Semantic Graph, Audit Source, diagnostic JSON, bytecode, package, LSP, or runtime schema changed.
- CI: a source marker must use `TODO(spec:GAP-...)` and reference a registered gap; bare or unknown specification TODO markers now fail the governance gate.
- Semantic IDs and language behavior: unchanged.

## Determinism and Unicode

Reports sort by earliest blocked release, priority, lifecycle status, and gap ID. Validation errors, scanned paths, and relations use stable ordering and forward-slash repository-relative paths; symlinked source paths are not traversed. Unicode tables and the pinned Unicode 17.0.0 behavior are unchanged.

## Intentionally deferred

- Selecting or accepting any semantic option: requires the candidate RFC/decision shown in each gap.
- RFC and decision templates/lifecycle automation: `GOV-0103`.
- Full public protocol inventory: `GOV-0104`.
- Schema golden corpora and compatibility readers: `GOV-0106`.
- Aggregate G0 release policy beyond the two current checks: `GOV-0110`.
- Remote CI evidence: available only after an authorized push.
