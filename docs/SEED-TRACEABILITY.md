# Seed 规范追踪矩阵 / Seed Specification Traceability

> 状态 / Status: working evidence index for `v0.0.1-dev`
> 基线 / Baseline: `a88e5ef89abc3c26e0910016dc6305ee79c53e3e`
> 更新日期 / Updated: 2026-08-19

本文把 RFC-0001 §18 和 `IMPLEMENTATION.md` §10 映射到可重复执行的实现与测试证据。`已验证（本地）` 不等同于发布通过；需要 Accepted 决议、候选 commit 和同一 SHA 的三平台 CI 结果时，状态明确保留为阻断。

This document maps RFC-0001 §18 and `IMPLEMENTATION.md` §10 to repeatable implementation and test evidence. “Verified locally” is not a release claim; requirements for accepted decisions, a candidate commit, and three-platform CI remain explicit blockers.

| 条款 / Clause | 正向证据 / Positive evidence | 反向证据 / Negative evidence | 实现路径 / Implementation | 状态 / Status |
| --- | --- | --- | --- | --- |
| §18.1 人物可运行 | `examples/人物.ling`; `p9-person-run`; CLI example matrix | host Console failure unit test | `ling-cli` → shared compile → `ling-eval` | 已验证（本地） / Verified locally |
| §18.2 可检查与稳定 JSON | CLI example matrix; diagnostic JSON unit tests | `p7-type-error`, `p8-record-missing`, `p8-match-nonexhaustive` | `ling-diagnostics`, `ling-cli` | 已验证（本地） / Verified locally |
| §18.3 中文完整性 | `人物.ling`, `adt-match.ling`; Semantic/Audit name assertions; Chinese REPL process test | Unicode invalid/confusable suites | Source → HIR → Semantic/Audit → Eval/REPL | 已验证（本地） / Verified locally |
| §18.4 类型与 Place | `ling-types` generic record/ADT/Prelude, expected record, record/wildcard/tuple/constructor patterns, exhaustive match and mutable field tests; `p8-prelude-run`, `p8-record-pattern-run` | mismatch, missing/duplicate/unknown field, `p8-prelude-redefinition`, `p8-constructor-arity`, `p8-match-nonexhaustive`, `p8-unreachable-case`, `p8-immutable-field` | `ling-resolve`, `ling-types`, `ling-eval` | 已验证（本地） / Verified locally |
| §18.5 Effect/Capability | pure/Console tests; `map` callback propagation test | `p7-missing-capability`, `p9-map-missing-capability` | `ling-effects` resolved call graph | 已验证（本地） / Verified locally |
| §18.6 Semantic Graph / Audit | all RFC §6.11 categories (`Module`, `Type`, `Field`, `Variant`, `Binding`, `Function`, `Parameter`, `Pattern`, `Expression`, `Effect`, `Capability`); deterministic node/source/owner IDs; canonical Semantic/Audit round-trip and independent-process byte equality | bad Graph/Audit schema/version/ID/node kind, duplicate IDs, dangling/cyclic owners, dangling source IDs/targets, unknown core fields | `ling-semantic`, `ling-format`, `ling-cli audit` | 已验证（本地） / Verified locally |
| §18.7 稳定性 | workspace tests; Rust 1.85 MSRV check; shared compiler library; transactional REPL unit/process tests; Rustyline interrupt path; Linux/macOS real-PTY interrupt fixture; parser fuzz corpora; CI matrix config | parser recovery; compile/runtime rollback; incomplete/invalid submissions | `.github/workflows/ci.yml`, `fuzz/`, `ling-cli::session` | Windows 本地 Core 已验证；Unix PTY fixture、Ubuntu fuzz 与候选 SHA 三平台 CI 待执行 / Windows-local Core verified; Unix PTY fixture, Ubuntu fuzz, and candidate-SHA matrix pending |

## Seed 增量条款 / Seed Incremental Requirements

| 要求 / Requirement | 证据 / Evidence | 状态 / Status |
| --- | --- | --- |
| 泛型 nominal record/ADT、alias 与 Prelude | `ling-types::instantiates_generic_records_and_prelude_variants_independently`; cross-module alias; `p8-prelude-run` | 已验证（本地） / Verified locally |
| Bool/variant 穷尽性与 witness | `checks_boolean_and_variant_exhaustiveness_with_stable_witnesses` | 已验证（本地） / Verified locally |
| 不可达分支与 guard 规则 | `reports_unreachable_cases_and_ignores_guards_for_coverage` | 已验证（本地） / Verified locally |
| wildcard/tuple/record/constructor pattern | parser/type/evaluator tests; `p8-record-pattern-run`, `p8-constructor-arity` | 已验证（本地） / Verified locally |
| 三个规范示例 | `p9-person-run`, `p9-adt-match-run`, `p9-pipeline-run`; CLI example matrix | 已验证（本地） / Verified locally |
| `map` callback Effect 与顺序 | `map_propagates_its_callback_effect_and_capability`; evaluator ordering test | 已验证（本地） / Verified locally |
| Local/higher-order Effect | checked `A -> B ! ε` view; latent unused local closure; callback parameter propagation through user wrapper; root-typed `State<T>`/unused-Capability test | 已验证（本地） / Verified locally |
| 任意精度 `max/min/sum` | evaluator big integer/negative/empty-list test | 已验证（本地） / Verified locally |
| Builtin namespace | module-scope `max/min/map/sum` redefinition rejection plus local lexical shadowing | 已验证（本地） / Verified locally |
| Record Value semantics | copy, mutable-field update, and immutable record-update evaluator test (`1/2/3`) | 已验证（本地） / Verified locally |
| `Text.format` fault | zero/multiple-placeholder structured Runtime Fault tests | 已验证（本地） / Verified locally |
| Semantic reader 兼容性 | `semantic_reader_round_trips_and_accepts_namespaced_extensions`; negative validation test | 已验证（本地） / Verified locally |
| Semantic ID canonical properties | whitespace/comment/CRLF/path/alpha invariance; literal/operator/effect/name changes; dependency-body non-cascade plus ProgramId invalidation | 已验证（本地） / Verified locally |
| Audit model/format/CLI | Accepted DEC-0015; `ling-format` round-trip/negative tests; CLI independent-process test | 已验证（本地） / Verified locally |
| Transactional REPL | Accepted DEC-0016; shared compiler; generation, rollback, span-map and human/JSON process tests | 已验证（本地） / Verified locally |
| 内部错误与快照失败 / Internal and snapshot failures | `L-INTERNAL-0001`, stable BLAKE3 incident fingerprint, local `ling.internal-incident/0.1` reproduction report; independent Semantic reader round-trip; exit 5/6 separation; host I/O remains exit 4 | 已验证（本地） / Verified locally |
| Seed `main` 入口 | explicit/implicit Main unit tests; `p7-invalid-main`; `p12-implicit-main-run`; missing/non-Main fixtures | 已验证（本地） / Verified locally |
| Seed module/import 边界 | happy alias import plus missing/declaration/case/duplicate-alias/two- and three-cycle fixtures | 已验证（本地） / Verified locally |
| Unicode 可疑混写 | resolver module/alias/definition/local tests; `p12-mixed-script` | 已验证（本地） / Verified locally |
| 依赖许可证、MSRV 与 `unsafe` 清单 | `DEPENDENCIES.md`; root/fuzz lockfiles and metadata audit; CI fetch/build network separation with offline gates | 已验证（当前锁文件） / Verified for current lockfiles |
| Seed 外 Char 字面量显式拒绝 | `p12-char-literal-unsupported`; lexer type-variable regression test | 已验证（本地） / Verified locally |
| Seed 相等性边界 | primitive/tuple/list/record/variant type tests; `p12-function-equality` | 已验证（本地） / Verified locally |
| Seed `f64` 字面量 | finite lexer validation; IEEE equality evaluator test; `p12-f64-equality-run`; non-finite overflow rejection | 已验证（本地） / Verified locally |
| Seed Value Restriction | polymorphic list/record/constructor values; mutable/expansive/mutable-field restriction reasons and JSON Facts | 已验证（本地） / Verified locally |
| 三平台候选结果 | CI matrix exists | 阻断：尚无候选 commit 的远程结果 / Blocked: no remote candidate results |

## 重复执行 / Reproduction

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
cargo doc --workspace --all-features --no-deps --locked --offline
cargo build --workspace --all-features --release --locked --offline
cargo +1.85 check --workspace --all-features --locked --offline
cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline
```

只有矩阵中所有阻断项关闭、同一候选 SHA 的 Windows/Linux/macOS CI 全绿且工作区干净时，才能把本文作为发布证据。

This matrix becomes release evidence only after every blocker is closed, Windows/Linux/macOS CI is green for the same candidate SHA, and the release worktree is clean.
