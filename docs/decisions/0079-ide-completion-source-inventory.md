# DEC-0079: Internal completion-source inventory / IDE 内部补全来源清单

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0002` | `DEC-0012` | `DEC-0019` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable inventory of
resolver-backed name sources that can be used by future completion analysis.
It does not accept completion contexts, candidate visibility, ranking,
insertion text, or an LSP completion protocol.

本决定只授权对 resolver 已确认的名称来源建立进程内不可变清单，供未来补全
分析使用；不接受补全上下文、候选可见性、排序、插入文本或 LSP 补全协议。

## Question

`IDE-2307` needs a deterministic compiler-side source for future completion
work, while its public request/response and ranking contracts remain open. The
validated resolver already owns user definitions, local bindings, import
aliases, identities, and original UTF-8 spans. Those facts can be copied
without inventing a completion policy.

## Decision

1. `ling-db` may expose `ResolvedCompletionSourceIndex`, an owned immutable
   collection built only from one validated `ResolvedProgram`.
2. The inventory includes resolver-backed user definitions with a source span,
   local/parameter bindings, and resolved import aliases. Non-user definitions
   without an original source span, including builtins and Prelude entries, are
   omitted rather than synthesized.
3. Each entry preserves its owning resolver module, normalized name, logical
   source name, original `Span`, source kind, and existing resolver identity:
   `DefinitionId`, `(ModuleId, BindingId)`, or resolved import target module.
   No identity is re-hashed, merged, or promoted to a Semantic ID.
4. Entries are sorted deterministically by logical source name, source ID,
   original start/end byte offsets, module ID, normalized name, source kind,
   and existing identity. Repeated construction from equal resolver input is
   equal and independent of host paths, allocation addresses, or map order.
5. The index provides only read-only source/module/name lookups. It performs no
   context classification, scope-distance calculation, type/effect/capability
   ranking, visibility or privacy decision, duplicate suppression, keyword or
   alias policy, insertion-text generation, position conversion, snapshot or
   version binding, cancellation, JSON, or publication. The public `IDE-2307`
   parent remains `BlockedSpec`.

## Conformance plan

- Cover user definitions, local/parameter bindings, explicit import aliases,
  Unicode names, leading BOM, CRLF, exact original alias/name spans, and
  resolver-backed identities.
- Compare repeated construction and source/module/name lookups, and verify
  that missing source spans or unresolved import records do not produce
  synthetic entries.
- Keep the six completion contexts, visibility, scope proximity, type/effect/
  capability fit, ranking, duplicate policy, builtins/Prelude/dependency and
  generated-symbol policy, insertion edits, request positions, stale versions,
  limits, cancellation, and protocol fixtures deferred.

## Compatibility impact

- Adds only internal `ling-db` source-inventory values and a read-only query.
  Ling syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI/LSP
  wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 tables remain
  unchanged.
- Existing resolver identities and original spans are copied as evidence; no
  protocol-inventory entry or Stable 1.0 completion claim is introduced.

## Unresolved alternatives

Completion context grammars, candidate visibility and ranking, type/effect/
capability fit, builtins/Prelude/alias/localization policy, generated and
dependency documents, insertion text and formatter interaction, request
snapshot/version/position/cancellation/stale behavior, resource limits,
Semantic Graph lifecycle, protocol negotiation, and migration remain open
under `IDE-2307` and the registered LSP/semantic lifecycle gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
