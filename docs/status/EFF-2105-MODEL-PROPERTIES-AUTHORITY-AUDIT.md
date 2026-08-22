# EFF-2105-MODEL-PROPERTIES Authority Audit

## Outcome

`EFF-2105-MODEL-PROPERTIES` is a bounded evidence child authorized by
Accepted `RFC-0006`, `DEC-0062`, and `DEC-0067`. It covers only deterministic
in-process properties of the accepted Effect model. The parent `EFF-2105`
remains `BlockedSpec` for source/Core generation, handler execution, and
interpreter/VM differential semantics.

## Normative basis

- `RFC-0006` fixes canonical Effect identities, rows, operation signatures,
  first-order handler contracts, residual subtraction, resume modes, and
  source-span/path-free determinism.
- `DEC-0062` fixes collected row constraints, deterministic solving, occurs
  checks, generalization boundaries, and handler subtraction.
- `DEC-0067` limits this child to a finite offline corpus and explicitly
  excludes source generation, evaluator/VM execution, public protocols, and
  runtime Fault/cancellation behavior.

## Evidence boundary

The child exercises `ling-effects` public model values only:

- bounded permutations and duplicate elimination for rows and open tails;
- insertion-order-independent row constraint substitutions;
- nested first-order handler residuals, clause ordering, and resume limits;
- graph/Core canonical bytes independent of source spans and presentation order.

No unresolved AST/HIR is interpreted. No source program, handler continuation,
bytecode instruction, VM operation, diagnostic allocation, Semantic ID, schema,
CLI, LSP, or protocol is added.

## Intentionally deferred

The parent remains blocked for a well-typed Core generator, shrinking and
reproducible fuzz corpus, handler runtime semantics, residual-row observations
at evaluation time, Fault/cancellation/State interaction, and interpreter/VM
differential equivalence. Those require additional accepted runtime and
generator authorities.
