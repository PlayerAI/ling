# Trait Solver Performance and Termination Inventory / Trait Solver 性能与终止盘点

## Scope

This inventory separates the accepted deterministic termination boundary from
the unresolved production performance and editor contracts. The full
`TRAIT-1309` task remains `BlockedSpec`. The accepted semantic nesting limit
remains exactly 64. Host wall-clock time is not Ling language semantics.

本盘点区分已接受的确定性终止边界与尚未确定的生产性能及编辑器契约。
完整 `TRAIT-1309` 继续保持 `BlockedSpec`，语义嵌套上限仍严格为 64。

## Current surface matrix

| Trait performance surface | Current evidence | State | Authority / blocker |
| --- | --- | --- | --- |
| Active-obligation cycle rejection | Ordered active-key tracking and exact `Cycle { depth: 1 }` negative test | Internal | Accepted RFC-0005 §2.5 and DEC-0026 |
| Nested-obligation depth limit | `MAX_NESTED_OBLIGATIONS = 64` and exact `DepthLimit { depth: 64 }` test | Internal | Accepted RFC-0005 §2.5 and DEC-0026 |
| Source-evidence-independent selection | Equivalent logical source names produce the same selected Trait/impl/member projection | Internal | Accepted DEC-0068 |
| Production HIR and Typed Core integration | The bounded solver seam is not a production incremental/LSP host | BlockedSpec | `GAP-TRAIT-COHERENCE-001` |
| Deterministic solver work budget | No accepted work unit, precedence, exhaustion result, or diagnostic contract | BlockedSpec | Missing performance/resource decision |
| Trait benchmark corpus and thresholds | No accepted deep-chain/diamond/failure/cross-package corpus, metric, variance, or environment policy | BlockedSpec | Missing benchmark decision |
| LSP cancellation and stale results | No accepted deadline, cancellation, version, or stale-result behavior | BlockedSpec | `GAP-LSP-TRANSACTION-PROTOCOL-001` |
| Public benchmark evidence protocol | No public timing/result schema or compatibility lifecycle exists | BlockedSpec | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` |

## Evidence contract

`cargo xtask trait-performance verify` validates the exact eight-row matrix,
seven authority/implementation/report files, and two task states:
`TRAIT-1309` is `BlockedSpec` and `TRAIT-1309-TERMINATION` is `Done`.

No benchmark command, timing threshold, allocation or candidate budget,
cancellation API, diagnostic, schema, or public Trait service is implemented
by this inventory. The verifier is deterministic, read-only, and offline. It
does not run a benchmark, parse user source, execute Trait selection, mutate
files, access the network, install software, or change system state.

No language semantics, public diagnostic allocation, schema, Semantic ID,
runtime, bytecode, VM, ABI, or Unicode 17.0.0 behavior changes. Only `ling` and
`.ling` are valid public names.

## Deferred acceptance

The five `BlockedSpec` rows require Accepted production-query, deterministic
resource, benchmark, cancellation, and lifecycle decisions plus executable
fixtures. Timing observations cannot be promoted into public guarantees by
implementation or planning text alone.
