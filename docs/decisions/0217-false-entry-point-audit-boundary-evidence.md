# DEC-0217: Internal False-Entry-Point Audit boundary evidence / 内部虚假入口审计边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: stabilization
> 相关规范/缺口：`DEC-0216` | `DEC-0036` | `ROADMAP-1.0` | `GAP-REGISTER` | `SUPPORT-MATRIX`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only bounded test evidence for
`STAB-6102-OBSERVATION`. It distinguishes possible false public entry points
from explicit negative evidence and verifies that plan-only root commands are
not advertised or accepted by the current CLI. It authorizes no deletion,
hiding, deprecation, migration, or new public capability.

本决定只授权 `STAB-6102-OBSERVATION` 使用有界测试证据，区分潜在虚假公共
入口与显式负面证据，并验证当前 CLI 不宣传或接受仅存在于计划中的根命令。
本决定不授权删除、隐藏、弃用、迁移或新增任何公共能力。

## Question

Which false-entry-point audit facts can be retained while the complete G6
public-surface inventory, classification, and compatibility policy remain
unresolved?

## Decision

1. `crates/ling-types/tests/false_entry_point_audit_evidence.rs` keeps sixty
   test-local categories covering public surfaces, placeholder forms, support
   states, classification actions, authority, compatibility, and evidence.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.false-entry-point-audit-observation/0`. The bytes
   are neither a public protocol nor a cleanup manifest.
3. `crates/ling-cli/tests/help.rs` verifies that the plan-only `build`,
   `query`, `patch`, `replay`, `explain`, `evidence`, `version`, `support`, and
   `features` root commands are absent from help and rejected with usage exit
   code 2. Accepted current commands remain unchanged.
4. Explicit Future, Unavailable, Unsupported, recovery, negative-fixture, and
   internal dispatch-invariant evidence is not classified as a false entry
   merely because it names an unavailable capability.
5. Existing `cargo xtask support verify` remains the authority for the current
   draft matrix's truthful negative claims. It does not prove G6 completion.
6. No file, command, API, grammar/completion item, backend/profile, default,
   diagnostic, protocol, fixture, or support state is deleted or hidden.
   Public `STAB-6102` remains `BlockedSpec`.

## Normative basis

- `DEC-0036` restricts the internal CLI command catalog to implemented current
  commands and defers future command spellings.
- `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:51-61` is a
  non-normative search list, not a cleanup manifest.
- `docs/status/STAB-6102-AUTHORITY-AUDIT.md` records missing inventory,
  classification, compatibility, migration, diagnostic, and fixture authority.
- `docs/governance/support-matrix.toml` intentionally distinguishes current,
  future, unavailable, and unsupported states in a `1.0-draft` matrix.
- `DEC-0216` records audit vocabulary without defining a Stable candidate set
  or completing parent `STAB-6101`.

## Conformance plan

- Assert all sixty local categories, deterministic rank order, duplicate
  rejection, and equal opaque bytes for forward/reverse input.
- Verify all nine plan-only CLI roots are absent from help and fail closed with
  exit code 2, empty stdout, and usage on stderr.
- Run the current support-matrix verifier without changing matrix claims.
- Defer any cleanup until an Accepted decision identifies the exact public
  surface and required delete/hide/reject/retain/migrate action.

## Compatibility impact

Accepted Seed behavior, current CLI commands, diagnostics, schemas, Semantic
IDs, source spans, editor grammar, runtime, dependencies, support states, and
Unicode 17.0.0 remain unchanged. Only regression and test-local observation
evidence is added.

## Unresolved alternatives

Complete public-surface ownership; classification of success/no-op/recovery/
negative/internal/future/unsupported surfaces; exact cleanup inventory;
delete/hide/reject/retain/deprecate/migrate rules; compatibility and release
notes; aliases, completion, grammar, profiles, backends, defaults, schemas and
protocol consumers; diagnostic allocations; positive, negative, malformed,
migration, editor, Unicode, source-span, deterministic/offline and release
fixtures remain open under STAB-6102, STAB-6101, STAB-6103, incomplete G1-G5
exits, ROADMAP-1.0, the draft SUPPORT-MATRIX, and registered gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
