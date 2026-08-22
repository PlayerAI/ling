# DAP-3603-OBSERVATION Authority Audit — Staged Debugger Capability Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DAP-3603-OBSERVATION is limited to test-local vocabulary for future staged
debugger capabilities. It does not implement launch/continue, breakpoints,
stepping, stacks/scopes/variables, conditions/logpoints, attach, or Actor/Task
views. Public DAP-3603 remains `BlockedSpec`; stale `zero` commands are not
implemented.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:538-547` is non-normative and
  lists stages without defining stop/step granularity, expression evaluation,
  variable identity/lifetime, attach security, or Actor/Task observability.
- DAP-3601 and DAP-3602 remain `BlockedSpec`; their adapter, wire, lifecycle,
  extension, launch, and source-map contracts are not accepted. The stale
  `zero build`/`zero dap` spellings cannot override authoritative `ling` and
  `.ling` names.
- `docs/SEMANTICS.md` excludes Task and Actor execution from v0.0.1 and leaves
  lifecycle/reentry/mailbox/supervision rules open. Accepted RFC-0014/
  RFC-0018/RFC-0019 provide experimental VM/source-map/Fault/cancellation/
  differential foundations only.
- `GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-AWAIT-REENTRY-001`,
  `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, `GAP-NATIVE-BACKEND-ABI-001`, and the
  semantic/editor protocol gaps remain Open. No debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Current implementation evidence

- The workspace has no debugger capability implementation, runtime
  breakpoint/step/attach hooks, condition/logpoint evaluator, stack/scope/
  variable mapping, Native debug metadata, Actor/Task runtime, or debugger
  protocol/extension surface.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule fixes breakpoint timing, effect interaction, callbacks,
  ownership/suspension/cancellation, condition sandboxing, attach
  authentication, or asynchronous Task/Actor observations.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. DAP wire/session/capability and VM/Native debug metadata contracts with stop
   reasons, breakpoint identity/location, step granularity, source/binary/
   ProgramSnapshot identity, stack/frame/scope/variable schema, Fault mapping,
   and source/UTF-8 conversion.
2. Stage-by-stage semantics and support gates for launch, continue, steps,
   conditional breakpoints/logpoints, attach, and target/profile/engine
   selection. Unsupported stages must reject visibly.
3. Safe condition/logpoint evaluation and inspection: no unchecked AST,
   arbitrary effects, foreign calls, capability escalation, host I/O,
   address/layout leakage, or unbounded allocation; values and
   Resource/Managed ownership must follow runtime rules.
4. Task/Actor lifecycle, suspension/reentry, cancellation, mailbox,
   supervision, Fault, and cleanup semantics before exposing their views.
5. Attach authentication, session isolation, limits, redaction, deterministic
   bilingual diagnostics, protocol inventory/migrations, and VM/Native
   conformance/property/smoke fixtures with explicit support claims.

## Compatibility and intentionally deferred work

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode 17.0.0 behavior. All staged
capabilities, runtime/debug metadata, security/inspection policies,
Task/Actor views, protocol readers/migrations, fixtures, and support claims
remain deferred.
