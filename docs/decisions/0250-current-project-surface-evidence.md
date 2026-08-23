# DEC-0250: Current project-surface evidence / 当前工程表面证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：package engineering
> 相关 RFC/缺口：RFC-0024 | DEC-0058 | DEC-0083 | GAP-PROJECT-CLI-INTERFACE-001 | PRJ-1107
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes one internal, read-only evidence gate that composes
the three completed PRJ-1107 children and preserves the unresolved public
project surface. It does not authorize new project behavior.

本决定只授权一个内部只读证据门禁，用于组合 PRJ-1107 的三个已完成子任务，并保持
公开工程表面的未决状态；它不授权新的工程行为。

## Question

How should the repository prove the current project CLI/API boundary without
mistaking three bounded implementations for completion of the full PRJ-1107
surface?

## Decision

1. Add `docs/testing/PROJECT-CLI-STATUS.md` with an exact inventory of eight
   project surfaces: one Experimental graph-check command, two Internal
   library boundaries, and five `BlockedSpec` public surfaces.
2. Add `cargo xtask project verify` to validate the matrix, implementation
   sources, focused tests, reports, and the required parent/child task states.
3. Run the verifier in the always-on `governance-authority` CI gate.
4. Fail closed when an implemented marker disappears, a bounded child is no
   longer `Done`, or the PRJ-1107 parent/public surfaces are promoted without
   an Accepted authority change.
5. Keep public semantic project check, project run/test/build, artifacts,
   workspace/member selection, and implicit discovery deferred under
   `GAP-PROJECT-CLI-INTERFACE-001`.
6. The evidence gate is deterministic, read-only, and offline. It creates no
   public API, protocol, diagnostic, artifact, lock mutation, network request,
   install, or system change.

## Conformance plan

- Run `cargo xtask project verify` and require eight surfaces, twelve evidence
  files, and four parent/child status assertions.
- Replace an implemented state, parent state, matrix state, and source marker
  in focused tests and require fail-closed internal governance errors.
- Run the focused `ling-cli` project-check, `ling-project`, and `ling-db`
  suites independently.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Internal evidence and documentation only. Ling syntax, semantics, diagnostics,
schemas, Semantic IDs, packages, dependencies, CLI/LSP/DAP/runtime behavior,
bytecode, VM, ABI, Unicode 17.0.0, protocol states, support states, and public
APIs are unchanged. No migration is required.

## Unresolved alternatives

Public semantic project checking; compiler-host lifecycle; project run, test,
and build behavior; entry selection; capabilities; workspace/member selection;
implicit discovery; artifact targets/layout/reproducibility; result schemas;
and Stable project compatibility remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
