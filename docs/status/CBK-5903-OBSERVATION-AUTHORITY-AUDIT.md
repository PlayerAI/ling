# CBK-5903-OBSERVATION Authority Audit — Critical Runtime/Target Package Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0215` permits only test-local Critical-runtime/Target-Package
vocabulary. It does not authorize a scheduler, resource bounds, runtime
lifecycle, watchdog/safe-state behavior, Target Package/primitive, host
service, ABI, evidence verifier, diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:611-620` is a
  non-normative runtime/target checklist.
- `docs/status/CBK-5903-AUTHORITY-AUDIT.md` records missing Critical profile,
  schedule/time, memory/resource, lifecycle/Fault, target/ABI, evidence, and
  fixture authority.
- `docs/IMPLEMENTATION.md` excludes these capabilities from Seed; Critical,
  Native/ABI, ownership, and kernel/device gaps remain Open.
- `PROTO-ABI` and `PROTO-EVIDENCE` are Future, the support matrix keeps the
  capability unavailable, and accepted VM behavior has a distinct scope.

## Current implementation evidence

The observation adds one isolated test with sixty explicit profile/Core,
scheduling/time, memory/resource, lifecycle/Fault/watchdog, Target Package/
primitive, identity, evidence/trust, and fixture boundaries. It sorts by
explicit local rank, rejects duplicates, compares opaque bytes for forward/
reverse input order, and keeps every checklist item distinct. No scheduler,
runtime, resource checker, target, primitive, ABI, evidence, diagnostic,
protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define Critical profile/runtime lifecycle; deterministic
scheduling/time and resource bounds; ownership/drop/Fault/watchdog/safe state;
Target Package, primitive, ABI and host-service behavior; artifact/toolchain/
target identities; evidence, trust, TCB and fail-closed rules; bilingual
diagnostics; stable Semantic IDs and original UTF-8 spans; and offline target
fixtures. Seed behavior, dependencies, support state, and Unicode 17.0.0 remain
unchanged.

## Deferred work

CBK-5903 runtime/scheduler, resource enforcement, lifecycle/watchdog,
Target Package/ABI/primitive integration, evidence verification, diagnostics,
protocols, and support remain deferred until Accepted authority and executable
target evidence exist. No placeholder Critical-runtime API is created.
