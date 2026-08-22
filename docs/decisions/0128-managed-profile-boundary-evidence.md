# DEC-0128: Internal Managed Profile boundary evidence / 内部 Managed Profile 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: profile-policy
> 相关规范/缺口：`DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0013` | `ROADMAP-1.0` | `GAP-CRITICAL-PROFILE-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Profile
and `no_gc` boundaries for the bounded `GC-3304-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not define Profile syntax,
manifests, feature legality, capability propagation, allocation bounds,
runtime assertions, diagnostics, or runtime semantics.

本决定只授权 test-only 的拟议 Profile 与 `no_gc` 边界清单，供
`GC-3304-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 Profile 语法、manifest、feature legality、
capability propagation、allocation bound、runtime assertion、诊断或运行时语义。

## Question

GC-3304 sketches Explore Managed support, Native Managed Islands, Critical
restrictions, and static/runtime `no_gc` checks. Which boundary vocabulary can
be retained without choosing a profile contract or silently enabling reserved
features?

GC-3304 草拟 Explore Managed 支持、Native Managed Island、Critical 限制，以及静态/运行时 `no_gc` 检查。
哪些边界词汇可以保留，而不会选择 Profile 契约或静默启用保留功能？

## Decision

1. `crates/ling-types/tests/managed_profile_evidence.rs` keeps a test-local
   inventory of forty-four provisional boundaries: Profile identity/version and
   target manifests, inheritance, Explore/Native-Island/Critical matrix,
   `no_gc` annotation and transitive call/closure/generic/import/callback/
   Task/Actor/FFI rules, allocation and safepoint legality, Resource Drop,
   dynamic/reflection restrictions, boundedness, capability limits, Native
   transitions, pin/handle/ABI, cross-Profile calls/transfers, Critical
   timing/target/security, pre-execution rejection, runtime assertions,
   diagnostics, migration, support matrix, projections, Semantic IDs, Unicode
   spans, and differential evidence.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.managed-profile-observation/0`. These bytes are not Profile syntax,
   manifest data, capability, `no_gc` checker, allocation proof, runtime
   assertion, Fault, diagnostic, public protocol, or runtime contract.
3. The child adds no profile-policy crate, profile selection/validation pass,
   `no_gc` AST/Typed Core form, Managed capability, Native-Island schema,
   runtime assertion, profile diagnostic, public protocol, or placeholder G3
   API. Public `GC-3304` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative and cannot define Profile
  versioning, feature legality, source syntax, diagnostics, or runtime
  behavior.
- Accepted `DEC-0127`, `DEC-0126`, and `DEC-0125` record only interop/
  collector/object-model vocabulary; `DEC-0013` preserves fault separation.
- `GAP-CRITICAL-PROFILE-001`, `GAP-NATIVE-BACKEND-ABI-001`, and
  `GAP-OWNERSHIP-MODEL-001` remain Open. RFC-0012 and RFC-N303/RFC-N304/
  RFC-N305/RFC-N306/RFC-0007 are not Accepted. This decision records
  vocabulary without resolving those gaps.

## Conformance plan

- Assert all forty-four provisional Profile boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep Profile syntax/manifests, capability and `no_gc` propagation,
  allocation/boundedness, Native-Island transitions, Critical restrictions,
  assertions/Faults, diagnostics, migration, and differential semantics
  deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No Profile, `no_gc`, capability,
  diagnostic, Semantic ID, or public protocol claim is registered.

## Unresolved alternatives

Profile identity/versioning and manifests, feature legality and inheritance,
`no_gc` syntax/propagation, Managed allocation and safepoints, Native-Island
transitions, Critical boundedness/timing/Fault/security, diagnostics, support
matrix, migration, and interpreter/VM/Native differential semantics remain
open under `GC-3304`, `GC-3303`, `GC-3302`, `GC-3301`,
`GAP-CRITICAL-PROFILE-001`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-OWNERSHIP-MODEL-001`, and missing RFC-0012/RFC-N303/RFC-N304/RFC-N305/
RFC-N306/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
