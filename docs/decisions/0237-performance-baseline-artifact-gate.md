# DEC-0237: Performance-baseline artifact gate / 性能基线产物门禁

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: performance engineering
> 相关规范/缺口：`DEC-0019` | `DEC-0021` | `DEC-0044` | `REL-6604`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes structural validation of the checked-in
`ling.performance-baseline/1` evidence artifact. It protects measurement
shape and observable query work; it does not compare elapsed time or define a
performance threshold.

本决定授权对已检入的 `ling.performance-baseline/1` 证据产物进行结构校验。它保护测量形状与可观察
查询工作量，但不比较耗时，也不定义性能阈值。

## Question

How should the existing INC-1410 JSON remain usable evidence when the Markdown
matrix verifier currently ignores it?

## Decision

1. `cargo xtask performance verify` parses
   `docs/status/INC-1410-PERFORMANCE-BASELINE.json` with unknown fields denied.
2. The artifact must retain internal schema `ling.performance-baseline/1`,
   three samples, a 10,000-file synthetic fixture, and explicit exclusion of
   fixture setup from timed regions.
3. The exact eight scenario names and order are shared with the opt-in timing
   harness. Every numeric array must contain exactly three samples.
4. Duration samples must be non-zero, but their values are never compared to a
   threshold, previous run, host, or one another.
5. Deterministic work observations are checked: each scenario's trace, miss,
   and hit counts; equality of trace count to misses plus hits; 10,000 completed
   items for synthetic scenarios; and a stable non-zero completed-item count
   within each checked-query scenario.
6. The verifier reads historical evidence only. It does not execute the timing
   harness, update the artifact, measure memory/IO, or claim cross-host
   reproducibility.
7. The artifact schema remains internal evidence, not a public Ling protocol.
   No diagnostic, CLI behavior, runtime semantic, or support promise is added.
8. Parent `REL-6604` remains `BlockedSpec` for accepted comparison policy,
   thresholds, hardware tiers, missing surfaces, and G6 release evidence.

## Normative basis

- Accepted DEC-0019 and DEC-0021 authorize the deterministic query identity,
  invalidation, and trace evidence consumed by INC-1410.
- Accepted DEC-0044 authorizes a bounded internal performance inventory gate
  without timing thresholds or public performance semantics.
- The existing INC-1410 report defines the checked-in JSON as measurement-only
  evidence and explicitly excludes fixture construction.

## Conformance plan

- Accept the current eight-scenario artifact and report its scenario count.
- Reject schema drift, unknown fields, scenario/order drift, sample-cardinality
  drift, zero timing evidence, query-work drift, and synthetic completion drift.
- Mutate schema and miss counts in an isolated test and verify fail-closed
  errors.
- Run focused xtask, performance, governance, status, workspace, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

This changes only an internal verifier and shares existing harness constants.
Ling source/runtime semantics, diagnostics, schemas, Semantic IDs, cache,
packages, dependencies, CLI/editor behavior, Unicode 17.0.0, and the historical
timing values remain unchanged.

## Unresolved alternatives

New timing capture; baseline replacement/retention policy; warm-up and sample
statistics; regression thresholds; hardware/OS/toolchain tiers; memory and IO;
package build, LSP, VM, Native, Actor, Replay, device/Kernel, and Zed
measurements; cross-host execution; and release ownership remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
