# DEC-0233: Deprecation-policy readiness boundary evidence / 弃用政策就绪边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: compatibility governance
> 相关规范/缺口：`DEC-0001` | `DEC-0231` | `DEC-0232` | `COMPAT-6504`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded, executable readiness evidence for the seven
policy areas named by `COMPAT-6504`. It does not publish a deprecation policy,
promise 1.x compatibility, or authorize warnings, transitions, or removals.

本决定授权为 `COMPAT-6504` 列出的七个政策领域建立有界、可执行的就绪证据，但不
发布弃用政策、不承诺 1.x 兼容性，也不授权警告、状态迁移或移除行为。

## Question

What deprecation-policy evidence can be made executable before Ling has a
released major version and Accepted lifecycle authorities?

## Decision

1. Ling currently has zero released major versions and no public deprecation
   policy. Both facts must remain explicit; readiness evidence is not a policy.
2. The exact seven plan areas are 1.x compatibility promise, minimum
   deprecation period, diagnostic lifecycle, schema N-1 policy, target/profile
   support lifecycle, security exception, and migration-tooling commitment.
3. Six areas remain `Unavailable`. Diagnostic lifecycle is only a
   `GuardedSubset`: Accepted DEC-0001 preserves allocated identities, prevents
   code reuse, retains retired allocations, and excludes retired codes from
   canonical constants and emitters. It defines no general warning, notice,
   replacement, suppression, or removal lifecycle.
4. `SCHEMA-LIFECYCLE-POLICY` and `SUPPORT-MATRIX` must remain Draft with
   `stable_basis = false` until their own acceptance processes complete.
   Existing schema fixtures and support states do not establish a general N-1
   or support-lifecycle promise.
5. Migration tooling remains unavailable while there is no Accepted version
   pair and no public migration command, as required by DEC-0232.
6. `cargo xtask deprecation verify` is an internal governance drift gate. Its
   manifest, generated report, states, and `GOV-DEPRECATION-*` labels are not a
   public protocol or diagnostic domain.
7. Parent `COMPAT-6504` remains `BlockedSpec`. An actual policy requires an
   Accepted authority that defines subjects, timing, transitions, diagnostics,
   schema/protocol ranges, security exceptions, migration commitments, and
   executable cross-version evidence.

## Normative basis

- Accepted DEC-0001 defines the bounded diagnostic-code non-reuse and
  retirement invariants used by the sole `GuardedSubset` row.
- DEC-0231 records only a v0.0.1 current-compiler boundary and explicitly adds
  no 1.x promise or general N-1 edge.
- DEC-0232 keeps migration tooling absent without an Accepted version pair.
- The roadmap and G6 execution checklist require a future published policy but
  do not define its semantics.

## Conformance plan

- Verify zero released major versions, an absent public policy, and the exact
  seven rows in canonical order and state.
- Validate every blocker/evidence path and generated report drift.
- Verify the retired diagnostic table and lock retain `L-IMPL-0001` as retired.
- Verify schema lifecycle and support-matrix authorities remain Draft and
  non-stable, and migration readiness still records no pair or command.
- Require `cargo xtask deprecation verify` in the always-on CI contract.
- Run error-code, schema, support, migration, compatibility, governance,
  status, workspace, lint, formatting, deterministic, and offline gates.

## Compatibility impact

This decision adds internal evidence and explicit non-claims only. It changes
no source, diagnostic allocation, warning severity, schema reader/writer,
protocol, support state, profile, target, package, CLI, Semantic ID, dependency,
Unicode 17.0.0, compiler, or runtime behavior.

## Unresolved alternatives

Eligible subjects; 1.x ranges; minimum periods; warnings, suppressions, and
facts; replacement and removal obligations; schema/protocol N-1 windows;
target/profile transitions; security-exception authority and notification;
migration commitments; offline distribution; cross-version fixtures; and
cross-platform guarantees remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
