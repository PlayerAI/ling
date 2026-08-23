# IDE-2311 Authority Audit: Workspace Symbols

## Outcome

`IDE-2311` is authorized by Accepted RFC-0045 and implemented as the bounded
Preview `ling.lsp.workspace-symbol/0.1`. The earlier audit correctly stopped
public implementation while workspace scope, matching, ordering, projection,
invalidation, limits, and cancellation were unspecified. RFC-0045 now resolves
exactly those decisions without promoting any broader index, filesystem
discovery, Semantic Transaction, or Stable surface.

## Normative traceability

- RFC-0045 defines the exact provider and Experimental discovery object, the
  `workspace/symbol` request, tracked-workspace scope, query validation,
  resolver-kind mapping, deterministic ordering and truncation, standard wire
  shape, complete-snapshot cache key, cooperative cancellation, freshness, and
  atomic failures.
- RFC-0004 retains JSON-RPC framing and lifecycle behavior. RFC-0023 and
  RFC-0029 define tracked document identities, overlays, and negotiated
  positions. RFC-0030 and RFC-0036 provide the immutable checked-analysis and
  resolved presentation boundaries.
- DEC-0002 preserves original UTF-8 spans; DEC-0012 prevents internal identity
  leakage; DEC-0019 and DEC-0071 define immutable revision/snapshot inputs;
  DEC-0031 authorizes the in-process cooperative token; DEC-0073 and DEC-0082
  define the resolver-owned definition inventory and bounded source lookups.

## Current interface evidence

- Every successful initialize advertises the exact provider and
  `lingWorkspaceSymbols` marker.
- The resolver-backed query includes only non-temporary writable tracked
  workspace sources. Dependency and temporary sources may participate in
  compilation but never appear in results.
- Empty, exact, and case-sensitive prefix queries return the first 256 matches
  in the RFC-0045 total order, with exact matches first.
- Original spans project through UTF-8, UTF-16, or UTF-32; results contain only
  name, mapped kind, module container, workspace URI, and range.
- One disposable complete-snapshot cache is published only after successful,
  fresh, bounded, uncancelled construction. Invalid params use `-32602`,
  cancellation uses `-32800`, and other complete-query failures use `-32803`.

## Acceptance evidence

`crates/ling-lsp/tests/workspace_symbols.rs` covers exact discovery and wire
shape, lifecycle, notifications, malformed and bounded queries, exact/prefix
and case-sensitive matching, every resolver kind, module/URI context,
workspace/dependency/temporary scope, Unicode/BOM/CRLF projection in all three
encodings, insertion order, repeated snapshots, invalidation, deterministic
truncation, cancellation, compiler failure, and recovery. Diagnostic transcript
fixtures pin the expanded initialize response byte-for-byte.

## Evidence and compatibility

The implementation evidence is `crates/ling-lsp/src/workspace_symbols.rs`,
`crates/ling-lsp/tests/workspace_symbols.rs`,
`tests/protocols/lsp-workspace-symbol/README.md`,
`docs/governance/protocol-inventory.toml`, and
`docs/status/IDE-2311-IMPLEMENTATION-REPORT.md`.

This task adds one Preview protocol and one session-local presentation cache.
It changes no `L-*` diagnostic, schema, Semantic ID, canonical bytes, language
semantics, Typed Core evaluation, interpreter, VM, bytecode, ABI, package,
filesystem/network behavior, source truth, or Unicode 17.0.0 tables.

## Intentionally deferred

Dependency/generated/builtin/Prelude results, filesystem discovery, fuzzy or
normalized matching, module-name search, persistent/concurrent indexes, partial
results, work-done progress, resolve, stdio `$/cancelRequest`, Semantic
Transactions, and Stable compatibility remain out of scope.
