# Ling fault-injection matrix

Status: implemented-surface evidence inventory (2026-08-23)

This matrix records the `REL-6602` scenarios against the implementation that
actually exists. “Covered” means a deterministic test exercises an accepted
Seed boundary. “Partial” means the safe behavior exists but there is no
portable fault injector for the exact event. “Deferred” means the scenario
belongs to a Future/Unsupported protocol and must not be simulated by a
placeholder API.

| Scenario | State | Current evidence | Missing evidence / owner |
| --- | --- | --- | --- |
| cache corruption | Covered | `ling-cache` treats malformed/version-mismatched envelopes as safe misses; `ling-db` reconstructs a corrupted line-index cache from the current source. | Cross-process persistence policy remains outside the disposable cache decision; query/cache maintainers own future expansion. |
| disk full | Covered | The private lock-persistence seam injects `StorageFull` after a partial temporary-file write; `L-IO-0002` reports `operation=write` and `io_kind=storage_full`, the prior lock remains exact, and the temporary file is removed. | OS-level disk exhaustion and cross-platform filesystem matrices remain release evidence owned by package/project maintainers. |
| interrupted write | Covered | The private seam injects `Interrupted` after complete write/sync and before replacement; `L-IO-0002` reports `operation=replace`, the prior lock remains exact, and the temporary file is removed. | Abrupt process termination and directory-durability guarantees remain outside this library-level injected interruption. |
| process crash | Deferred | No process-crash injector or restart protocol is part of the Seed implementation. | Accepted process/recovery contract and crash artifact policy; runtime maintainers. |
| network partition | Deferred | Project graph resolution is deliberately local and has no network/process execution surface. | A package-service protocol and offline/partition semantics; package maintainers. |
| remote duplicate / reorder | Deferred | No remote event or transport protocol exists. | Accepted event identity/order/divergence contract; protocol/runtime maintainers. |
| device lost / OOM | Deferred | VM frame/heap resource-limit tests cover the implemented interpreter/VM boundary, not a device runtime. | Accepted device lifecycle, resource, and recovery semantics; device maintainers. |
| actor restart storm | Deferred | Actor/task execution is not in the Seed language/runtime surface. | Accepted Actor supervision/restart and mailbox protocol; concurrency maintainers. |
| replay truncation | Deferred | No replay reader, writer, event envelope, or player exists. | Accepted replay schema, checksum, truncation, and divergence rules; replay maintainers. |
| invalid proof / evidence | Deferred | No proof checker or evidence bundle decoder is implemented for Seed. | Accepted proof/evidence schema and fail-closed checker behavior; verification maintainers. |
| language-server crash / restart | Deferred | LSP/DAP is explicitly unsupported/future; there is no server process to restart. | Accepted LSP transaction/lifecycle contract and executable server; editor maintainers. |

## Deterministic fault policy

The current evidence uses fixed byte corruptions, bounded temporary test
directories, and repeatable input order. It does not use randomness, wall
clock, host paths, network access, device state, or process scheduling as a
language or protocol oracle. Any future injector must record:

- the fault point and precondition;
- whether the operation is retried, rolled back, committed, or converted to a
  stable diagnostic;
- cleanup and partial-output rules;
- source/schema/protocol version and compatibility impact;
- bounded resources and timeout;
- deterministic replay input and cross-process/platform expectations; and
- a named triage owner and retained minimized reproduction.

No fault scenario may turn an unchecked AST, an unresolved future protocol, or
host allocation/order into Ling semantics. Public diagnostics must remain
registered `L-<DOMAIN>-<NUMBER>` codes with original UTF-8 byte spans and
bilingual messages.

## Reproduction evidence

The scenario/state table is checked for drift by the internal governance gate:

```text
cargo xtask fault verify
```

The implemented Seed checks are run by the normal locked offline suite:

```text
cargo test -p ling-cache --locked --offline
cargo test -p ling-db --locked --offline
cargo test -p ling-project --test lockfile_fixtures --locked --offline
cargo test -p ling-project lockfile::tests --locked --offline
cargo test --workspace --all-targets --locked --offline
```

The eight deferred future scenarios (process, remote, device, concurrency,
replay, proof/evidence, and editor lifecycle)
remain explicit gaps. No public fault-injection command, network adapter,
device simulator, actor API, replay tool, proof checker, or LSP server is
created by this inventory.
