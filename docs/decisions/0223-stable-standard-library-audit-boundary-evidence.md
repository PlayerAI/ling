# DEC-0223: Internal Stable Standard Library Audit boundary evidence / 内部稳定标准库审计边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: standard-library governance
> 相关规范/缺口：`DEC-0011` | `DEC-0014` | `ROADMAP-1.0` | `SUPPORT-MATRIX`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `STD-6301-OBSERVATION`. It
freezes the exact compiler-injected Seed inventory and truthful support state
without declaring a packaged or Stable standard library.

本决定授权 `STD-6301-OBSERVATION` 使用有界证据，固定精确的编译器注入 Seed
清单与真实支持状态；不声明已打包或 Stable 的标准库。

## Question

Which built-in and Prelude inventory facts can be checked today without
inventing package, profile, complexity, migration, or Stable API commitments?

## Decision

1. The exact function-like built-in inventory remains `Console.write`,
   `Text.format`, `max`, `min`, `map`, and `sum`, each with builtin origin, no
   source file, and no source span.
2. The exact logical `Ling.Prelude` inventory remains types `Option` and
   `Result` plus constructors `Some`, `None`, `Ok`, and `Error`, each with
   Prelude origin, no source file, and no source span.
3. The support matrix retains exactly one standard-package record:
   `STD-LING-PRELUDE`, version `0.0.1-dev`, state `BuiltinOnly`, stability
   `Preview`, implemented, un-packaged, and available in no selectable profile.
4. `crates/ling-types/tests/stable_standard_library_audit_evidence.rs` records
   sixty test-local symbol, semantic, package, profile, lifecycle, and fixture
   boundaries with explicit ordering and duplicate rejection.
5. Opaque bytes tagged
   `ling.stable-standard-library-audit-observation/0` are test evidence only;
   they are not a package manifest, symbol ABI, Semantic ID input, support
   protocol, or stability marker.
6. No symbol, type, Effect, Capability, Fault, package, profile, complexity,
   locale, migration, diagnostic, or stability claim is added. Public
   `STD-6301` remains `BlockedSpec`.

## Normative basis

- Accepted `DEC-0011` fixes the six Seed built-ins and their scoped semantics.
- Accepted `DEC-0014` fixes the logical injected Prelude types and constructors,
  namespace behavior, identity/origin, and no-disk-loading rule.
- The Draft support matrix truthfully records `BuiltinOnly`/`Preview`, no
  package artifact, no profile, and explicit installation/registry exclusions;
  it is evidence, not Stable authority.
- `docs/status/STD-6301-AUTHORITY-AUDIT.md` records the missing complete symbol,
  package, profile, resource, Unicode/locale, and migration contract.

## Conformance plan

- Assert exact six built-ins and six Prelude definitions, origins, kinds,
  logical modules, and absence of invented source locations.
- Assert the exact single standard-package record and its non-Stable,
  un-packaged, unprofiled state.
- Assert all sixty local boundaries, exact ordering, duplicate rejection, and
  order-independent opaque bytes.
- Run resolver, support, governance, status, workspace, lint, formatting,
  deterministic, and offline gates.

## Compatibility impact

Existing built-ins, Prelude types/constructors, signatures, Effects,
Capabilities, Faults, resolver/evaluator behavior, Semantic IDs, support
claims, source spans, diagnostics, and Unicode 17.0.0 remain unchanged.

## Unresolved alternatives

A complete public symbol table; packaged distribution and versioning; profiles
and targets; complexity/resource, panic/termination, locale/text behavior;
deprecation and migration; stable diagnostics; compatibility fixtures; and
release policy remain open under `STD-6301` through `STD-6303`, incomplete
G1-G5 exits, ROADMAP-1.0, and future Accepted standard-library authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
