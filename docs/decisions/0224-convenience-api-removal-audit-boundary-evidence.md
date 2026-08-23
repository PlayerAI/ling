# DEC-0224: Internal Convenience API Removal Audit boundary evidence / 内部便利 API 删除审计边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: standard-library governance
> 相关规范/缺口：`DEC-0011` | `DEC-0014` | `DEC-0223` | `ROADMAP-1.0`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `STD-6302-OBSERVATION`. The
current removal set is explicitly empty: accepted Seed symbols are retained,
and plan-only convenience surfaces are verified absent from compiler injection.

本决定授权 `STD-6302-OBSERVATION` 使用有界证据。当前删除集合明确为空：保留
已接受的 Seed 符号，并验证计划中的便利接口未进入编译器注入面。

## Question

Which convenience APIs can be removed now without violating Accepted Seed
semantics or inventing a source API lifecycle?

## Decision

1. The exact authorized removal, hiding, deprecation, and migration set is
   empty. The six DEC-0011 built-ins and six DEC-0014 Prelude definitions are
   Accepted and must not be deleted or renamed by a non-normative checklist.
2. The injected Seed surface must not contain plan-only convenience names for
   Clock, Random, Network/retry, a global runtime, dynamic reflection, FFI
   calls, or unbounded collections.
3. The current support record remains `BuiltinOnly`/`Preview`, un-packaged,
   and unprofiled; no `core` or Preview package split is inferred.
4. `crates/ling-types/tests/convenience_api_removal_audit_evidence.rs` records
   sixty test-local lifecycle, risk, symbol, semantic, package, and fixture
   boundaries with explicit ordering and duplicate rejection.
5. Opaque bytes tagged
   `ling.convenience-api-removal-audit-observation/0` are test evidence only;
   they are not a removal manifest, API version, deprecation protocol,
   Semantic ID input, or migration record.
6. No API is deleted, hidden, rejected, deprecated, migrated, replaced, or
   added. Public `STD-6302` remains `BlockedSpec`.

## Normative basis

- Accepted `DEC-0011` and `DEC-0014` define the exact current Seed built-in
  and Prelude surfaces; extensions or incompatible changes require new
  authority.
- Accepted `DEC-0223` freezes exact inventory and truthful non-Stable support
  evidence without creating a packaged library.
- `LANGUAGE.md` and `SEMANTICS.md` reject ambient implicit authority but do
  not authorize source-symbol removal.
- `docs/status/STD-6302-AUTHORITY-AUDIT.md` records the missing exact removal
  set, lifecycle, replacement, diagnostics, compatibility window, and
  migration contract.

## Conformance plan

- Assert the twelve accepted injected symbols remain exact and no plan-only
  convenience name enters that surface.
- Assert all sixty local boundaries, exact ordering, duplicate rejection, and
  order-independent opaque bytes.
- Run resolver, support, governance, status, workspace, lint, formatting,
  deterministic, and offline gates.
- Defer every real removal/deprecation/migration until Accepted authority names
  exact symbols and executable compatibility evidence.

## Compatibility impact

Existing symbols, types, Effects, Capabilities, Faults, resolver/evaluator
behavior, packages, profiles, diagnostics, Semantic IDs, source spans, and
Unicode 17.0.0 remain unchanged.

## Unresolved alternatives

Public/internal ownership; core/Preview packages; exact future removal set;
delete/hide/reject/retain/deprecate/migrate rules; replacements; compatibility
windows; diagnostics; package/lock effects; migration fixtures; complexity and
resource contracts; profiles; and release policy remain open under `STD-6302`,
`STD-6303`, incomplete G1-G5 exits, and future API-lifecycle authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
