# TRAIT-1301 Authority Audit

Status: **Accepted design / implementation gate closed**

## Outcome

TRAIT-1301 is the G1 semantic design gate rather than a compiler implementation
file. RFC-0005 is now Accepted and fixes the restricted first-slice Trait
contract. The design milestone is complete; TRAIT-1302 is the next executable
implementation target.

Implementing parser, resolver, solver, or lowering behavior now would make Draft
or illustrative text observable language semantics and would block compatible
cross-package evolution.

## Normative traceability

- `docs/SEMANTICS.md` §3.7 reserves `trait` and `impl`; §6.8 defines only the
  Seed boundary (Trait is not implemented) and records coherence/dynamic-dispatch
  requirements whose orphan rule is deferred to a later RFC.
- `docs/LANGUAGE.md` §6.4 gives an illustrative Trait/impl shape and rejects
  inheritance, but does not define a parser grammar, obligation model, selection
  algorithm, diagnostics, or lowering schema.
- `docs/ling_execution_plan/03-G1-V0.1-LIVING.md` §5 requires TRAIT-1301 to
  decide declaration/impl syntax, nominal versus structural semantics,
  constraints, coherence, orphan and overlap rules, inference boundaries, public
  API presentation, lowering, and Semantic ID treatment.
- `docs/RFC-0001.md` §22 originally listed RFC-0005 as a future RFC, and
  RFC-0001 remains Draft; it is not authority to implement Trait semantics.
  `docs/RFC-0005.md` is now the dedicated Accepted authority with
  `stable_basis = true`.
- `docs/governance/gap-register.toml` records `GAP-TRAIT-COHERENCE-001` as an
  open P0 gap blocking TRAIT-1301 through TRAIT-1307.

## Missing contract

The following externally observable decisions are still open:

1. declaration and `impl` grammar, including source-span and duplicate-member
   behavior;
2. nominal/structural identity and constraint syntax;
3. global coherence, orphan ownership, overlap, and ambiguity termination;
4. inference and package-boundary obligations;
5. public API and diagnostic representation of constraints;
6. static dictionary passing versus other lowering, including Checked Core shape;
7. whether implementation selection participates in Semantic IDs and cache keys;
8. deterministic cross-package positive/negative conformance fixtures and a
   migration policy for future rule changes.

## Evidence and compatibility impact

- `rg` finds only Rust host capability traits; no Ling Trait AST, solver, or
  lowering implementation exists in `crates/`, and no Trait conformance corpus
  is present under `tests/`.
- No diagnostic allocation, protocol schema, Semantic ID rule, source-span
  behavior, or Unicode table is changed by this design milestone.
- No compiler Trait behavior is claimed yet. Governance/status checks cover the
  authority transition; implementation evidence remains required by RFC-0005.

## Required next action

Implement TRAIT-1302 through TRAIT-1307 against RFC-0005, adding the required
cross-package positive, negative, termination, determinism, and migration
evidence. TRAIT-1301 itself is complete as a design milestone.

Intentionally deferred: all Trait syntax, AST/HIR, constraint collection,
coherence/orphan checking, solving, Checked Core lowering, IDE support, and
performance work.
