# ZED-6804 Authority Audit

- Task: `ZED-6804` — DAP status
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:473-475`
- Release: G6
- Status: `BlockedSpec`; DAP remains unavailable and non-blocking.

## Decision

`ZED-6804` remains `BlockedSpec`. The release policy is recorded: incomplete
debugger support must not block language/editor 1.0 support, and no debugger
button or adapter may be presented as if it worked. The current DAP state is
`Unavailable / Future`, not `Preview` or `Stable`, because DAP-3601 through
DAP-3603 have no accepted protocol implementation, Zed registration, runtime
debug hooks, or executable evidence.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:473-475` is a non-normative release policy;
  it does not authorize DAP wire fields, debugger semantics, or a Preview
  protocol.
- Existing DAP-3601/3602/3603 authority audits and the G3 execution plan
  identify the missing adapter, registration, runtime/source-map, lifecycle,
  security, and capability contracts; they remain `BlockedSpec`.
- `docs/governance/support-matrix.toml` and the protocol inventory do not
  register a DAP protocol or debugger support; VM control/source-map evidence is
  explicitly experimental and not an editor debugger contract.
- `docs/ROADMAP-1.0.md` requires accepted semantics, protocol/schema
  compatibility, security, deterministic/offline behavior, and release
  evidence before Stable support.
- `AGENTS.md` requires checked Typed Core boundaries, original UTF-8 spans,
  Unicode 17.0.0, bilingual diagnostics, and no placeholder public APIs or
  stale legacy names.

## Evidence and gaps

`docs/testing/DAP-STATUS.md` maps adapter, registration, launch/attach,
breakpoint/step, stack/variable, Fault/stop, Task/Actor, protocol, and security
surfaces. It records the existing VM evidence as only a partial foundation and
the four repository gates used to verify the negative boundary.

The missing evidence is an Accepted DAP protocol and lifecycle, executable
adapter, Zed registration, VM/Native debug metadata, deterministic fixtures,
security/resource policy, platform/offline artifacts, and independent
verification. DAP remains intentionally non-blocking for language/editor
support until those conditions exist.

The internal `cargo xtask dap verify` command protects the nine-surface
inventory and three DAP authority-audit markers without registering DAP,
starting a debugger, or exposing controls.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI commands, package behavior, runtime, editor protocol, dependencies, or
public API. It preserves `ling`/`.ling`, Unicode 17.0.0, original UTF-8 spans,
deterministic/offline Rust validation, and the explicit unsupported DAP
boundary.

No debugger process, extension registration, network request, system
configuration, or public protocol was created. Preview/Stable promotion,
debugger controls, acquisition, migration, and release artifacts remain
deferred.
