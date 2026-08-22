# DEC-0042: Seed fault-matrix drift gate / Seed 故障矩阵漂移门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: reliability-engineering  
> Related authority/gap: `DEC-0022`, `RFC-0002`, `RFC-0020`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `REL-6602-SEED` child. It does not
complete the G6 fault-injection release gate or authorize crash, network,
device, Actor, replay, proof/evidence, or LSP recovery behavior. The parent
`REL-6602` remains `BlockedSpec` for those surfaces and for G6 release exits.

## Question

The repository has a reviewed fault-injection matrix that distinguishes
Covered, Partial, and Deferred Seed evidence. Without a drift check, scenario
names or states could change while the corresponding authority and tests stay
unchanged. An internal verifier can protect the inventory without fabricating
an injector or recovery contract.

## Decision

1. `cargo xtask fault verify` is an internal governance command. It validates
   the exact eleven scenario names and their accepted states in
   `docs/testing/FAULT-INJECTION.md`: one `Covered`, two `Partial`, and eight
   `Deferred` rows.
2. The verifier rejects duplicate, missing, or unexpected rows, state drift,
   and removal of the matrix policy phrases that require a fault point and
   precondition, retry/rollback/commit behavior, cleanup, deterministic replay
   input, and a named triage owner. It fails closed with internal
   `GOV-FAULT-*` messages.
3. The command validates documentation only. It does not inject faults, run
   crash simulators, define retry/rollback semantics, emit a public diagnostic
   or protocol, or convert Partial/Deferred rows into implementation claims.
   Existing Seed tests and future accepted seams remain the execution evidence.
4. The command is included in the existing Seed reproducibility CI gate. Any
   future scenario requires its own accepted fault seam, resource policy,
   deterministic oracle, and retained failure evidence before changing the
   matrix or adding an implementation.

## Conformance plan

- Run `cargo xtask fault verify` offline and assert the eleven rows and state
  counts are deterministic.
- Mutate an isolated matrix row in the xtask unit fixture and verify missing
  or changed states fail closed.
- Run the existing locked cache, database, project-lock, and VM fault tests;
  do not treat the matrix gate as crash or sanitizer evidence.
- Repeat independent processes and verify no language, runtime, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` documentation validator and CI preflight.
  Ling syntax, Checked Core, runtime, bytecode, diagnostics, schemas, Semantic
  IDs, public protocols, dependencies, and Unicode 17.0.0 behavior are
  unchanged.
- No fault injector, recovery API, diagnostic allocation, or public support
  claim is introduced.

## Unresolved alternatives

Portable write/crash seams, process restart, network partitions, remote event
ordering, device loss/OOM, Actor supervision, replay truncation, proof/evidence
checking, LSP restart, crash artifact retention, and cross-platform fault
oracles remain deferred to later Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
