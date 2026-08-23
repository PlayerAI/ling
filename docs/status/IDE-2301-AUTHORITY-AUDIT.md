# IDE-2301 authority audit: Document symbols

## Outcome

`IDE-2301` is authorized by Accepted RFC-0036. The RFC closes the previously
recorded public-protocol gap for the bounded Preview
`textDocument/documentSymbol` surface while retaining Accepted DEC-0073 as the
narrower internal resolved-definition-index authority.

The implementation may expose only RFC-0036's fixed hierarchical
`DocumentSymbol` or flat `SymbolInformation` projections. It must not infer
local bindings, publish Semantic IDs, invent unaccepted symbol kinds, or imply
support for edits, transactions, progress, partial results, or Stable
compatibility.

## Normative traceability

- RFC-0036 §§1–2 fixes capability negotiation, discovery, Ready-state request
  behavior, immutable request snapshots, exact URI identity, temporary-source
  isolation, freshness, and failure atomicity.
- RFC-0036 §§3–4 fixes the compiler outline taxonomy, original UTF-8 spans,
  hierarchy, 4096-node bound, display names, and deterministic source order.
- RFC-0036 §§5–7 fixes hierarchical and flat wire fields, UTF-8/16/32 range
  projection, failure codes/messages, compatibility, and migration policy.
- Accepted RFC-0004, RFC-0023, RFC-0029, and RFC-0030 remain authoritative for
  JSON-RPC lifecycle, overlays, position encoding, and LSP framing.
- Accepted DEC-0012 keeps Semantic IDs path-free and separate from
  presentation; DEC-0019, DEC-0071, and DEC-0073 provide the compiler query,
  immutable snapshot, and resolved-definition boundaries composed here.

## Scope resolution

The earlier audit identified missing decisions for kinds, hierarchy, ranges,
URI/snapshot binding, limits, errors, and lifecycle. RFC-0036 now fixes those
items narrowly:

1. module/type/member/value/Trait/implementation structures have exact
   compiler and LSP kind mappings;
2. hierarchy, flat fallback, full versus selection ranges, display names, and
   ordering are explicit;
3. requests use one exact current snapshot and negotiated source projection,
   with isolated temporary documents and stale-result rejection;
4. malformed params use InvalidParams, while compiler/projection/limit/stale
   failures use one fixed bilingual RequestFailed result; and
5. the Preview marker, discovery object, protocol inventory, migration rule,
   and executable conformance boundaries are registered.

Generated/dependency-only symbols, local bindings, inferred presentation,
documentation, tags, cancellation channels, dynamic registration, background
work, edits, transactions, and Stable lifecycle remain deliberately outside
the authorized surface.

## Evidence and compatibility

The implementation evidence is recorded in
`docs/status/IDE-2301-IMPLEMENTATION-REPORT.md`, the protocol fixture README,
the compiler/LSP tests, and the exact diagnostic transcript migration. No
compiler diagnostic code, language semantics, Typed Core, Semantic ID schema,
runtime, bytecode, VM, ABI, or Unicode 17.0.0 data changes are authorized or
introduced by this decision.

## Historical note

Before RFC-0036 was Accepted, this audit correctly held the public task at
`BlockedSpec` and allowed only `IDE-2301-INDEX` under DEC-0073. That historical
block no longer applies to the exact RFC-0036 Preview surface; it continues to
protect all deferred editor behavior from accidental implementation.
