# IDE-2311 implementation report

## Result

IDE-2311 is complete for Accepted RFC-0045's bounded workspace-symbol Preview.
`ling.lsp.workspace-symbol/0.1` exposes deterministic resolver-backed search
over one immutable tracked-workspace snapshot with exact snapshot reuse,
original-position projection, a 256-result cap, cooperative cancellation, and
atomic failure behavior.

## Normative clauses covered

- RFC-0045 §1: exact standard provider and Experimental discovery marker.
- RFC-0045 §2: request-only method, Ready lifecycle, required bounded query,
  unknown-member policy, and notification silence.
- RFC-0045 §3: complete `RequestSnapshot`, tracked writable non-temporary
  workspace scope, exact URI/module context, and `ResolvedDefinitionIndex`.
- RFC-0045 §4: one session-local complete-snapshot cache, equal-key reuse,
  deterministic invalidation, and publish-only-on-success behavior.
- RFC-0045 §5–6: exact/prefix matching, kind mapping, total order, deterministic
  truncation, standard exact wire shape, and UTF-8/16/32 source projection.
- RFC-0045 §7: cooperative cancellation checkpoints, freshness recapture,
  1 MiB response bound, fixed bilingual failures, and no partial publication.

## Implementation and tests

`crates/ling-lsp/src/workspace_symbols.rs` owns request validation, immutable
plan construction, the one-entry cache, query selection, ordering, truncation,
position projection, and fixed errors. `LspServer` only wires lifecycle dispatch,
initialize discovery, and the host-token entry point, keeping the feature's
responsibility isolated.

`crates/ling-lsp/tests/workspace_symbols.rs` verifies provider/discovery,
wire shape, all resolver-kind mappings, exact-before-prefix behavior,
case-sensitive matching, workspace scope and dependency/temporary exclusion,
module and URI context, Unicode/BOM/CRLF positions in UTF-8/16/32, insertion
order, equal-snapshot repetition, edit invalidation, deterministic 256-result
truncation, lifecycle, malformed queries, notification silence, cancellation,
compiler failure, atomicity, and recovery. Unit coverage directly checks
truncation and cancelled filtering. Diagnostic transcript fixtures pin the
expanded initialize response byte-for-byte.

## Specification gaps or conflicts

The historical authority audit found a real gap and prohibited implementation.
Accepted RFC-0045 now supplies the missing public decisions, using the bounded
source lookup already authorized by DEC-0082. No lower-authority plan wording
was treated as semantics, and no stale `zero`/`.zero` name entered the surface.

General incremental dependency graphs, persistent indexes, filesystem
discovery, asynchronous transport cancellation, partial results, and Stable
protocol lifecycle remain open or future work; this task does not imply them.

## Compatibility and determinism

- Protocol: adds Public Preview `ling.lsp.workspace-symbol/0.1` with no
  predecessor; incompatible behavior requires a new marker and migration.
- Diagnostics/schema/identity: no error-code allocation, schema, Semantic ID,
  DefinitionId, or canonical-byte change; internal identities are tie-breakers
  only and are never serialized.
- Determinism: output depends only on the captured snapshot, negotiated
  encoding, and query. Document insertion, maps, allocation, clock,
  environment, filesystem, and cache-hit order do not participate.
- Unicode: original UTF-8 spans and Unicode 17.0.0 remain unchanged; matching
  performs no normalization, folding, or locale conversion.
- Runtime: no language semantics, interpreter, VM, bytecode, ABI, package,
  filesystem, or network behavior changes.

## Intentionally deferred

Dependency/generated/builtin/Prelude search, package fields, top-level-only
filtering, fuzzy/case-folded/normalized/module search, scores, configurable
limits, resolve, partial results, work-done progress, stdio `$/cancelRequest`,
concurrent scheduling, persistent indexes, filesystem discovery/watchers,
Semantic IDs, documentation/types, Semantic Transactions, and Stable
compatibility remain out of scope.
