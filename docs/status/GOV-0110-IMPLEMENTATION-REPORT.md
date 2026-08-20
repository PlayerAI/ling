# GOV-0110 G0 CI gate implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `a7f2b03270f0e4a163cd8f927cd56475c5062daf`
> Verified baseline: `main@76c99a28b18c70a9f8393c1e8c42d81a4928fcb0`

## Outcome

GOV-0110 closes the G0 governance and compatibility stage with eight explicit, always-on GitHub Actions gates:

- `governance-authority`
- `gap-register`
- `protocol-schema`
- `error-code-registry`
- `traceability-links`
- `support-matrix`
- `canonical-determinism`
- `seed-reproducibility`

The implementation also adds three bounded maintainer commands. `cargo xtask governance check-all` aggregates the five governance registries without stopping after the first failing registry. `cargo xtask ci verify` rejects drift in the named G0 jobs, their required commands, the three-host workspace test matrix, fuzz smoke, MSRV, permissions, triggers, and the locked/offline `xtask` alias. `cargo xtask seed reproduce` builds the Seed CLI once and compares exact output from two independent processes for each of `check`, `run`, Semantic Graph, and Audit Source.

The gates codify existing Seed behavior and governance evidence. They do not add a Ling language feature, widen the supported profile or target set, promote a protocol, or create a public compiler interface.

## Normative clauses and decisions covered

- [`02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) GOV-0110: eight named jobs, specification/conformance coverage, schema compatibility, bilingual diagnostic validation, canonical determinism, and Seed reproducibility.
- The G0 integration exit criteria in the same plan: authority, gaps, protocols, schemas, diagnostic codes, traceability, support state, canonical bytes, and Seed behavior are all checked by executable repository commands.
- Accepted DEC-0001 and DEC-0002: public diagnostics remain registered and bilingual, and source positions remain original UTF-8 byte spans.
- Accepted DEC-0012: Semantic Graph canonical bytes and Semantic IDs remain deterministic and independent of ordinary JSON member order or Rust hash-map iteration.
- Accepted DEC-0016: REPL and Audit boundaries retain their existing protocol roles; reproducibility does not create a new reader or evaluator path.
- [`authority.toml`](../governance/authority.toml), [`gap-register.toml`](../governance/gap-register.toml), [`protocol-inventory.toml`](../governance/protocol-inventory.toml), [`ERROR-CODES.md`](../ERROR-CODES.md), [`registry.toml`](../traceability/registry.toml), [`support-matrix.toml`](../governance/support-matrix.toml), and [`implementation-status.toml`](implementation-status.toml) remain the machine authorities consumed by the gates.

The CI contract is Active engineering evidence, not Accepted language semantics and not a Stable compatibility basis.

## Implementation

- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) now exposes the eight required G0 gate names through one fail-fast-disabled Ubuntu matrix. Every pull request and every push to `main` runs the complete matrix.
- Specification, schema, diagnostic, and canonical-writer path requirements are implemented as an always-on conservative superset. This avoids a path-filter omission silently skipping a required gate.
- The existing Ubuntu/macOS/Windows formatting, Clippy, test, documentation, and release-build matrix remains intact. The pinned nightly fuzz-corpus smoke job and Rust 1.85 MSRV job also remain intact.
- [`ci.rs`](../../tools/xtask/src/ci.rs) validates the workflow contract directly from repository text. It uses no YAML dependency and fails closed on missing or duplicate G0 gates, missing commands, path filters, platform drift, missing quality/fuzz/MSRV evidence, or an unlocked/online `xtask` alias.
- [`g0.rs`](../../tools/xtask/src/g0.rs) executes all five governance validators, accumulates and deterministically sorts their errors, and reports one aggregate summary.
- [`seed.rs`](../../tools/xtask/src/seed.rs) uses the already built `ling` executable through eight independent child processes. It requires successful empty-stderr execution and exact pairwise bytes for all four surfaces.
- The `run` surface must emit the exact Seed greeting `你好，零` plus one LF. `check` must emit no stdout. Semantic Graph output must match the registered canonical golden byte-for-byte and pass the real isolated `ling_semantic::read_json` reader. Audit output must satisfy the accepted BOM-free, LF-only, version-header boundary; the existing conformance test separately parses and round-trips it through the real Audit reader.
- Binary discovery respects `CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET`, and the host executable suffix. The prerequisite build and all normal Cargo gates remain locked and offline.
- The CI contract and all three implementations are registered as positive traceability evidence for the Seed CLI/tooling feature. The bilingual README documents the aggregate commands and their scope.
- No external dependency, placeholder public API, alternate compiler pipeline, or JSON-to-Typed-Core path was introduced.

The design follows KISS and YAGNI by validating exactly the current workflow contract and four implemented Seed surfaces. It follows DRY by composing the existing registry validators and real protocol readers instead of reimplementing their rules. Each module has one responsibility: aggregate governance, CI-contract validation, or process-level reproducibility.

## Specification gaps or conflicts

- The execution-plan integration example says `traceability verify --release v0.1`, but no v0.1 release authority or corpus exists. The gate correctly validates the implemented and registered `v0.0.1` release rather than fabricating v0.1 evidence.
- The plan describes path-triggered obligations. GitHub workflow path filters can suppress an entire workflow and are easy to make incomplete as the repository evolves. Running all eight bounded G0 gates for every pull request is a stricter, deterministic implementation of those obligations.
- Rust's standard `RandomState` initializes process-local hash keys. Two independent processes therefore exercise separate hash-map state without exposing or standardizing an implementation seed as Ling semantics. The check compares observable bytes only.
- The Audit reproducibility boundary checks canonical bytes and relies on the existing parser round-trip conformance test for grammar validation. It does not create a second Audit parser in `xtask`.
- Remote GitHub Actions execution is not claimed until the local commits are pushed and GitHub reports a completed run.

No unresolved language specification conflict was decided through code or snapshots, and no new gap record was needed for this bounded CI integration.

## Tests and verification

Executed locally on 2026-08-20 against the final verified implementation commit:

- `cargo xtask ci verify` — passed: eight named G0 gates, 19 required commands, and three workspace-test hosts.
- `cargo xtask governance check-all` — passed: five checks covering 41 authority documents, 25 gaps, 17 lifecycle records, 18 protocols, and 56 diagnostic codes.
- `cargo xtask schema validate-all` — passed: three schemas, four valid fixtures, six invalid fixtures, and one canonical byte fixture.
- `cargo xtask schema compatibility --from N-1 --to N` — passed: zero verified N-1 edges and three explicit `NoPreviousVersion` records.
- `cargo xtask schema corrupt-inputs` — passed: 23 deterministic mutations.
- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 50 evidence records, and seven deferred differential paths.
- `cargo xtask support verify` — passed: seven features, three profiles, three hosts, one Native target, six backends, one standard package, 18 protocols, and nine explicit unsupported records.
- `cargo xtask seed reproduce` — passed: four surfaces, eight independent processes, and 41,866 compared output bytes.
- Focused Semantic, CLI canonical-determinism, Unicode-generation drift, and Seed-example tests passed using the exact CI commands.
- `cargo test --package xtask --locked --offline` — 91 tests passed, including six new aggregate, CI-contract, and reproducibility tests.
- `cargo test --workspace --all-features --locked --offline` — 234 tests passed, including doc-test harnesses.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo +1.85 check --workspace --all-features --locked --offline` — passed with the declared MSRV.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Execution-plan `SHA256SUMS.txt` — all 27 entries passed after the backlog transition.
- Active local Markdown target audit — 598 targets resolved, zero missing; the frozen execution-plan baseline was excluded.
- `git diff --check` — passed.

Negative coverage rejects omitted or renamed G0 gates, duplicate gates, missing gate commands, path filters, restricted triggers, widened permissions, platform/quality/fuzz/MSRV drift, an unlocked or online `xtask` alias, nondeterministic process output, stderr on successful Seed surfaces, canonical-golden drift, unreadable Semantic Graph JSON, and invalid Audit byte boundaries.

## Compatibility impact

- Language semantics: unchanged. Evaluation still consumes checked Typed Core only.
- Public diagnostics: no code, severity, bilingual template, Fact, Repair, or original UTF-8 byte-span behavior changed. `GOV-CI-*` and `GOV-REPRO-*` are maintainer-tool failures, not public compiler diagnostics.
- Public schemas and protocols: no marker, version, field, reader/writer range, canonical encoding, or stability changed.
- CLI: no `ling` command, option, output, or exit code changed. Three maintainer commands were added under `cargo xtask`.
- Semantic IDs: no hash algorithm, prefix, canonical domain encoding, normalization input, or identity input changed.
- Status schema: unchanged; GOV-0110 is appended as another completed task record.
- Dependencies: unchanged.

## Determinism and Unicode

Gate names and command contracts are exact and ordered. Aggregate failures are sorted and deduplicated. Seed surfaces compare raw stdout bytes across independent processes and reject successful stderr. Semantic output is also compared with its registered canonical golden and real reader; Audit bytes are constrained to the accepted header and line-ending boundary. No timestamp, absolute host path, allocation identity, Rust debug output, arena index, or hash-map iteration order enters a public artifact.

Unicode remains pinned to 17.0.0. The Unicode generator drift command remains an explicit G0 gate. No generated table, NFC normalization, XID/security behavior, source decoding, or original UTF-8 byte-span mapping changed.

## Intentionally deferred

- Remote GitHub Actions evidence and branch-protection enforcement until the commits are pushed and the remote run completes.
- Any v0.1 traceability corpus or release claim.
- A general YAML parser or general-purpose CI policy engine.
- Stability promotion for Experimental or Preview protocols.
- Previous-version protocol readers or migration adapters not already registered.
- G1 project/package, VM, Trait, incremental, formatter, LSP, Tree-sitter, and Zed implementation beyond their separately authorized tasks and prerequisite decisions.
