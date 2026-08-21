# VM-1208 Implementation Report: Effect / Capability / Fault

## Outcome

VM-1208 is implemented under Accepted RFC-0018. The slice integrates the
existing checked Effect and Capability metadata with the verified bytecode and
runtime boundary. It does not add a source feature or a bytecode revision.

## Normative traceability

- `RFC-0018` §Normative changes 1–3: checked Effect closure, module Capability
  metadata, verifier recomputation, and local `State<T>` remaining an SSA-only
  runtime concern are covered by the existing lowerer/verifier paths and their
  aggregate/mutable-place fixtures.
- `RFC-0018` §Normative changes 4–5: `Console.write` is resolved through the
  explicitly injected host table, preflighted before limits/effects, and
  normalized to stable `L-RUNTIME-0001` host Fault categories with committed
  state.
- `RFC-0018` §Normative change 6: an unwinding panic from a host adapter is
  caught at the VM call boundary and converted to `HostErrorCategory::Other`
  with committed state set conservatively to `true`.
- `RFC-0018` §Normative changes 7–8: Runtime Faults retain verified original
  source spans and deterministic bilingual diagnostic facts; no protocol,
  schema, Semantic ID, or Unicode behavior changes.

## Implementation and evidence

- `crates/ling-vm/src/execute.rs` catches host-adapter unwinding panics with
  `catch_unwind(AssertUnwindSafe(...))` and reuses the existing `HostError` /
  `RuntimeFaultKind::HostCapability` contract.
- `crates/ling-vm/tests/execution.rs` covers the panic normalization, stable
  `other` category, committed state, source span, and diagnostic code, in
  addition to the existing missing-capability, category, preflight, source-map,
  closure, aggregate, and mutable-place tests.
- `docs/RFC-0018.md`, the authority/lifecycle/gap registries, and the
  `PROTO-BYTECODE` inventory record the accepted boundary and its deferred
  VM-1209/VM-1210 scope.

## Compatibility and deferred work

- No source syntax, opcode, wire field, bytecode revision, CLI contract,
  diagnostic allocation, JSON schema, Semantic ID, canonical semantic bytes,
  ABI/FFI layout, or Unicode 17.0.0 table changed.
- `ling.bytecode/1.0`, `1.1`, and `1.2` remain Experimental; existing valid
  artifacts retain their behavior.
- VM-1209 owns the broader interpreter/VM differential corpus. VM-1210 owns
  decoder/verifier fuzzing, cancellation, and additional resource-model work.

## Validation

The final validation commands and commit identifier are recorded in
`docs/status/implementation-status.toml` once the repository gates complete.
