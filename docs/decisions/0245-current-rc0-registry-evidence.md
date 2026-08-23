# DEC-0245: Current RC0 registry evidence / 当前 RC0 注册表证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：release engineering
> 相关 RFC/缺口：DEC-0052 | RC-6901
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a bounded correction and executable binding of the
RC0 matrix to the current implementation-status and protocol inventories. It
does not authorize an RC0 freeze or satisfy any release exit.

本决定授权对 RC0 矩阵进行有限修正，并以可执行方式绑定当前实现状态与协议清单；它不授权 RC0 冻结，也不满足任何发布退出条件。

## Question

How should the RC0 inventory stop reporting stale task and protocol totals
while preserving all eight `BlockedSpec` conclusions and the explicit
no-freeze/no-publication boundary?

## Decision

1. Amend the bounded DEC-0052 inventory gate by composing the existing
   `status` and `protocols` repository validators from `cargo xtask rc0 verify`.
2. Require the Feature-freeze evidence cell to match the validated task and
   `Done` totals from `docs/status/implementation-status.toml`.
3. Require the Protocol-freeze evidence cell to match the validated total and
   Stable/Experimental/Preview/Internal/Future distribution from
   `docs/governance/protocol-inventory.toml`.
4. Fail closed with internal `GOV-RC0-FREEZE-0011` evidence-drift errors when
   either rendered statement stops matching its authoritative registry.
5. Keep every RC0 criterion `BlockedSpec`; the current totals are inventory
   evidence, not a candidate identity, freeze decision, support promotion,
   protocol stability claim, artifact, or release approval.
6. The verifier remains deterministic, read-only, and offline and creates no
   tag, artifact, issue disposition, network request, or system change.

## Conformance plan

- Run `cargo xtask rc0 verify` and require eight blocked criteria, ten audit
  files, and two current-evidence checks.
- Run the focused xtask tests, including a negative status/protocol drift case.
- Run the composed status and protocol validators and the repository-wide CI,
  governance, support, traceability, Clippy, formatting, and offline tests.
- Confirm that no release tag, artifact, protocol state, support state, issue
  disposition, network resource, or system configuration changes.

## Compatibility impact

Documentation correction and stronger internal evidence validation only. Ling
semantics, source syntax, diagnostics, public schemas, Semantic IDs, packages,
dependencies, CLI/LSP/DAP behavior, runtime, Unicode 17.0.0, protocol states,
support states, and public APIs are unchanged. No migration is required.

## Unresolved alternatives

Candidate identity and change control; accepted 1.0 scope; Stable protocols;
Tier1 artifacts; P0/P1 disposition; historical corpus; security sign-off;
artifact rehearsal; complete bilingual documentation; independent review; and
the RC0 freeze itself remain deferred to Accepted authorities and executable
release evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
