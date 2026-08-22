# DEC-0154: Internal Kernel corpus boundary evidence / 内部 Kernel Corpus 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0153` | `DEC-0152` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`CPU-4203-OBSERVATION` Kernel corpus boundary. It records provisional fixture,
manifest, expected-result, and differential vocabulary while Kernel corpus and
execution authorities remain unresolved.

本决定只授权 `CPU-4203-OBSERVATION` 使用 test-local 的拟议 Kernel Corpus 边界清单，
在 Kernel corpus 与 execution 权威尚未解决时，只记录临时 fixture、manifest、expected-result 与 differential 词汇。

## Question

CPU-4203 proposes vector addition, a small matrix multiply, an image filter,
reductions, optional histogram/atomic behavior, invalid bounds, alias
conflicts, floating-point edges, and Unicode source mapping. Which planning
vocabulary can be retained as bounded evidence without defining Kernel syntax,
expected-result semantics, or a conformance protocol?

## Decision

1. `crates/ling-types/tests/kernel_corpus_evidence.rs` keeps a test-local
   inventory of sixty provisional manifest, identity, source, operation,
   fixture, expected-result, differential, diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.kernel-corpus-observation/0`. They are not source
   fixtures, expected outputs, Fault classifications, diagnostics, Semantic
   IDs, public schemas, or support claims.
3. No Kernel fixture, manifest, expected-output snapshot, corpus runner,
   differential runner, dependency, diagnostic, protocol, or placeholder API
   is added. Public `CPU-4203` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:165-180` is
  non-normative; Kernel corpus behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel syntax, execution,
  expected results, numeric determinism, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; Seed conformance fixtures
  do not authorize Kernel fixtures.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer fixture syntax, manifest fields, expected outputs, Faults, exact or
  tolerance comparisons, migration, diagnostics, differential, and protocol
  behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Kernel corpus or support claim exists.

## Unresolved alternatives

Manifest/fixture identity and versioning; source bytes and `.ling` mapping;
profiles/targets, inputs/outputs/Faults/traces; vector/matrix/filter/reduction/
histogram/atomic cases; bounds/alias/numeric/determinism; positive/negative/
property/corruption/migration/Unicode/source-map fixtures; exact/tolerance
comparison; CPU/device differential; diagnostics, host exclusion, protocol
inventory, and public corpus status remain open under CPU-4203, CPU-4201/4202,
KCHK-4101 through KCHK-4105, GAP-KERNEL-DEVICE-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
