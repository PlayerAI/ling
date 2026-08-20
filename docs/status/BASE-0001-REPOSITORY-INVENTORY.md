# BASE-0001 仓库基线盘点 / Repository Baseline Inventory

> 状态：**Done**  
> 完成日期：2026-08-20  
> 实现提交：`aa8c02894bd2fdd696ab60c97423d07c0ce9614a`  
> 盘点基线：`main@639790f4c609d137932d8432d9c5be681aa3e3c1`  
> 已发布基线：annotated tag `v0.0.1`，peeled target `652d19b9eaec2ab607edfe1a1e7ea742c861cf91`  
> 任务来源：[14-FIRST-SPRINT-CODEX-TASKS.md](../ling_execution_plan/14-FIRST-SPRINT-CODEX-TASKS.md) 中的 `BASE-0001`  
> 地位：本文记录工程事实和计划适配，不新增语言语义。

## 1. 结论

1. `v0.0.1 Seed` 已发布，其实现、测试和发布证据已闭合；RFC-0001 的源文件仍标为 Draft，该生命周期不一致由 GOV-0101 记录为治理缺口，不能把发布事实等同于 RFC Accepted。
2. 当前真实命名是 Ling / 零、CLI `ling`、源码扩展名 `.ling`。执行计划中的 `zero`、`.zero`、`zero.*` 和 `zero-*` 是过时占位符，不得进入实现。
3. 当前仓库已经具备 Seed 的共享检查管线、解释器、事务式 REPL、Semantic Graph、Audit Source 和 conformance runner；尚无 VM、project/package manager、增量数据库、LSP、Tree-sitter/Zed 仓库或 Native backend。
4. `docs/ling_execution_plan/` 可以作为非规范性任务包使用，但必须先按本文差异表适配。其 `baseline/` 只能用于历史对照。
5. G0 的只读盘点、治理和状态工作可以开始。G1 及以后涉及语言语义或公开协议的实现仍受 Accepted RFC/decision 门禁约束。

## 2. 已核对的权威材料

| 材料 | 当前路径 | 结论 |
| --- | --- | --- |
| 仓库约束 | [AGENTS.md](../../AGENTS.md) | 已保留原规则并补充执行计划治理规则 |
| Seed RFC | [RFC-0001.md](../RFC-0001.md) | Draft；不得作为 Accepted 稳定依据；生命周期不一致见 `GAP-GOV-RFC-STATUS-001` |
| 正式语义 | [SEMANTICS.md](../SEMANTICS.md) | 高于路线图和执行计划；§31 未决项不得由代码决定 |
| 语言设计 | [LANGUAGE.md](../LANGUAGE.md) | 确定 G0～G6 的能力方向和 Profile 边界 |
| 1.0 路线图 | [ROADMAP-1.0.md](../ROADMAP-1.0.md) | 非规范性工程顺序，不新增语义 |
| Accepted decisions | [decisions/](../decisions/) | DEC-0001～DEC-0016 已存在；其中 DEC-0001 固定错误码格式 |
| Seed 发布证据 | [SEED-RELEASE-REPORT.md](../SEED-RELEASE-REPORT.md) | `v0.0.1` tag 与跨平台发布门禁已记录 |
| 执行计划入口 | [ling_execution_plan/README.md](../ling_execution_plan/README.md) | 任务包可用，但命名、路径和状态假设需要适配 |
| 总体执行计划 | [00-MASTER-EXECUTION-PLAN.md](../ling_execution_plan/00-MASTER-EXECUTION-PLAN.md) | 批次顺序可保留；所有 `zero` 命令需改读为 `ling` |

规范权威顺序仍以根 [AGENTS.md](../../AGENTS.md) 为准。执行计划和本文均不能覆盖 Accepted RFC、`SEMANTICS` 或 `LANGUAGE`。

## 3. Git 与发布基线

| 项目 | 当前事实 |
| --- | --- |
| 分支 | `main`，与 `origin/main` 同为 `639790f4c609d137932d8432d9c5be681aa3e3c1` |
| 当前描述 | `v0.0.1-2-g639790f` |
| 发布 tag | annotated `v0.0.1` |
| tag target | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| tag 后提交 | `f06ab9e`、`639790f`，均为发布证据/记录文档 |
| BASE-0001 实现提交 | `aa8c02894bd2fdd696ab60c97423d07c0ce9614a` |
| Workspace 版本 | `0.0.1-dev` |
| Rust 工具链 | 默认 `1.97.1`；manifest 声明 MSRV `1.85` |
| Rust edition | `2024` |
| License | Apache-2.0 |

BASE-0001 开始前工作区已有以下未提交内容：

```text
 M README.md
?? docs/ROADMAP-1.0.md
?? docs/ling_execution_plan/
```

这些内容不是 BASE-0001 创建的实现变更。BASE-0001 不移动、删除或重写它们。

## 4. Workspace 与编译管线盘点

根 workspace 当前包含 14 个 member：13 个 `ling-*` crate 和一个 Unicode 生成工具。`fuzz/` 有独立 lockfile，并被根 workspace 排除。

| Member | 当前职责 |
| --- | --- |
| `ling-source` | 原始 bytes、UTF-8 解码、换行和 byte span |
| `ling-unicode` | Unicode 17.0.0 XID、NFC 与安全检查 |
| `ling-syntax` | lexer、offside/layout、error-bounded parser 与 CST |
| `ling-ast` | 从有效 CST lowering AST，保留来源信息 |
| `ling-hir` | HIR、pipeline lowering 与 Place 分类 |
| `ling-resolve` | module/import、名称解析、Prelude 与 confusable 检查 |
| `ling-types` | Seed 类型推导、nominal record/ADT/match 与 Value Restriction |
| `ling-effects` | Seed Effect、Capability 与入口检查 |
| `ling-semantic` | Checked `ProgramSnapshot`、Semantic Graph、canonical identity 与 reader |
| `ling-format` | 当前是 canonical Audit Source renderer/parser；不是通用 Author Source formatter |
| `ling-eval` | 只消费 checked `ProgramSnapshot` 的参考解释器 |
| `ling-diagnostics` | 稳定错误码、双语 human/JSON 诊断 |
| `ling-cli` | 共享文件/内存编译编排、CLI、REPL 与 conformance runner |
| `unicode-gen` | 固定 Unicode 17.0.0 生成表与一致性检查 |

当前真实管线是：

```text
Source → Syntax/CST → AST → HIR → Resolve → Type → Effect/Capability
                                                      ↓
                                             ProgramSnapshot
                                              ├─ Semantic Graph
                                              ├─ Audit Source
                                              └─ Interpreter / REPL
```

`crates/ling-cli/src/lib.rs` 暂时提供 `compile_path` / `compile_source` 共享编排。计划中的 `CompilerHost`、VFS、immutable `AnalysisSnapshot` 和独立 compiler-service 尚未实现；它们必须先经过 `ARCH-0101` 设计审查，不能通过一次大重构直接替换当前管线。

## 5. 当前命令面

BASE-0001 实际执行：

```text
cargo run --locked --offline -- --version
```

结果：

```text
ling 0.0.1-dev
Unicode 17.0.0
```

当前 binary 和源码确认的命令：

| 命令 | 状态 | 说明 |
| --- | --- | --- |
| `ling --version` | 已验证 | 输出工具版本和 Unicode 版本 |
| `ling check <file>` | Seed 已实现 | 复用共享 checked pipeline |
| `ling run <file>` | Seed 已实现 | 只解释 checked `ProgramSnapshot` |
| `ling semantic <file>` | Seed 已实现 | 输出 `ling.semantic/0.1` |
| `ling audit <file>` | Seed 已实现 | 输出可 round-trip 的 `ling.audit/0.1` |
| `ling repl` | Seed 已实现 | 事务式 human/JSON session |

`run/check/semantic/audit` 支持 `--format human|json`；`repl` 另支持 `--capability Console.Write`。

以下计划命令当前不存在，不能写入“已支持”文档或 Zed/LSP 配置：

```text
ling init
ling test
ling fmt
ling build
ling query
ling patch
ling lsp
ling replay
ling explain
ling evidence
ling migrate
ling dap
cargo xtask ...
```

所有 `zero ...` 形式与 Accepted RFC 冲突，不能作为兼容别名擅自实现。

## 6. 测试、Fuzz 与 CI 盘点

### 6.1 本次直接验证

执行：

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
```

结果：格式检查和 Clippy 通过；测试为 `138 passed; 0 failed`，另有全部 doc-test harness 通过。当前 corpus 包含：

- 32 个 `tests/conformance/` case；
- 3 个 `tests/snapshots/` case；
- 7 组 `tests/cli/` module fixture；
- 3 个 fuzz target：`source_bytes`、`lexer_utf8`、`parser_utf8`。

### 6.2 CI 现状

`.github/workflows/ci.yml` 当前定义：

- Windows、Linux、macOS 三平台 workspace job；
- Unicode generated-table diff、fmt、Clippy、tests、Rustdoc、release build；
- Ubuntu pinned-nightly 三个 fuzz corpus smoke；
- Rust 1.85 MSRV check；
- 下载依赖后，常规构建和测试显式 `--locked --offline`。

[SEED-RELEASE-REPORT.md](../SEED-RELEASE-REPORT.md) 记录候选 SHA 的五个远程 job 全部成功。本次 BASE-0001 没有重新运行远程 CI、长 fuzz、Rustdoc、release build、MSRV 或三平台矩阵；不得把历史证据表述为本次执行结果。

## 7. 执行计划假设与真实仓库差异

| 计划假设 | 当前事实 | BASE-0001 处理 |
| --- | --- | --- |
| CLI 为 `zero` | `SEMANTICS.md` 与 `LANGUAGE.md` 已确定 `ling`；RFC-0001 也记录该名称但仍是 Draft | 根 AGENTS 明确禁止采用过时占位符；后续计划应机械修订 |
| baseline 中使用 `.zero`、`zero.*`、`zero-*` | 当前使用 `.ling`、`ling.*`、`ling-*` | `baseline/` 仅作历史输入，永不覆盖当前规范 |
| 错误码采用 `L0000/P0000/...` 分区 | DEC-0001 和 `ERROR-CODES.md` 已采用 `L-<DOMAIN>-<NUMBER>` | GOV-0105 必须扩展现有注册表，不得建立第二套编码 |
| 规范文件位于仓库根 | 当前位于 `docs/` | 所有任务 prompt 在执行前校正路径 |
| 计划位于 `docs/execution/` | 当前包位于 `docs/ling_execution_plan/` | 本任务保留当前位置并提出最终路径；不做未授权批量移动 |
| root `conformance/` | 当前为 `tests/conformance/`，runner 在 `crates/ling-cli/tests/conformance.rs` | 继续使用 Accepted DEC-0003 路径 |
| 已有 `docs/status/` 与 `docs/traceability/` | BASE 前二者均不存在 | 本任务只建立最小 status；traceability 由 GOV-0107 建立 |
| 已有 `rfcs/`、`schemas/`、`editors/` | 当前均不存在 | 只有对应任务和 Accepted 决议允许时创建 |
| 已有 `ling-core-ir/project/db/ide/bytecode/vm/lsp` | 当前均不存在 | 视为目标架构；禁止建立空 crate 或占位 API |
| 可运行 `cargo xtask ...` | 当前没有 xtask member | 命令仅为建议，需独立工具任务落地后才能验收 |
| `ling.toml` / `ling.lock` 已定 | 文档明确标注需 decision | 保持 BlockedSpec，不创建格式或解析器 |
| Backlog 的 `Ready` 是实时状态 | 它只是生成时建议，BASE 前没有机器状态文件 | 从本任务起以 `implementation-status.toml` 为状态入口 |
| Tree-sitter/Zed 仓库和工具存在 | 当前仓库没有 `editors/`，外部仓库/工具未验证 | TS/Zed 任务开始前重新核对官方 API、路径和工具安装 |

## 8. 执行计划最终路径建议

最终建议使用：

```text
docs/execution/
```

理由：路径简短，与计划内部引用一致，并能清楚区分正式规范与工程执行文档。迁移必须作为单独、可审查的纯文档任务完成：

1. 先修正所有 `zero`/`.zero`/旧 schema 与旧 crate 引用；
2. 将 `baseline/` 明确标记为历史快照，或在不需要时从最终计划包排除；
3. 原子移动目录并更新全部相对链接；
4. 运行链接和 checksum 校验；
5. 在 README 中只链接当前计划入口，不建立第二份规范权威。

BASE-0001 不执行该批量移动。当前机器状态使用真实路径 `docs/ling_execution_plan/`；在迁移合并前，任务 prompt 必须据此校正。

## 9. 未确认的命令、目录与外部状态

以下项目没有在 BASE-0001 中得到直接运行证据：

- `cargo xtask` 及计划中的 governance/schema/traceability helper；
- `ling init/test/fmt/build/query/patch/lsp/replay/explain/evidence/migrate/dap`；
- Tree-sitter CLI、`tree-sitter-ling` repository 和 corpus；
- Zed extension API、Dev Extension、registry 和 `wasm32-wasip2` 构建；
- LSP 3.17 transcript、UTF-16 position adapter 和编辑器 smoke；
- VM bytecode/verifier、增量缓存、package manifest/lock；
- G2～G5 的 runtime、backend、device、proof 和 evidence 工具；
- 本次 worktree 的 Linux/macOS CI、nightly fuzz 运行、MSRV、Rustdoc 和 release build；
- `docs/execution/`、`docs/traceability/`、`rfcs/`、`schemas/`、`editors/` 及计划新增 crates。

它们必须保持“未实现/未验证”，不能根据计划文字推断为可用。

## 10. 规范冲突与阻断项

BASE-0001 当时未识别出规范生命周期冲突。GOV-0101 随后核对源文件发现：`RFC-0001.md` 明确标为 Draft，但本盘点旧版和根 AGENTS 扩展段曾误称其为 Accepted。该问题现记录为 [GAP-GOV-RFC-STATUS-001](spec-gaps/GAP-GOV-RFC-STATUS-001.md)，不得静默提升 RFC 状态。

发现的两个执行计划冲突均已被更高权威明确解决：

1. `zero` 与 `ling`：`SEMANTICS.md` §31 与 `LANGUAGE.md` 的项目命名说明选择 `ling`；Draft RFC-0001 记录相同结果但不是 Accepted 依据；
2. `L0000/P0000/...` 与 `L-<DOMAIN>-<NUMBER>`：Accepted DEC-0001 明确选择后者。

后续修改低权威计划即可，不得为已解决事项重新发明 RFC。真正阻断 G1 的未决项仍包括 package identity/module/lock、bytecode/verifier observable semantics、Trait coherence/orphan/lowering、incremental/hash lifecycle、Formatter preservation 和 LSP/Semantic Transaction protocol boundaries；它们在 Accepted RFC/decision 前只能进行隔离研究或测试设计。

## 11. BASE-0001 验收证据

| 要求 | 证据 |
| --- | --- |
| workspace/crate/command/test/CI 盘点 | 本文 §§3～6 |
| 计划假设差异 | 本文 §7 |
| 最终路径建议 | 本文 §8 |
| 根 AGENTS 合并 | [AGENTS.md](../../AGENTS.md) 保留原规则并新增计划治理/任务工作流 |
| 最小状态文件 | [implementation-status.toml](implementation-status.toml) |
| 不修改语言语义 | BASE diff 限定为 AGENTS、计划和 `docs/status/`；未修改 `crates/`、`tests/` 或规范语义 |
| 测试 | 当前 worktree 的 fmt、locked/offline Clippy 与 workspace tests 通过；测试 138 passed |
| 未确认项 | 本文 §9 |
| 规范冲突处理 | 本文 §10；GOV-0101 后续登记 `GAP-GOV-RFC-STATUS-001`，不改变 BASE 的代码/测试结论 |

下一项可执行工作应从 G0 治理任务选择。`GOV-0101`、`GOV-0102` 和 `GOV-0104` 可以在不修改语言语义的前提下准备；任何会冻结公开行为的结果仍须经过相应决议门禁。
