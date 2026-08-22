# DEC-0076: Internal resolved-reference reverse index / IDE 内部已解析引用反向索引

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0019` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable target-to-source
reverse lookup built from the existing resolver reference inventory. It does
not accept relation taxonomy, incremental persistence, or an LSP references
request/response.

本决定只授权基于现有 resolver 引用索引建立进程内不可变的目标到源引用反向查找，不接受
关系分类、增量持久化或 LSP references 请求/响应。

## Question

`IDE-2304` needs a reusable internal reverse lookup for future references and
rename work, but the relation list and incremental protocol remain
unspecified. The forward index already retains resolver target keys and source
reference identities. Grouping those exact facts does not require a new
semantic relation or lifecycle policy.

## Decision

1. `ling-db` may expose `ResolvedReferenceReverseIndex`, an owned immutable
   grouping of forward-index entries by the existing definition ID or binding
   module/local identity.
2. Each grouped source preserves logical source name, source module identity,
   and resolver reference ID. Target keys are copied and are not converted to
   Semantic IDs, URI locations, or editor ranges.
3. The reverse index performs no `read`, `write`, `call`, `type`, or
   `implementation` classification, overlap precedence, declaration policy,
   generated/dependency policy, or unresolved-target synthesis.
4. Target groups and their source lists are sorted deterministically by the
   existing resolver key and source/module/reference identity. Equal forward
   inputs produce equal reverse observations independent of map order, host
   paths, allocation addresses, or locale.
5. The observation is an in-process compiler value only. It has no revision
   cache, dependency invalidation, persistence, corruption recovery, resource
   limit, URI/version, cancellation, publication, stale-result, JSON, or
   JSON-RPC state. The public `IDE-2304` parent remains `BlockedSpec`.

## Conformance plan

- Group definition and binding targets from Unicode, BOM, CRLF, imported, and
  repeated-reference inputs and compare source identities and reference IDs.
- Repeat construction and verify deterministic target grouping, source order,
  empty/missing lookup behavior, and equality with the forward index.
- Keep relation taxonomy, source ranges, incremental edits, dependency
  invalidation, persistence, corruption, request positions, URI/version,
  cancellation, stale results, and JSON-RPC fixtures deferred.

## Compatibility impact

- Adds only internal compiler-query values and a read-only accessor. Ling
  syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI output,
  LSP wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 data are
  unchanged.
- Existing resolver target identities and source references are copied, not
  re-hashed or reinterpreted. No protocol-inventory entry or Stable 1.0
  references/rename claim is introduced.

## Unresolved alternatives

Relation taxonomy, overlap/precedence, source reference spans, declaration
versus use semantics, target eligibility, revision keys, dependency
propagation, invalidation, persistence/corruption, resource limits,
URI/version/snapshot binding, position conversion, cancellation, stale
publication, protocol negotiation, and migration remain open under
`IDE-2304`, `GAP-INCREMENTAL-CACHE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
and `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
