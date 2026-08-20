# Ling 规范权威索引 / Specification Authority Index

> 状态：由 `authority.toml` 确定性生成
> 更新日期：2026-08-20
> 本索引描述现有权威关系，不新增语言语义。

## Authority order

```text
Accepted RFC
    > docs/SEMANTICS.md
    > docs/LANGUAGE.md
    > tests/conformance/
    > docs/ROADMAP-1.0.md and engineering plans
    > Rust implementation
    > code comments
```

Accepted decisions are scoped normative records for the questions they close; they cannot override an Accepted RFC. A Draft RFC is indexed for discovery but is not an Accepted implementation basis. If two normative sources conflict, implementation stops and records a specification gap. A lower-authority plan is corrected to match the higher source.

## Documents

| ID | Kind | Status | Version | Authority | Stable basis | Path | Covers | Depends on | Supersedes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `DEC-0001` | Decision | `Accepted` | `1` | `Accepted` | yes | [Diagnostic code allocation policy](../decisions/0001-error-code-policy.md) | `diagnostic codes`, `compatibility` | — | — |
| `DEC-0002` | Decision | `Accepted` | `1` | `Accepted` | yes | [Source position units](../decisions/0002-source-position-units.md) | `UTF-8 byte spans`, `line and column projection` | — | — |
| `DEC-0003` | Decision | `Accepted` | `1` | `Accepted` | yes | [M0 tooling](../decisions/0003-m0-tooling.md) | `CLI parsing`, `serialization`, `snapshot testing` | — | — |
| `DEC-0004` | Decision | `Accepted` | `1` | `Accepted` | yes | [Pipeline syntax and lowering](../decisions/0004-pipeline-syntax.md) | `pipeline grammar`, `precedence`, `lowering` | — | — |
| `DEC-0005` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed literals and delimiters](../decisions/0005-seed-literals-and-delimiters.md) | `literals`, `record delimiters`, `list delimiters` | — | — |
| `DEC-0006` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed offside and layout rules](../decisions/0006-offside-layout.md) | `indentation`, `continuation`, `layout recovery` | — | — |
| `DEC-0007` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed module and file boundaries](../decisions/0007-module-and-file-boundaries.md) | `modules`, `imports`, `entry module` | — | — |
| `DEC-0008` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed value restriction](../decisions/0008-seed-value-restriction.md) | `generalization`, `mutable values`, `effects` | — | — |
| `DEC-0009` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed borrow and mutation boundary](../decisions/0009-seed-borrow-and-mutation-boundary.md) | `borrow scope`, `mutation`, `aliasing` | — | — |
| `DEC-0010` | Decision | `Accepted` | `1` | `Accepted` | yes | [State and capability model](../decisions/0010-state-and-capability-model.md) | `State<T>`, `capabilities`, `host failures` | — | — |
| `DEC-0011` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed built-ins](../decisions/0011-seed-builtins.md) | `built-ins`, `effects`, `capability propagation` | — | — |
| `DEC-0012` | Decision | `Accepted` | `1` | `Accepted` | yes | [Semantic identity and canonical bytes](../decisions/0012-semantic-identity-and-canonical-bytes.md) | `Semantic ID`, `canonical bytes`, `dependency identity` | — | — |
| `DEC-0013` | Decision | `Accepted` | `1` | `Accepted` | yes | [Main and runtime failures](../decisions/0013-main-and-runtime-failures.md) | `entry point`, `runtime failures`, `exit codes` | — | — |
| `DEC-0014` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed prelude Option and Result](../decisions/0014-seed-prelude-option-result.md) | `Option`, `Result`, `prelude namespace` | `DEC-0011`, `DEC-0012` | — |
| `DEC-0015` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed Audit Source format and round trip](../decisions/0015-audit-source-format.md) | `Audit Source`, `canonical ordering`, `round trip` | `DEC-0012` | — |
| `DEC-0016` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed REPL session semantics](../decisions/0016-repl-session-semantics.md) | `REPL sessions`, `transactions`, `script mode` | `DEC-0010`, `DEC-0013` | — |
| `SEMANTICS` | Core specification | `Draft` | `0.1` | `Semantics` | no | [Ling core semantics](../SEMANTICS.md) | `language semantics`, `Semantic Graph`, `execution profiles` | — | — |
| `LANGUAGE` | Design specification | `Draft` | `0.1` | `Language` | no | [Ling language design](../LANGUAGE.md) | `language goals`, `surface design`, `roadmap` | — | — |
| `CONFORMANCE` | Conformance corpus | `Active` | `0.0.1` | `Conformance` | no | [Ling conformance corpus](../../tests/conformance) | `observable Seed behavior`, `positive and negative cases` | `SEMANTICS`, `LANGUAGE` | — |
| `ROADMAP-1.0` | Roadmap | `Planning` | `1.0-baseline` | `Roadmap` | no | [Roadmap to Ling 1.0](../ROADMAP-1.0.md) | `release sequence`, `delivery gates`, `compatibility milestones` | `SEMANTICS`, `LANGUAGE` | — |
| `EXECUTION-PLAN-1.0` | Engineering plan | `Planning` | `2026-08-20` | `Planning` | no | [Ling 1.0 detailed execution plan](../ling_execution_plan/README.md) | `G0 through G6 task decomposition`, `Codex workflow` | `ROADMAP-1.0` | — |
| `IMPLEMENTATION-SEED` | Engineering plan | `Planning` | `0.2` | `Planning` | no | [v0.0.1 Seed implementation plan](../IMPLEMENTATION.md) | `Seed engineering order`, `milestone gates` | `SEMANTICS`, `LANGUAGE` | — |
| `NEXT-STEPS-HELLO` | Engineering record | `Completed` | `0.2` | `Planning` | no | [AST-to-Hello-World implementation record](../NEXT-STEPS.md) | `Hello World vertical slice`, `acceptance evidence` | `IMPLEMENTATION-SEED` | — |
| `NEXT-STEPS-SEED` | Engineering record | `Completed` | `0.0.1` | `Planning` | no | [Hello-World-to-Seed implementation record](../NEXT-STEPS-SEED.md) | `Seed closure`, `release gates` | `NEXT-STEPS-HELLO` | — |
| `ERROR-CODES` | Compatibility registry | `Active` | `ling.diagnostic/0.1` | `Registry` | no | [Ling diagnostic code registry](../ERROR-CODES.md) | `stable diagnostic codes`, `bilingual messages`, `payload facts` | `DEC-0001` | — |
| `GAP-REGISTER` | Governance registry | `Active` | `1` | `Registry` | no | [Specification gap register](../governance/gap-register.toml) | `unresolved semantics`, `release blockers`, `candidate RFCs`, `required evidence` | `SEMANTICS`, `ROADMAP-1.0`, `RFC-0001` | — |
| `TASK-STATUS` | Status registry | `Active` | `1` | `Registry` | no | [Implementation task status registry](../status/implementation-status.toml) | `task lifecycle`, `verification evidence` | `EXECUTION-PLAN-1.0` | — |
| `DEPENDENCIES` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [Rust dependency record](../DEPENDENCIES.md) | `dependency licenses`, `MSRV`, `offline lock set` | — | — |
| `EVIDENCE-GAP-GOV-RFC-STATUS-001` | Gap discovery evidence | `Active` | `1` | `Evidence` | no | [RFC-0001 lifecycle status mismatch](../status/spec-gaps/GAP-GOV-RFC-STATUS-001.md) | `RFC lifecycle`, `governance consistency` | `RFC-0001` | — |
| `SEED-RELEASE-REPORT` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [v0.0.1 Seed release gate report](../SEED-RELEASE-REPORT.md) | `release gates`, `CI evidence`, `published tag` | `SEED-TRACEABILITY` | — |
| `SEED-TRACEABILITY` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [Seed specification traceability](../SEED-TRACEABILITY.md) | `normative clause mapping`, `test evidence` | `CONFORMANCE` | — |
| `DESIGN-REVIEW` | Review | `Active` | `2026-08-17` | `Opinion` | no | [Ling design review](../design-review.html) | `non-normative design critique` | `SEMANTICS`, `LANGUAGE` | — |
| `RUST-IMPLEMENTATION` | Implementation | `Active` | `0.0.1` | `Implementation` | no | [Ling Rust implementation](../../crates) | `compiler`, `interpreter`, `CLI` | `CONFORMANCE` | — |
| `RFC-0001` | RFC | `Draft` | `0.0.1-draft` | `Draft` | no | [Ling foundation and v0.0.1 Seed](../RFC-0001.md) | `Seed scope`, `syntax`, `compiler architecture`, `CLI`, `governance` | `SEMANTICS`, `LANGUAGE` | — |

## Conflict and correction workflow

1. Verify the document lifecycle state in the source file and this index.
2. Stop implementation when higher-authority normative sources conflict or required behavior is unspecified.
3. Record a spec-gap with observable impact, affected tasks, alternatives, and the required RFC/decision.
4. Correct lower-authority plans and implementation only after the authority is clear.
5. Run `cargo xtask governance check-authority` and the relevant conformance suite.

## Machine source

The machine-readable source is [`authority.toml`](authority.toml). The checker rejects duplicate IDs, missing paths, unknown relations, dependency cycles, lifecycle mismatches, Draft documents used as Stable bases, and report drift.
