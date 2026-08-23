# DEC-0244: Current DAP boundary evidence / 当前 DAP 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：debugger quality
> 相关 RFC/缺口：DEC-0051 | DEC-0144 | DEC-0145 | DEC-0146 | ZED-6804
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes incorporating the three completed test-only debugger
observation suites into the DAP status gate without implementing DAP.

本决定授权将三个已完成的测试专用调试器观察套件纳入 DAP 状态门禁，但不实现 DAP。

## Question

How should the G6 DAP status inventory include the 180 provisional boundaries
now checked under DEC-0144/0145/0146 while preserving the exact conclusion that
no adapter, Zed registration, debugger semantics, or public protocol exists?

## Decision

1. Amend DEC-0051 Decision item 2 by retaining its three authority-audit files
   and adding six current observation test/report files.
2. Keep all nine DAP surface states unchanged: four `Unavailable`, three
   `Future`, one `Partial foundation only`, and one `Unsupported`.
3. Record the three test-local inventories as 60 provisional boundaries each,
   with fixed local order, duplicate rejection, deterministic forward/reverse
   opaque evidence, and explicit no-authority assertions.
4. Update the matrix evidence cells to distinguish completeness vocabulary for
   future specifications from executable debugger behavior.
5. Extend `cargo xtask dap verify` to fail closed when any observation count,
   tag, completeness/order test, duplicate test, no-authority test, report
   scope, or parent `BlockedSpec` marker disappears.
6. The verifier remains read-only and offline. It does not run a debugger,
   define DAP framing, start a process, register Zed, read settings, allocate a
   diagnostic, modify runtime behavior, or create a public protocol.
7. Parent `ZED-6804` and DAP-3601/3602/3603 remain `BlockedSpec`; DAP remains
   non-blocking for independently supported language/editor work.

## Conformance plan

- Run all three `ling-types` observation integration tests and require nine
  passing tests across the 180 exact provisional boundaries.
- Validate nine exact status rows, three authority audits, and six current
  observation files.
- Remove representative observation markers in a focused test and require a
  fail-closed internal governance error.
- Run DAP, workspace, CI, governance, status, support, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Documentation correction and stronger internal evidence validation only. Ling
semantics, source syntax, diagnostics, public schemas, Semantic IDs, packages,
dependencies, CLI/LSP/DAP behavior, runtime, Unicode 17.0.0, protocols, support
states, and editor APIs are unchanged. No migration is required.

No DAP adapter, command, framing, lifecycle, debugger capability, runtime hook,
Zed registration, extension artifact, public diagnostic, network behavior, or
Preview/Stable support claim is added.

## Unresolved alternatives

DAP wire/framing/lifecycle; launch/attach/session semantics; breakpoints/step;
stack/scope/variable/Fault mappings; Task/Actor views; runtime/source-map and
binary identity; security/resource policy; Zed registration; acquisition;
platform/offline fixtures; migration; and release support remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
