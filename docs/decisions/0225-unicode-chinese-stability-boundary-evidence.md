# DEC-0225: Unicode and Chinese-programming stability boundary evidence / Unicode 与中文编程稳定性边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: Unicode and source governance
> 相关规范/缺口：`SEMANTICS` | `DEC-0002` | `DEC-0007` | `DEC-0012` | `ROADMAP-1.0`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes bounded evidence for `STD-6303-OBSERVATION`. It
freezes the exact Unicode 17.0.0 input manifest already used by the Seed
implementation and representative accepted identifier behavior. It creates no
Unicode-upgrade, localization, formatter, editor, CLI, or path protocol.

本决定授权 `STD-6303-OBSERVATION` 使用有界证据。它固定 Seed 实现已经使用的
Unicode 17.0.0 精确输入清单和代表性的已接受标识符行为，但不创建 Unicode 升级、
本地化、格式化器、编辑器、CLI 或路径协议。

## Question

Which Unicode and Chinese-programming properties can be made executable now
without promoting unresolved cross-tool and localization plans to language
semantics?

## Decision

1. The observation must assert the exact Unicode version tuple `17.0.0` and
   the eleven path/checksum pairs returned by `unicode_data_checksums()`.
2. It must cover representative Chinese XID acceptance, retained original
   spelling, NFC semantic equality, script metadata, confusable skeletons,
   mixed-script observation, and all ten Seed forbidden-property classes.
3. Forbidden-character evidence must retain the original UTF-8 byte offset;
   generated tables remain reproducible through the existing offline
   `unicode-gen` route and its exhaustive conformance tests.
4. A sixty-category test-local inventory records Unicode, text, Chinese-name,
   tooling, migration, authority, and fixture boundaries with deterministic
   ordering and duplicate rejection.
5. Opaque bytes tagged `ling.unicode-chinese-stability-observation/0` are test
   evidence only. They are not a data format, Semantic ID input, Unicode
   upgrade record, localization schema, or cross-tool protocol.
6. No alias grammar, localized view, profile policy, Unicode upgrade,
   migration behavior, formatter/LSP/Zed contract, CLI feature, Windows-path
   rule, diagnostic, or Stable support claim is authorized. Public `STD-6303`
   remains `BlockedSpec`.

## Normative basis

- `SEMANTICS.md` sections 3.1 through 3.6 require UTF-8 source, Unicode
  17.0.0, XID identifiers, NFC equality, the ten forbidden-property classes,
  confusable/script checks, and a migration report for a Unicode upgrade.
- Accepted `DEC-0002` fixes original UTF-8 byte spans and position units.
- Accepted `DEC-0007` and `DEC-0012` use NFC names for module and Semantic
  identity while excluding source spelling and host paths.
- `ROADMAP-1.0` preserves Unicode 17.0.0, deterministic/offline behavior,
  original spans, and bilingual diagnostics.
- `docs/status/STD-6303-AUTHORITY-AUDIT.md` records the unresolved alias,
  localization, formatter/LSP/Zed, CLI, Windows-path, and upgrade surfaces.

## Conformance plan

- Assert the exact Unicode version and eleven-file SHA-256 manifest.
- Assert representative Chinese, XID, NFC, script, confusable, mixed-script,
  forbidden-property, and original-byte-offset behavior.
- Assert all sixty local boundaries, exact ordering, duplicate rejection, and
  order-independent opaque bytes.
- Run `ling-unicode`, Unicode generation, conformance, governance, status,
  workspace, lint, formatting, deterministic, and offline gates.
- Defer cross-tool stabilization and any Unicode upgrade until Accepted
  authority defines those public contracts and migration evidence.

## Compatibility impact

Unicode data, generated tables, dependency versions, identifier acceptance,
normalization, security behavior, diagnostics, Semantic IDs, source spans,
packages, profiles, formatter/editor/CLI behavior, and paths remain unchanged.
The change adds regression and test-local evidence only.

## Unresolved alternatives

Unicode upgrade ownership and migration; localized aliases and display views;
profile-specific identifier policy; diagnostic wording/stability; formatter,
LSP, Zed, and CLI protocols; UTF-16 transaction semantics; Windows path
display and identity; package/module migration; and cross-process compatibility
remain open under `STD-6303`, registered gaps, and future Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
