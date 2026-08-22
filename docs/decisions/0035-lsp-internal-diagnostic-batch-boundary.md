# DEC-0035: LSP internal diagnostic batch boundary / LSP 内部诊断批次边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0034`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only an internal immutable collection boundary for
diagnostic work. It does not define `publishDiagnostics`, snapshot/version
association, clearing, suppression, or an LSP diagnostic schema.

## Question

Future LSP diagnostic work needs to collect already-created diagnostic facts
and order them deterministically before an adapter exists. LSP-2202 remains
blocked on public trigger, freshness, publication, and lifecycle contracts;
DEC-0034 supplies an internal ordering key but does not define batch ownership.

## Decision

1. `ling_lsp` may contain an internal `DiagnosticBatch` that owns opaque `u64`
   diagnostic IDs together with `DiagnosticOrderKey` values authorized by
   DEC-0034. The batch is `pub(crate)` and is not a wire or editor API.
2. `finish` consumes the batch, sorts items by the DEC-0034 key, and returns an
   immutable boxed slice. Equal keys remain distinct in insertion order; the
   batch does not deduplicate, suppress, truncate, or reinterpret diagnostics.
3. The batch performs no position conversion, JSON serialization, severity or
   message mapping, URI/version association, snapshot capture, cancellation,
   timer, publication, or host-memory observation. It stores no partial-result
   or clear/replace state.
4. `LspServer`, compiler diagnostics, the stdio transport, protocol inventory,
   and support matrix remain unchanged. Public push/pull diagnostics and
   lifecycle behavior require Accepted authority for the parent LSP-2202 task.

## Conformance plan

- Verify an empty batch finishes as an empty immutable collection.
- Verify diagnostic IDs are ordered by the DEC-0034 key, duplicate keys remain
  distinct and insertion-stable, and repeated batches are byte/order stable.
- Verify finish consumes ownership and no JSON, request/version, position,
  cancellation, suppression, truncation, or publication surface exists.

## Compatibility impact

- Adds only `pub(crate)` Rust collection values in `ling-lsp`; source syntax,
  semantics, diagnostics, schemas, Semantic IDs, source spans, CLI, LSP wire
  methods, bytecode, VM, protocol inventory, support matrix, and Unicode
  17.0.0 behavior remain unchanged.
- No diagnostic code, schema, migration, or public capability is introduced.

## Unresolved alternatives

Public diagnostic trigger/debounce, push/pull selection, snapshot/version
association, URI/range projection, severity/tags, clear/replace, suppression,
caps/truncation, cancellation/stale precedence, localization, and Stable versus
Experimental lifecycle require later Accepted LSP-2202/LSP-2203/LSP-2204
authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
