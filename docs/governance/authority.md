# Ling 规范权威索引 / Specification Authority Index

> 状态：由 `authority.toml` 确定性生成
> 更新日期：2026-08-21
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
| `DEC-0003` | Decision | `Accepted` | `1` | `Accepted` | yes | [M0 tooling](../decisions/0003-m0-tooling.md) | `CLI parsing`, `scriptable REPL baseline`, `conformance runner`, `dependency discipline` | — | — |
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
| `DEC-0017` | Decision | `Accepted` | `1` | `Accepted` | yes | [Seed boolean operators and expression precedence](../decisions/0017-seed-boolean-operators.md) | `boolean operators`, `expression precedence`, `short-circuit evaluation`, `Typed Core lowering` | `DEC-0004`, `DEC-0005`, `DEC-0009`, `DEC-0010`, `SEMANTICS` | — |
| `DEC-0018` | Decision | `Accepted` | `1` | `Accepted` | yes | [Keep RFC-0001 as a Draft design baseline](../decisions/0018-rfc-0001-lifecycle.md) | `RFC-0001 lifecycle`, `post-Seed authority`, `release versus acceptance` | `RFC-0001`, `EVIDENCE-GAP-GOV-RFC-STATUS-001` | — |
| `DEC-0019` | Decision | `Accepted` | `1` | `Accepted` | yes | [Incremental query boundary and invalidation policy](../decisions/0019-incremental-query-boundary.md) | `internal query graph`, `source revisions`, `dependency invalidation`, `deterministic scheduling`, `cache boundary` | `DEC-0002`, `DEC-0012`, `DEC-0013`, `ROADMAP-1.0`, `SEMANTICS`, `LANGUAGE` | — |
| `DEC-0021` | Decision | `Accepted` | `1` | `Accepted` | yes | [Deterministic parallel internal query scheduling](../decisions/0021-deterministic-parallel-scheduling.md) | `bounded parallel query jobs`, `canonical publication`, `deterministic scheduling evidence` | `DEC-0019`, `DEC-0012`, `ROADMAP-1.0`, `GAP-REGISTER` | — |
| `DEC-0022` | Decision | `Accepted` | `1` | `Accepted` | yes | [Disposable persistent query-cache envelope](../decisions/0022-disposable-persistent-query-cache.md) | `persistent cache key dimensions`, `bounded cache envelope`, `corruption-safe misses`, `checked line-index reconstruction` | `DEC-0012`, `DEC-0019`, `DEC-0021`, `ROADMAP-1.0`, `GAP-REGISTER` | — |
| `DEC-0023` | Decision | `Accepted` | `1` | `Accepted` | yes | [Author Source formatter preservation boundary](../decisions/0023-author-source-formatter-preservation.md) | `Author Source preservation`, `formatter idempotence`, `comment and Unicode spelling retention`, `incomplete-source safety`, `Audit separation` | `DEC-0002`, `DEC-0006`, `DEC-0015`, `ROADMAP-1.0`, `GAP-REGISTER` | — |
| `RFC-0002` | RFC | `Accepted` | `1` | `Accepted` | yes | [Local package identity and project protocol](../RFC-0002.md) | `ling.toml`, `package identity`, `local dependencies`, `module exports`, `ling.lock`, `offline resolution` | `DEC-0002`, `DEC-0007`, `DEC-0012`, `DEC-0018`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0005` | RFC | `Accepted` | `1` | `Accepted` | yes | [Restricted Traits and constraint solving](../RFC-0005.md) | `Trait declarations`, `nominal constraints`, `coherence and orphan rules`, `deterministic selection`, `Checked Core dictionary witnesses` | `RFC-0001`, `SEMANTICS`, `LANGUAGE`, `ROADMAP-1.0`, `GAP-REGISTER` | — |
| `RFC-0014` | RFC | `Accepted` | `1` | `Accepted` | yes | [Portable verified bytecode and VM contract](../RFC-0014.md) | `bytecode encoding`, `register VM`, `bytecode verifier`, `VM evaluation order`, `Runtime Fault mapping`, `source maps`, `bytecode compatibility` | `DEC-0002`, `DEC-0007`, `DEC-0010`, `DEC-0011`, `DEC-0012`, `DEC-0013`, `DEC-0017`, `RFC-0002`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0015` | RFC | `Accepted` | `1` | `Accepted` | yes | [First-class functions, lexical closures, and recursion in bytecode](../RFC-0015.md) | `bytecode function types`, `lexical closure capture`, `partial application`, `indirect calls`, `recursion`, `VM frame limits` | `DEC-0008`, `DEC-0009`, `DEC-0010`, `DEC-0011`, `DEC-0012`, `DEC-0013`, `DEC-0016`, `RFC-0014`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0016` | RFC | `Accepted` | `1` | `Accepted` | yes | [Aggregate values and checked pattern matching in bytecode](../RFC-0016.md) | `tuple types`, `nominal records`, `nominal variants`, `immutable record update`, `field projection`, `checked match lowering` | `DEC-0009`, `DEC-0012`, `DEC-0013`, `DEC-0014`, `RFC-0014`, `RFC-0015`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0017` | RFC | `Accepted` | `1` | `Accepted` | yes | [Seed mutable-place lowering](../RFC-0017.md) | `mutable local assignment`, `mutable record-field assignment`, `nested place updates`, `mutation CFG joins` | `DEC-0009`, `RFC-0014`, `RFC-0016`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0018` | RFC | `Accepted` | `1` | `Accepted` | yes | [Seed VM Effect, Capability, and Fault boundary](../RFC-0018.md) | `Effect closure`, `Capability preflight`, `host failure normalization`, `source-mapped Runtime Faults`, `host panic containment` | `DEC-0010`, `RFC-0014`, `RFC-0016`, `RFC-0017`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0019` | RFC | `Accepted` | `1` | `Accepted` | yes | [Seed Interpreter–VM Differential Contract](../RFC-0019.md) | `interpreter–VM logical event equivalence`, `Runtime Fault projection`, `checked snapshot identity`, `bytecode round-trip differential evidence` | `DEC-0012`, `RFC-0014`, `RFC-0015`, `RFC-0016`, `RFC-0017`, `RFC-0018`, `SEMANTICS`, `LANGUAGE` | — |
| `RFC-0020` | RFC | `Accepted` | `1` | `Accepted` | yes | [Seed VM Robustness, Cancellation, and Resource Evidence](../RFC-0020.md) | `cooperative VM cancellation`, `Runtime Fault cancellation projection`, `bytecode fuzz determinism`, `resource-limit evidence` | `DEC-0013`, `RFC-0014`, `RFC-0018`, `RFC-0019`, `SEMANTICS`, `LANGUAGE` | — |
| `SEMANTICS` | Core specification | `Draft` | `0.1` | `Semantics` | no | [Ling core semantics](../SEMANTICS.md) | `language semantics`, `Semantic Graph`, `execution profiles` | — | — |
| `LANGUAGE` | Design specification | `Draft` | `0.1` | `Language` | no | [Ling language design](../LANGUAGE.md) | `language goals`, `surface design`, `roadmap` | — | — |
| `CONFORMANCE` | Conformance corpus | `Active` | `0.0.1` | `Conformance` | no | [Ling conformance corpus](../../tests/conformance) | `observable Seed behavior`, `positive and negative cases` | `SEMANTICS`, `LANGUAGE` | — |
| `ROADMAP-1.0` | Roadmap | `Planning` | `1.0-baseline` | `Roadmap` | no | [Roadmap to Ling 1.0](../ROADMAP-1.0.md) | `release sequence`, `delivery gates`, `compatibility milestones` | `SEMANTICS`, `LANGUAGE` | — |
| `EXECUTION-PLAN-1.0` | Engineering plan | `Planning` | `2026-08-20` | `Planning` | no | [Ling 1.0 detailed execution plan](../ling_execution_plan/README.md) | `G0 through G6 task decomposition`, `Codex workflow` | `ROADMAP-1.0` | — |
| `IMPLEMENTATION-SEED` | Engineering plan | `Planning` | `0.2` | `Planning` | no | [v0.0.1 Seed implementation plan](../IMPLEMENTATION.md) | `Seed engineering order`, `milestone gates` | `SEMANTICS`, `LANGUAGE` | — |
| `NEXT-STEPS-HELLO` | Engineering record | `Completed` | `0.2` | `Planning` | no | [AST-to-Hello-World implementation record](../NEXT-STEPS.md) | `Hello World vertical slice`, `acceptance evidence` | `IMPLEMENTATION-SEED` | — |
| `NEXT-STEPS-SEED` | Engineering record | `Completed` | `0.0.1` | `Planning` | no | [Hello-World-to-Seed implementation record](../NEXT-STEPS-SEED.md) | `Seed closure`, `release gates` | `NEXT-STEPS-HELLO` | — |
| `ERROR-CODES` | Compatibility registry | `Active` | `ling.diagnostic/0.1` | `Registry` | no | [Ling diagnostic code registry](../ERROR-CODES.md) | `stable diagnostic codes`, `bilingual messages`, `typed payload facts`, `retired allocations`, `generated compatibility lock` | `DEC-0001` | — |
| `GAP-REGISTER` | Governance registry | `Active` | `1` | `Registry` | no | [Specification gap register](../governance/gap-register.toml) | `unresolved semantics`, `release blockers`, `candidate RFCs`, `required evidence` | `SEMANTICS`, `ROADMAP-1.0`, `RFC-0001` | — |
| `LIFECYCLE-REGISTER` | Governance registry | `Active` | `1` | `Registry` | no | [RFC and decision lifecycle registry](../governance/lifecycle.toml) | `RFC lifecycle`, `decision lifecycle`, `acceptance evidence`, `supersession`, `Stable implementation basis` | — | — |
| `PROTOCOL-INVENTORY` | Governance registry | `Active` | `1` | `Registry` | no | [Public interface and protocol inventory](../governance/protocol-inventory.toml) | `public protocol versions`, `stability levels`, `reader and writer policies`, `unknown-field behavior`, `canonical encodings`, `Future protocol boundaries` | `GAP-REGISTER`, `LIFECYCLE-REGISTER` | — |
| `SCHEMA-LIFECYCLE-POLICY` | Governance policy | `Draft` | `1` | `Registry` | no | [Schema lifecycle and golden corpus policy](../governance/SCHEMA-LIFECYCLE.md) | `schema version meaning`, `writer and reader ranges`, `unknown and missing fields`, `canonical encoding`, `hash scheme boundaries`, `golden and corrupt corpus requirements` | `GAP-REGISTER`, `LIFECYCLE-REGISTER`, `PROTOCOL-INVENTORY` | — |
| `SCHEMA-REGISTRY` | Compatibility registry | `Active` | `1` | `Registry` | no | [Versioned public JSON schema registry and corpus](../../schemas/registry.toml) | `public JSON schema inventory`, `reader and writer versions`, `valid and invalid fixtures`, `canonical byte goldens`, `N-1 compatibility evidence` | `SCHEMA-LIFECYCLE-POLICY`, `PROTOCOL-INVENTORY` | — |
| `SUPPORT-MATRIX` | Governance registry | `Draft` | `1.0-draft` | `Registry` | no | [Ling 1.0 support matrix draft](../governance/support-matrix.toml) | `feature and profile stability`, `host and target tiers`, `backend and device tiers`, `standard package stability`, `protocol versions`, `explicitly unsupported scope` | `GAP-REGISTER`, `PROTOCOL-INVENTORY`, `TRACEABILITY-REGISTER` | — |
| `TASK-STATUS` | Status registry | `Active` | `2` | `Registry` | no | [Implementation task and feature status registry](../status/implementation-status.toml) | `task lifecycle`, `feature state`, `implementation, test, and documentation evidence`, `stabilization blockers`, `supported Profile and target claims`, `generated status consumers` | `EXECUTION-PLAN-1.0`, `GAP-REGISTER`, `SUPPORT-MATRIX`, `TRACEABILITY-REGISTER` | — |
| `TRACEABILITY-REGISTER` | Governance registry | `Active` | `1` | `Registry` | no | [Unified traceability registry](../traceability/registry.toml) | `feature IDs`, `fixture IDs`, `requirement evidence chains`, `release evidence indexes` | `CONFORMANCE`, `LIFECYCLE-REGISTER`, `PROTOCOL-INVENTORY` | — |
| `DEPENDENCIES` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [Rust dependency record](../DEPENDENCIES.md) | `dependency licenses`, `MSRV`, `offline lock set` | — | — |
| `EVIDENCE-GAP-GOV-RFC-STATUS-001` | Gap discovery evidence | `Active` | `1` | `Evidence` | no | [RFC-0001 lifecycle status mismatch](../status/spec-gaps/GAP-GOV-RFC-STATUS-001.md) | `RFC lifecycle`, `governance consistency` | `RFC-0001` | — |
| `G0-CI-CONTRACT` | Automation contract | `Active` | `1` | `Evidence` | no | [G0 continuous-integration gate contract](../../.github/workflows/ci.yml) | `always-on pull-request gates`, `governance and compatibility validation`, `canonical determinism`, `Seed reproducibility`, `multi-platform tests`, `fuzz smoke`, `Rust MSRV` | `GAP-REGISTER`, `PROTOCOL-INVENTORY`, `SCHEMA-REGISTRY`, `TRACEABILITY-REGISTER`, `SUPPORT-MATRIX`, `TASK-STATUS` | — |
| `GRAMMAR-MAP` | Implementation map | `Active` | `1` | `Evidence` | no | [Ling Seed compiler and Tree-sitter grammar map](../grammar-map.md) | `Seed Author Source rule inventory`, `compiler CST and AST mapping`, `proposed Tree-sitter node mapping`, `shared corpus obligations`, `recovery-only helper boundary`, `deferred syntax exclusions` | `DEC-0002`, `DEC-0004`, `DEC-0005`, `DEC-0006`, `DEC-0007`, `DEC-0009`, `DEC-0010`, `DEC-0013`, `DEC-0014`, `DEC-0017`, `RFC-0001`, `SEMANTICS`, `LANGUAGE`, `GAP-REGISTER` | — |
| `SEED-RELEASE-REPORT` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [v0.0.1 Seed release gate report](../SEED-RELEASE-REPORT.md) | `release gates`, `CI evidence`, `published tag` | `SEED-TRACEABILITY` | — |
| `SEED-TRACEABILITY` | Evidence | `Evidence` | `0.0.1` | `Evidence` | no | [v0.0.1 unified specification traceability](../traceability/v0.0.1.md) | `stable feature IDs`, `normative clause mapping`, `Core and implementation links`, `positive and negative test evidence`, `differential state`, `release artifacts` | `TRACEABILITY-REGISTER`, `CONFORMANCE` | — |
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
