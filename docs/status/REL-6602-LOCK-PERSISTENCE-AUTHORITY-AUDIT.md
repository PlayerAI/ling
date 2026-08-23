# REL-6602-LOCK-PERSISTENCE Authority Audit

- Task: `REL-6602-LOCK-PERSISTENCE` — Lock-persistence fault injection
- Parent: `REL-6602` — Fault Injection
- Decision: Accepted `DEC-0235`
- Release: G6
- Status: authorized bounded implementation

## Authority conclusion

RFC-0002 §§6–7 require a failed lock operation not to replace a prior lock.
Accepted DEC-0235 authorizes a private two-stage persistence boundary and
deterministic `StorageFull`/`Interrupted` injection around the existing
write-sync-replace implementation.

This closes the library-level disk-full and interrupted-write rows only.
Parent `REL-6602` remains `BlockedSpec` for OS crash guarantees and eight
future/process fault scenarios.

## Authorized implementation

1. Preserve the existing host write, sync, and rename sequence behind a
   private interface.
2. Inject partial-write storage exhaustion and post-sync/pre-replace
   interruption.
3. Assert exact old-target preservation, temporary cleanup, and existing
   `L-IO-0002` code/facts.
4. Update the fault matrix and drift verifier to three Covered rows.

## Explicit exclusions

No public injector, CLI/environment switch, retry policy, directory-sync
guarantee, process killer, network/device/Actor/replay/evidence/editor system,
diagnostic allocation, schema, dependency, or runtime behavior is added.
