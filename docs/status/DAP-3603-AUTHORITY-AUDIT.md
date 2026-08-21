# DAP-3603 Authority Audit — Staged Debugger Capabilities

Status: `BlockedSpec`

Date: 2026-08-22

## Outcome

DAP-3603 proposes staged debugger capabilities: launch/breakpoint/continue;
step in/over/out; stack traces, scopes, and variables; conditional
breakpoints/logpoints; attach; and future Actor/Task views. These stages depend
on an accepted DAP adapter, runtime/debug metadata, VM/Native semantics, and
for the final stage the unresolved Task/Actor contracts.

No breakpoint or stepping implementation, stack/locals projection, condition
evaluator, attach path, Actor/Task debugger view, capability negotiation,
runtime hook, editor integration, diagnostic, or public debugger API is added.
The plan's stale `zero` commands remain historical text and are not carried
into implementation; no `ling` debugger command is claimed.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:538-547` is non-normative and
  explicitly says DAP must not block v0.1 or provide false buttons/placeholders
  before G3. It lists stages but does not define stop/step granularity,
  expression evaluation, variable identity/lifetime, attach security, or
  Actor/Task observability.
- DAP-3601 and DAP-3602 remain `BlockedSpec`; their adapter, wire, lifecycle,
  extension, launch, and source-map contracts are not accepted. The plan's
  `zero build`/`zero dap` spellings cannot override the authoritative `ling`
  CLI and `.ling` extension names.
- `docs/SEMANTICS.md:1242-1279` and `:1283-1358` describe future Task and
  Actor concepts, but Task detach is explicitly not implemented before v0.2
  and their lifecycle/reentry/mailbox/supervision rules are tracked as open
  gaps. v0.0.1 formally excludes Task and Actor execution.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental VM bytecode,
  source-map, Fault, cancellation, and Interpreter–VM differential
  foundations; they do not define debugger stops, conditional expressions,
  attach, Actor/Task views, or a Native runtime.
- `GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-AWAIT-REENTRY-001`,
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-NATIVE-BACKEND-ABI-001`,
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, and
  `GAP-LSP-TRANSACTION-PROTOCOL-001` remain Open. No debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.
- RFC-N304, RFC-N305, RFC-N306, RFC-0008, RFC-0009, RFC-0011, and any
  debugger RFC are not Accepted authorities; RFC-0001 remains Draft under
  DEC-0018.

## Current implementation evidence

- The workspace has no DAP adapter, runtime breakpoint/step/attach hooks,
  condition/logpoint evaluator, stack/scope/variable mapping, Native debug
  metadata, Actor/Task runtime, or debugger protocol/extension surface.
  Existing VM source maps and Faults are experimental foundations only.
- No accepted rule fixes whether a breakpoint observes before/after an
  expression, how stepping interacts with effects, callbacks, ownership,
  suspension, cancellation, or asynchronous turns, how conditional/logpoint
  expressions are sandboxed, or how attach authenticates and controls a
  running process.
- No debugger dependency, transport, toolchain, unsafe evaluator, diagnostic
  allocation, public protocol implementation, or stale `zero` command is
  required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The DAP wire/session/capability and VM/Native debug metadata contracts,
   including stop reasons, breakpoint identity/locations, step granularity,
   source/binary/ProgramSnapshot identity, stack/frame/scope/variable schema,
   Fault/exception mapping, and source/UTF-8 position conversion.
2. Stage-by-stage semantics and support gates for launch, continue, step,
   conditional breakpoints/logpoints, attach, and target/profile/engine
   selection. Unsupported stages must reject visibly rather than expose a
   nonfunctional control.
3. Safe condition/logpoint evaluation and inspection rules: no unchecked AST,
   arbitrary side effects, foreign calls, capability escalation, host file or
   network access, address/layout leakage, or unbounded allocation; values and
   Resource/Managed ownership must be read consistently with runtime rules.
4. Task/Actor lifecycle, suspension/reentry, cancellation, mailbox,
   supervision, and Fault/cleanup semantics before exposing Actor/Task views;
   no debugger display can choose unresolved language behavior.
5. Attach authentication, process/session isolation, concurrency, timeout and
   resource limits, redaction, deterministic diagnostics, protocol inventory,
   migrations, and VM/Native conformance/property/smoke fixtures, with
   bilingual stable errors and explicit Preview/Experimental claims.

## Evidence and compatibility impact

The eventual implementation needs positive/negative fixtures for every stage,
source-map and breakpoint identity, stop/step/continue ordering,
condition/logpoint sandboxing, stack/scope/variable/ownership/Fault behavior,
attach authentication and process failure, target/profile rejection,
Task/Actor suspension/reentry/cancellation/mailbox views, malformed messages,
timeouts/resource limits, multi-session isolation, VM/Native differential and
cross-target runs, deterministic diagnostics, schema migration, and offline
reproducibility. It must preserve original UTF-8 spans, stable Semantic IDs,
Unicode 17.0.0, and `L-<DOMAIN>-<NUMBER>` diagnostics without exposing host
paths, addresses, timing, allocation, or debug text as Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode behavior. It adds no
debugger capability, runtime hook, condition evaluator, dependency, toolchain,
diagnostic, public protocol implementation, or placeholder API.

## Intentionally deferred

All staged debugger capabilities, including launch/continue, breakpoints,
stepping, stacks/scopes/variables, conditional/logpoints, attach, Actor/Task
views, runtime/debug metadata, security and inspection policies, protocol
readers/migrations, editor integration, fixtures, and support claims remain
deferred until DAP-3601/3602 and the VM, Native, Task, Actor, identity, and
semantic-protocol authorities are Accepted.
