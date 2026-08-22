# DEC-0135: Internal Native optimization boundary evidence / 内部 Native Optimization 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: native-backend
> 相关规范/缺口：`DEC-0134` | `DEC-0133` | `DEC-0132` | `DEC-0131` | `DEC-0130` | `DEC-0129` | `DEC-0128` | `DEC-0127` | `DEC-0126` | `DEC-0125` | `DEC-0012` | `ROADMAP-1.0` | `GAP-NATIVE-BACKEND-ABI-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of the proposed Native
optimization and verification boundary for the bounded
`BACK-3504-OBSERVATION` child. It records pass and evidence vocabulary without
implementing transformations, proof rules, pass ordering, or observable
optimization behavior.

本决定只授权 test-only 的拟议 Native optimization 与 verification 边界清单，供
`BACK-3504-OBSERVATION` 子任务使用。它记录 pass 与 evidence 词汇，但不实现 transformation、proof rule、pass ordering 或可观察的 optimization behavior。

## Question

BACK-3504 proposes constant folding, dead-block elimination, trivial inlining,
proof-backed bounds-check elimination, copy propagation, and explicit tail
calls, each with pre/post verification and differential/property tests. Which
planning vocabulary can be retained while NIR, ABI, memory, ownership,
profiles, and optimization proof authorities remain unaccepted?

BACK-3504 提议 constant folding、dead-block elimination、trivial inlining、有证明的 bounds-check elimination、copy propagation 与语义明确时的
tail call，并要求每个 pass 具备 pre/post verification 与 differential/property test。在 NIR、ABI、memory、ownership、Profile 与 optimization proof
权威尚未 Accepted 时，哪些规划词汇可以保留？

## Decision

1. `crates/ling-types/tests/native_optimization_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries: numeric/effect/Fault
   constant folding; dead blocks/reachability; inlining/closure/recursion;
   copy/alias and proof-backed bounds checks; tail calls/cleanup/evaluation
   order; capabilities, Resource/Managed, borrow, Task/Actor, cancellation,
   FFI, Profile, ABI, target numeric model and endianness; pre/post verifier,
   pass order/invalidation, proof certificates, deterministic diagnostics and
   optimization failures; source/debug/stack/Semantic ID/Unicode identity;
   reproducibility, bounds, security, migration, fixtures, property/fuzz,
   interpreter/VM/Native differential and optimized/unoptimized equivalence;
   host timing/allocation/address/map exclusions, unsupported forms, and Seed
   compatibility.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.native-optimization-observation/0`. These bytes are not an optimizer,
   pass manager, proof/certificate, verifier hook, diagnostic, performance
   result, semantic-preservation proof, public protocol, or Native behavior.
3. The child adds no optimizer, pass manager, proof representation, verifier
   hook, optimization diagnostic, benchmark/performance claim, public
   protocol, or placeholder crate. Public `BACK-3504` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its pass list cannot define
  semantic-preservation rules, numeric behavior, pass order, proofs, or
  observable debug/stack behavior.
- Accepted `DEC-0134` through `DEC-0131` define only test-local ABI/codegen/
  backend-selection/verifier vocabulary. They do not authorize optimization.
- RFC-N304/RFC-N306 and candidate RFC-0011 are absent or not Accepted.
  `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and dependent
  Profile/Task/Actor/FFI gaps remain Open.
- `docs/SEMANTICS.md` requires optimization to preserve observable semantics;
  accepted Seed decisions do not define Native pass proofs or legality.

## Conformance plan

- Assert all sixty provisional optimization boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep transformation semantics, proof certificates, pre/post verifier hooks,
  pass ordering, diagnostics, debug/stack behavior, performance claims, and
  differential/property execution deferred until their authorities are
  Accepted.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No optimizer, proof, diagnostic,
  performance, Semantic ID, dependency, or public protocol claim is
  registered.

## Unresolved alternatives

Numeric/effect/Fault folding; reachability/dead blocks; inlining/closure/
recursion; copy/alias/bounds proofs; tail/cleanup/evaluation order;
capability/Resource/Managed/borrow/Task/Actor/cancellation/FFI/Profile/ABI/
target constraints; verifier/proof/pass-order/invalidation; diagnostics,
source/debug/stack/Semantic ID; reproducibility/bounds/security/migration;
fixtures/property/fuzz/differential equivalence; and host-output exclusions
remain open under `BACK-3504`, `BACK-3503`, `BACK-3502`, `BACK-3501`, `NIR-3403`,
`NIR-3402`, `NIR-3401`, `GAP-NATIVE-BACKEND-ABI-001`,
`GAP-OWNERSHIP-MODEL-001`, and missing RFC-N306/RFC-N304/RFC-N303/RFC-0007
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
