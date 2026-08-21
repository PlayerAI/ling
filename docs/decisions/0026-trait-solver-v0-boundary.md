# DEC-0026: Trait solver v0 boundary

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-21  
> Decision date: 2026-08-21  
> Owner role: type-system-design  
> Related authority/gap: `RFC-0005`, `GAP-TRAIT-COHERENCE-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

## Question

TRAIT-1305 needs a solver interface after obligation collection and coherence
indexing. The interface must select only a legal concrete nominal implementation,
distinguish zero and multiple candidates, and terminate recursive requirements
without exposing a public diagnostic or claiming executable Trait support.

## Decision

1. `ling-types` adds an internal solver that consumes a resolved program, the
   deterministic coherence index, ordered obligations, and an internal map of
   selected-implementation requirements. It does not mutate the index and does
   not perform a second coherence or name-resolution pass.
2. The first slice treats the first obligation type argument as the nominal
   receiver. A candidate is applicable only when its Trait ID matches and its
   canonical receiver is exactly equal to that concrete argument. Variables or
   variable-containing applications never match a concrete implementation and
   produce the same bounded unsatisfied result as zero candidates; this avoids
   inventing blanket-impl semantics.
3. Exactly one applicable candidate produces an internal immutable selection
   record containing the obligation ordinal, Trait ID, Impl ID, receiver, and
   ordered member names. Zero candidates produce `Unsatisfied`; multiple
   candidates produce `Ambiguous` with stable candidate IDs. Candidate order is
   evidence only and cannot resolve ambiguity.
4. A selected implementation may carry internal nested requirements for tests
   and later HIR integration. Requirements are visited in vector order. The
   active recursion key contains `(TraitId, receiver type head, canonical
   arguments)` and the reported evidence also records the current obligation
   depth. Repeated active keys are rejected as `Cycle`; nesting at or beyond 64
   obligations is rejected as `DepthLimit`.
5. Solver results and errors preserve the obligation source name and original
   UTF-8 span and are sorted by source/span/kind for repeatable evidence. The
   module remains crate-private; `ling-types::check` continues to reject Trait
   syntax and unresolved obligations through the existing non-executable
   boundary until dictionary lowering is accepted.
6. This decision adds no diagnostic allocation, schema, Semantic ID, CLI/LSP or
   package protocol, ABI, bytecode, runtime, or Unicode behavior. It does not
   define generic/blanket implementations, specialization, dictionary layout,
   or public Trait support.

## Conformance plan

- Select one exact concrete nominal receiver and verify the immutable selection
  carries the expected Trait/impl/member identity and original span.
- Exercise zero candidates, variable receivers, unknown Traits, malformed
  receiver arity, and multiple candidates; compare error kinds and stable
  candidate order across repeated runs.
- Exercise nested requirements, an active recursion cycle, and a 64-level
  resource boundary without stack overflow or unbounded allocation.
- Verify the solver consumes the coherence index as-is and does not produce a
  successful executable Typed Core or public Trait diagnostic.
- Run targeted and full offline locked workspace checks plus governance/status
  validation.

## Compatibility impact

- Adds one internal `ling-types` solver module and no public source or runtime
  behavior.
- Existing diagnostic codes, bilingual rendering, schemas, Semantic IDs,
  package protocols, ABI, bytecode, and Unicode 17.0.0 remain unchanged.
- Determinism uses normalized HIR names, ordered vectors, stable IDs, and
  original source spans; filesystem paths, allocation addresses, and hash-map
  iteration are not semantic inputs.

## Unresolved alternatives

- Generic or blanket impl matching, type-variable substitution from inference,
  specialization, public Trait diagnostics, dictionary witness schema,
  Semantic Graph projection, and runtime lowering require later accepted
  decisions or TRAIT tasks.
- A future solver may widen the obligation-to-receiver mapping only with a
  complete RFC-backed overlap and termination proof.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
