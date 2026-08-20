# GOV-0101 Implementation Report / 实施报告

> Status: **Done; committed locally**
> Verification date: 2026-08-20
> Verified base: `main@98024740450fbf5b8943fe31167fc8b03dfbafce`
> Implementation commit: `7bba2adf9104d7d7f96c7ef50343647f649e229e`
> Task source: [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) `GOV-0101`

## Outcome

GOV-0101 now has one machine-readable authority source, one deterministic human report, and an offline Rust checker exposed as:

```text
cargo xtask governance check-authority
```

The checker indexes 33 specification, decision, conformance, planning, registry, evidence, review, and implementation artifacts. It reports 16 Accepted records and correctly preserves RFC-0001 as Draft.

## Delivered artifacts

- [`authority.toml`](../governance/authority.toml): schema-versioned document IDs, lifecycle state, version, authority class, coverage, path, dependencies, supersession relations, and Stable-basis flag.
- [`authority.md`](../governance/authority.md): stable authority-ordered rendering plus the conflict and correction workflow.
- `tools/xtask`: the offline `governance check-authority` and `render-authority` commands.
- [`GAP-GOV-RFC-STATUS-001`](spec-gaps/GAP-GOV-RFC-STATUS-001.md): an Open governance gap for the Draft RFC-0001 versus prior Accepted claims.

## Authority and clauses covered

- Root `AGENTS.md` authority order and conflict-stop rule.
- `GOV-0101` implementation and acceptance clauses in the G0 execution plan.
- First-sprint Task B fields and fixtures: document identity/status/version/coverage/supersession/path, missing files, duplicate IDs, stable sorting, nonzero failure, supersession chains, and Chinese paths.

This task is governance infrastructure only. It does not claim or modify a language-semantic clause.

## Specification gaps or conflicts

`docs/RFC-0001.md` states `Draft`, while prior repository guidance described it as Accepted. The implementation does not promote the RFC. It records [`GAP-GOV-RFC-STATUS-001`](spec-gaps/GAP-GOV-RFC-STATUS-001.md) and treats Draft material as non-Stable until a reviewed lifecycle decision exists.

The public `ling` command and `.ling` extension remain supported by `SEMANTICS.md` and `LANGUAGE.md`; this correction does not reintroduce stale `zero` naming.

## Tests and verification

Executed locally on 2026-08-20:

- `cargo xtask governance check-authority` — passed: 33 documents, 16 Accepted.
- `cargo test --package xtask --locked --offline` — 10 passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --locked --offline` — 148 passed.
- Local Markdown path check across touched documents — 87 targets resolved.

The checker fixtures cover duplicate IDs, missing Accepted paths, Draft-as-Stable misuse, dangling dependencies, dependency cycles, source/manifest lifecycle mismatches, valid supersession chains, deterministic rendering, repository-relative Chinese paths, and the live repository report.

## Compatibility impact

- Diagnostics: none; no error code or public diagnostic payload changed.
- Schema: adds internal governance manifest schema version `1`; no Ling Semantic Graph, diagnostic, CLI JSON, or Audit Source schema changed.
- Semantic IDs: none.
- Language behavior: none.

## Determinism and Unicode

The human report uses authority rank plus document ID ordering, normalized LF comparison, sorted/deduplicated validation errors, and forward-slash repository-relative paths. The task neither changes Unicode tables nor changes the pinned Unicode 17.0.0 behavior.

## Intentionally deferred

- Unified specification-gap registry and blocker mapping: `GOV-0102`.
- RFC and decision lifecycle templates/checks: `GOV-0103`.
- Full public protocol inventory: `GOV-0104`.
- CI governance aggregation: `GOV-0110`.
- Resolution of RFC-0001 lifecycle: requires an explicit reviewed RFC/decision; GOV-0101 does not choose it.
