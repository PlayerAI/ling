# Ling 1.0 总体执行计划

> 地位：对 `ROADMAP-1.0.md` 的工程细化，不新增语言语义。  
> 目标：把 G0～G6 拆成可独立验收、可并行但不会产生方言的纵向工作包。

## 1. 成功定义

Ling 1.0 的完成不是“功能很多”，而是稳定支持面可以由第三方重复验证：

```text
规范条款
  ↕
conformance corpus
  ↕
Checked Typed Core / Semantic Graph
  ↕
Interpreter / VM / Native / Device
  ↕
CLI / LSP / Zed / packages
  ↕
兼容性与发布证据
```

所有发布块都遵循：先关闭规范门禁，再实现最小纵向切片，最后扩大覆盖。

## 2. 工作流编号

| 前缀 | 工作流 |
| --- | --- |
| `GOV` | RFC、decision、支持矩阵、协议生命周期 |
| `SRC` | Source、Unicode、CST/AST、Formatter |
| `CORE` | HIR、Resolve、Type、Effect、Checked Core |
| `SEM` | Semantic Graph、Canonical Bytes、Semantic ID、Audit |
| `PRJ` | project/module/package/lock/build graph |
| `VM` | bytecode、verifier、VM、replay hooks |
| `IDE` | 增量数据库、分析 API、LSP |
| `ZED` | Tree-sitter 与 Zed extension |
| `CON` | Task、Actor、Supervisor、Replay、Remote |
| `NAT` | Ownership、Managed、Native、FFI |
| `HET` | Kernel、SIMD、GPU/TPU、Placement |
| `CRT` | Critical、Node、Contract、Model checking、Evidence |
| `REL` | 兼容、标准库、供应链、发行 |
| `QA` | conformance、fuzz、差分、性能、安全 |

## 3. 发布块依赖

```text
G0 Governance
    │
    ├──────────────┐
    ▼              ▼
G1 Living       ZED-0 grammar-only
    │              │
    ├─────┬────────┘
    ▼     ▼
G2 Concurrent   ZED-1 LSP complete
    │
    ▼
G3 Native       ZED-2 DAP preview
    │
    ▼
G4 Heterogeneous
    │
    ▼
G5 Critical
    │
    ▼
G6 Stabilization
    │
    ▼
Ling 1.0
```

Zed 的 grammar-only 工作可以和 G0/G1 前期并行；类型错误和重构必须等待共享 IDE 分析层；调试器必须等待稳定 source map/runtime protocol。

## 4. 批次定义

### B00：Seed 证据闭合

目标：确认 `v0.0.1 Seed` 的规范、代码、测试和产物是可复现基线。

任务：

- `GOV-0001`：建立权威文档索引和冲突检查脚本；
- `QA-0001`：运行 Seed conformance、Unicode、fuzz smoke；
- `SEM-0001`：生成当前 Diagnostic/Semantic Graph golden corpus；
- `REL-0001`：冻结 `seed-baseline` tag 和工具链 manifest；
- `GOV-0002`：记录 Ling/零、`.ling`、`zero`、LSP languageId 的集中式命名 decision。

门禁：基线测试全绿；所有公开 JSON 都能被当前 reader 读取；未闭合缺口进入台账。

### B01：G0 治理与协议基础

- 规范缺口台账；
- RFC/decision 模板；
- Diagnostic、Semantic Graph、Audit、CLI 的版本位置；
- 1.0 支持矩阵草案；
- traceability schema；
- CI 检查规范链接、错误码重复和 Schema fixtures。

门禁：任何新增稳定行为都有 RFC/decision 入口；公开协议均声明 Experimental/Preview/Stable。

### B02：共享编译服务骨架

- 建立 `CompilerSession`、`ProgramSnapshot`、VFS、LineIndex；
- 把 CLI/REPL/未来 LSP 对 checker 的调用收敛到同一 facade；
- 定义 `CheckedModule` 与 `CheckedProgram` 不变量；
- 使 Diagnostic 只引用稳定 file ID + byte span；
- 建立 clean build 与 snapshot hash fixture。

门禁：CLI 和测试不再直接拼装 parser/type checker；用户输入失败不 panic。

### B03：Tree-sitter 与 Zed grammar-only

- 创建 `tree-sitter-ling`；
- 从已接受语法构建宽度优先 grammar；
- 共享 parser corpus；
- 提供 highlights/brackets/indents/outline/textobjects；
- 创建 `zed-ling` dev extension，只注册 grammar；
- 在 Zed 中验证 `.ling`、中文标识符和错误恢复。

门禁：grammar 不声称决定语言合法性；query tests 和 compiler corpus 交叉通过。

### B04：Project / Module Graph

- manifest、source roots、entry、local dependency；
- deterministic module graph；
- visibility、cycle、duplicate、version conflict diagnostics；
- lock identity；
- offline/locked fixtures。

门禁：相同输入的 graph/lock byte-identical；无中心 registry 依赖。

### B05：Bytecode / Verifier / VM 最小纵向切片

第一条路径：

```text
Hello World
→ Checked Typed Core
→ bytecode v0
→ verifier
→ VM
```

再按函数、record、ADT、match、mutable place、Effect/Capability、Fault 逐类扩展。

门禁：解释器/VM differential 全绿；非法 bytecode 不 panic。

### B06：增量查询

- query key 与 revision；
- Source→Parse→Resolve→Type→Semantic 分层缓存；
- invalidation tests；
- clean/incremental byte-identical；
- 缓存损坏回退；
- 并行调度随机化。

门禁：增量只影响速度，不影响输出、诊断顺序或 Semantic ID。

### B07：LSP 最小可用

- `zero lsp --stdio`；
- initialize/shutdown；
- didOpen/didChange/didClose；
- diagnostics；
- document symbols、hover、definition；
- UTF-8 byte span ↔ LSP position；
- 无编辑器协议 fixtures。

门禁：中文/emoji/CRLF/增量编辑位置正确；文档版本过期时拒绝 stale result。

### B08：Zed + LSP 集成

- Zed extension Rust/Wasm；
- 查找 PATH 中 `zero`；
- 启动 `zero lsp --stdio`；
- 错误下划线、Problems、hover、definition；
- `zero check/run/test/fmt` tasks；
- dev extension 安装与日志诊断文档。

门禁：Windows/Linux/macOS 至少在计划支持平台进行 smoke；扩展不打包 LSP 二进制。

### B09：完整编辑闭环

- Formatter；
- references、prepareRename/rename；
- completion；
- code actions；
- semantic tokens；
- runnables、snippets；
- workspace symbols；
- LSP cancellation 和压力测试。

门禁：rename 基于 resolved symbol/Semantic ID，不基于文本替换；Formatter 幂等并保持 Checked Core 等价。

### B10：Trait 与 v0.1 收口

- Trait RFC、coherence/orphan；
- constraint solving；
- Typed Core 显式 instance；
- VM lowering；
- LSP hover/completion；
- v0.1 追踪矩阵、支持矩阵和发布候选。

门禁：G1 出口标准全部满足后才能进入 G2。

## 5. G2～G6 的主批次

| 批次 | 核心交付 | 详细文档 |
| --- | --- | --- |
| B20 | Effect Handler + Structured Task | `06-G2-V0.2-CONCURRENT.md` |
| B21 | Actor + bounded mailbox + Supervisor | 同上 |
| B22 | Replay + Remote Actor | 同上 |
| B30 | Value/Managed/Resource + Ownership/Region | `07-G3-V0.3-NATIVE.md` |
| B31 | Native IR/backend + Typed FFI | 同上 |
| B40 | Kernel verifier + CPU reference/SIMD | `08-G4-V0.4-HETEROGENEOUS.md` |
| B41 | GPU backend + Placement/cache | 同上 |
| B50 | Critical checker + Node | `09-G5-V0.5-CRITICAL.md` |
| B51 | Contract + bounded model checking + Evidence | 同上 |
| B60 | 协议与支持面冻结 | `10-G6-V1.0-STABILIZATION.md` |
| B61 | RC、独立验证与 v1.0 | 同上 |

## 6. 并行策略

### 可直接并行

- RFC 初稿与 conformance corpus；
- Tree-sitter query 文件之间；
- 解释器/VM differential harness 与 VM opcode 实现；
- LSP protocol fixtures 与 IDE service；
- 文档、示例、fuzz harness、性能基线；
- 不同平台 smoke scripts。

### 需要单一集成者串行

- AST/HIR/Checked Core 公共类型；
- Semantic Graph Schema；
- Diagnostic code registry；
- Cargo workspace 与 dependency graph；
- bytecode format；
- Incremental query key；
- Ownership/Effect/Trait 核心求解器。

### 禁止并行发明

- 两个 agent 分别决定同一个语法或求值规则；
- Zed grammar 与 compiler parser 各自扩展关键字；
- VM 和 interpreter 各自定义 Fault；
- LSP 自行推导与 compiler 不同的类型或 symbol identity。

## 7. 任务规模

使用相对规模而非虚假日历承诺：

| 规模 | 典型含义 |
| --- | --- |
| `XS` | 单一 fixture、文档、query 小改动 |
| `S` | 单 crate、小接口、完整测试，通常一个 PR |
| `M` | 多 crate 纵向切片，需要设计审查 |
| `L` | 公开协议或核心求解器，应拆成多个有序 PR |
| `XL` | 发布块，不得作为单一 Codex 任务 |

任何 `L/XL` 项必须先拆解，Codex 不应一次性“完成整个 VM/Actor/Native”。

## 8. 每批次统一退出清单

- [ ] 规范门禁已关闭，或明确处于隔离 Experimental；
- [ ] 正例、反例和错误码 fixture 完整；
- [ ] Checked Core/Semantic Graph 路径一致；
- [ ] canonical/deterministic 检查通过；
- [ ] 无未分类 panic；
- [ ] 公开 Schema/CLI 影响有迁移说明；
- [ ] 中英文与 Unicode 位置测试通过；
- [ ] 文档、traceability、status 更新；
- [ ] 非目标和后续任务被记录，没有偷跑。
