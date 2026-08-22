# DEC-0080: Internal completion checked-metadata observation / IDE 内部补全已检查元数据观察

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0010` | `DEC-0012` | `DEC-0019` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable observation joining
existing checked type, Effect Row, and module Capability facts to resolver-backed
definitions and bindings. It does not accept a completion-item presentation,
documentation, signature formatter, capability disclosure policy, or insertion
edit.

本决定只授权将现有已检查的类型、Effect Row、模块 Capability 事实与
resolver 定义和绑定关联，形成进程内不可变观察；不接受补全 item 展示、文档、
签名格式化、能力披露策略或插入编辑。

## Question

`IDE-2308` needs checked facts for future completion-item resolve work, while
the public handle, display, redaction, and insertion contracts remain open.
`ling-types` and `ling-effects` already produce deterministic facts for the
validated program. They can be copied without inventing presentation policy.

## Decision

1. `ling-db` may expose `ResolvedCompletionMetadataIndex`, built only from one
   validated `CheckedProgram`.
2. The bounded inventory covers resolver-backed user definitions and local or
   parameter bindings that retain an original source span. Import aliases are
   source inventory facts under DEC-0079 and are not assigned fabricated type
   metadata here.
3. Each entry preserves the existing module, normalized name, logical source
   name, original `Span`, and resolver identity. Optional checked facts are
   copied as optional values: `TypedProgram::display_type`, canonical Effect
   names, and canonical module Capability names. Missing facts remain absent;
   no placeholder or inferred presentation value is created.
4. Entries are sorted deterministically by logical source name, source ID,
   original start/end byte offsets, module ID, normalized name, source kind,
   and existing identity. Repeated construction from equal checked input is
   equal and independent of host paths, allocation addresses, or map order.
5. The index provides only read-only source/name/identity lookup. It does not
   render documentation or full signatures, disclose capabilities to an
   editor, redact or localize fields, choose a candidate, generate insertion
   text, convert positions, bind snapshots or versions, handle cancellation,
   serialize JSON, or publish a protocol response. The public `IDE-2308`
   parent remains `BlockedSpec`.

## Conformance plan

- Cover a checked definition and a local/parameter binding, exact original
  Unicode/BOM/CRLF spans, existing resolver identities, type display, optional
  canonical effects, optional capabilities, repeated equality, and identity or
  source lookup.
- Preserve absent checked facts without placeholders and reject invalid source
  before publishing an index value.
- Keep resolve handles, documentation safety/localization, full signature
  grammar, capability redaction, insertion text, formatter interaction,
  position/version/snapshot/stale behavior, limits, cancellation, and protocol
  fixtures deferred.

## Compatibility impact

- Adds only internal `ling-db` metadata values and a read-only query. Ling
  syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI/LSP wire
  behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 tables remain
  unchanged.
- Existing checked facts and resolver identities are copied, not re-hashed or
  promoted to a presentation or protocol identity.

## Unresolved alternatives

Completion-item handles/lifetimes, documentation source and localization,
signature grammar, Effect/Capability rendering and redaction, profile labels,
insertion/edit and formatter policy, URI/version/position/snapshot/cancellation
and stale behavior, dependency/generated/builtin policy, protocol lifecycle,
and migration remain open under `IDE-2308` and the registered LSP/semantic/
formatter gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
