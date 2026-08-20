# Ling 1.0 详细执行编程计划包

> 状态：执行规划基线，非语言规范  
> 生成日期：2026-08-20  
> 语言名称：**Ling（中文：零）**  
> 默认命令：**`zero`**  
> 默认源码扩展名：**`.ling`**

## 1. 用途

本计划包把 `ROADMAP-1.0.md` 中的 G0～G6 路线图拆成 Codex 可以逐项实现、测试、审查和合并的工程任务，并补充：

- Rust workspace 与编译器分层；
- Checked Typed Core、Semantic Graph 与增量查询的共享架构；
- LSP 3.17 语言服务器的实现步骤；
- Zed 的 Tree-sitter 语法、高亮、缩进、轮廓、运行任务与 LSP 集成；
- 后续 DAP 调试器集成；
- 测试、模糊测试、差分测试、兼容性与发布流程；
- Codex 的 `AGENTS.md`、worktree、子代理和 PR 验收方式。

本文档包**不新增语言语义**。遇到语言行为、公开 Schema、ABI、Profile 或兼容性问题时，必须回到 Accepted RFC / `SEMANTICS.md` / `LANGUAGE.md`，不得由实现或编辑器插件自行决定。

## 2. 规范权威顺序

```text
Accepted RFC / Accepted decision
    > SEMANTICS.md
    > LANGUAGE.md
    > conformance corpus
    > ROADMAP-1.0.md
    > 本执行计划
    > implementation
```

若本计划与更高权威文档冲突，Codex 必须停止该任务，记录规范缺口，不得“按常见做法”自行补齐。

## 3. 文档清单与阅读顺序

1. `AGENTS.md`：直接放入仓库根目录的 Codex 总约束。
2. `00-MASTER-EXECUTION-PLAN.md`：总体批次、依赖、并行策略与里程碑门禁。
3. `01-REPOSITORY-AND-COMPILER-ARCHITECTURE.md`：目标 workspace、crate 边界与单向编译管线。
4. `02-G0-GOVERNANCE-AND-COMPATIBILITY.md`：规范缺口、协议版本和支持矩阵。
5. `03-G1-V0.1-LIVING.md`：模块、VM、Trait、增量、Formatter、CLI 和工具闭环。
6. `04-LSP-IMPLEMENTATION.md`：`zero lsp --stdio` 的完整实施计划。
7. `05-ZED-EXTENSION.md`：Zed 扩展、Tree-sitter、错误提示、语义高亮和调试路线。
8. `06-G2-V0.2-CONCURRENT.md`：Effect Handler、Task、Actor、Supervisor、Replay、Remote Actor。
9. `07-G3-V0.3-NATIVE.md`：Value/Managed/Resource、Ownership/Region、Native、FFI。
10. `08-G4-V0.4-HETEROGENEOUS.md`：Kernel、CPU 参考、SIMD、GPU/TPU、Placement。
11. `09-G5-V0.5-CRITICAL.md`：Critical、Node、Contract、模型检查、Evidence bundle。
12. `10-G6-V1.0-STABILIZATION.md`：稳定支持面、协议冻结、生态与发布。
13. `11-QUALITY-CI-RELEASE.md`：跨阶段测试矩阵、CI、fuzz、安全和性能基线。
14. `12-CODEX-WORKFLOW.md`：如何把任务交给本地 Codex，并进行并行、审查与合并。
15. `13-IMPLEMENTATION-BACKLOG.md`：按 ID 排列的总任务台账。
16. `14-FIRST-SPRINT-CODEX-TASKS.md`：首轮可直接复制给 Codex 的任务 prompt。
17. `REFERENCES.md`：本计划使用的官方技术资料。
18. `templates/`：任务、PR、Codex 配置和状态文件模板。
19. `baseline/`：生成计划时采用的路线图与规范快照。

## 4. 建议的仓库命名

本计划采用以下实现命名，避免把语言名称、二进制名称和 Rust crate 名称混在一起：

```text
语言显示名：Ling / 零
源码扩展名：.ling
CLI：zero
LSP 启动：zero lsp --stdio
DAP 启动：zero dap --stdio       # 后续 Preview
Rust crate 前缀：ling-
Tree-sitter：tree-sitter-ling
Zed 扩展：zed-ling
Zed Language name：Ling
LSP languageId：ling
```

若命名再次调整，应只修改集中式元数据和 release tooling，不得散落硬编码。

## 5. Codex 执行规则

每次只交付一个可审查的纵向任务：

```text
规范/decision
  → 正例与反例
  → Source/CST/AST
  → HIR/Checked Core
  → Semantic Graph
  → 执行或协议
  → 文档与验收命令
```

一个任务必须包含：

- 明确的 Task ID；
- 前置依赖；
- 允许修改的目录；
- 明确的非目标；
- 自动化验收命令；
- 规范冲突报告；
- 兼容性、Unicode、确定性影响说明。

禁止把“编译通过”当作完成。完成标准始终是：规范、实现、测试、诊断和证据闭环。

## 6. 推荐的第一轮执行顺序

```text
B00 关闭 Seed 基线与命名元数据
B01 建立 G0 台账、Schema 生命周期和 CI 骨架
B02 固化 CompilerSession / VFS / LineIndex / CheckedCore 边界
B03 建立 tree-sitter-ling 语法骨架和 Zed grammar-only 扩展
B04 实现最小 project/module graph
B05 实现 Typed Core → bytecode → verifier → VM 的最小纵向切片
B06 引入增量查询与 clean/incremental 等价测试
B07 实现 zero lsp 的 diagnostics / symbols / hover / definition
B08 把 zero lsp 接入 Zed，完成中文错误位置与快速修复测试
B09 Formatter、rename、completion、code action 与 Zed tasks
B10 基础 Trait 与 v0.1 全量差分/兼容验收
```

在 B10 之前，不实现 Actor、完整 Ownership、GPU、Critical 或多 Native backend。

## 7. 交付物使用方式

把本目录复制到目标仓库的 `docs/execution/`，并把本目录的 `AGENTS.md` 合并到仓库根目录。随后按 `13-IMPLEMENTATION-BACKLOG.md` 的 Ready 任务逐项交给 Codex。

每完成一个任务，应同步更新：

- `docs/traceability/<release>.md`；
- `docs/status/implementation-status.toml`；
- 对应 RFC/decision 的实现状态；
- conformance 测试索引；
- `CHANGELOG.md` 中的用户可见变化。


## 8. 推荐导入方式

```text
本包/AGENTS.md
    → 合并到目标仓库根 AGENTS.md

本包/00..14 + REFERENCES.md
    → docs/execution/

本包/templates/
    → docs/execution/templates/ 或项目对应模板目录

本包/baseline/
    → 仅作对照；真实仓库已有更新规范时不要复制成第二份权威
```

首轮直接从 `14-FIRST-SPRINT-CODEX-TASKS.md` 开始；每个 prompt 仍需根据真实目录做一次路径校正。
