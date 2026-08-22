# BND-5203-OBSERVATION Authority Audit — Memory-Budget Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0183` permits only test-local memory-budget vocabulary. It does
not authorize a memory model, analyzer, allocation/ownership semantics,
target/ABI binding, proof or estimate contract, diagnostics, CLI/LSP actions,
or support claims.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:169-182` is a
  non-normative memory-budget proposal.
- `docs/ROADMAP-1.0.md:118` and `:433-498` sequence bounded allocation after
  the required concurrency, resource/Native, and lowering gates.
- `docs/status/BND-5203-AUTHORITY-AUDIT.md` records missing RFC-K504 and the
  dependent Critical, ownership, mailbox, Kernel/Device, and Native/ABI
  authorities.
- RFC-0014/0015 limits protect bytecode input or one VM execution; they are
  not source-level logical memory guarantees.

## Current implementation evidence

The observation adds one isolated test with sixty explicit memory-budget,
allocation/lifetime, queue/task/device, proof/target, fallback, diagnostic,
and fixture boundaries, deterministic local ordering, duplicate rejection, and
an opaque observation tag. No production analyzer, memory model, target
schema, diagnostic, CLI/LSP option, runtime, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define units and layout, ownership/regions/aliasing,
lifetime/drop and peak/path accounting, queue/task/device rules, proof states,
target/compiler identity and migration, host-safety versus logical guarantees,
fallback/fault semantics, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
offline fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

BND-5203 implementation, memory-budget analysis, allocation/ABI model,
diagnostics, CLI/LSP/evidence protocol, and public support remain deferred
until RFC-K504 (or an Accepted replacement), the dependent Critical/resource
authorities, and executable offline evidence exist.
