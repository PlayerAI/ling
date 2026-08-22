# NODE-5305-OBSERVATION Authority Audit — Native Node Runtime Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0189` permits only test-local Native Node vocabulary. It does
not authorize a Native backend, Native IR, ABI/layout, target package,
ownership/static-memory model, timer/watchdog runtime, lifecycle state
machine, diagnostics, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:266-275` is a
  non-normative Native Node proposal.
- `docs/status/NODE-5305-AUTHORITY-AUDIT.md` records the missing Native ABI,
  ownership, Critical, Kernel/Device, target, timer, watchdog, and lifecycle
  authority.
- `docs/governance/support-matrix.toml` keeps `BACKEND-NATIVE` Unsupported.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`,
  `GAP-KERNEL-DEVICE-001`, and `GAP-CRITICAL-PROFILE-001` remain open.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Native Node
runtime boundaries covering checked inputs, Native IR and ABI/layout,
ownership and static memory, timing/lifecycle, target/evidence,
diagnostics, and fixtures. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No backend, runtime, target, schema,
diagnostic, CLI/LSP action, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define Native IR validity, ABI/layout/calling
conventions, target/toolchain identity, ownership/drop/region and bounded
memory rules, timer/watchdog and schedule semantics, startup/shutdown and
safe-state transitions, Critical/Kernel/Device placement, unsupported-target
behavior, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
target/differential fixtures. Seed behavior, Semantic IDs, UTF-8 spans,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

NODE-5305 implementation, Native backend/IR, ABI and target integration,
ownership/static-memory enforcement, timer/watchdog/lifecycle runtime,
diagnostics, CLI/LSP/runtime protocols, and support claims remain deferred
until RFC-K502/RFC-0011 (or Accepted replacements), the listed gaps, and
executable offline target evidence exist. No placeholder Native API is
created.
