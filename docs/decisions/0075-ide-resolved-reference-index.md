# DEC-0075: Internal resolved-reference index / IDE 内部已解析引用索引

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0012` | `DEC-0019` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable inventory of resolver
reference keys and their existing definition or binding targets. It does not
accept an LSP definition-navigation request/response, source URI policy, or
editor position mapping.

本决定只授权对 resolver 引用键及其已有定义或绑定目标建立进程内不可变索引，不接受
LSP 定义导航请求/响应、源 URI 策略或编辑器位置映射。

## Question

`IDE-2303` needs a deterministic internal bridge from resolved references to
target identities before any navigation protocol can be designed. The resolver
already owns reference keys, target kinds, definition IDs, binding keys, and
target source spans. Those facts can be copied without selecting a request or
transport contract.

## Decision

1. `ling-db` may expose `ResolvedReferenceIndex`, an owned immutable collection
   copied from one validated `ResolvedProgram`. Each entry preserves the
   source module/logical source name, resolver reference ID, and the existing
   target kind.
2. Definition targets preserve the existing `DefinitionId`, optional resolver
   name/source name/span, and no synthesized location. Binding targets preserve
   the existing module/binding IDs, binding name, logical source name, and
   original UTF-8 `Span`.
3. Missing resolver metadata is represented as absent or causes the malformed
   entry to be omitted; no path, URI, source range, declaration/type-definition
   distinction, dependency policy, or generated-document target is invented.
4. Entries are sorted deterministically by source logical name, source module
   ID/name, reference ID, target kind, target identity, and target span/name.
   Equal resolver inputs produce equal observations independent of map order,
   host paths, allocation addresses, or locale.
5. The index is an in-process compiler observation only. It contains no request
   position, negotiated encoding, URI, document version, snapshot pin,
   cancellation, limits, publication, stale-result state, JSON, or JSON-RPC.
   The public `IDE-2303` parent remains `BlockedSpec`.

## Conformance plan

- Cover user-definition, builtin, Prelude, local-binding, imported, Unicode,
  BOM, and CRLF references and compare every target span with original UTF-8
  bytes and resolver source identity.
- Repeat construction from equal resolved results and verify source/module/
  reference ordering, target-kind preservation, and source-local lookup.
- Keep request-position lookup, URI/package scope, dependency/generated/
  primitive policy, declaration/type-definition semantics, cancellation,
  stale-result handling, and JSON-RPC fixtures deferred.

## Compatibility impact

- Adds only internal compiler-query values and a read-only accessor. Ling
  syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI output,
  LSP wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 data are
  unchanged.
- Existing resolver identities and spans are copied, not re-hashed or
  reinterpreted. No protocol-inventory entry or Stable 1.0 navigation claim is
  introduced.

## Unresolved alternatives

Reference source-span lookup, request target/position conversion, URI/version
and snapshot binding, declaration versus type-definition policy, aliases,
constructors, builtins/Prelude/primitives, generated/dependency documents,
limits, cancellation, stale publication, Semantic ID presentation, protocol
negotiation, and migration remain open under `IDE-2303`,
`GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
