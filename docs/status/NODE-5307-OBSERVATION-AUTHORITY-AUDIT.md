# NODE-5307-OBSERVATION Authority Audit — Node Conformance Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0191` permits only test-local Node-conformance vocabulary. It
does not authorize a conformance runner, fixture manifest, oracle, timing or
state semantics, replay protocol, diagnostics, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:288-300` is a
  non-normative conformance checklist.
- `docs/status/NODE-5307-AUTHORITY-AUDIT.md` records the missing protocol,
  manifest, oracle, exact behavior, replay, bridge, and target evidence.
- `GAP-CRITICAL-PROFILE-001`, `GAP-DETERMINISTIC-REPLAY-001`,
  `GAP-STRUCTURED-TASK-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`,
  `GAP-NATIVE-BACKEND-ABI-001`, and ownership/Device gaps remain open.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Node-conformance
protocol, fixture/oracle, state/timing/input, Fault/fallback, replay/target,
diagnostic, and evidence boundaries. It sorts by explicit local rank,
rejects duplicates, compares canonical opaque bytes for forward/reverse input
order, and uses an observation-only tag. No runner, manifest, oracle,
diagnostic, CLI/LSP action, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned conformance protocol, fixture
manifest, oracle, expected state/output/Fault evidence, exact tick/rate/input/
deadline/fallback/restart behavior, static schedule and target evidence,
replay identity/corruption/privacy/migration, bridge/ownership rules, stable
bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures. Seed
behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

NODE-5307 implementation, conformance runner/manifest/oracle, fixture
protocol, Node behavior, replay/target evidence, diagnostics, CLI/LSP/runtime
protocols, and support claims remain deferred until accepted authority and
executable offline evidence exist. No placeholder runner or public API is
created.
