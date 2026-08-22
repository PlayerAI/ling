# DEC-0044: Seed performance-matrix drift gate / Seed 性能矩阵漂移门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: performance-engineering  
> Related authority/gap: `DEC-0019`, `DEC-0021`, `RFC-0002`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `REL-6604-SEED` child. It does not
complete the G6 performance release gate or authorize benchmark thresholds,
hardware tiers, memory/IO policy, LSP/Native/Actor/Replay/device/Kernel/Zed
measurements, or a public performance protocol. The parent `REL-6604` remains
`BlockedSpec` until those authorities and release evidence are Accepted.

## Question

The Seed performance document already records a measurement-only trend baseline
and maps the twelve planned measurements to current evidence or explicit
deferral. Without a drift check, a row or state could change while the
authority audit continues to describe a different scope. A documentation-only
verifier can protect that inventory without running timing code or inventing a
regression threshold.

## Decision

1. `cargo xtask performance verify` is an internal governance command. It
   reads `docs/testing/PERFORMANCE-BASELINE.md` and validates the exact twelve
   plan-coverage rows: two Covered variants, two Partial rows, and eight
   Deferred rows.
2. The verifier rejects duplicate, missing, or unexpected rows, state drift,
   a missing Plan coverage section, and removal of the measurement-boundary
   phrases for trend-only evidence, fixture exclusion, no absolute claim, and
   Accepted threshold policy. It fails closed with internal
   `GOV-PERF-MATRIX-*` messages.
3. The command validates inventory only. It does not run
   `cargo xtask performance baseline`, freeze a threshold, measure memory or
   IO, add a benchmark dependency, claim cross-host reproducibility, or emit a
   public diagnostic, schema, protocol, or support state.
4. The command is included in the existing Seed reproducibility CI gate. A
   future measurement or state promotion requires its own Accepted policy,
   deterministic fixture and resource boundary, owner, and retained evidence.

## Conformance plan

- Run `cargo xtask performance verify` offline and assert twelve rows with the
  expected 2/2/8 Covered/Partial/Deferred distribution.
- Mutate an isolated row or remove a policy phrase and verify the gate fails
  closed.
- Run the existing opt-in performance baseline only as measurement evidence;
  do not treat the matrix gate as a timing result or release threshold.
- Repeat independent processes and verify no source, semantic, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` documentation validator and CI preflight.
  Ling syntax, Checked Core, runtime, bytecode, diagnostics, schemas,
  Semantic IDs, dependencies, public protocols, and Unicode 17.0.0 behavior
  are unchanged.
- No benchmark dependency, threshold, memory claim, unsupported backend
  harness, or placeholder public API is introduced.

## Unresolved alternatives

Benchmark schema, sample/warm-up/variance rules, hardware tiers, memory/IO
measurement, package-build/LSP/Native/Actor/Replay/device/Kernel/Zed scope,
regression ownership, and release threshold policy remain governed by the
parent `REL-6604` and later Accepted performance authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
