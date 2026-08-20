# RFC 与 Decision 生命周期 / Lifecycle Registry

> 状态：由 `lifecycle.toml` 确定性生成
> 更新日期：2026-08-20
> 本文件定义治理状态和证据要求，不新增语言语义。

## State machine

```text
Open → Draft → Proposed → Accepted / Rejected → Superseded
```

Draft and Proposed documents cannot authorize Stable implementation. Accepted records require conformance, compatibility, and unresolved-alternative metadata. Superseded records name an Accepted successor.

## Records

| ID | Kind | Status | History | Stable basis | Legacy format | Decided | Path |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `RFC-0001` | RFC | `Draft` | `Draft` | no | yes | — | [RFC-0001](../RFC-0001.md) |
| `DEC-0001` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0001](../decisions/0001-error-code-policy.md) |
| `DEC-0002` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0002](../decisions/0002-source-position-units.md) |
| `DEC-0003` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0003](../decisions/0003-m0-tooling.md) |
| `DEC-0004` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0004](../decisions/0004-pipeline-syntax.md) |
| `DEC-0005` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0005](../decisions/0005-seed-literals-and-delimiters.md) |
| `DEC-0006` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0006](../decisions/0006-offside-layout.md) |
| `DEC-0007` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0007](../decisions/0007-module-and-file-boundaries.md) |
| `DEC-0008` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0008](../decisions/0008-seed-value-restriction.md) |
| `DEC-0009` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0009](../decisions/0009-seed-borrow-and-mutation-boundary.md) |
| `DEC-0010` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0010](../decisions/0010-state-and-capability-model.md) |
| `DEC-0011` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0011](../decisions/0011-seed-builtins.md) |
| `DEC-0012` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0012](../decisions/0012-semantic-identity-and-canonical-bytes.md) |
| `DEC-0013` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-18` | [DEC-0013](../decisions/0013-main-and-runtime-failures.md) |
| `DEC-0014` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-19` | [DEC-0014](../decisions/0014-seed-prelude-option-result.md) |
| `DEC-0015` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-19` | [DEC-0015](../decisions/0015-audit-source-format.md) |
| `DEC-0016` | Decision | `Accepted` | `Accepted` | yes | yes | `2026-08-19` | [DEC-0016](../decisions/0016-repl-session-semantics.md) |

## Migration boundary

RFC-0001 and DEC-0001 through DEC-0016 predate the section template and are listed in a closed legacy-format allowlist. Their required Accepted metadata is carried by `lifecycle.toml`. Every later RFC/decision must use the checked template headings; new legacy exemptions are rejected.

## Merge policy

- Experimental implementation must map to a Draft RFC or registered specification gap.
- A language-semantic pull request must cite the Accepted specification IDs and normative clauses that authorize it. A Draft, Proposed record, roadmap item, snapshot, or gap is not authorization.
- Supersession preserves the historical record and points to an Accepted replacement; IDs and published meanings are not silently reused.

## Machine source and templates

- [`lifecycle.toml`](lifecycle.toml)
- [`templates/RFC.md`](templates/RFC.md)
- [`templates/DECISION.md`](templates/DECISION.md)
- [Pull request template](../../.github/pull_request_template.md)

Run `cargo xtask governance check-lifecycle` to reject invalid states/transitions, Draft Stable bases, incomplete Accepted metadata, dangling/cyclic supersession, unindexed specifications, template drift, and report drift.
