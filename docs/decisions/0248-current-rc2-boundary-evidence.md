# DEC-0248: Current RC2 boundary evidence / 当前 RC2 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：release engineering
> 相关 RFC/缺口：DEC-0055 | DEC-0247 | RC-6904
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes composing the current RC3→RC1→RC0 inventory chain
and correcting the protocol total in the RC2 readiness gate. It does not
authorize a blocker fix, candidate, or Final decision.

本决定授权 RC2 就绪门禁组合当前 RC3→RC1→RC0 盘点链并修正协议总数；它不授权阻断修复、候选版本或 Final 决策。

## Question

How should RC2 keep its predecessor and protocol evidence current without
turning a passing repository inventory into blocker classification or change
approval?

## Decision

1. Amend the bounded DEC-0055 gate by calling
   `rc3_verification::check_repository`; RC3 composes RC1, which composes RC0.
2. Correct the affected-protocol row to the validated 27-record inventory.
3. Require explicit markers that the bounded chain passes and all three
   predecessor release parents remain `BlockedSpec`.
4. Fail closed with internal `GOV-RC2-CHANGE-CONTROL-0011` errors on upstream
   or protocol-evidence drift.
5. Keep all six evidence-class states unchanged and retain blocker taxonomy,
   regression, risk, impact, matrix, candidate, reviewer, and Final exits.
6. The verifier remains deterministic, read-only, and offline and creates no
   source fix, blocker status, candidate, artifact, tag, network request, or
   system mutation.

## Conformance plan

- Run `cargo xtask rc2 verify` and require six classes, seven audits, and one
  composed upstream gate.
- Run RC3, RC1, RC0, and protocol inventory verification independently.
- Replace current upstream/protocol markers with stale facts in a focused test
  and require fail-closed internal governance errors.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Documentation correction and internal evidence composition only. Ling syntax,
semantics, diagnostics, schemas, Semantic IDs, packages, dependencies,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0, protocol states, support states,
and public APIs are unchanged. No migration is required.

## Unresolved alternatives

Blocker/P0/P1 taxonomy and disposition; candidate baseline; regression, risk,
impact, and rollback records; full relevant matrix; immutable candidate and
provenance; reviewer approval; candidate regeneration; and Final/Go remain
deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
