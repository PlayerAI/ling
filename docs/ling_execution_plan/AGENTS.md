# AGENTS.md — Ling / 零语言仓库开发约束

## 1. 项目目标

本仓库实现 Ling（中文名“零”）语言。人类表达层接近 F#/OCaml；编译器以 Type、Effect、Capability、Ownership、Contract 和 Semantic Graph 为语义基础；CLI 名为 `zero`。

你的职责是实现、测试、审查和报告，不得自行发明语言语义。

## 2. 权威顺序

发生冲突时按以下顺序执行：

1. Accepted RFC 与 Accepted decision；
2. `SEMANTICS.md`；
3. `LANGUAGE.md`；
4. conformance tests；
5. `ROADMAP-1.0.md` 与 `docs/execution/`；
6. implementation。

实现与规范冲突时，修改实现；规范之间冲突时，停止并提交规范缺口，不得猜测。

## 3. 不可违反的不变量

- 所有用户源码位置以原始 UTF-8 byte span 为内部权威表示。
- Unicode XID、NFC 与安全规则固定到规范声明的 Unicode 版本。
- 公共诊断必须有稳定错误码，并提供简体中文和英文消息。
- 用户输入错误不得触发 host panic、越界、死循环或不受控资源增长。
- Canonical 输出不得依赖 HashMap 顺序、线程调度、主机路径、地址或 Rust `Debug` 文本。
- Interpreter、VM、Native、Kernel 不得直接执行未检查 AST；必须消费 Checked Typed Core 或版本化派生 IR。
- CLI、LSP、Formatter、Zed 插件不得复制一套独立语言语义。
- 未实现能力必须明确拒绝，禁止静默降级或伪装为可用。
- AI 生成的修改不自动可信；必须通过 checker、tests 和门禁。

## 4. 语言语义变更门禁

下列变更必须先有 Accepted RFC/decision：

- 语法、名称解析、类型、求值顺序；
- Effect、Capability、Trait、Ownership、Task、Actor、Node、Kernel；
- Semantic ID、Canonical Bytes、公开 JSON Schema；
- 包身份、ABI、FFI、远程协议；
- Profile、确定性和兼容性承诺。

在缺少 RFC 时，可以提交隔离的实验原型，但必须：默认关闭、标记 Experimental、不得污染稳定 API、不得更新规范快照以“证明”实现正确。

## 5. Rust 工程规则

- 保持 crate 单向依赖，禁止编译阶段环依赖。
- 公共类型优先表达领域不变量，不使用松散 `serde_json::Value` 传递核心语义。
- 解析错误、类型错误和用户程序 Fault 使用结构化类型；`panic!` 只用于进程不可恢复的内部不变量，并应尽量转为可测试错误。
- 禁止在 canonical 路径直接遍历无序集合；先按规范 key 排序。
- 避免在前端核心引入 async；异步只存在于 LSP/runtime adapter 等边界。
- 新增第三方生产依赖前，记录用途、替代方案、许可证、维护状态和最小 feature 集。
- `unsafe` 只能位于经记录的 TCB/Target Primitive 边界；普通 compiler/tooling crate 不得随意新增。

## 6. 测试要求

每个行为变更至少包含适用的以下证据：

- 正向 conformance；
- 负向 conformance；
- 诊断错误码与 span；
- round-trip / canonicality；
- property test；
- parser/decoder/verifier fuzz target；
- Interpreter/VM/Native differential；
- clean/incremental equivalence；
- 中英文与 Unicode/emoji/CRLF fixture；
- 离线/锁定构建。

修复 bug 时先添加最小复现测试，再修改实现。

## 7. LSP 与编辑器规则

- `ling-lsp` 只是协议适配层；分析逻辑放在可独立测试的 `ling-ide`/compiler service。
- LSP position 与内部 byte span 的转换必须集中在 `LineIndex`，不得散落计算。
- 支持客户端协商的位置编码；至少正确处理 UTF-16 和 UTF-8。
- open document overlay、磁盘文件和 package graph 必须通过 VFS 统一。
- rename、code action 和 semantic patch 必须检查文档版本或 ProgramSnapshot，拒绝 stale edit。
- Tree-sitter grammar 只负责编辑器 CST，不是语言语义权威；必须与 compiler corpus 做交叉测试。

## 8. Zed 扩展规则

- Zed 扩展独立于编译器核心；不得把 checker 编译进扩展 Wasm。
- 语法高亮由 Tree-sitter 与 `.scm` queries 提供；类型错误、跳转、重命名等由 `zero lsp --stdio` 提供。
- 扩展优先查找用户 PATH 中的 `zero`；发布版可按官方扩展 API 下载对应平台 release，但不得把 language server 二进制直接打包进扩展。
- grammar revision 必须固定到具体 Git commit。
- 高亮、缩进和 outline 必须有 query fixtures；中文标识符、组合字符、emoji 前缀和错误恢复必须测试。

## 9. 每个任务的工作流程

1. 读取 Task ID 指定的规范、计划和前置实现。
2. 先写或更新验收测试。
3. 若发现未决语义，立即停止语义实现并记录 gap。
4. 实现最小纵向闭环，不顺手实现相邻未来能力。
5. 运行任务指定命令与仓库默认检查。
6. 更新 traceability/status 文档。
7. 提交总结：实现、测试、规范冲突、兼容影响、推迟项。

## 10. PR/提交要求

分支建议：`task/<TASK-ID>-<short-name>`。

PR 描述必须包含：

- Task ID 与规范链接；
- 变更的 pipeline 层；
- 新增测试；
- Diagnostic/Schema/Semantic ID/CLI/ABI 影响；
- Unicode 与确定性影响；
- 未实现或有意推迟的相邻能力；
- 完整验收命令和结果。

一个 PR 不应同时修改互不依赖的语言语义与编辑器功能。

## 11. Codex 并行规则

可以并行：只读探索、测试 corpus、文档、独立 query 文件、独立 fuzz harness。

谨慎并行：多个 agent 同时修改 parser、Typed Core、Semantic Graph schema、公共诊断注册表或 Cargo workspace。

写冲突高的工作必须分 worktree，并由一个集成 agent 顺序合并；不得让多个 agent 直接修改同一核心文件。
