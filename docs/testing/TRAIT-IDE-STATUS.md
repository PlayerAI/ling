# Trait IDE Current-Surface Inventory / Trait IDE 当前表面盘点

## Scope

This inventory distinguishes the accepted data-only Trait projection and its
in-process lookups from unimplemented public editor behavior. The full
`TRAIT-1308` task remains `BlockedSpec`. Any future editor operation must
consume the checked immutable witness and must not re-run Trait selection.

本盘点区分已接受的 Trait 数据投影及进程内查找，与尚未实现的公开编辑器行为。
完整 `TRAIT-1308` 继续保持 `BlockedSpec`。

## Current surface matrix

| Trait IDE surface | Current evidence | State | Authority / blocker |
| --- | --- | --- | --- |
| Trait Semantic Graph projection | Optional `x-ling-trait-ide` witness/member data, deterministic IDs, original byte spans, and strict reader tests | Experimental | Accepted RFC-0022 |
| Projection identity lookups | Four immutable in-process identity filters over validated projection order | Internal | Accepted DEC-0059 |
| Trait hover | No accepted rendering, position, snapshot, or response contract | BlockedSpec | `GAP-LSP-TRANSACTION-PROTOCOL-001` |
| Go to Trait or implementation | No accepted LSP method/position/version mapping or cross-package editor fixture | BlockedSpec | `GAP-TRAIT-COHERENCE-001` and transaction authority |
| Trait completion | No accepted candidate, ranking, resolve, or stale-document contract | BlockedSpec | `GAP-TRAIT-COHERENCE-001` and transaction authority |
| Identity-preserving Trait rename | No accepted edit planning, version guard, conflict, or atomicity contract | BlockedSpec | `GAP-LSP-TRANSACTION-PROTOCOL-001` |
| Trait diagnostics and repairs | No allocated bilingual Trait codes or accepted safe repair facts | BlockedSpec | RFC-0005 §5 and `docs/ERROR-CODES.md` |
| Trait LSP transactions and versions | No accepted Trait request/response lifecycle or Stable compatibility contract | BlockedSpec | `GAP-LSP-TRANSACTION-PROTOCOL-001` and `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` |

## Evidence contract

`cargo xtask trait-ide verify` checks the exact eight-row matrix, seven
authority/implementation/test/report files, and the status relation:
`TRAIT-1308` is `BlockedSpec`, while `TRAIT-1308-PROJECTION` and
`TRAIT-1308-QUERY` are `Done`. Evidence drift fails closed.

No hover, navigation request, completion, rename, diagnostic, repair,
JSON-RPC method, Workspace Edit, or Semantic Transaction is implemented by
this inventory. The verifier is deterministic, read-only, and offline. It
does not parse user source, run Trait selection, execute code, mutate files,
access the network, install software, or change system state.

No language semantics, public diagnostic allocation, core schema, Semantic
ID, runtime, bytecode, VM, ABI, or Unicode 17.0.0 behavior changes. Only
`ling` and `.ling` are valid public names.

## Deferred acceptance

The six `BlockedSpec` rows require Accepted protocol and diagnostic authority,
plus positive, negative, deterministic, cross-package, stale-document,
transaction, rename, and semantic/editor differential fixtures. Internal Rust
lookups and execution-plan text are not substitutes for those contracts.
