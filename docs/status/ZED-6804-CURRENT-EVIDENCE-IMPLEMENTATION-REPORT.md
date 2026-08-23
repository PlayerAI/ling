# ZED-6804-CURRENT-EVIDENCE Implementation Report

## Result

The DAP status inventory now validates all three completed test-only debugger
observation children: 180 provisional boundaries across adapter protocol,
Zed registration, and staged capabilities. Every DAP status remains unchanged,
and DAP remains unavailable, future, or unsupported rather than Preview.

The parent `ZED-6804` remains `BlockedSpec`. No debugger is claimed.

## Implementation

- `docs/testing/DAP-STATUS.md` links each relevant status row to the bounded
  DEC-0144/0145/0146 observation evidence and explains why labels are not
  behavior.
- `tools/xtask/src/dap_status.rs` retains the nine exact states and three
  authority audits, then validates three observation tests and three reports.
- Current checks bind each 60-entry inventory, private evidence tag,
  completeness/order test, duplicate-rejection test, no-authority test, report
  scope, and parent `BlockedSpec` handoff.
- A focused negative test rejects missing observation evidence.

## Executed evidence

The three `ling-types` observation integrations pass nine tests total. They
prove exact inventory coverage, deterministic forward/reverse opaque bytes,
duplicate rejection, and explicit absence of protocol authority.

Full repository gates provide offline, deterministic, governance, formatting,
and lint evidence. No DAP or Zed process, debugger control, runtime mutation,
network request, installation, or platform integration was executed.

## Compatibility and deferrals

No Ling, runtime, editor, or protocol behavior changes. DAP framing/lifecycle,
debugger semantics and runtime hooks, source/binary identity, Zed registration,
security/resource policy, acquisition, cross-platform/offline debugger fixtures,
migration, Preview/Stable promotion, and release support remain deferred.
