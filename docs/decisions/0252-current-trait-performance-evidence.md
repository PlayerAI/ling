# DEC-0252: Current Trait performance evidence / 当前 Trait 性能证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：type-system engineering
> 相关 RFC/缺口：RFC-0005 | DEC-0026 | DEC-0068 | GAP-LSP-TRANSACTION-PROTOCOL-001 | TRAIT-1309
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a read-only evidence gate over the accepted bounded
termination facts. It does not define a performance or cancellation contract.

本决定授权对已接受的有界终止事实建立只读证据门禁；它不定义性能或取消契约。

## Question

How should the repository prove its current solver termination boundary
without treating correctness tests or host timing as a complete production
performance contract?

## Decision

1. Add `docs/testing/TRAIT-PERFORMANCE-STATUS.md` with three Internal
   termination surfaces and five `BlockedSpec` production surfaces.
2. Add `cargo xtask trait-performance verify` to validate the exact matrix,
   solver markers, authority, audits, report, and parent/child task states.
3. Run the verifier in the always-on `governance-authority` CI gate.
4. Fail closed if the 64-level/cycle/source-independence evidence disappears
   or if blocked performance surfaces are promoted without Accepted authority.
5. Preserve the exact RFC-0005 nesting boundary and keep production
   integration, budgets, benchmarks, cancellation, and public formats deferred.
6. The gate is deterministic, read-only, and offline and runs no benchmark,
   user program, network request, install, or system mutation.

## Conformance plan

- Run `cargo xtask trait-performance verify` and require eight surfaces, seven
  evidence files, and two parent/child status assertions.
- Mutate a matrix state, parent state, child presence, and solver marker in
  focused tests and require fail-closed internal governance errors.
- Run the `ling-types` cycle/depth/source-independence solver tests.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Internal evidence and documentation only. Ling syntax, semantics, diagnostics,
schemas, Semantic IDs, packages, dependencies, CLI/LSP/DAP/runtime behavior,
bytecode, VM, ABI, Unicode 17.0.0, protocol states, support states, and public
APIs are unchanged. No migration is required.

## Unresolved alternatives

Production obligation integration; deterministic work units and exhaustion;
deep-chain/diamond/failure/cross-package benchmarks; metric, variance,
environment, and threshold policy; LSP cancellation/version/stale-result
behavior; public evidence schemas; and Stable lifecycle remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
