# 使用本地 Codex 执行 Ling 开发计划

> 目标：把本计划转换成可审查、可并行、可回滚的本地 Codex 工作流  
> 核心原则：Codex 负责实现、测试、挑战和提出 RFC；不得自行创造稳定语言语义

## 1. 第一次交给 Codex 前的仓库准备

建议目录：

```text
ling/
├── AGENTS.md
├── Cargo.toml
├── rust-toolchain.toml
├── docs/
│   ├── LANGUAGE.md
│   ├── SEMANTICS.md
│   ├── RFC-0001.md
│   ├── ROADMAP-1.0.md
│   ├── decisions/
│   ├── rfcs/
│   ├── traceability/
│   ├── status/
│   └── execution/              # 本计划包，除 AGENTS.md
├── crates/
├── tests/
├── examples/
├── fuzz/
├── benchmarks/
├── tools/
└── .codex/
    ├── config.toml             # 可选项目配置
    └── agents/                 # 可选自定义 agent 配置
```

准备动作：

1. 将本包 `AGENTS.md` 放入根目录，合并已有规则而不是盲目覆盖；
2. 把规范与路线图放入固定路径；
3. 建立 `docs/status/implementation-status.toml`；
4. 建立 `docs/traceability/`；
5. 建立最小 CI 和统一命令；
6. 确保 Codex 可以运行 build/test/fmt/lint，但默认不能访问密钥、生产系统或无关目录；
7. 提交一个干净基线 tag，例如 `seed-plan-baseline`。

## 2. AGENTS.md 分层

根 `AGENTS.md` 只放所有任务都必须遵守的规则：

- 规范权威顺序；
- 禁止自行补语义；
- Unicode/span/diagnostic/determinism 不变量；
- 通用 build/test 命令；
- PR/任务完成条件。

复杂子目录可放更近的 `AGENTS.md`：

```text
crates/ling-syntax/AGENTS.md
    parser/CST/span/corpus 规则

crates/ling-types/AGENTS.md
    type/effect solver 不变量

crates/ling-lsp/AGENTS.md
    UTF-8↔UTF-16、stale request、protocol fixtures

editors/zed-ling/AGENTS.md
    不复制语言语义、Zed extension build/test

tests/AGENTS.md
    conformance fixture 格式
```

目录级规则只覆盖本目录的实现细节，不得修改语言语义权威顺序。

## 3. 一次 Codex 任务的标准输入

不要直接说：

> “实现 Ling v0.1。”

应使用 `templates/TASK-PROMPT.md`，至少包含：

```text
Task ID
目标
规范依据
前置依赖
允许修改目录
禁止修改目录
现有实现入口
必须先阅读的文件
明确非目标
实现步骤
必须新增的测试
验收命令
规范冲突处理
输出/提交要求
```

### 示例：VM 最小纵向切片

```text
执行任务 VM-1201：定义 v0 bytecode instruction model。

先阅读：
- docs/SEMANTICS.md 中求值与 Fault 章节
- docs/RFC-0001.md
- docs/execution/03-G1-V0.1-LIVING.md 的 VM-1201
- crates/ling-core-ir 与 interpreter

允许修改：
- crates/ling-bytecode/**
- tests/bytecode/**
- docs/traceability/v0.1.md

不得修改：
- parser grammar
- source language semantics
- public diagnostic code meaning

先提交设计说明和 opcode table；发现未规范行为就停下并创建 spec-gap，不得猜测。
```

## 4. Codex 会话分工

建议角色：

| 角色 | 职责 | 默认写权限 |
| --- | --- | --- |
| Integrator | 维护任务图、公共接口、合并和最终验收 | 全仓，但谨慎 |
| Spec Auditor | 查 RFC/规范冲突，生成 gap，不发明答案 | docs/rfcs/status |
| Compiler Agent | 单一纵向切片实现 | 指定 crates/tests |
| Test Agent | 先写 conformance/property/fuzz | tests/fuzz |
| Reviewer Agent | 只读审查 diff、风险、遗漏 | 无写或仅 review |
| LSP Agent | IDE service/protocol fixtures | ling-ide/ling-lsp/tests |
| Zed Agent | Tree-sitter/query/extension | 独立扩展仓库 |
| Release Agent | CI、repro、artifacts、protocol compatibility | tools/ci/release |

一个 agent 不应同时是同一 PR 的唯一实现者和唯一审查者。

## 5. 并行与 Git Worktree

### 5.1 适合并行的任务

- Spec Auditor 列缺口；
- Test Agent 写尚未通过的 corpus；
- Compiler Agent 实现隔离 crate；
- LSP Agent 写 protocol fixture；
- Zed Agent 写 Tree-sitter queries；
- Reviewer Agent只读分析；
- 文档、benchmark、fuzz harness。

### 5.2 不适合并行写同一接口

- AST/HIR/Checked Core 公共 enum；
- Semantic Graph schema；
- Error code registry；
- bytecode opcode/encoding；
- Effect/Ownership/Trait solver；
- Cargo workspace/dependency root；
- protocol version registry。

这类任务由单一 integrator 先冻结接口，再把外围工作分发出去。

### 5.3 Worktree 命名

```text
wt/spec-g0-schema
wt/test-vm-differential
wt/impl-vm-verifier
wt/lsp-diagnostics
wt/zed-grammar
wt/review-vm
```

分支命名：

```text
codex/<task-id>-<short-name>
```

每个 worktree 只负责一个 coherent unit，不在同一分支顺便修无关问题。

## 6. 推荐的子代理工作流

主 agent 保持需求、规范和决策上下文，子代理处理边界清晰的工作：

```text
Main / Integrator
├── Spec subagent: 找对应规范和未决点
├── Test subagent: 提出正反例、property、fuzz
├── Implementation subagent: 在限定目录实现
└── Review subagent: 检查语义复制、兼容和风险
```

推荐顺序：

1. 主 agent 写 task brief；
2. Spec subagent 返回约束/缺口；
3. Test subagent先生成或审查验收案例；
4. 主 agent确认没有阻塞 RFC；
5. Implementation subagent 修改；
6. Review subagent只读检查；
7. 主 agent运行完整命令并生成 PR 说明。

避免让多个写代理同时编辑同一公共文件。并行最适合读密集、测试、独立模块和审查任务。

## 7. 每个任务的执行状态机

```text
Ready
  ↓
SpecCheck
  ├── gap → BlockedSpec
  └── clear
        ↓
TestFirst
        ↓
Implementing
        ↓
SelfCheck
        ↓
IndependentReview
  ├── changes → Implementing
  └── pass
        ↓
Integration
        ↓
Done
```

`BlockedSpec` 不是失败。它意味着实现正确地拒绝替语言设计者做决定。

## 8. Codex 停止条件

Codex 遇到以下情况必须停止修改并报告：

- Accepted RFC、SEMANTICS、LANGUAGE 相互冲突；
- 缺少决定可观察行为的规则；
- 需要改变 Semantic ID/Canonical Bytes；
- 需要新增/改义公开错误码；
- 需要改变包、bytecode、replay、ABI、evidence schema；
- 需要选择 Trait coherence、Effect Handler、Actor reentrancy、Ownership 等未决语义；
- 测试表明 interpreter 与规范不一致；
- 只能通过静默 fallback 让功能“工作”；
- 发现安全/数据损坏风险；
- 任务需要修改超出允许路径的核心接口。

输出一个 `spec-gap`：

```markdown
# SPEC-GAP-<id>

## 触发任务
## 冲突/缺失
## 可观察行为
## 受影响模块
## 候选方案（不做决定）
## 需要的 RFC/decision
## 暂停的测试/实现
```

## 9. 自检要求

实现 agent 完成前必须：

1. 阅读 `git diff --stat` 和完整 diff；
2. 运行任务指定命令；
3. 运行相关 crate test/conformance；
4. 检查 `cargo fmt --check`、lint；
5. 检查无未授权 TODO、panic、unwrap；
6. 检查 diagnostics/Schema/Semantic ID 影响；
7. 检查 Unicode/span/position；
8. 更新 traceability/status；
9. 列明未运行的测试和原因；
10. 不声称未验证的性能/正确性。

## 10. Reviewer Prompt

Reviewer 只读回答：

```text
1. 是否改变了语言语义？有无 RFC？
2. 是否存在 parser/VM/LSP/Zed 的重复语义？
3. 是否所有执行路径都消费 Checked Core？
4. UTF-8 byte span 和 LSP UTF-16 转换是否正确？
5. canonical/deterministic 是否受影响？
6. 是否新增未注册 error/schema/CLI 行为？
7. 正例、反例、property、fuzz、differential 是否充分？
8. 是否有 host panic、unsafe、unbounded resource、silent fallback？
9. 是否更新 traceability/status/docs？
10. 给出 P0/P1/P2/P3 findings；没有则明确“无”。
```

Reviewer 不应只评价代码风格。

## 11. 合并顺序

公共接口任务建议：

```text
RFC/decision
→ corpus/test schema
→ domain model/Core type
→ checker/verifier
→ interpreter/reference
→ VM/native/runtime
→ LSP/CLI
→ Zed
→ docs/release
```

外围 agent 的 branch 应定期 rebase 到接口冻结 commit，避免长期漂移。

## 12. Commit 与 PR 规则

Commit 建议：

```text
<task-id>: <imperative summary>
```

例如：

```text
VM-1202: add independent bytecode verifier
LSP-2104: publish compiler diagnostics over LSP
ZQ-3203: add Ling outline queries
```

PR 标题包含 Task ID。PR 描述使用 `templates/PR-DESCRIPTION.md`。

禁止：

- 混合多个无关 Task ID；
- 生成超大“完成整个版本”PR；
- 用更新 snapshot 隐藏不理解的差异；
- 在 PR 中偷偷决定语言语义；
- 声称所有测试通过但未列命令。

## 13. 建议本地命令入口

实际命令以仓库为准，建议统一为：

```bash
cargo xtask check
cargo xtask test-unit
cargo xtask test-conformance
cargo xtask test-differential
cargo xtask test-lsp
cargo xtask test-zed
cargo xtask test-fuzz-smoke
cargo xtask traceability
cargo xtask reproducible-smoke
cargo xtask ci-pr
cargo xtask ci-full
```

Codex 的任务 prompt 应引用这些稳定入口，而不是复制大量脆弱命令。

## 14. 首轮 Codex 并行编组

在当前路线图下建议：

```text
Worktree A / GOV-0101
    建规范缺口台账和状态 Schema

Worktree B / ARCH-0101
    CompilerSession/VFS/LineIndex 架构调研与接口草案

Worktree C / TS-3101
    Tree-sitter grammar 映射和 corpus，不修改 compiler

Worktree D / TEST-VM
    为 VM 最小纵向切片写 failing conformance/differential

Worktree E / LSP-FIXTURE
    JSON-RPC、UTF-16、多语言位置 fixtures

Main / Integrator
    冻结命名、workspace、public registries，审查所有分支
```

第一个集成点完成后，再启动 project graph、VM、incremental 和 Zed extension 实现。

## 15. 自动化任务编排（后置）

当手工 Codex 工作流稳定后，才考虑：

- `codex exec` 非交互任务；
- Codex MCP server + orchestration；
- 定期 traceability/schema drift 审计；
- 自动生成 issue/PR 草案；
- nightly fuzz triage；
- 自动 dependency/license review。

不要在任务本身尚不可靠时先把错误流程自动化。

## 16. Codex 任务完成输出格式

每次要求 Codex 最终返回：

```markdown
## Result

## Files changed

## Spec/RFC coverage

## Tests executed

## Diagnostics/Schema/Semantic ID impact

## Unicode/Determinism impact

## Security/Performance impact

## Spec gaps or conflicts

## Deferred/non-goals

## Suggested next ready tasks
```

“Suggested next”只推荐台账中依赖已满足的任务，不自行扩大范围。

## 17. 工作流完成门禁

- [ ] 根与关键子目录 AGENTS.md 已建立且不过度冗长；
- [ ] 每个任务有 Task ID、规范、边界和验收命令；
- [ ] 公共语义/Schema/registry 有单一 owner；
- [ ] worktree 分离，写冲突任务不并行；
- [ ] spec/test/implementation/review 至少角色分离；
- [ ] BlockedSpec 流程可用；
- [ ] PR 自动检查 traceability/status；
- [ ] Codex 输出不以“编译通过”代替完成证据；
- [ ] 自动化只建立在已手工验证的可靠流程上。
