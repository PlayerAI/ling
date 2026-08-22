# DEC-0073: Internal resolved-definition index / IDE 内部已解析定义索引

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: compiler-query-design  
> 相关 RFC/缺口：`DEC-0012` | `DEC-0019` | `GAP-REGISTER`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable inventory of user
definitions already emitted by the validated resolver. It does not accept an
LSP `DocumentSymbol` schema, editor symbol kinds, hierarchy, locations, or
document lifecycle.

本决定只授权对已通过验证的 resolver 结果建立进程内不可变用户定义索引，不接受
LSP `DocumentSymbol` schema、编辑器 symbol kind、层级、位置或文档生命周期。

## Question

`IDE-2301` needs a deterministic semantic source for future document-symbol
work, but the public LSP symbol contract remains open. The resolver already
owns stable definition identities, original UTF-8 spans, normalized names, and
member classifications. Those facts can be indexed without choosing any
presentation or transport policy.

## Decision

1. `ling-db` may expose `ResolvedDefinitionIndex`, an owned immutable
   collection of user definitions copied from one validated `ResolvedProgram`.
   Builtins and Prelude entries are excluded; entries without the resolver's
   source name or span are omitted rather than synthesized.
2. Each record preserves the existing definition ID, module name, normalized
   name, source spelling, mutability, logical source name, and original
   `Span`. The span is not converted, clamped, or split into a selection range.
3. The existing resolver member tables classify records as ordinary values,
   types, constructors, Trait members, or implementation members. No new
   language namespace, identity, or symbol-kind rule is introduced.
4. Records are sorted deterministically by logical source name, source ID,
   original start/end byte offsets, classification, source name, and definition
   ID. Repeated construction from an equal resolver result produces equal
   values and never observes host paths, allocation addresses, or map order.
5. The index is an in-process compiler query only. It contains no URI,
   document version, negotiated position encoding, LSP range, hierarchy,
   documentation, cancellation, JSON, publication, or stale-result state. The
   public `IDE-2301` parent remains `BlockedSpec`.

## Conformance plan

- Build an index for Unicode, leading-BOM, and CRLF source and compare every
  stored span against the exact original UTF-8 bytes and resolver source ID.
- Cover ordinary values, types, variant constructors, Trait members, and
  implementation members; verify source-order output, classification, and
  deterministic repeated results.
- Reject invalid source input without publishing a partial index and verify
  source-specific lookup does not expose entries from another logical file.
- Keep LSP symbol ranges, hierarchy/flat fallback, URI/version association,
  generated/dependency policy, position conversion, cancellation, and JSON-RPC
  fixtures deferred to the accepted parent protocol decisions.

## Compatibility impact

- Adds only internal compiler-query values and a read-only accessor. Ling
  syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI output,
  LSP wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 data are
  unchanged.
- Existing resolver IDs and spans are copied, not re-hashed or reinterpreted;
  no protocol-inventory entry or Stable 1.0 editor claim is introduced.

## Unresolved alternatives

LSP symbol-kind mapping, full versus selection ranges, nested hierarchy,
package/dependency and generated-file policy, URI/version/snapshot identity,
position encoding, documentation/detail rendering, limits, cancellation,
rename/reference linkage, protocol negotiation, and migration remain open
under `IDE-2301`, `GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

