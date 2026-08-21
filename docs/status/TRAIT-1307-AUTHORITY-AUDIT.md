# TRAIT-1307 Authority Audit: Interpreter/VM dictionary lowering

## Outcome

TRAIT-1307 is correctly recorded as `BlockedSpec`. The next implementation
file named by the execution plan is `crates/ling-eval/src/lib.rs`, but the
repository does not yet have an accepted runtime-facing dictionary contract.
Implementing a dictionary lookup, method slot, or VM operand now would make a
backend choose semantics that the accepted authorities deliberately leave
open.

No evaluator API, VM API, bytecode field/opcode, public Trait diagnostic, or
placeholder method table was added. Existing Seed execution remains unchanged.

## Normative traceability

- Accepted RFC-0005 §4.1 requires every successful selection to lower to an
  immutable witness and requires later backends to consume that witness
  without re-running Trait selection.
- Accepted RFC-0005 §4.2 requires the resolved witness to be part of the
  checked semantic projection and forbids unresolved obligations in executable
  Typed Core.
- Accepted RFC-0005 compatibility clauses state that the RFC makes no current
  bytecode or runtime change; any bytecode witness encoding requires a separate
  accepted, versioned contract.
- Accepted DEC-0027 intentionally keeps the witness module crate-private and
  explicitly leaves `TypedProgram`, the Semantic Graph, interpreter/VM
  calling convention, and method-slot representation to later work.
- `docs/SEMANTICS.md` still excludes Trait execution from the v0.0.1 Seed
  surface, so this task cannot make Trait-bearing source executable as a side
  effect of backend work.

## Current interface evidence

The current repository confirms the missing boundary:

- `ling-types::check` rejects Trait items and obligations with the existing
  Seed `UnsupportedTypeSyntax` boundary.
- `ling-effects::CheckedProgram` and `ling-semantic::ProgramSnapshot` carry no
  dictionary table or witness identity.
- `ling-eval::execute_main` accepts only a checked `ProgramSnapshot`; the
  evaluator has no Trait-call HIR or checked-core operation to lower.
- `ling-types::checked_core` records selected Trait/impl/receiver/member-name
  identity, but it does not identify executable method definitions or slots.
- `ling-bytecode::ProgramParts` has no dictionary table or instruction, and
  `ling-vm` executes only independently verified bytecode. Adding a wire field
  would require a new accepted bytecode revision and verifier evidence.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. how the witness is attached to `TypedProgram`, `CheckedProgram`, and the
   Semantic Graph, including the program/identity binding;
2. how each ordered member maps to a checked executable definition or method
   slot (member names alone are not callable definitions);
3. the interpreter dictionary environment and call convention, including
   lifetime, lookup failure, and source-span projection;
4. the versioned bytecode representation, verifier rules, VM handoff, and
   compatibility behavior; and
5. positive, negative, deterministic, and interpreter/VM differential
   fixtures for selected, missing, ambiguous, and mismatched witnesses.

Until those decisions are Accepted, changing `crates/ling-eval/src/lib.rs`,
`crates/ling-bytecode/src/model.rs`, or `crates/ling-vm/src/execute.rs` would
either re-run selection, expose an invented ABI, or claim execution that the
Seed support matrix does not authorize.

## Evidence and compatibility

The audit was checked against `crates/ling-eval/src/lib.rs`,
`crates/ling-effects/src/lib.rs`, `crates/ling-semantic/src/lib.rs`,
`crates/ling-types/src/checked_core.rs`,
`crates/ling-bytecode/src/model.rs`, `crates/ling-bytecode/src/lib.rs`,
`crates/ling-vm/src/lib.rs`, RFC-0005, and DEC-0027. No code or public
protocol behavior changed; no diagnostic allocation, schema, Semantic ID,
source-span, bytecode, VM, or Unicode 17.0.0 claim is made.

## Intentionally deferred

TRAIT-1307 can start after the runtime dictionary and bytecode authority is
Accepted. The next executable source file remains
`crates/ling-eval/src/lib.rs`; its first change should consume the approved
immutable witness rather than search the coherence index. VM and bytecode
changes must follow the separately versioned bytecode decision.
