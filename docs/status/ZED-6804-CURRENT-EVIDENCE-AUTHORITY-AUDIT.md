# ZED-6804-CURRENT-EVIDENCE Authority Audit

- Parent: `ZED-6804` — DAP status
- Child: `ZED-6804-CURRENT-EVIDENCE` — Current DAP observation evidence
- Release: G6
- Decision: `Done` is authorized only for this bounded internal child by
  Accepted `DEC-0244`; the parent remains `BlockedSpec`.

## Authority and drift

Accepted DEC-0051 protects a nine-row negative DAP inventory and three
authority audits. Accepted DEC-0144/0145/0146 later authorized three test-only
observation children with sixty provisional adapter, Zed-registration, and
staged-capability boundaries each. The G6 matrix did not yet reference or
validate those completed evidence suites.

DEC-0244 authorizes current-evidence integration. The opaque observation bytes
are explicitly not DAP messages, extension metadata, debugger capabilities,
diagnostics, schemas, or public protocols.

## Authorized implementation

- Correct matrix evidence cells to reference the three Accepted observation
  children without changing any support state.
- Validate three observation tests and three implementation reports in addition
  to the original authority audits.
- Require exact count/tag/order/duplicate/no-authority and parent-blocked
  markers, with a focused negative test.
- Preserve the non-blocking DAP release policy and every Preview/Stable gate.

## Explicit exclusions

No adapter/process, DAP frame, debugger command, runtime hook, source-map bridge,
launch/attach session, breakpoint/step, stack/scope/variable projection,
Task/Actor view, Zed registration, extension package, acquisition, diagnostic,
schema, protocol, platform claim, or support promotion is created.

No language semantic, source, diagnostic, schema, Semantic ID, dependency,
CLI/LSP/DAP runtime, Unicode, protocol, support-state, or public API change.
