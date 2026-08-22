# DEC-0074: Internal typed-definition observation / IDE 内部类型定义观测

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: compiler-query-design
> 相关 RFC/缺口：`DEC-0012` | `DEC-0019` | `GAP-REGISTER`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only an in-process, immutable observation of exact
type/effect/capability facts already produced by a successful checked program.
It does not accept an LSP hover request/response schema or a presentation
format.

本决定只授权对成功检查的程序已经产生的类型、Effect 和 Capability 事实建立进程内
不可变观测，不接受 LSP hover 请求/响应 schema 或展示格式。

## Question

`IDE-2302` needs a checked source for future hover work, while the public
hover contract remains open. `ling-types` and `ling-effects` already retain
the facts, and the resolver retains their source identity and original spans.
Those facts can be joined without inventing display, transport, or lifecycle
semantics.

## Decision

1. `ling-db` may expose `TypedDefinitionIndex`, an owned immutable collection
   built only from a validated `CheckedProgram` and its resolver result.
   Builtins and Prelude entries are excluded; definitions without a resolver
   source name or span are omitted rather than synthesized.
2. Each record preserves the existing definition ID, module/name spelling,
   resolver classification, mutability, logical source name, and original
   UTF-8 `Span`. It carries optional exact type display, effect labels, and
   module capability names; absent facts remain absent and are never guessed.
3. Type text is the existing `TypedProgram::display_type` result and effect
   labels are the existing canonical `EffectRow` names. Capability names are
   the existing checked module capability names. No new type, Effect,
   Capability, namespace, or Semantic ID is introduced.
4. Records are sorted deterministically by logical source name, source ID,
   original start/end byte offsets, classification, source spelling, and
   definition ID. Equal checked inputs produce equal observations independent
   of map order, allocation addresses, host paths, or display locale.
5. The observation is an in-process compiler query only. It contains no hover
   markup, Markdown, documentation, URI, document version, negotiated
   position encoding, LSP range, cancellation, JSON, publication, stale
   result, or request lifecycle state. The public `IDE-2302` parent remains
   `BlockedSpec`.

## Conformance plan

- Build Unicode, BOM, and CRLF observations and compare all stored spans with
  exact original UTF-8 bytes and resolver source IDs.
- Verify inferred/declared type text, canonical effect labels, module
  capability names, user-only filtering, source ordering, and deterministic
  repeated results.
- Preserve missing optional facts without inventing placeholders and reject
  invalid source or failed checking without publishing an observation.
- Keep hover display policy, selected-expression lookup, Markdown/localization,
  URI/version/snapshot binding, position conversion, cancellation, stale
  publication, and JSON-RPC fixtures deferred to accepted parent authority.

## Compatibility impact

- Adds only internal compiler-query values and a read-only accessor. Ling
  syntax, language semantics, diagnostics, schemas, Semantic IDs, CLI output,
  LSP wire behavior, runtime, bytecode, VM, ABI, and Unicode 17.0.0 data are
  unchanged.
- Existing resolver IDs, spans, typed display text, effect rows, and checked
  capabilities are copied, not re-hashed or reinterpreted. No protocol
  inventory entry or Stable 1.0 hover claim is introduced.

## Unresolved alternatives

Hover target selection, expression-level inference, declared-versus-inferred
policy, documentation/detail rendering, Markdown safety, localization,
related Semantic IDs/Trait witnesses, URI/version/snapshot identity,
position encoding, limits, cancellation, stale-result handling, protocol
negotiation, and migration remain open under `IDE-2302`,
`GAP-LSP-TRANSACTION-PROTOCOL-001`, and
`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
