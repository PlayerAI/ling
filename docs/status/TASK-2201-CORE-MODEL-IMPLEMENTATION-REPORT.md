# TASK-2201-CORE-MODEL Implementation Report

## Scope

This child implements Accepted `DEC-0091` in the publish-disabled
`ling-concurrency` crate. It is a deterministic checked-data boundary only;
it does not implement Structured Task syntax, compiler publication, runtime
execution, scheduling, cancellation, cleanup, Fault handling, bytecode, VM,
or a public protocol.

## Implemented

- Nonzero typed identities for scopes, tasks, opaque checked bodies,
  suspension points, cancellation tokens, and cleanup regions.
- Immutable `TaskNode` values with parent identity, suspension evidence, and
  optional detach metadata.
- `TaskCore::new` validation for one parentless root, duplicate-free task and
  suspension identities, known acyclic parents, and complete detach evidence.
- Canonical identity ordering and path/span-free `ling.task-core/0` bytes.
- Original `Span` retention as evidence only; source evidence never enters
  canonical identity.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --offline` — 4 tests passed

The locked rerun and full repository gates are required before the milestone
commit is finalized.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain
unchanged. The crate is not publishable and does not register a protocol.
`TASK-2201` remains `BlockedSpec` for all language and runtime semantics.
