# ZED-6804 DAP Status

Status: DAP boundary inventory (2026-08-22). The purpose of this document is
to state whether debugger support blocks the 1.0 language/editor surface. It
does not implement or register a Debug Adapter Protocol (DAP) server.

## Release policy

The G6 plan says an incomplete debugger must not block the language and basic
editor support. That policy is adopted as a planning constraint: DAP is not a
1.0 blocker while it remains unavailable, but it also cannot be labeled
`Preview` or `Stable` without a versioned protocol, executable adapter, and
fixtures. The current state is therefore `Unavailable / Future`, not a usable
debugger.

## Current matrix

| DAP surface | Repository evidence | Current state | Required before Preview |
| --- | --- | --- | --- |
| Adapter process and stdio framing | DAP-3601 authority audit; no adapter crate, executable, command, reader, or writer | Unavailable | Accepted DAP wire/lifecycle contract, bounded framing, diagnostics, and implementation |
| Zed debugger registration | DAP-3602 authority audit; no extension manifest or debugger configuration | Unavailable | Accepted Zed registration/discovery/version policy and extension artifact |
| Launch/attach/session | No launch, attach, capability negotiation, or session implementation | Future | Accepted engine-neutral launch/attach/security semantics and fixtures |
| Breakpoints/continue/step | No runtime hooks or source-map bridge | Future | VM/Native debug metadata, deterministic breakpoint/step behavior, and tests |
| Stack/scopes/variables | No debugger projection or ownership/managed-value contract | Future | Accepted value visibility, lifetime, redaction, and resource limits |
| Fault/stop/exit | Runtime Fault and VM control evidence is not a DAP protocol | Partial foundation only | Accepted mapping, committed-effect policy, cancellation, and protocol schema |
| Task/Actor views | Structured Task/Actor semantics remain future | Unsupported | G2 authority and runtime implementation before any view |
| Protocol inventory and compatibility | No DAP protocol record, version, migration, or golden corpus | Unavailable | Registered versioned protocol, corruption/compatibility suite, and release notes |
| Security and platform support | No debugger process, permissions, target artifacts, or acquisition path | Unavailable | Threat model, sandbox/permission policy, per-platform artifacts, and offline evidence |

## Evidence and verification

Existing DAP-3601/3602/3603 audits document the missing adapter, registration,
runtime hooks, source-map/identity, launch/attach, and capability contracts.
Existing VM bytecode/source-map tests are implementation evidence for the VM
library only; they do not provide a DAP transport or editor debugger.

Repository gates that verify the negative boundary are:

```text
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

No DAP process, network request, extension registration, debugger button, or
system configuration was exercised by this audit.

## Promotion rules

DAP may be marked `Preview` only after DAP-3601 through DAP-3603 (or an
accepted replacement) provide a versioned adapter, registration contract,
source-map/identity rules, launch/attach/step/breakpoint/stack/variable/Fault
fixtures, crash/cancellation/resource limits, security evidence, and
cross-platform/offline validation. `Stable` requires an additional release
support matrix, compatibility policy, migration/rollback evidence, and
independent verification.

Until then, the extension must expose no misleading debugger controls and the
unavailable state must not block language checking, running, grammar-only
editing, or other independently supported capabilities.

No placeholder command, protocol, backend, schema, migration promise, or
stale legacy name is added here.
