# RFC 与 Decision 生命周期 / Lifecycle Registry

> 状态：由 `lifecycle.toml` 确定性生成
> 更新日期：2026-08-22
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
| `RFC-0002` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0002](../RFC-0002.md) |
| `RFC-0004` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0004](../RFC-0004.md) |
| `RFC-0005` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0005](../RFC-0005.md) |
| `RFC-0014` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0014](../RFC-0014.md) |
| `RFC-0015` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0015](../RFC-0015.md) |
| `RFC-0016` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0016](../RFC-0016.md) |
| `RFC-0017` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0017](../RFC-0017.md) |
| `RFC-0018` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0018](../RFC-0018.md) |
| `RFC-0019` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0019](../RFC-0019.md) |
| `RFC-0020` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [RFC-0020](../RFC-0020.md) |
| `RFC-0021` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0021](../RFC-0021.md) |
| `RFC-0022` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0022](../RFC-0022.md) |
| `RFC-0023` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0023](../RFC-0023.md) |
| `RFC-0024` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0024](../RFC-0024.md) |
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
| `DEC-0017` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-20` | [DEC-0017](../decisions/0017-seed-boolean-operators.md) |
| `DEC-0018` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0018](../decisions/0018-rfc-0001-lifecycle.md) |
| `DEC-0019` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0019](../decisions/0019-incremental-query-boundary.md) |
| `DEC-0021` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0021](../decisions/0021-deterministic-parallel-scheduling.md) |
| `DEC-0022` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0022](../decisions/0022-disposable-persistent-query-cache.md) |
| `DEC-0023` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0023](../decisions/0023-author-source-formatter-preservation.md) |
| `DEC-0024` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0024](../decisions/0024-trait-obligation-collection-boundary.md) |
| `DEC-0025` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0025](../decisions/0025-trait-coherence-index-boundary.md) |
| `DEC-0026` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0026](../decisions/0026-trait-solver-v0-boundary.md) |
| `DEC-0027` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-21` | [DEC-0027](../decisions/0027-trait-checked-core-dictionary-witness.md) |
| `DEC-0028` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0028](../decisions/0028-formatter-cli-contract.md) |
| `DEC-0029` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0029](../decisions/0029-lsp-position-encoding-projection.md) |
| `DEC-0030` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0030](../decisions/0030-lsp-request-snapshot-boundary.md) |
| `DEC-0031` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0031](../decisions/0031-lsp-internal-cancellation-boundary.md) |
| `DEC-0032` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0032](../decisions/0032-lsp-internal-scheduling-boundary.md) |
| `DEC-0033` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0033](../decisions/0033-lsp-internal-byte-accounting-boundary.md) |
| `DEC-0034` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0034](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md) |
| `DEC-0035` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0035](../decisions/0035-lsp-internal-diagnostic-batch-boundary.md) |
| `DEC-0036` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0036](../decisions/0036-cli-internal-command-catalog-boundary.md) |
| `DEC-0037` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0037](../decisions/0037-cli-internal-exit-catalog-boundary.md) |
| `DEC-0038` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0038](../decisions/0038-cli-init-command.md) |
| `DEC-0039` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0039](../decisions/0039-cli-test-file-runner.md) |
| `DEC-0040` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0040](../decisions/0040-cli-help-truth-fixture.md) |
| `DEC-0041` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0041](../decisions/0041-seed-fuzz-inventory-gate.md) |
| `DEC-0042` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0042](../decisions/0042-seed-fault-matrix-gate.md) |
| `DEC-0043` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0043](../decisions/0043-seed-security-matrix-gate.md) |
| `DEC-0044` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0044](../decisions/0044-seed-performance-matrix-gate.md) |
| `DEC-0045` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0045](../decisions/0045-seed-documentation-inventory-gate.md) |
| `DEC-0046` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0046](../decisions/0046-seed-example-matrix-gate.md) |
| `DEC-0047` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0047](../decisions/0047-seed-bilingual-tutorial-gate.md) |
| `DEC-0048` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0048](../decisions/0048-seed-zed-compatibility-matrix-gate.md) |
| `DEC-0049` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0049](../decisions/0049-seed-lsp-discovery-inventory-gate.md) |
| `DEC-0050` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0050](../decisions/0050-seed-zed-extension-acceptance-inventory-gate.md) |
| `DEC-0051` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0051](../decisions/0051-seed-dap-status-inventory-gate.md) |
| `DEC-0052` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0052](../decisions/0052-seed-rc0-internal-freeze-inventory-gate.md) |
| `DEC-0053` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0053](../decisions/0053-seed-rc1-public-validation-inventory-gate.md) |
| `DEC-0054` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0054](../decisions/0054-seed-rc3-independent-verification-inventory-gate.md) |
| `DEC-0055` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0055](../decisions/0055-seed-rc2-final-change-control-inventory-gate.md) |
| `DEC-0056` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0056](../decisions/0056-seed-v1-release-artifact-inventory-gate.md) |
| `DEC-0057` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0057](../decisions/0057-formatter-in-process-edit-projection.md) |
| `DEC-0058` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0058](../decisions/0058-locked-project-snapshot-boundary.md) |

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
