# DEC-0235: Lock-persistence fault injection / 锁文件持久化故障注入

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: project reliability
> 相关规范/缺口：`RFC-0002` | `DEC-0042` | `REL-6602`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a private lock-persistence seam and deterministic
tests for storage exhaustion and interruption after sync but before replace.
It does not expose a fault API or claim operating-system crash durability.

本决定授权增加私有锁文件持久化 seam，并确定性测试存储耗尽以及 sync 完成后、replace
之前的中断；它不暴露故障 API，也不声称具备操作系统崩溃持久性。

## Question

How should the implemented `ling.lock/1` writer prove RFC-0002 failure
atomicity for the two currently Partial REL-6602 write scenarios?

## Decision

1. Factor lock publication behind a private `LockPersistence` boundary with
   exactly two responsibilities: complete write-and-sync of the adjacent
   temporary file, and commit by replacement. The production implementation
   retains the existing `File::write_all`/`sync_all`/`fs::rename` sequence.
2. Inject `StorageFull` after a deterministic partial temporary-file write.
   The operation fails as registered `L-IO-0002` with `operation = "write"`
   and `io_kind = "storage_full"`; the prior target is unchanged and the
   temporary file is removed.
3. Inject `Interrupted` after complete write/sync and immediately before
   replacement. The diagnostic uses `operation = "replace"` and
   `io_kind = "interrupted"`; the prior target is unchanged and the temporary
   file is removed.
4. `storage_full` is a compatible value in the existing `io_kind:string`
   field. No code, field, field type, severity, message, or span changes.
5. The seam and injectors remain private to `ling-project`; no public API,
   environment variable, CLI flag, global hook, randomness, or host path enters
   Ling behavior.
6. These tests promote only REL-6602 `disk full` and `interrupted write` from
   Partial to Covered at the library injection boundary. OS-level disk
   exhaustion, abrupt process termination, directory fsync, and cross-platform
   crash recovery remain unclaimed.
7. Parent `REL-6602` remains `BlockedSpec` for the eight future/process fault
   scenarios and the final G6 release gate.

## Normative basis

- RFC-0002 §6 requires lock creation/replacement only after full graph
  validation; §7 requires failed reads/resolutions to publish no partial graph
  and not replace a prior lock.
- `L-IO-0002` already allocates bilingual lock I/O failure diagnostics with
  `io_kind:string` and `operation:string` facts.
- DEC-0042 records the exact eleven-scenario matrix and permits later Accepted
  fault seams to become executable evidence.

## Conformance plan

- Inject a partial write followed by `StorageFull`; assert code/facts, exact
  preservation of the old target, and zero adjacent temporary files.
- Inject `Interrupted` after sync and before replace; assert the same atomicity
  and cleanup properties with the replace operation.
- Retain all canonical lock, update/locked mode, corrupt-input, offline, and
  no-network/process fixtures.
- Update the exact fault matrix to three Covered, zero Partial, eight Deferred.
- Run project, diagnostic-registry, fault, governance, status, workspace,
  Clippy, formatting, deterministic, and offline gates.

## Compatibility impact

The existing lock bytes, target filename, temporary naming, write/sync/rename
production sequence, error code, bilingual messages, fact types, source span,
package identities, graph identities, CLI, and Unicode 17.0.0 remain unchanged.
Actual host `StorageFull` errors become more precise (`storage_full` rather
than the fallback `other`) within the existing open string field.

## Unresolved alternatives

Directory sync; Windows replacement durability; OS/filesystem quota harnesses;
power-loss and abrupt-kill tests; retry policy; retained failed temporaries;
cross-process recovery; and fault seams for process, remote, device, Actor,
replay, evidence, and editor systems remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
