# DEC-0120: Internal region-inference boundary evidence / 内部 Region Inference 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: ownership-design
> 相关规范/缺口：`DEC-0119` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001` | `GAP-OWNERSHIP-PUBLIC-LIFETIME-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed region and
lifetime boundaries for the bounded `OWN-3203-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not define region
variables, lifetime inference, outlives constraints, escape rules, public
API projection, diagnostics, or ownership semantics.

本决定只授权 test-only 的拟议 region 与 lifetime 边界清单，供
`OWN-3203-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 region variable、lifetime inference、
outlives constraint、escape rule、public API projection、diagnostic 或 ownership 语义。

## Question

The G3 plan sketches lexical and non-lexical regions, returned borrows,
closure captures, public lifetime parameters, outlives constraints, local/
Actor/Task escapes, and suspension crossing. Which evidence can be retained
without freezing a public lifetime or ownership contract?

G3 计划列出 lexical/non-lexical region、returned borrow、closure capture、public lifetime parameter、outlives
constraint、local/Actor/Task escape 与 suspension crossing。在不冻结 public lifetime 或 ownership 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/region_inference_evidence.rs` keeps a test-local
   inventory of thirty-nine provisional boundaries: region/lifetime
   variables and lexical/non-lexical scopes, outlives/inference/fixed-point
   termination, reborrow and Place/Copy/Move/Borrow/Resource/Managed/Trait
   interactions, returned borrows and closure/local/Actor/Task escapes,
   suspension/await/pinning/cancellation/Drop, explicit versus inferred
   public lifetimes, separate compilation and cross-package boundaries,
   FFI/Native ABI, diagnostics, projections, Unicode spans, differential
   evidence, deterministic inference, and Seed migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.region-inference-observation/0`. These bytes are not a region
   variable, lifetime, constraint, escape result, public signature, diagnostic,
   or ownership contract.
3. The child adds no region/lifetime Core node, inference solver, outlives
   graph, escape checker, public API field, diagnostic, Semantic ID, protocol,
   or migration rule. Public `OWN-3203` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its region-inference sketch
  cannot authorize lifetime syntax, public ABI, escape rules, or diagnostics.
- `DEC-0119` keeps borrow vocabulary test-only while ownership authority is
  absent.
- `DEC-0009` governs Seed mutable-place writes and excludes Borrow, lifetime,
  and region semantics.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain
  Open; this decision records region vocabulary without resolving either gap.

## Conformance plan

- Assert all thirty-nine provisional region/lifetime boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep inference, fixed points, escape judgments, public lifetime projection,
  diagnostics, migration, fuzzing, and interpreter/VM/Native fixtures
  deferred.

## Compatibility impact

- Accepted Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public region, lifetime, or
  ownership protocol claim is registered.

## Unresolved alternatives

Region/lifetime variable identity, lexical versus non-lexical scope,
constraint solving and termination, reborrow/escape rules, public explicit
versus inferred lifetimes, separate compilation, suspension/Task/Actor,
pinning/cancellation/Drop, FFI/Native ABI, diagnostics, migration, and
differential semantics remain open under `GAP-OWNERSHIP-MODEL-001`,
`GAP-OWNERSHIP-PUBLIC-LIFETIME-001`, and `OWN-3203`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
