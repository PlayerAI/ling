# DEC-0251: Current Trait IDE surface evidence / 当前 Trait IDE 表面证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：type-system engineering
> 相关 RFC/缺口：RFC-0022 | DEC-0059 | GAP-TRAIT-COHERENCE-001 | GAP-LSP-TRANSACTION-PROTOCOL-001 | TRAIT-1308
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes an internal evidence gate over the completed Trait
projection and lookup children. It does not authorize an editor protocol.

本决定授权对已完成的 Trait 投影与查找子任务建立内部证据门禁；它不授权编辑器协议。

## Question

How should the repository prove the current Trait IDE data boundary without
promoting it to hover, navigation, completion, rename, diagnostics, repairs,
or LSP transaction support?

## Decision

1. Add `docs/testing/TRAIT-IDE-STATUS.md` with one Experimental projection,
   one Internal lookup surface, and six `BlockedSpec` editor surfaces.
2. Add `cargo xtask trait-ide verify` to validate the matrix, RFC/decision,
   current source and tests, implementation reports, protocol registration,
   and required parent/child status states.
3. Run the verifier in the always-on `governance-authority` CI gate.
4. Fail closed if projection/lookup evidence disappears, a child ceases to be
   `Done`, or the parent/public surfaces are promoted without new Accepted
   authority.
5. Keep hover, navigation requests, completion, rename, diagnostics, repairs,
   and LSP transaction/version behavior deferred under the registered gaps.
6. The gate is deterministic, read-only, and offline and adds no public API,
   protocol, diagnostic, transaction, network request, install, or mutation.

## Conformance plan

- Run `cargo xtask trait-ide verify` and require eight surfaces, seven evidence
  files, and three parent/child status assertions.
- Mutate a matrix state, parent state, child presence, and evidence marker in
  focused tests and require fail-closed internal governance errors.
- Run the `ling-semantic` Trait projection and lookup tests independently.
- Run workspace, CI, governance, protocol, support, status, traceability,
  Clippy, formatting, deterministic, and offline gates.

## Compatibility impact

Internal evidence and documentation only. Ling syntax, semantics, public
diagnostics, core schemas, Semantic IDs, packages, dependencies,
CLI/LSP/DAP/runtime behavior, bytecode, VM, ABI, Unicode 17.0.0, protocol
states, support states, and public APIs are unchanged. No migration is needed.

## Unresolved alternatives

Trait hover rendering; definition/implementation navigation; completion and
resolve; identity-preserving rename; bilingual diagnostics and safe repairs;
URI, position, snapshot, version, cancellation, Workspace Edit, Semantic
Transaction, cross-package editor fixtures, and Stable lifecycle remain
deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
