# DEC-0059: Trait projection read-only lookups / Trait 投影只读查找

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: type-system-design  
> Related authority/gap: `RFC-0022`, `GAP-TRAIT-COHERENCE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes a small in-process query boundary over the already
validated, immutable `x-ling-trait-ide` projection. It does not add an editor
protocol or repeat Trait selection.

## Question

Consumers of RFC-0022 need to find the selected witness and member records by
their stable identities. How can that lookup be exposed without introducing
LSP/JSON-RPC methods, document state, diagnostics, repairs, or a second Trait
selection algorithm?

## Decision

1. `SemanticTraitIdeProjection` exposes read-only lookup helpers for the
   existing records:
   - `witnesses_by_trait_id` returns all matching witnesses in their existing
     projection order;
   - `witness_by_implementation_id` returns the first matching witness;
   - `members_by_trait_definition_id` returns all matching members in witness
     and member projection order; and
   - `member_by_implementation_definition_id` returns the first matching
     member.
2. Helpers compare the supplied string with the exact projected identity and
   never mutate, revalidate, normalize, reselect, or synthesize a record.
3. First-match helpers are deterministic for directly constructed values;
   graph readers remain responsible for structural validation before a graph
   is consumed. No uniqueness rule beyond the RFC-0022 reader contract is
   added here.
4. The helpers are in-process Rust APIs only. They carry no URI, document
   version, position, JSON-RPC, Workspace Edit, transaction, diagnostic,
   repair, CLI, bytecode, VM, or Stable 1.0 behavior.

## Conformance plan

- Verify exact identity hits and misses for witness and member lookups.
- Verify repeated Trait/member identities are returned in the original
  projection order and first-match helpers select the first record without
  changing the projection.
- Verify the helpers do not alter JSON serialization, program identity,
  source spans, diagnostics, runtime behavior, or Unicode 17.0.0 data.
- Keep the parent IDE/LSP fixtures for positions, versions, transactions,
  rename, diagnostics, and repairs deferred until their protocol authority is
  Accepted.

## Compatibility impact

- Adds only read-only `ling-semantic` Rust methods over an existing
  Experimental extension. Source syntax, checked semantics, diagnostics,
  schemas, Semantic IDs, CLI, protocols, bytecode, VM, package data, and
  Unicode 17.0.0 remain unchanged.
- No new protocol inventory entry or diagnostic allocation is required.

## Unresolved alternatives

LSP/JSON-RPC request and response schemas, document synchronization and
version guards, URI and position projection, Workspace Edits, Semantic
Transactions, rename/repair behavior, diagnostic facts, generic/blanket Trait
queries, and Stable 1.0 editor compatibility remain governed by the blocked
`TRAIT-1308` parent and its registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
