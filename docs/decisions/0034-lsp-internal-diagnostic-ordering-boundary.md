# DEC-0034: LSP internal diagnostic ordering boundary / LSP 内部诊断排序边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0001`, `DEC-0002`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only a path-free, byte-span-based ordering key for
future LSP diagnostic projection. It does not define an LSP diagnostic schema,
severity mapping, publication lifecycle, or error-storm policy.

## Question

An eventual LSP adapter needs stable ordering before it can publish diagnostics,
but LSP-2201 and LSP-2204 lack accepted field, snapshot, suppression, and
publication contracts. DEC-0001 and DEC-0002 already require stable diagnostic
codes and original UTF-8 byte spans, so a neutral internal key can be fixed
without deciding any editor-visible behavior.

## Decision

1. `ling_lsp` may contain an internal `DiagnosticOrderKey` containing only a
   logical file/URI string, original UTF-8 start and end byte offsets, a stable
   diagnostic-code string, and an explicit local tie-breaker.
2. Canonical ordering compares logical file bytes, start byte, code bytes, end
   byte, and tie-breaker in that order. The key never converts or normalizes
   byte spans, uses map order, or relies on severity, localized messages,
   allocation identity, host paths, or process timing.
3. The key is `pub(crate)` and is an ordering value only. It carries no request
   or document version, snapshot identity, negotiated position, diagnostic
   facts/repairs, Workspace Edit, cancellation, cap, suppression marker, or
   JSON representation.
4. `LspServer`, compiler diagnostics, the stdio transport, and protocol
   inventory remain unchanged. The key is not a diagnostic adapter and must not
   be used to publish or suppress results until the parent LSP-2201/LSP-2204
   authorities are Accepted.

## Conformance plan

- Verify file, start-byte, code, end-byte, and tie-breaker ordering at equal
  and distinct spans, including Unicode logical names and CRLF byte offsets.
- Verify equal logical facts remain distinguishable only by the explicit local
  tie-breaker and that repeated key sequences sort identically.
- Verify the primitive contains no position conversion, severity, message,
  facts, repairs, request/version, transport, cap, suppression, or JSON state.

## Compatibility impact

- Adds only `pub(crate)` Rust ordering values in `ling-lsp`; source syntax,
  semantics, diagnostics, schemas, Semantic IDs, source spans, CLI, LSP wire
  methods, bytecode, VM, protocol inventory, support matrix, and Unicode
  17.0.0 behavior remain unchanged.
- Original UTF-8 byte spans remain the only numeric span identity; no LSP
  position encoding or migration contract is introduced.

## Unresolved alternatives

Severity/tag mapping, related information, localization, URI policy, position
projection, version/snapshot association, root-cause and dependent ordering,
deduplication, caps/truncation, clear/replace, suppression, cancellation,
publication, and Stable versus Experimental lifecycle require later Accepted
LSP-2201/LSP-2204 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
