# DEC-0057: Formatter in-process edit projection / 格式化器进程内编辑投影

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: formatter-maintainer  
> Related authority/gap: `DEC-0023`, `GAP-FORMATTER-CLI-PROTOCOL-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only a deterministic in-process edit value for the
existing Author Source formatter. It does not define an LSP method, a JSON
schema, a Workspace Edit, a document-version precondition, or a Semantic
Transaction.

## Question

The formatter already has a conservative publication result, but a later
editor adapter needs an explicit source-edit boundary. How can that boundary
be represented without guessing a diff, position encoding, URI, version, or
wire lifecycle that the accepted LSP gaps still leave unresolved?

## Decision

1. `ling-format` exposes `FormatEdit` and `format_core_edit` as an in-process
   library boundary. `FormatEdit` carries the originating `SourceId`, one
   original UTF-8 byte range, and the exact replacement text.
2. For a valid source whose candidate is accepted by the existing formatter
   publication gates, `format_core_edit` returns either no value when bytes are
   unchanged or exactly one replacement covering `0..original_byte_length`.
   The replacement is the already validated formatter result.
3. Invalid sources, rejected candidates, and unchanged valid sources return
   `Ok(None)`. A source whose byte length cannot fit the accepted `u32` span
   unit returns a deterministic `FormatEditError` rather than truncating the
   range.
4. The boundary performs no minimal-diff calculation, range formatting,
   position conversion, URI or document-version handling, stale-edit check,
   transaction state, JSON serialization, transport dispatch, or source-file
   mutation. Future LSP or CLI adapters must obtain separate Accepted
   authority for those behaviors.

## Conformance plan

- Verify changed valid source produces one whole-document edit with the exact
  original byte range and replacement.
- Verify already formatted, invalid, and rejected-candidate inputs produce no
  edit and never partially rewrite a source prefix.
- Verify BOM, CRLF, Unicode text, source identity, repeated execution, and
  replacement bytes remain deterministic.
- Verify the API has no URI, version, position, Workspace Edit, JSON-RPC,
  diagnostic, or transaction fields.

## Compatibility impact

- Adds only an in-process `ling-format` Rust value and function. No Ling
  syntax, semantics, Checked Core, diagnostics, schemas, Semantic IDs,
  canonical Audit bytes, CLI command, LSP wire method, package behavior,
  protocol inventory, or Unicode 17.0.0 data changes.
- The value is not a public protocol and makes no Stable 1.0 editor claim.
  Its range remains in the accepted original UTF-8 byte-span unit.

## Unresolved alternatives

Minimal or token-level diffs, UTF-16/UTF-8/UTF-32 editor positions, URI and
document-version association, stale-edit preconditions, `TextEdit` and
`WorkspaceEdit` schemas, range formatting, format-on-save, and Semantic
Transaction lifecycle remain governed by later accepted formatter and LSP
decisions.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

