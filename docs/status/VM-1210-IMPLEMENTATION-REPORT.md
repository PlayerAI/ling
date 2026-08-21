# VM-1210 Implementation Report: VM Robustness, Cancellation, and Resource Evidence

## Outcome

VM-1210 is implemented under Accepted RFC-0020. The verified VM now exposes
an explicit Experimental cancellation entry point, reports cooperative
source-mapped cancellation through the existing Runtime Fault contract, and
extends decoder fuzz evidence with bounded deterministic mutations of the
checked Hello artifact. Existing step, frame, heap, oversized-table, and
malformed-UTF-8 evidence remains the resource boundary. No Ling source
semantics or bytecode wire revision changed.

## Normative traceability

- `RFC-0020` §Normative changes 1–2: `CancellationToken` is host-owned,
  monotonic, clone-shared, idempotent, and available only through the explicit
  `execute_v1_with_cancellation` API; `execute_v1` remains unchanged.
- `RFC-0020` §Normative changes 3–4: the VM checks cancellation before
  Capability preflight and before every instruction/terminator, preserves the
  committed-effect rule, and never rolls back host output.
- `RFC-0020` §Normative change 5: `RuntimeFaultKind::Cancelled` projects
  category `cancelled`, operation `execution.cancelled`, the original source
  name/span, committed state, and existing bilingual `L-RUNTIME-0001` JSON.
- `RFC-0020` §Normative changes 6–7: the bytecode fuzz target repeats
  decode/verify for arbitrary input and bounded valid-seed mutations, bounds
  rendered diagnostics, and the execution corpus covers cancellation before
  and after a committed effect alongside the existing resource/verifier cases.

## Implementation and evidence

- `crates/ling-vm/src/cancel.rs` defines the host cancellation token;
  `crates/ling-vm/src/execute.rs` adds deterministic checkpoints and the
  explicit control entry point; `crates/ling-vm/src/fault.rs` adds the stable
  cancellation projection.
- `crates/ling-vm/tests/execution.rs` verifies preflight precedence,
  source-mapped uncommitted cancellation, post-Console cancellation,
  committed output, and bilingual diagnostic facts. The differential harness
  remains exhaustive over all Runtime Fault variants.
- `fuzz/fuzz_targets/bytecode_bytes.rs` runs arbitrary bytes and at most 128
  deterministic mutations of `hello.lbc.hex` through decode/verify twice with
  bounded JSON diagnostics; `cargo check --manifest-path fuzz/Cargo.toml
  --bins --locked --offline` validates the fuzz workspace.
- RFC, lifecycle, gap, protocol, support-matrix, backlog, and generated
  governance reports register `ling.vm.control/0.1` as Experimental and keep
  structured Task/LSP cancellation outside this slice.

## Compatibility and deferred work

- No source syntax, Typed Core rule, opcode, wire field, format revision, CLI
  command, diagnostic allocation, JSON schema, Semantic ID, canonical semantic
  bytes, ABI/FFI layout, or Unicode 17.0.0 table changed.
- `ling.vm.control/0.1` is an Experimental host API only. It has no serialized
  reader/writer, no Stable promise, and no default execution-limit policy.
- Preemptive interruption, rollback, deadlines, scheduler/heap/replay models,
  structured Task cancellation, and LSP cancellation remain separately
  governed work.

## Validation

The final validation commands and commit identifier are recorded in
`docs/status/implementation-status.toml` after the status registry is updated.
