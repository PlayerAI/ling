# EFF-2103-CORE Authority Audit: First-order Handler Typed Core

## Outcome

`EFF-2103-CORE` is `Done` under Accepted RFC-0006, DEC-0062, and DEC-0063. The
child is intentionally bounded to a checked, in-process, first-order Handler
Core projection. The EFF-2103 parent remains `BlockedSpec` for source grammar
and full AST/HIR lowering.

Implementation commit: `e1dc5334d15e25e959fa0da6e3462a90210c6fdf`.

## Normative traceability

- RFC-0006 §§4, 6, 7, and 9 authorize explicit operation contracts, lexical
  residual rows, resume cardinality, Capability separation, and deterministic
  path-free projections, while assigning source syntax and lowering to
  EFF-2103.
- DEC-0062 authorizes canonical row constraints, spans as evidence only, and
  `L-EFFECT-*` conflict diagnostics.
- Accepted DEC-0063 fixes the checked Core container, clause body identities,
  resume-use limits, closed-boundary residual diagnostic, and explicit
  exclusions for execution, protocols, and Task/Actor crossing.

## Current implementation evidence

- `ling-effects/src/handler_core.rs` now contains the bounded Core projection,
  clause validation, residual computation, resume-use checks, canonical bytes,
  unresolved-body rejection, and `L-EFFECT-0003` diagnostics.
- The projection consumes only canonical checked values and preserves original
  UTF-8 spans as diagnostic evidence. It does not parse or interpret AST/HIR.
- Focused tests cover nested residuals, canonical bytes, resume boundaries,
  unresolved bodies, bilingual JSON/human residual diagnostics, and source
  spans.

## Parent boundary and deferred work

The EFF-2103 parent still requires an Accepted source grammar and complete
AST/HIR lowering contract for `handle`/`with operation`. Runtime continuations,
Fault/cancellation interaction, Audit Source/Semantic Graph schema, bytecode,
VM execution, Task/Actor, Replay, Remote, Native, GPU, and FFI remain deferred.

No Seed syntax, existing diagnostic meaning, Semantic ID, schema, CLI, LSP,
runtime, bytecode, VM, protocol, or Unicode 17.0.0 behavior is changed by the
child.
