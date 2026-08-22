# DEC-0077: Internal rename-identifier Unicode observation / IDE 内部重命名标识符 Unicode 观察

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0019` | `GAP-UNICODE-ALIAS-SYNTAX-001` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, read-only observation of the
Unicode facts for a raw string that a future rename request might propose. It
does not authorize prepare-rename, name acceptance, target selection, ranges,
diagnostics, edits, or any LSP/protocol field.

本决定只授权对未来重命名请求可能提供的原始字符串进行进程内、只读的 Unicode 事实观察，
不授权 prepare-rename、名称接受、目标选择、范围、诊断、编辑或任何 LSP/协议字段。

## Question

`IDE-2305` needs a reusable compiler-owned boundary for Unicode facts, while
its public target/range, keyword, alias, collision, version, and edit
contracts remain blocked. The lexer and resolver already use the Unicode 17
XID/NFC/security implementation, so copying those facts into an owned
observation does not invent a rename policy.

## Decision

1. `ling-db` may expose `observe_rename_identifier` and the immutable
   `RenameIdentifierObservation` value for in-process consumers.
2. The observation delegates validation and security computation to
   `ling-unicode::inspect_identifier`, preserving the existing Unicode
   17.0.0 XID and forbidden-character errors. Invalid input is returned as the
   existing `IdentifierError`; no new diagnostic code is allocated.
3. A successful observation retains the raw spelling, NFC-normalized name,
   UTS #39 skeleton, sorted Script and Identifier_Type names, Identifier_Status,
   and suspicious mixed-script flag. The raw spelling is retained for audit;
   the normalized name remains the only name-equality form.
4. `Allowed`, `Restricted`, and suspicious mixed-script facts are observations,
   not acceptance or rejection decisions. The function does not classify
   keywords, select a definition or binding, inspect aliases/collisions,
   evaluate visibility/coherence, or compare snapshots and versions.
5. The value has no source span, URI, document version, position encoding,
   placeholder, edit, cancellation, persistence, cache key, JSON, or JSON-RPC
   representation. The public `IDE-2305` prepare-rename task remains
   `BlockedSpec`.

## Conformance plan

- Verify ASCII, Chinese, decomposed/NFC, CRLF/BOM-adjacent, confusable mixed
  script, and invalid XID/forbidden-character inputs against the existing
  Unicode implementation.
- Compare repeated observations for exact equality and assert sorted script
  facts, raw spelling preservation, normalized equality, and no partial result
  on invalid input.
- Keep keyword/raw-identifier, builtins, target eligibility, aliases,
  collisions, visibility/coherence, ranges, snapshots, stale requests,
  diagnostics, edits, and protocol fixtures deferred.

## Compatibility impact

- Adds only internal `ling-db` observation types and a pure read-only helper;
  language syntax, name resolution, diagnostics, schemas, Semantic IDs, CLI
  output, LSP wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0
  tables are unchanged.
- The helper copies existing Unicode facts and does not re-hash or reinterpret
  DefinitionId, BodyId, ProgramId, source spans, or semantic graph data. No
  protocol-inventory entry or Stable 1.0 rename claim is introduced.

## Unresolved alternatives

Keyword and raw-identifier policy, target/range selection, aliases and
confusable collisions, builtins/generated/dependency mutability, visibility and
coherence, diagnostics/localization, snapshot/version preconditions, position
conversion, cancellation, stale publication, Workspace Edit semantics,
DefinitionId migration, protocol negotiation, and Semantic Graph lifecycle
remain open under `IDE-2305`, `IDE-2306`, `GAP-UNICODE-ALIAS-SYNTAX-001`,
`GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

