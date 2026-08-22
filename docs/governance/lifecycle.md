# RFC 与 Decision 生命周期 / Lifecycle Registry

> 状态：由 `lifecycle.toml` 确定性生成
> 更新日期：2026-08-23
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
| `RFC-0006` | RFC | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [RFC-0006](../RFC-0006.md) |
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
| `DEC-0059` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0059](../decisions/0059-trait-ide-projection-lookups.md) |
| `DEC-0060` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0060](../decisions/0060-seed-effect-row-snapshot.md) |
| `DEC-0061` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0061](../decisions/0061-seed-type-value-classification.md) |
| `DEC-0062` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0062](../decisions/0062-effect-row-constraint-solver.md) |
| `DEC-0063` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0063](../decisions/0063-first-order-handler-core.md) |
| `DEC-0064` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0064](../decisions/0064-handler-source-cst-projection.md) |
| `DEC-0065` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0065](../decisions/0065-handler-ast-projection.md) |
| `DEC-0066` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0066](../decisions/0066-handler-hir-projection.md) |
| `DEC-0067` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0067](../decisions/0067-effect-model-property-corpus.md) |
| `DEC-0068` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0068](../decisions/0068-trait-termination-corpus.md) |
| `DEC-0069` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0069](../decisions/0069-lsp-utf8-edit-primitive.md) |
| `DEC-0070` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0070](../decisions/0070-lsp-position-edit-projection.md) |
| `DEC-0071` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0071](../decisions/0071-lsp-workspace-state-snapshot.md) |
| `DEC-0072` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0072](../decisions/0072-lsp-diagnostic-span-projection.md) |
| `DEC-0073` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0073](../decisions/0073-ide-resolved-definition-index.md) |
| `DEC-0074` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0074](../decisions/0074-ide-typed-definition-observation.md) |
| `DEC-0075` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0075](../decisions/0075-ide-resolved-reference-index.md) |
| `DEC-0076` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0076](../decisions/0076-ide-resolved-reference-reverse-index.md) |
| `DEC-0077` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0077](../decisions/0077-ide-rename-identifier-observation.md) |
| `DEC-0078` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0078](../decisions/0078-ide-rename-reference-span-observation.md) |
| `DEC-0079` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0079](../decisions/0079-ide-completion-source-inventory.md) |
| `DEC-0080` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0080](../decisions/0080-ide-completion-checked-metadata.md) |
| `DEC-0081` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0081](../decisions/0081-ide-code-action-repair-index.md) |
| `DEC-0082` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0082](../decisions/0082-ide-workspace-symbol-lookups.md) |
| `DEC-0083` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0083](../decisions/0083-prj-locked-project-semantic-snapshot.md) |
| `DEC-0084` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0084](../decisions/0084-lsp-lexical-token-source-index.md) |
| `DEC-0085` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0085](../decisions/0085-lsp-checked-token-identity-observation.md) |
| `DEC-0086` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0086](../decisions/0086-lsp-checked-token-snapshot-identity.md) |
| `DEC-0087` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0087](../decisions/0087-lsp-checked-token-source-fixtures.md) |
| `DEC-0088` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0088](../decisions/0088-effect-handler-execution-rejection-gate.md) |
| `DEC-0089` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0089](../decisions/0089-task-syntax-rejection-gate.md) |
| `DEC-0090` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0090](../decisions/0090-actor-syntax-rejection-gate.md) |
| `DEC-0091` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0091](../decisions/0091-task-checked-core-model.md) |
| `DEC-0092` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0092](../decisions/0092-task-state-machine-model.md) |
| `DEC-0093` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0093](../decisions/0093-task-lifecycle-observation-trace.md) |
| `DEC-0094` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0094](../decisions/0094-task-scheduler-observation-trace.md) |
| `DEC-0095` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0095](../decisions/0095-actor-identity-reference-model.md) |
| `DEC-0096` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0096](../decisions/0096-actor-message-schema-identity.md) |
| `DEC-0097` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0097](../decisions/0097-actor-mailbox-observation.md) |
| `DEC-0098` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0098](../decisions/0098-actor-turn-observation.md) |
| `DEC-0099` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0099](../decisions/0099-actor-runtime-observation.md) |
| `DEC-0100` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0100](../decisions/0100-actor-property-observation.md) |
| `DEC-0101` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0101](../decisions/0101-supervisor-observation.md) |
| `DEC-0102` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0102](../decisions/0102-restart-budget-observation.md) |
| `DEC-0103` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0103](../decisions/0103-supervision-test-evidence.md) |
| `DEC-0104` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0104](../decisions/0104-determinism-class-evidence.md) |
| `DEC-0105` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0105](../decisions/0105-replay-schema-field-evidence.md) |
| `DEC-0106` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0106](../decisions/0106-effect-recorder-boundary-evidence.md) |
| `DEC-0107` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0107](../decisions/0107-replay-player-boundary-evidence.md) |
| `DEC-0108` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0108](../decisions/0108-replay-privacy-integrity-boundary-evidence.md) |
| `DEC-0109` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0109](../decisions/0109-cross-process-replay-acceptance-boundary-evidence.md) |
| `DEC-0110` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0110](../decisions/0110-remote-ref-endpoint-boundary-evidence.md) |
| `DEC-0111` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0111](../decisions/0111-transport-neutral-envelope-boundary-evidence.md) |
| `DEC-0112` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0112](../decisions/0112-remote-delivery-boundary-evidence.md) |
| `DEC-0113` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0113](../decisions/0113-reference-transport-boundary-evidence.md) |
| `DEC-0114` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0114](../decisions/0114-security-resource-boundary-evidence.md) |
| `DEC-0115` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0115](../decisions/0115-memory-layout-copy-move-boundary-evidence.md) |
| `DEC-0116` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0116](../decisions/0116-resource-drop-boundary-evidence.md) |
| `DEC-0117` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0117](../decisions/0117-managed-island-boundary-evidence.md) |
| `DEC-0118` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0118](../decisions/0118-place-move-boundary-evidence.md) |
| `DEC-0119` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0119](../decisions/0119-borrow-exclusivity-boundary-evidence.md) |
| `DEC-0120` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0120](../decisions/0120-region-inference-boundary-evidence.md) |
| `DEC-0121` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0121](../decisions/0121-borrow-await-turn-boundary-evidence.md) |
| `DEC-0122` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0122](../decisions/0122-drop-order-boundary-evidence.md) |
| `DEC-0123` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0123](../decisions/0123-ownership-diagnostic-boundary-evidence.md) |
| `DEC-0124` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-22` | [DEC-0124](../decisions/0124-ownership-corpus-boundary-evidence.md) |
| `DEC-0125` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0125](../decisions/0125-managed-object-model-boundary-evidence.md) |
| `DEC-0126` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0126](../decisions/0126-managed-collector-boundary-evidence.md) |
| `DEC-0127` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0127](../decisions/0127-managed-ffi-boundary-evidence.md) |
| `DEC-0128` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0128](../decisions/0128-managed-profile-boundary-evidence.md) |
| `DEC-0129` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0129](../decisions/0129-native-ir-design-boundary-evidence.md) |
| `DEC-0130` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0130](../decisions/0130-native-ir-lowering-boundary-evidence.md) |
| `DEC-0131` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0131](../decisions/0131-native-ir-verifier-boundary-evidence.md) |
| `DEC-0132` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0132](../decisions/0132-native-backend-selection-boundary-evidence.md) |
| `DEC-0133` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0133](../decisions/0133-native-codegen-boundary-evidence.md) |
| `DEC-0134` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0134](../decisions/0134-native-runtime-abi-boundary-evidence.md) |
| `DEC-0135` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0135](../decisions/0135-native-optimization-boundary-evidence.md) |
| `DEC-0136` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0136](../decisions/0136-native-reproducible-build-boundary-evidence.md) |
| `DEC-0137` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0137](../decisions/0137-ffi-declaration-boundary-evidence.md) |
| `DEC-0138` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0138](../decisions/0138-c-abi-interoperability-boundary-evidence.md) |
| `DEC-0139` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0139](../decisions/0139-ffi-shim-generator-boundary-evidence.md) |
| `DEC-0140` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0140](../decisions/0140-target-primitive-package-boundary-evidence.md) |
| `DEC-0141` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0141](../decisions/0141-ffi-fuzz-sanitizer-boundary-evidence.md) |
| `DEC-0142` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0142](../decisions/0142-differential-harness-boundary-evidence.md) |
| `DEC-0143` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0143](../decisions/0143-allowed-difference-registry-boundary-evidence.md) |
| `DEC-0144` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0144](../decisions/0144-dap-debugger-boundary-evidence.md) |
| `DEC-0145` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0145](../decisions/0145-zed-debugger-registration-boundary-evidence.md) |
| `DEC-0146` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0146](../decisions/0146-staged-debugger-capability-boundary-evidence.md) |
| `DEC-0147` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0147](../decisions/0147-kernel-capability-matrix-boundary-evidence.md) |
| `DEC-0148` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0148](../decisions/0148-kernel-effect-capability-boundary-evidence.md) |
| `DEC-0149` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0149](../decisions/0149-kernel-shape-index-bounds-boundary-evidence.md) |
| `DEC-0150` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0150](../decisions/0150-kernel-alias-parallel-write-boundary-evidence.md) |
| `DEC-0151` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0151](../decisions/0151-kernel-core-verifier-boundary-evidence.md) |
| `DEC-0152` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0152](../decisions/0152-cpu-scalar-reference-boundary-evidence.md) |
| `DEC-0153` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0153](../decisions/0153-cpu-reference-trace-boundary-evidence.md) |
| `DEC-0154` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0154](../decisions/0154-kernel-corpus-boundary-evidence.md) |
| `DEC-0155` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0155](../decisions/0155-simd-legality-boundary-evidence.md) |
| `DEC-0156` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0156](../decisions/0156-portable-simd-ir-boundary-evidence.md) |
| `DEC-0157` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0157](../decisions/0157-simd-differential-boundary-evidence.md) |
| `DEC-0158` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0158](../decisions/0158-device-capability-boundary-evidence.md) |
| `DEC-0159` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0159](../decisions/0159-buffer-ownership-boundary-evidence.md) |
| `DEC-0160` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0160](../decisions/0160-transfer-effect-boundary-evidence.md) |
| `DEC-0161` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0161](../decisions/0161-device-synchronization-boundary-evidence.md) |
| `DEC-0162` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0162](../decisions/0162-device-ir-schema-boundary-evidence.md) |
| `DEC-0163` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0163](../decisions/0163-kernel-device-lowering-boundary-evidence.md) |
| `DEC-0164` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0164](../decisions/0164-device-ir-canonicalization-boundary-evidence.md) |
| `DEC-0165` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0165](../decisions/0165-backend-spike-selection-boundary-evidence.md) |
| `DEC-0166` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0166](../decisions/0166-backend-adapter-boundary-evidence.md) |
| `DEC-0167` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0167](../decisions/0167-launch-runtime-boundary-evidence.md) |
| `DEC-0168` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0168](../decisions/0168-differential-hardware-matrix-boundary-evidence.md) |
| `DEC-0169` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0169](../decisions/0169-error-normalization-boundary-evidence.md) |
| `DEC-0170` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0170](../decisions/0170-accelerator-plugin-interface-boundary-evidence.md) |
| `DEC-0171` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0171](../decisions/0171-experimental-accelerator-adapter-boundary-evidence.md) |
| `DEC-0172` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0172](../decisions/0172-placement-constraint-boundary-evidence.md) |
| `DEC-0173` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0173](../decisions/0173-placement-selection-boundary-evidence.md) |
| `DEC-0174` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0174](../decisions/0174-cost-model-boundary-evidence.md) |
| `DEC-0175` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0175](../decisions/0175-placement-explain-boundary-evidence.md) |
| `DEC-0176` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0176](../decisions/0176-device-binary-cache-boundary-evidence.md) |
| `DEC-0177` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0177](../decisions/0177-critical-profile-boundary-evidence.md) |
| `DEC-0178` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0178](../decisions/0178-forbidden-capability-boundary-evidence.md) |
| `DEC-0179` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0179](../decisions/0179-profile-composition-boundary-evidence.md) |
| `DEC-0180` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0180](../decisions/0180-profile-audit-lsp-boundary-evidence.md) |
| `DEC-0181` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0181](../decisions/0181-bound-types-expressions-boundary-evidence.md) |
| `DEC-0182` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0182](../decisions/0182-loop-recursion-checks-boundary-evidence.md) |
| `DEC-0183` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0183](../decisions/0183-memory-budgets-boundary-evidence.md) |
| `DEC-0184` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0184](../decisions/0184-resource-budget-diagnostics-boundary-evidence.md) |
| `DEC-0185` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0185](../decisions/0185-node-syntax-semantics-boundary-evidence.md) |
| `DEC-0186` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0186](../decisions/0186-node-checked-core-boundary-evidence.md) |
| `DEC-0187` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0187](../decisions/0187-node-static-scheduling-boundary-evidence.md) |
| `DEC-0188` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0188](../decisions/0188-node-virtual-time-runtime-boundary-evidence.md) |
| `DEC-0189` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0189](../decisions/0189-node-native-runtime-boundary-evidence.md) |
| `DEC-0190` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0190](../decisions/0190-node-actor-boundary-evidence.md) |
| `DEC-0191` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0191](../decisions/0191-node-conformance-boundary-evidence.md) |
| `DEC-0192` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0192](../decisions/0192-contract-syntax-core-boundary-evidence.md) |
| `DEC-0193` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0193](../decisions/0193-contract-status-model-boundary-evidence.md) |
| `DEC-0194` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0194](../decisions/0194-contract-runtime-check-boundary-evidence.md) |
| `DEC-0195` | Decision | `Accepted` | `Open` → `Draft` → `Proposed` → `Accepted` | yes | no | `2026-08-23` | [DEC-0195](../decisions/0195-contract-vc-boundary-evidence.md) |

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
