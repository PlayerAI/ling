# TRAIT-1308-QUERY implementation report

## Outcome

**Status: Done (bounded read-only lookup slice).**

`ling-semantic` now provides deterministic in-process lookup helpers over the
accepted RFC-0022 `x-ling-trait-ide` projection. The parent `TRAIT-1308` task
remains `BlockedSpec` for the full IDE/LSP surface; this slice does not claim
hover, completion, rename, repairs, diagnostics, or a Stable 1.0 editor
protocol.

Implementation commit: `feb2be24fc78abc73010f283e830d3844f49b303`.

## Normative traceability

- Accepted RFC-0022 §§1–8 continues to govern the optional projection,
  immutable witness/member identities, original UTF-8 spans, and reader
  validation.
- Accepted DEC-0027 and RFC-0005 require consumers to use the selected
  immutable dictionary witness rather than re-run Trait selection.
- Accepted DEC-0059 §§1–4 authorizes only read-only identity filtering and
  deterministic first-match access over those existing records.

## Implemented slice

- Added `witnesses_by_trait_id`, preserving all matching witnesses in their
  existing projection order.
- Added `witness_by_implementation_id`, returning the first projection-order
  match.
- Added `members_by_trait_definition_id`, preserving witness/member projection
  order across matching members.
- Added `member_by_implementation_definition_id`, returning the first
  projection-order match.
- The helpers borrow the projection, do not mutate it, and do not validate,
  normalize, select, or synthesize Trait records.

## Evidence

- `cargo fmt --all -- --check` passed.
- `cargo test -p ling-semantic --all-features --locked --offline` passed with
  16 semantic unit tests and 5 project snapshot tests.
- Tests cover exact identity hits and misses, repeated identities, first-match
  determinism, and projection-order preservation.

## Compatibility and deferred work

No source syntax, checked semantics, diagnostics, JSON schema, Semantic ID,
source span, CLI, protocol inventory, bytecode, VM, package, or Unicode
17.0.0 behavior changed. No new public protocol or diagnostic code was added.

LSP/JSON-RPC requests, document versions and positions, Workspace Edits,
Semantic Transactions, rename, diagnostics, repairs, generic/blanket Trait
queries, and Stable 1.0 editor compatibility remain deferred to the blocked
`TRAIT-1308` parent and its registered authority gaps.
