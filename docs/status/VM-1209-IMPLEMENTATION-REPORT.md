# VM-1209 Implementation Report: Interpreter–VM Differential Contract

## Outcome

VM-1209 is implemented under Accepted RFC-0019. The repository now has a
bounded, table-driven differential harness that builds one checked
`ProgramSnapshot`, executes the checked interpreter and verifier-created VM
program, and compares only the accepted logical outcome. It does not add
source syntax, bytecode instructions, a wire revision, or a new public
diagnostic.

## Normative traceability

- `RFC-0019` §Normative changes 1–2: every case shares the same checked
  snapshot and compares ordered logical `Console.write` events plus the Unit
  entry result.
- `RFC-0019` §Normative change 3: Runtime Faults are projected to category,
  operation, source name, original UTF-8 byte span, and committed state.
- `RFC-0019` §Normative change 4: VM internal errors and verification failures
  fail the harness as infrastructure defects; they are not coerced into user
  Faults.
- `RFC-0019` §Normative changes 5–7: repeated snapshot construction checks
  deterministic `ProgramId`, encode/decode/verify execution checks logical
  round-trip behavior, explicit bounds cap fixture/source/event/Fault data,
  and failure messages identify only the fixture label and projection.

## Implementation and evidence

- `crates/ling-vm/tests/differential.rs` defines the table-driven corpus for
  v1.0 hello/direct-call/format Faults, v1.1 closure and recursion lowering,
  and v1.2 skipped effects, aggregates, checked match, and mutable-place
  observation.
- The harness compares successful event sequences, Unit results, structured
  Fault projections, original source spans, committed state, deterministic
  snapshot identity, and verifier-created bytecode execution.
- `docs/RFC-0019.md`, the authority/lifecycle/gap registries,
  `PROTO-BYTECODE`, the traceability registry, and the implementation backlog
  record the accepted contract and its VM-1210 boundary.

## Compatibility and deferred work

- No source syntax, opcode, wire field, bytecode revision, CLI contract,
  diagnostic allocation, JSON schema, Semantic ID format, canonical semantic
  bytes, ABI/FFI layout, or Unicode 17.0.0 table changed.
- `ling.bytecode/1.0`, `1.1`, and `1.2` remain Experimental; the differential
  evidence does not imply a Stable protocol or a general cross-version
  compatibility promise.
- VM-1210 still owns decoder/verifier fuzzing, cancellation, resource-stress
  coverage, and any additional execution model not authorized by RFC-0019.

## Validation

The final validation commands and commit identifier are recorded in
`docs/status/implementation-status.toml` after repository gates complete.
