# 首轮可直接交给 Codex 的任务包

> 用途：项目从当前 Seed 基线进入 G0 / v0.1 的首批复制粘贴任务  
> 使用前：确认真实仓库路径与已有实现；Codex 必须先做只读盘点，不得假定本计划中的目标目录已经存在

## 0. 首轮目标

首轮不追求大量语言功能，只建立未来开发不会返工的地基：

```text
规范/状态/错误码/协议 registry
    +
CompilerSession / VFS / LineIndex 边界
    +
Tree-sitter grammar skeleton
    +
VM 最小纵向测试
    +
LSP protocol fixtures
```

建议用 5 个 worktree 并行，主分支由 Integrator 合并。

# Task A：仓库基线盘点与执行计划落位

> 状态：**Done**（2026-08-20，当前 worktree 已验证，尚未提交）  
> 机器状态：[implementation-status.toml](../status/implementation-status.toml)  
> 验收证据：[BASE-0001-REPOSITORY-INVENTORY.md](../status/BASE-0001-REPOSITORY-INVENTORY.md)

```text
执行 BASE-0001。

目标：只读分析当前 Ling/零仓库，确认 v0.0.1 Seed 的真实实现状态，并把 docs/execution 计划接入仓库，不修改语言语义。

必须阅读：
- 根 AGENTS.md（若存在）
- LANGUAGE.md / SEMANTICS.md / RFC-0001.md
- ROADMAP-1.0.md
- docs/execution/README.md
- docs/execution/00-MASTER-EXECUTION-PLAN.md

工作：
1. 输出当前 workspace/crate/command/test/CI 盘点；
2. 标出本计划假设与真实仓库差异；
3. 建议执行计划文档的最终路径；
4. 创建或合并根 AGENTS.md，但不要覆盖已有有效规则；
5. 建立 docs/status/implementation-status.toml 的最小骨架；
6. 不移动核心实现，不改 parser/type/eval 行为。

验收：
- 文档链接可解析；
- git diff 仅包含文档/状态/agent guidance；
- 明确列出未确认的命令和目录；
- 发现规范冲突则创建 spec-gap，不自行决定。
```

# Task B：GOV-0101 规范权威索引

```text
执行 GOV-0101。

目标：创建机器和人类都可读取的规范权威索引，确保 Codex/CI 能判断某项行为由哪个 Accepted RFC/规范决定。

允许修改：
- docs/spec-index.*
- docs/status/**
- tools/traceability/**
- 对应测试

不得修改：
- compiler/runtime/editor implementation
- 任何语言语义

实现：
1. 扫描 RFC、decision、SEMANTICS、LANGUAGE；
2. 记录文档 ID、状态、版本、覆盖领域、supersedes、路径；
3. 检测重复 ID、悬空链接、Accepted 文档缺文件；
4. 生成稳定排序的人类报告；
5. 为 CI 提供非零退出码。

测试：重复 ID、missing file、superseded chain、路径含中文。
```

# Task C：GOV-0102 规范缺口台账

```text
执行 GOV-0102。

目标：把 SEMANTICS 待决项、ROADMAP 规范门禁、实现中的 TODO/冲突归并成单一 spec-gap registry。

先只读搜索，不批量改实现。

每项字段：
- id/title/status
- blocked release/tasks
- observable behavior
- authority/candidate RFC
- irreversible consequences
- required positive/negative/migration tests
- owner/next action

要求：
- 不替未决问题选择答案；
- 输出按 blocker/phase 稳定排序；
- 建测试保证 Accepted/Rejected 状态合法；
- 将 v0.1 的 package/module、bytecode、Trait、hash/schema、formatter/LSP 缺口标为最高优先级。
```

# Task D：GOV-0105 Diagnostic 注册表

```text
执行 GOV-0105。

目标：建立公共错误码的单一 registry 和生成/验证工具。

要求：
1. 盘点现有 parser/resolver/type/effect/capability/runtime/tool 错误；
2. 不改变现有错误含义；冲突项先报告；
3. registry 包含 code、phase、stability、Chinese/English title、payload schema、first version；
4. 生成 Rust 常量/文档或提供验证器，但不要建立两份手写权威；
5. CI 拒绝重复 code、缺翻译、未注册 public error；
6. 测试稳定排序和 JSON 输出；
7. span 保持原始 UTF-8 bytes。
```

# Task E：ARCH-0101 CompilerSession / VFS / LineIndex 设计

```text
执行 ARCH-0101（设计先行，不直接大重构）。

目标：为 CLI、增量编译和 LSP 定义共享 CompilerSession、VFS、SourceFile、LineIndex 和 snapshot 边界。

先阅读：
- docs/execution/01-REPOSITORY-AND-COMPILER-ARCHITECTURE.md
- docs/execution/04-LSP-IMPLEMENTATION.md
- 当前 parser/source/diagnostic API

交付：
1. ADR：当前结构、目标接口、依赖方向、迁移顺序；
2. 原始 byte span 不变量；
3. UTF-8 byte ↔ line/UTF-16 conversion 只在 adapter；
4. disk source 与 open-document overlay；
5. immutable snapshot/revision；
6. cancellation/stale result；
7. 最小接口原型和 unit fixtures（若不会倒逼迁移）。

非目标：本任务不完整实现 LSP 或增量引擎。
```

# Task F：TS-3101 Tree-sitter 语法映射

```text
在独立 tree-sitter-ling worktree/repo 执行 TS-3101。

目标：创建 Accepted Ling Seed 语法到 compiler node、Tree-sitter node、corpus 的映射表。

要求：
- 从正式语法和现有 parser 测试提取，不凭印象；
- 中文标识符是一等 case；
- 明确哪些节点仅用于 Tree-sitter error recovery；
- 列出 offside/缩进、precedence、pattern/type 的风险；
- future 关键字不得提前成为正式语法；
- 为每一 major construct 创建最小 corpus fixture；
- 不修改 compiler parser。
```

# Task G：TS-3102 Tree-sitter Grammar Skeleton

```text
执行 TS-3102，依赖 TS-3101。

目标：建立可运行的宽度优先 grammar skeleton，覆盖 source_file、declaration、type、expression、pattern、identifier、literal、comment 的主要分类。

要求：
- CST 结构直观，不机械复制规范中的多层 precedence nonterminal；
- identifier 支持规范允许的 Unicode；
- 每条 rule 有 test/corpus；
- 保留不完整编辑的 error recovery；
- 不使用 external scanner，除非有独立 ADR 证明必要；
- 运行 tree-sitter generate/test/parse examples；
- 记录与 compiler parser 的已知差异，不把差异静默接受为 Ling 语义。
```

# Task H：VM 测试先行

```text
执行 TEST-VM-0001，为 VM-1201～1204 建立 failing-first corpus 和 differential harness 骨架，不实现 VM。

目标：定义首个纵向切片的可观察行为：
- integer/bool/text constants
- let binding
- function call
- Console.print capability/effect
- return value
- stable Fault/diagnostic

要求：
- 输入来自 Checked Typed Core fixture 或经现有 checker；
- 不允许 VM 直接吃 AST；
- harness 可运行 interpreter 并保存规范结果；
- VM 未实现时测试明确 skip/fail reason；
- 设计 malformed bytecode verifier cases；
- 不决定尚未 RFC 的 opcode encoding。
```

# Task I：LSP UTF/协议 Fixtures

```text
执行 LSP-FIXTURE-0001，不实现完整语言服务。

目标：建立 LSP 3.17 initialize/text sync/diagnostic 的协议 fixture 基础，重点验证 Ling 的原始 UTF-8 span 到 LSP position 转换。

fixtures 至少：
- ASCII
- 人物.血量 = 100
- emoji 位于错误前
- combining mark
- supplementary-plane character
- CRLF
- 多行 incremental edit
- stale document version

要求：
- compiler 内部不存 LSP Position；
- fixture 断言 UTF-16 character；
- JSON-RPC 输入大小/错误有界；
- expected output 稳定排序；
- 为 diagnostics/hover/rename 后续复用。
```

# Task J：Zed Grammar-only 扩展

```text
执行 ZEXT-3301，依赖 TS-3102。

目标：建立最小 zed-ling 开发扩展，让 Zed 识别 .ling，并使用固定 revision 的 tree-sitter-ling 提供基础高亮。

交付：
- extension.toml
- languages/ling/config.toml
- grammar pin
- minimum highlights.scm
- README 的本地 dev extension 安装步骤
- accepted license

要求：
- 不嵌入 checker；
- 不宣称已有错误提示/LSP；
- Rust Wasm extension 可后置，本任务优先 grammar-only；
- Zed 启动后验证中文字段/函数名高亮；
- 记录 Zed/grammar version 和测试环境。
```

# Task K：首轮独立审查

```text
只读审查 BASE-0001、GOV-0101/0102/0105、ARCH-0101、TS-3101/3102、TEST-VM-0001、LSP-FIXTURE-0001、ZEXT-3301 的合并候选。

重点：
1. 是否由实现/Tree-sitter/LSP 偷偷创造语义；
2. registry 是否唯一且 deterministic；
3. UTF-8 bytes/UTF-16 adapter 是否清晰；
4. 是否存在公共接口多代理并行冲突；
5. 是否覆盖中文标识符和 Unicode 安全；
6. 是否有 host panic、silent fallback、任意 shell/network；
7. 是否更新 status/traceability；
8. 给出按 P0～P3 分类 findings。

不要直接大改；先输出审查报告和最小修复建议。
```

## 首轮合并门禁

- [ ] 规范索引、缺口、错误码、状态和 traceability 有唯一入口；
- [ ] CompilerSession/VFS/LineIndex 边界经过评审；
- [ ] Tree-sitter 能解析/高亮主要 Seed 结构，但没有成为语言权威；
- [ ] VM 测试路径明确要求 Checked Core；
- [ ] LSP Unicode position fixtures 先于功能实现；
- [ ] Zed 可识别 `.ling`，且功能声明诚实；
- [ ] 所有分支经独立审查后再开启 Project/VM/LSP 实现批次。
