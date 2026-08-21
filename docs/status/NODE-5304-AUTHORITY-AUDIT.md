# NODE-5304 Authority Audit — Virtual-Time Reference Runtime

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5304 proposes a conformance runtime with a deterministic virtual clock,
exact ticks, injected input, output traces, overrun simulation, Fault/fallback
behavior, and replay integration.

No Accepted RFC-K502 or replacement defines Node virtual-time semantics, clock
units/advancement, input injection, output-trace identity, overrun/Fault
behavior, or replay equivalence. Existing VM cancellation and interpreter–VM
differential RFCs are deliberately narrower, while the replay protocol is
registered as Future with no schema. Implementing this runtime now would either
invent Node behavior or incorrectly promote host/VM controls into language
semantics.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:254-264` is a
  non-normative plan fragment. It lists test-runtime capabilities but defines
  no virtual-clock type, time origin/units, advancement/overflow, tick/release
  rules, injected-input model, output-trace schema, overrun Fault, fallback,
  or replay relation.
- `docs/SEMANTICS.md:1380-1425` sketches Node ticks and deadlines, while
  `:1914-1931` reserves Node for a future version; v0.0.1 implements only the
  Seed Core. `docs/LANGUAGE.md:857-866` is a surface example, not runtime
  authority.
- `docs/RFC-0019.md` is Accepted for interpreter–VM logical event equivalence,
  Runtime Fault projection, and checked-snapshot identity. It does not define
  Node virtual time, injected inputs, overrun simulation, or replay logs.
- `docs/RFC-0020.md` is Accepted only for Experimental VM host cancellation,
  fuzz determinism, and resource-limit evidence. It explicitly excludes Ling
  Task/Node cancellation, a common wall-clock deadline, a scheduler, and a
  replay protocol.
- `PROTO-REPLAY` in `docs/governance/protocol-inventory.toml:473-491` is a
  Future planned public protocol with no version, schema, reader/writer policy,
  or fixtures. `GAP-DETERMINISTIC-REPLAY-001` leaves event order, effects,
  privacy, corruption, divergence, and migration Open.
- `GAP-CRITICAL-PROFILE-001` leaves Node timing/Fault semantics, boundedness,
  Critical boundaries, and evidence Open. The structured Task scheduler gap
  and Actor/mailbox gaps also block a shared virtual-time or replay boundary.
- Accepted DEC-0019/DEC-0021 internal compiler-query scheduling and the
  `TASK-2204` test-scheduler plan are not a production Node runtime or public
  replay authority.

## Current implementation evidence

- `ling-eval` and `ling-vm` execute checked Seed programs; there is no Node
  runtime, virtual-clock type, exact-tick loop, injected-input adapter,
  output-trace schema, overrun simulator, or Node Fault/fallback path.
- VM `step_limit`, `frame_limit`, `heap_byte_limit`, and RFC-0020 cooperative
  host cancellation are execution-safety controls. They do not model logical
  Node time, release/deadline behavior, input sampling, state commits, or
  replay equivalence.
- RFC-0019's differential events compare interpreter and VM behavior for the
  accepted bytecode slice; they contain no virtual clock, Node input/output,
  schedule, or replay-log identity.
- No deterministic time units, advancement/tie-break rule, trace event order,
  effect recording/redaction, corruption handling, divergence report, or
  migration rule exists for this runtime.
- No stable bilingual diagnostic or schema fixes time overflow, injected-input
  mismatch, trace mismatch, simulated overrun, Fault/fallback, replay
  divergence, unsupported Node form, or target/runtime mismatch.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A test-only/reference versus production-runtime boundary, accepted Node
   Checked Core input, and the exact virtual-clock model: epoch, units,
   advancement, overflow, tick/release ordering, deadline comparison, and
   fairness/tie-break behavior.
2. Injected input and output-trace semantics, port/state sampling and commit
   boundaries, stable event/source identities, bounded trace size, canonical
   serialization, and original UTF-8 span/Semantic-ID provenance.
3. Overrun, missed tick, deadline, Fault, fallback, cancellation, restart,
   committed-effect, and recovery behavior, including resource and queue
   limits and the distinction between simulated and target-observed time.
4. Replay equivalence, recorded Effect/input/output vocabulary, event order,
   privacy/redaction, corruption and truncation handling, divergence classes,
   version/migration rules, and the relationship to the Future `PROTO-REPLAY`.
5. Critical Profile, Task/Actor, Native/ABI, Kernel/Device, ownership, and
   target/evidence boundaries; no host cancellation or internal scheduler may
   be treated as Node semantics.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for invalid clock/input/trace data, overrun/Fault/fallback, unsupported
   forms, replay divergence/corruption, and target/runtime mismatch.
7. Offline executable virtual-clock/tick, injected-input/output, state,
   overrun/Fault, fallback, replay, corruption, migration, determinism,
   Unicode/CRLF/BOM, bounded-resource, and interpreter/VM/Native differential
   fixtures with no unchecked-AST execution.

## Evidence and compatibility impact

The eventual implementation must consume checked Node Core only after Node and
replay authorities are accepted. Virtual-time and trace evidence must be
deterministic and distinguish simulation from target claims; host wall-clock,
thread scheduling, physical paths, addresses, allocator behavior, hash order,
and debug text must not affect Ling identity. Diagnostics and traces must
preserve original UTF-8 spans and Semantic IDs and must not expose private
effect/input data without an accepted redaction policy.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, host-cancellation behavior, Node runtime, scheduler,
Task, Actor, Native backend, replay protocol, diagnostics, schemas, Semantic
IDs, source spans, CLI, LSP, dependency lock, target/toolchain support claim,
or Unicode 17.0.0 behavior.

## Intentionally deferred

NODE-5304 implementation, virtual clock and reference runtime, injected
input/output traces, overrun simulation, Fault/fallback, replay integration,
diagnostics, CLI/LSP/evidence protocols, and support claims remain deferred
until RFC-K502 (or an Accepted replacement), `GAP-CRITICAL-PROFILE-001`,
`GAP-DETERMINISTIC-REPLAY-001`, the Task/Actor authorities, and NODE-5301
through NODE-5303 are resolved with independent offline fixtures. No
placeholder clock, trace, replay adapter, or public API is created.
