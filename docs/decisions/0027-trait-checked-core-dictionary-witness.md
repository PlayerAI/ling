# DEC-0027: Trait Checked Core dictionary witness boundary

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-21  
> Decision date: 2026-08-21  
> Owner role: type-system-design  
> Related authority/gap: `RFC-0005`, `DEC-0026`, `GAP-TRAIT-COHERENCE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

## Question

TRAIT-1306 needs a Checked Core boundary after solver selection. The boundary
must preserve the selected implementation without allowing a later backend to
search candidates again, while keeping the v0.0.1 Seed checker and public
protocols unchanged.

## Decision

1. `ling-types` adds a crate-private `checked_core` module. It consumes the
   immutable `SolvedObligation` records produced by DEC-0026 and the existing
   coherence index; it does not invoke candidate selection or mutate either
   input.
2. Each lowered witness contains `(TraitId, ImplId, receiver type, ordered
   member definitions)` together with the obligation ordinal and original
   `ObligationOrigin`. A member definition is an immutable ordinal/name pair
   copied from the selected implementation and checked against the indexed
   Trait member order.
3. Lowering rejects duplicate obligation ordinals, unknown selected
   implementations, Trait/receiver identity mismatches, and member-order/set
   mismatches. Errors remain internal evidence with the original UTF-8 span;
   no diagnostic code or public Trait behavior is allocated.
4. The witness table is canonically ordered by obligation ordinal, Trait ID,
   implementation ID, and receiver. Canonical bytes encode only semantic
   identity, receiver, ordered member names, and ordinal. They exclude source
   paths, source spans, allocation addresses, map order, and hash seeds.
5. The module remains crate-private and is not attached to `TypedProgram`, the
   Semantic Graph, interpreter, VM, CLI, LSP, bytecode, or any public schema in
   this slice. `ling-types::check` continues to reject Trait syntax and
   unresolved obligations through the existing Seed boundary until a later
   accepted runtime/projection decision integrates the table.

## Conformance plan

- Lower one exact solver selection and verify Trait/impl/receiver identity,
  ordered members, source origin, and immutable table access.
- Reject duplicate ordinals, unknown implementations, identity mismatches, and
  member-order mismatches without selecting a replacement candidate.
- Compare canonical bytes across repeated lowering and source-origin path
  changes; verify source paths/spans do not enter semantic bytes.
- Verify existing Seed type checking remains unchanged and no public Trait
  diagnostic, schema, Semantic ID, or runtime entry point is added.
- Run targeted and full locked/offline workspace, governance, status,
  formatting, and diff checks.

## Compatibility impact

- Adds only an internal `ling-types` Checked Core witness module and tests.
- Existing diagnostics, JSON schemas, Semantic IDs, source spans, CLI/LSP and
  package protocols, ABI, bytecode, runtime behavior, and Unicode 17.0.0 remain
  unchanged.
- Determinism is defined by normalized identity and ordered vectors; host paths,
  source presentation, filesystem order, Rust map order, and hash seeds are not
  semantic inputs to canonical bytes.

## Unresolved alternatives

- Public Semantic Graph projection, Typed Core integration, interpreter/VM
  dictionary calling convention, bytecode encoding, public Trait diagnostics,
  generic/blanket implementations, and runtime conformance remain TRAIT-1307
  or later work and require their own accepted compatibility evidence.
- A future witness may carry resolved function definitions or method slots only
  after the HIR/Checked Core representation and backend contract are accepted.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
