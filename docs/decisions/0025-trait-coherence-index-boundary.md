# DEC-0025: Trait coherence index boundary

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-21  
> Decision date: 2026-08-21  
> Owner role: type-system-design  
> Related authority/gap: `RFC-0005`, `GAP-TRAIT-COHERENCE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

## Question

TRAIT-1304 needs a package-aware interface that can validate the restricted
first-slice Trait declarations and impls before solver work begins. It must
freeze ownership, duplicate/overlap, and deterministic indexing rules without
turning diagnostic candidate order into semantic selection or adding a public
Trait diagnostic surface.

## Decision

1. TRAIT-1304 adds an internal coherence index in `ling-types`. It indexes
   ordered Trait declarations and nominal impl records from resolved HIR,
   retaining module/package ownership, original declaration spans, canonical
   receiver terms, and ordered member names.
2. The index uses the resolved package identity and lock-derived project
   context when present. A package may own an impl when it owns the Trait or
   the receiver type; an impl whose Trait and receiver are both foreign is an
   orphan error. A local file-oriented program is treated as one local package
   for this ownership check, while prelude or missing nominal types remain
   foreign.
3. Generic impl receivers and type variables in receiver arguments are
   rejected in this first slice. Exact canonical duplicate receivers for one
   Trait are rejected, and any remaining applicable overlap is rejected rather
   than resolved by specialization or priority. Member completeness and
   duplicate member names are checked against the indexed Trait declaration.
4. Traits, impls, and errors are published in stable vector/B-tree order. A
   candidate order retained for diagnostics is derived only from canonical
   Trait/receiver/module/span keys and MUST NOT select an implementation.
5. The index records internal bilingual-neutral error data only. It does not
   allocate diagnostic codes, resolve obligations, select candidates, produce
   dictionary witnesses, mutate Semantic IDs, or feed executable Typed Core.
   `ling-types::check` continues to reject Trait-bearing programs through the
   existing unsupported-syntax boundary until TRAIT-1305 and TRAIT-1306.
6. This decision adds no CLI/LSP/package/bytecode protocol, schema, ABI, or
   Unicode table and does not expand the v0.0.1 Seed support matrix.

## Conformance plan

- Index one valid Trait and impl with ordered members and verify canonical
  ownership and receiver data.
- Exercise same-Trait duplicate receivers, foreign/foreign orphan impls,
  generic receiver rejection, missing members, and duplicate members.
- Build the same cross-module/package input in different supplied orders and
  compare the complete index/error result; no filesystem or hash-map order may
  change it.
- Verify that candidate ordering is observable only as deterministic internal
  evidence and that `check` still produces no executable Trait Typed Core.

## Compatibility impact

- Adds an internal coherence index and error model only; no public protocol,
  diagnostic allocation, schema, Semantic ID, CLI/LSP, ABI, or Unicode 17.0.0
  changes are made.
- Existing Seed programs remain on the unchanged checker path. Trait syntax
  remains outside the v0.0.1 support matrix.

## Unresolved alternatives

- Obligation candidate solving, recursion/cycle limits, stable user-facing
  diagnostics, explicit dictionary witnesses, Semantic Graph projection,
  runtime lowering, and generic/blanket impls are deferred to TRAIT-1305
  through TRAIT-1309.
- A public coherence report or serialized impl index requires a separate
  protocol decision after the solver and witness contracts are accepted.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
