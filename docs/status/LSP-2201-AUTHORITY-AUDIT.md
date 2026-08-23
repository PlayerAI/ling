# LSP-2201 Authority Audit: Compiler diagnostic adapter

## Outcome

`LSP-2201` is implementation-ready under Accepted `RFC-0031`. The RFC closes
the adapter-specific portion of `GAP-LSP-TRANSACTION-PROTOCOL-001` by defining
an Experimental pure in-process mapping. It deliberately does not authorize
publication, document-version association, stale-result behavior, debounce,
suppression, caps, or repair application.

Accepted DEC-0034 and DEC-0072 remain the bounded ordering and position
primitives consumed by the adapter. Accepted RFC-0004, RFC-0023, RFC-0029, and
RFC-0030 supply the already implemented lifecycle, URI, negotiated encoding,
overlay, and workspace foundations without making them part of this output.

## Normative traceability

- `docs/SEMANTICS.md` §26, `docs/ERROR-CODES.md`, and DEC-0001 require stable
  registered codes, bilingual messages, structured Facts/repairs, and
  deterministic diagnostic identity.
- DEC-0002 preserves original UTF-8 byte spans; DEC-0029 and DEC-0072 require
  strict explicitly encoded SourceMap projection without clamping.
- DEC-0034 defines the canonical path-free
  `(logical source, start, code, end, ordinal)` order.
- RFC-0031 defines the exact `ling.lsp.diagnostic/0.1` source/input boundary,
  LSP JSON fields, severity mapping, related-information representation,
  versioned data object, typed failures, and compatibility policy.
- `PROTO-DIAGNOSTIC-JSON` remains a separate Preview writer. RFC-0031 does not
  alter it or embed adapter-only related labels in the compiler schema.

## Implemented boundary

- The adapter accepts a non-empty set of unique validated non-temporary Ling
  URI/`SourceFile` identities and existing `Diagnostic` inputs.
- Every diagnostic requires an exact primary source span; explicit related
  labels use the same strict projection and preserve supplied semantic order.
- Output retains code, Chinese and English text, severity, Facts, Semantic ID,
  repairs, URI, and negotiated range in the exact RFC-0031 shape.
- Any source, identity, span, or position failure returns a typed error and no
  partial adapted set.
- `PROTO-LSP-DIAGNOSTIC` records the current-writer-only Experimental marker;
  the adapter is not a JSON-RPC method and performs no mutation or I/O.

## Evidence and compatibility

The conformance test covers exact JSON keys and values, all severities,
multiple repairs, related sources, UTF-8/16/32, Chinese, emoji, combining
marks, BOM, CRLF, deterministic ordering/serialization, and all specified
failure classes including an invalid later input. No language semantics,
diagnostic allocation, core diagnostic schema, source-span identity, runtime,
bytecode, VM, ABI, filesystem/network behavior, or Unicode 17.0.0 data changes.

## Intentionally deferred

LSP-2202 owns push publication, trigger/debounce, result replacement/clearance,
snapshot and document versions, cancellation, and stale-result rejection.
LSP-2203 owns pull diagnostics. LSP-2204 owns deduplication, root-cause/error-
storm caps, suppression, and omission reporting. LSP-2205 owns integration
evidence. Workspace Edits, Semantic Transactions, code-description URLs,
tags, repair application, and Stable compatibility require separate authority.
