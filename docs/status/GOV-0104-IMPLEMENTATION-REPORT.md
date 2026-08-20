# GOV-0104 Implementation Report / 实施报告

> Status: **Done; committed locally**
> Verification date: 2026-08-20
> Verified base: `main@8937824a61f140db0c3ba1cfc5f07611e64ec1e3`
> Implementation commit: `508ae4db327e5815d56093f5b5b107c916732904`
> Task source: [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) `GOV-0104`

## Outcome

GOV-0104 now has one machine-readable public-interface/protocol inventory, one deterministic policy report, source/version/fixture traceability, and an offline checker exposed as:

```text
cargo xtask governance check-protocols
```

The inventory contains 18 records: nine current public protocols (three Experimental, six Preview, zero Stable), one explicitly Internal incident artifact, and eight unimplemented Future protocol families. This is intentionally conservative: ROADMAP-1.0 defines Stable as the 1.x commitment after all stabilization gates, which no Seed protocol has yet passed.

## Delivered artifacts

- [`protocol-inventory.toml`](../governance/protocol-inventory.toml): CLI, exit codes, human output, Diagnostic JSON, Semantic Graph JSON, Canonical Bytes, Semantic IDs, Audit Source, REPL JSON, internal incidents, Semantic Transaction, manifest/lock/build metadata, bytecode, replay, ABI, and evidence records.
- [`protocol-inventory.md`](../governance/protocol-inventory.md): deterministic stability summary and complete producer/consumer/reader/writer/unknown-field/migration/source/fixture policies.
- `tools/xtask/src/protocols.rs`: schema, required-surface coverage, authority-state, version-marker, path, fixture, lifecycle, and report-drift validation.
- `.github/workflows/ci.yml`: all four governance gates now run in the normal CI matrix.
- Root `AGENTS.md` and bilingual `README.md`: contributor invariant, command, and registry documentation.

## Authority and clauses covered

- `ROADMAP-1.0 §2.3`: public `Experimental`, `Preview`, and `Stable` meanings; Internal and Future records are inventory boundaries, not public capability claims.
- `ROADMAP-1.0 §2.4` and `G0.2`: protocol versions, reader/writer policies, canonical-output exclusions, and the full CLI/Diagnostic/Semantic/Audit/Transaction/package/bytecode/replay/ABI/evidence surface.
- `GOV-0104`: every required field and required protocol family, including the negative gates for unversioned public schemas and Stable protocols without fixtures.
- Accepted DEC-0001/0002/0003/0012/0013/0015/0016: current diagnostic, position, CLI, semantic identity, exit, Audit, and REPL behavior only within each decision's scope.

This task records current behavior and absences. It does not create a new protocol, schema field, command, migration promise, or Ling language semantic.

## Specification gaps or conflicts

- No current protocol is marked Stable. Stable diagnostic code meanings remain a documented compatibility subset inside the Preview `ling.diagnostic/0.1` container; this avoids misreporting Seed as the ROADMAP-1.0 1.x support matrix.
- `ling.semantic/0.1`, v1 canonical identity domains, and `experimental:blake3:` IDs remain Experimental. `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and `GAP-SEMANTIC-HASH-UPGRADE-001` remain Open.
- Diagnostic JSON and REPL JSON have writers but no standalone public readers. Their records state the current conservative version/unknown-field boundary without inventing an N-1 promise.
- Semantic Transaction, Ling manifest/lock/build metadata, bytecode, replay, ABI, and evidence have no current version, writer, reader, or fixture and are explicitly Future. Cargo metadata is not relabeled as a Ling protocol.
- `ling.internal-incident/0.1` is recorded as Internal so its versioned local debug file cannot be mistaken for the Future public evidence bundle.

No new gap option was selected. Existing registered gaps remain the decision gates for package, semantic lifecycle/hash, bytecode, replay, ABI/backend, and evidence work.

## Tests and verification

Executed locally on 2026-08-20:

- `cargo xtask governance check-protocols` — passed: 18 records; nine public (three Experimental, six Preview, zero Stable), one Internal, eight Future.
- `cargo xtask governance check-authority` — passed: 36 documents, 16 Accepted.
- `cargo xtask governance check-gaps` — passed: 25 Open gaps, six gates.
- `cargo xtask governance check-lifecycle` — passed: 17 records, 16 Accepted, 17 legacy-format migrations.
- `cargo test --package xtask --locked --offline` — 43 passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 181 passed.
- Local Markdown path check across changed Markdown documents — 136 targets resolved.

Protocol fixtures cover a valid versioned Preview schema, missing public-schema version, Stable-without-fixture rejection, invalid Future implementation claims, Preview without Accepted authority, source/version-marker drift, duplicate IDs, deterministic rendering, and live coverage of every required surface.

The CI workflow was edited and inspected locally; no remote GitHub Actions result is claimed for this unpushed worktree.

## Compatibility impact

- Public protocols and language behavior: unchanged; the registry describes existing bytes, commands, policies, and explicit absences.
- Diagnostics: no code, meaning, severity, bilingual message, Facts type, Repair, span, or JSON field changed.
- Schema: adds internal governance schema `protocol-inventory` version `1`; no Diagnostic, Semantic Graph, Audit, REPL, Canonical Bytes, Semantic ID, runtime, package, bytecode, replay, ABI, or evidence schema changed.
- CI: implemented public protocols must carry versions/source markers; Preview/Stable records require Accepted authority; Stable additionally requires fixtures.

## Determinism and Unicode

The report sorts by visibility, fixed category rank, and protocol ID. Records, authority lookups, paths, validation errors, and duplicate checks use ordered collections and repository-relative forward-slash paths. Every implemented version marker must occur in a declared source file, and report parity normalizes only CRLF to LF.

Canonical protocol policies explicitly exclude paths, hash-map order, arena indices, allocation addresses, and Rust debug values where the accepted decisions require that boundary. Unicode source behavior and all pinned Unicode 17.0.0 tables are unchanged.

## Intentionally deferred

- Diagnostic code-registry evolution and LSP code reuse: `GOV-0105`.
- Schema lifecycle, N-1 readers, golden corpora, and migration fixtures: `GOV-0106`.
- Aggregate compatibility/release policy and support matrix: later G0 tasks.
- Every Future protocol implementation: blocked on its registered Accepted RFC/decision gate.
- Remote CI evidence: available only after an authorized push.
