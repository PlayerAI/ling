# DEC-0124: Internal ownership corpus and property boundary evidence / 内部 Ownership Corpus 与 Property 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: ownership-testing
> 相关规范/缺口：`DEC-0123` | `DEC-0122` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001` | `GAP-ACTOR-AWAIT-REENTRY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed ownership
negative-corpus and property-test boundaries for the bounded
`OWN-3207-OBSERVATION` child. It checks deterministic, duplicate-free
vocabulary. It does not define legal/illegal ownership outcomes, generators,
shrinking, fuzz targets, resource limits, expected diagnostics, or ownership
semantics.

本决定只授权 test-only 的拟议 ownership negative corpus 与 property test 边界清单，供
`OWN-3207-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 legal/illegal ownership outcome、generator、shrinking、fuzz target、resource limit、expected diagnostic 或 ownership 语义。

## Question

The G3 plan names use-after-move, double Drop, mutable aliases, partial moves,
match/loop/closure cases, cancellation, Task/Actor/await, FFI transfer, region
escape, and automatic-borrow ambiguity. Which evidence can be retained
without freezing future ownership or testing contracts?

G3 计划列出 use-after-move、double Drop、mutable alias、partial move、match/loop/closure、cancellation、Task/Actor/await、
FFI transfer、region escape 与 automatic-borrow ambiguity。在不冻结未来 ownership 或 testing 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/ownership_corpus_evidence.rs` keeps a test-local
   inventory of thirty-six provisional boundaries: legal/illegal oracle and
   Value/Managed/Resource/Copy/Move/Borrow/Region/Drop/alias/partial-move
   domains, match/loop/closure/Fault/cancellation/Task/Actor/await, FFI
   transfer, automatic borrow, public lifetime, Profile, generators/shrinking/
   bounds, state-machine/interleaving and failure/restart, deterministic seeds
   and resource limits, host-failure separation, negative diagnostics/repairs,
   Unicode spans, deterministic ordering, differential evidence, migration,
   and Seed preservation.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.ownership-corpus-observation/0`. These bytes are not an oracle,
   generated case, legal/illegal result, expected diagnostic, fuzz target,
   property invariant, or ownership contract.
3. The child adds no ownership corpus, generator, shrinking algorithm, fuzz
   target, expected diagnostic, error code, public protocol, or placeholder
   G3 API. Public `OWN-3207` remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its corpus checklist cannot
  authorize ownership outcomes, generators, diagnostics, or fuzz contracts.
- `DEC-0123` keeps ownership-diagnostic vocabulary test-only, and `DEC-0122`
  keeps Drop/cleanup vocabulary test-only while ownership authority is absent.
- `DEC-0009` governs Seed mutable-place behavior and excludes future ownership
  judgments.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-ACTOR-AWAIT-REENTRY-001` remain Open;
  this decision records corpus vocabulary without resolving either gap.

## Conformance plan

- Assert all thirty-six provisional corpus/property boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep legal/illegal outcomes, generators, shrinking, bounds, interleavings,
  diagnostics, fuzzing, migration, and interpreter/VM/Native fixtures
  deferred.

## Compatibility impact

- Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No ownership corpus, generator,
  property, fuzz, diagnostic, or public protocol claim is registered.

## Unresolved alternatives

Legal/illegal oracle semantics, domain invariants, generators/shrinking,
state-machine/interleaving, failure/cancellation/restart, deterministic seeds,
resource limits, host-failure separation, expected diagnostics/repairs,
Unicode spans, migration, and interpreter/VM/Native differential semantics
remain open under `GAP-OWNERSHIP-MODEL-001`,
`GAP-ACTOR-AWAIT-REENTRY-001`, `OWN-3207`, and missing RFC-N306/RFC-N303/
RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
