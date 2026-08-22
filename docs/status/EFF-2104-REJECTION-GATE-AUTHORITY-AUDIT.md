# EFF-2104-REJECTION-GATE Authority Audit

## Outcome

`EFF-2104-REJECTION-GATE` is an authorized bounded child under Accepted
DEC-0088. It proves that unresolved handler HIR is rejected by the shared CLI
compiler before a checked snapshot or execution path is published. It does not
implement handler execution or unblock public EFF-2104.

## Normative traceability

- RFC-0006 defines the Experimental Effect model but does not authorize
  runtime handler execution.
- DEC-0063 defines only checked first-order Handler Core data.
- DEC-0066 requires unresolved handler HIR to be rejected with
  `L-EFFECT-0004` before evaluator, bytecode, VM, or public paths consume it.
- DEC-0088 authorizes only the shared CLI negative boundary fixture and reuse
  of the existing diagnostic.

## Implemented boundary

`crates/ling-cli/tests/handler_boundary.rs` compiles the accepted unresolved
handler source through `ling_cli::compile_source` and asserts the registered
`L-EFFECT-0004` diagnostic, bilingual message text, and original source span.
The test also proves that compilation returns no `ProgramSnapshot`.

No operation dispatch, continuation, resume, residual runtime row, State/Fault
interaction, cancellation, bytecode, VM, ABI, or positive handler behavior is
introduced.

## Specification gap and deferred work

Public EFF-2104 remains blocked by missing checked Handler Core integration,
operation dispatch, continuation lifetime, runtime Fault/State/cancellation
rules, interpreter reference semantics, bytecode/VM ABI and verification,
differential equivalence, and migration decisions.

## Evidence and compatibility

- The focused CLI fixture exercises the shared compilation boundary and
  registered bilingual diagnostic serialization.
- No language, diagnostic registry, schema, Semantic ID, runtime, bytecode, VM,
  ABI, dependency, or Unicode 17.0.0 behavior changes.
