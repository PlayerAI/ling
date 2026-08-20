# Ling 1.0 后续开发路线图 / Roadmap to Ling 1.0

> 状态：规划基线（非规范性）  
> 日期：2026-08-20  
> 起点：已发布的 `v0.0.1 Seed`  
> 目标：`v1.0` 稳定语言核心、稳定 Semantic Graph Schema 与明确兼容性承诺  
> 规范依据：[RFC-0001](RFC-0001.md)、[SEMANTICS](SEMANTICS.md)、[LANGUAGE](LANGUAGE.md) 及已接受的 [decisions](decisions/)  
> 地位：本文只定义工程顺序、交付物和验收门禁，不新增语言语义。发生冲突时，按仓库规定的规范权威顺序处理。

---

## 1. 目标与“完整”的定义

Ling 1.0 的“完整”不是把所有可能的语言功能都纳入核心，也不是把 [LANGUAGE](LANGUAGE.md) 中的每个远景一次实现。它表示：

1. **承诺的语言核心完整**：被列入 1.0 支持矩阵的语法、类型、Effect、Capability、内存和计算模型都有规范、实现与一致性测试；
2. **工具链闭环完整**：检查、运行、构建、测试、格式化、语言服务、语义查询、审计和包解析形成可重复使用的工程流程；
3. **执行路径完整**：各 Build Profile 的可用能力和限制被明确声明，不以占位 API 暗示未实现能力；
4. **协议完整**：诊断、Semantic Graph、Audit Source、Semantic Transaction、构建元数据等公开格式具有版本策略和兼容性测试；
5. **发布能力完整**：支持平台、离线构建、依赖锁定、安全响应、弃用规则和升级路径均有公开承诺；
6. **证据完整**：每项稳定能力都可从规范条款追踪到实现、测试和发布证据。

1.0 不以“功能数量”作为完成标准，而以稳定支持面能否被独立验证作为完成标准。没有 Accepted RFC 的语义不得由实现先行固定；未进入支持矩阵的实验能力不得标记为 Stable。

## 2. 总体原则

### 2.1 规范先于不可逆实现

任何改变以下行为的工作都必须先有 Accepted RFC 或 Accepted decision：

- 源码语法、名称解析、类型规则与求值行为；
- Effect、Capability、Trait、Ownership、并发和失败模型；
- Semantic ID、Canonical Bytes、Semantic Graph Schema 与公开 JSON；
- 包身份、依赖解析、ABI、FFI、跨节点协议与设备语义；
- Profile 限制、确定性等级、兼容性和弃用规则。

原型可以验证设计，但必须与稳定路径隔离，默认关闭，且不得产生看似正式的公开 API。

### 2.2 纵向切片优先

每个能力按下列顺序形成最小闭环：

```text
规范条款
  → 正例/反例 conformance
  → Source/CST/AST
  → Resolved HIR
  → Checked Typed Core
  → Semantic Graph
  → 执行或 Lowering
  → CLI/工具协议
  → 文档与兼容性证据
```

解释器、VM、Native、Kernel 或工具不得各自解释未检查 AST。所有执行与生成路径必须消费经过检查的 Typed Core 或其版本化、可验证的派生表示。

### 2.3 稳定核心与实验扩展分离

每个公开能力必须处于以下状态之一：

| 状态 | 允许行为 | 兼容性承诺 |
| --- | --- | --- |
| `Experimental` | 受显式开关控制；允许根据 RFC 演进 | 不承诺向后兼容，但必须可识别版本 |
| `Preview` | 语义已接受，正在积累跨平台证据 | 变更必须提供迁移说明 |
| `Stable` | 进入 1.0 支持矩阵并通过全部门禁 | 遵守 1.x 兼容性政策 |

状态必须出现在用户可见文档和机器可读元数据中，不得只存在于代码注释。

### 2.4 固定的不变量

所有阶段持续满足：

- 保留原始 UTF-8 byte span，诊断位置不得由 Rust 字符索引或规范化文本替代；
- Unicode XID、归一化、安全检查和生成表继续固定为 Unicode 17.0.0，升级只能通过独立规范变更完成；
- 公共诊断保持中英双语、稳定错误码和版本化结构；
- Canonical 输出不得泄露 HashMap 顺序、主机路径、Rust debug 文本、分配地址或线程调度细节；
- 依赖锁定后，普通构建和测试可离线执行；
- 未实现能力必须明确报错，不得静默降级为不同语义。

## 3. 发布块与依赖关系

发布顺序保持 [LANGUAGE §20](LANGUAGE.md#20-路线图) 的既定结构：

```text
v0.0.1 Seed（已发布）
    │
    ▼
v0.1 Living
    │
    ▼
v0.2 Concurrent
    │
    ▼
v0.3 Native
    │
    ▼
v0.4 Heterogeneous
    │
    ▼
v0.5 Critical
    │
    ▼
1.0 Stabilization
    │
    ▼
v1.0
```

设计研究、测试框架和无语义承诺的原型可以并行，但发布门禁按上述顺序关闭。后续块不得迫使前序稳定语义发生无迁移路径的破坏性变化。

| 块 | 核心结果 | 主要前置依赖 |
| --- | --- | --- |
| G0 治理与兼容基础 | RFC 队列、支持矩阵、协议生命周期 | `v0.0.1` 证据闭合 |
| G1 `v0.1 Living` | VM、工程模块、增量工具链、基础 Trait | G0 |
| G2 `v0.2 Concurrent` | Structured Task、Actor、Replay | G1、Effect/Capability 扩展决议 |
| G3 `v0.3 Native` | 资源语义、Ownership/Region、Native/FFI | G1；与 G2 保持语义一致 |
| G4 `v0.4 Heterogeneous` | Kernel、SIMD、设备 Lowering、Placement | G3 的内存与 ABI 边界 |
| G5 `v0.5 Critical` | Node、Critical、Contract、模型检查、证据包 | G2 的 Replay、G3 的资源模型、G4 的受限 Lowering |
| G6 1.0 稳定化 | 冻结支持面、兼容套件、发布候选 | G1～G5 全部出口 |

## 4. G0：治理、兼容性与工程基础

### 4.1 目标

在扩大语言面之前建立可持续的决策和兼容机制，防止后续版本通过代码或快照意外创造规范。

### 4.2 分步工作

#### G0.1 建立规范缺口台账

把 [SEMANTICS §31](SEMANTICS.md#31-待-rfc-决定的问题) 的未决项和实现发现的冲突集中记录。每项至少包含：

- 状态：`Open / Proposed / Accepted / Rejected / Superseded`；
- 被阻断的能力与版本块；
- 权威文档、候选方案和不可逆后果；
- 所需正例、反例、迁移和兼容性测试。

优先关闭会阻断 `v0.1` 的包命名空间、Trait coherence、Semantic Hash 升级和协议版本策略。并发、Ownership、Critical 等议题保持在各自版本块前关闭，不提前绑定实现。

#### G0.2 定义公开兼容面

为下列接口分别规定 Stable/Experimental 字段、版本号位置、reader/writer 兼容规则和弃用流程：

- CLI 命令、退出码和 `--format json`；
- Diagnostic JSON 与错误码注册表；
- Semantic Graph、Canonical Bytes、Semantic ID；
- Audit Source 与 Semantic Transaction；
- 包清单、锁文件、构建图和产物元数据；
- 后续的 bytecode、replay log、ABI 与 evidence bundle。

内部 Rust API 不自动成为 Ling 兼容面；只有被规范和测试声明的接口才受 1.x 承诺保护。

#### G0.3 建立统一追踪矩阵

沿用 [SEED-TRACEABILITY](SEED-TRACEABILITY.md) 的形式，为后续每个版本建立：

```text
规范条款 → Semantic ID/Schema 影响 → 实现路径 → 测试证据 → 发布产物
```

矩阵由 CI 检查链接和测试标识是否存在，人工审查语义覆盖是否充分。

#### G0.4 明确 1.0 支持矩阵草案

支持矩阵至少声明：

- 稳定语言能力与仍属 Preview 的能力；
- Explore、Native、Critical 各自允许的 Effect、内存和运行模型；
- Tier 1/Tier 2 主机平台、Native target、GPU/TPU backend；
- 标准库包、FFI ABI、包协议和工具协议的稳定级别；
- 不支持或明确推迟的能力。

### 4.3 出口标准

- 所有已知未决语义都有责任边界和阻断关系；
- 公开 Schema/协议都有生命周期规则；
- 每个后续版本块都有可复用的追踪矩阵模板；
- 1.0 支持矩阵草案经过 RFC 评审，但仍可在预发布阶段收缩。

## 5. G1：`v0.1 Living`

### 5.1 目标

让 Ling 从单次解释执行扩展为可承载多模块工程、快速反馈和编辑器集成的“活语言”工具链，同时保持与 Seed 解释器一致的可观察语义。

### 5.2 规范门禁

实现前必须接受或补齐：

- 包身份、命名空间、模块可见性、依赖解析和锁定规则；
- bytecode 的版本、验证规则、求值顺序、Fault 和兼容边界；
- Trait declaration/implementation、约束求解、coherence 与 orphan rule；
- 增量缓存键、Semantic Hash 升级和失效边界；
- Formatter 对 Author Source 的保留/规范化边界；
- LSP 与 Semantic Transaction 暴露的稳定/实验字段。

### 5.3 分步工作

#### G1.1 工程与模块图

1. 定义最小项目清单、源目录、入口和依赖身份；
2. 在现有模块解析之上构建确定性 module/package graph；
3. 实现重复包、循环依赖、不可见符号和版本不一致诊断；
4. 生成锁定结果，确保相同输入产生 byte-identical 元数据；
5. 增加多包正例、反例和离线构建测试。

包管理的首个目标是可重复解析和本地/锁定依赖，不在此阶段建设不必要的中心化服务。

#### G1.2 版本化 bytecode 与 VM

1. 从 Checked Typed Core 定义最小 bytecode，不从 AST 直接生成；
2. 实现独立 verifier，拒绝版本错误、非法跳转、类型/栈不一致和越界引用；
3. 建立解释器与 VM 的 differential harness；
4. 逐类覆盖函数、ADT、match、mutable place、Effect、Capability 与 Fault；
5. 对 bytecode 输入做 fuzz，并保证失败只产生稳定诊断，不触发 host panic；
6. 只有覆盖全部 `v0.0.1` conformance 后，VM 才能成为默认 Explore 执行路径候选。

#### G1.3 基础 Trait

1. 先实现声明、约束收集和唯一候选解析；
2. 实现 coherence/orphan 规则及稳定歧义诊断；
3. 将已解析实例显式写入 Typed Core 和 Semantic Graph；
4. 选择并规范 dictionary passing 或其他 Lowering，不让后端自行猜测；
5. 增加跨包冲突、递归约束、不可满足约束和确定性选择测试；
6. specialization、动态分派和高级类型级编程继续推迟，除非独立 RFC 纳入。

#### G1.4 增量编译

1. 为 Source、解析、解析后符号、类型、Effect 和 Semantic 结果定义查询边界；
2. 缓存键只使用规范输入、工具链版本和目标配置；
3. 建立 edit-to-invalidation 测试，验证私有函数体、公开签名和依赖变化的不同失效范围；
4. 随机化任务调度并比较最终产物，证明并发执行不改变结果；
5. 缓存损坏必须安全回退到重算，不能改变语言行为。

#### G1.5 Formatter 与 Audit 协同

1. 明确 Author Source formatter 与 canonical Audit Source 的不同职责；
2. 实现 `fmt(fmt(source)) == fmt(source)`；
3. 验证格式化前后解析、解析引用和 Checked Core 等价；
4. 保留必要注释与用户语言表达，不把本地化文本强制改写为单一审计形式；
5. 对不完整源码提供可预测行为，不在 formatter 中实现第二套 parser。

#### G1.6 LSP 与语义工具

1. 复用同一增量查询和诊断模型，先交付 diagnostics、document symbols、hover、definition；
2. 再交付 references、rename、completion 和 code action；
3. rename/code action 必须通过版本化 Workspace Edit 或 Semantic Transaction，并检查 stale edit；
4. 所有位置协议显式转换 UTF-8 byte span 与 LSP position，不污染编译器内部单位；
5. 通过无编辑器依赖的协议 fixtures 验证多语言标识符、emoji 前缀、CRLF 和增量编辑。

#### G1.7 开发命令闭环

闭合并统一 `init/check/run/test/fmt/semantic/audit/query/patch/build` 的参数、退出码、人类输出与 JSON 输出；CLI 仅编排共享服务，不复制 checker、VM 或 formatter 逻辑。

### 5.4 出口标准

- 多模块/多包样例可在锁定依赖下离线 check、build、test；
- `v0.0.1` 全部程序在解释器与 VM 上具有相同规范可观察结果；
- Formatter 幂等且保持语义，LSP 位置和多语言 rename 通过 conformance；
- 增量结果与 clean build byte-identical，缓存损坏可安全恢复；
- 基础 Trait 的静态语义、Lowering 和跨包 coherence 有 Accepted RFC 与测试；
- 所有新增公开诊断仍为双语并使用注册错误码。

## 6. G2：`v0.2 Concurrent`

### 6.1 目标

交付结构化并发、隔离状态和可审计重放；时间、随机数、网络、调度与取消不得成为隐藏副作用。

### 6.2 规范门禁

- Effect Row、Effect polymorphism 与 Handler 的精确规则；
- Structured Task 的父子生命周期、取消、detach 和 suspension semantics；
- Actor turn 内 `await` 的重入规则；
- bounded mailbox、backpressure、message ordering 和 Supervisor 策略；
- Remote Actor 的身份、传输、交付策略和 Capability 边界；
- Determinism Class、Effect Log、Replay 版本和隐私边界。

### 6.3 分步工作

#### G2.1 Effect Handler 基础

先扩展 checker、Typed Core 和 Semantic Graph，再实现 handler 执行；验证 Effect 消除、Capability 不可伪造、handler 嵌套和未处理 Effect 诊断。

#### G2.2 Structured Task

1. 实现 lexical task scope 与结构化 join；
2. 定义取消传播、资源清理和 Fault 聚合；
3. 把 suspension point 写入可检查 IR；
4. 通过确定性测试调度器覆盖竞态、取消和超时；
5. `detach` 保持受限能力，默认路径不得泄露后台任务。

#### G2.3 Actor 与 bounded mailbox

1. 实现 actor identity、隔离 state 和串行 turn；
2. mailbox 容量必须显式，send 必须表现为可处理的背压结果或 Effect；
3. message 类型、顺序和不可共享状态由 checker 验证；
4. 压力测试覆盖满队列、慢消费者、actor 终止和资源上限。

#### G2.4 Supervisor

实现 restart/stop/escalate 等被 RFC 接受的策略；重启预算和 Fault provenance 必须可查询，禁止无限高速重启。

#### G2.5 Deterministic Replay

1. 对外部 Effect 建立版本化事件日志；
2. 记录足以重放的输入，不把线程调度细节错误提升为语言语义；
3. 在相同程序身份、配置和日志下验证结果一致；
4. 对程序、Schema 或 Capability 不匹配给出明确拒绝；
5. 提供日志裁剪、敏感字段策略和损坏检测。

#### G2.6 Remote Actor

先以 transport-neutral 协议定义身份和错误，再接入最小参考传输。远程调用必须显式暴露延迟、断连、重试、重复或丢失可能性，不承诺虚假的 location transparency。

### 6.4 出口标准

- Task 不泄露结构化作用域，取消与清理有性质测试；
- mailbox 始终有界，背压和 Supervisor 行为可观测、可诊断；
- Replay 在支持范围内可跨独立进程复现，并能拒绝身份/版本不匹配；
- 网络、时间和随机数均通过显式 Effect/Capability；
- 并发 fuzz、压力测试和模型化交错测试不产生未分类 host panic 或死锁。

## 7. G3：`v0.3 Native`

### 7.1 目标

完成 Value/Managed/Resource 的可执行内存模型、Ownership/Region 检查和 Native 构建，确保高层语义不依赖 Rust 的所有权或布局偶然性。

### 7.2 规范门禁

- Copy/Move、Borrow、Region、Drop、aliasing 和 public lifetime 规则；
- Managed runtime、GC root、finalization 与 Profile 边界；
- Native ABI、数据布局、unwinding/Fault、线程和重入规则；
- Typed FFI、Target Primitive Package 与 unsafe/TCB 边界；
- Native target 和 backend 的支持分级。

### 7.3 分步工作

#### G3.1 内存身份与资源检查

1. 在类型和 Typed Core 中区分 Value、Managed、Resource；
2. 实现 move/use-after-move、borrow exclusivity、region escape 与 drop-order 检查；
3. 将资源生命周期和隐式 drop 显式 Lower 到 Checked Core；
4. 诊断提供 source span、资源来源、冲突使用和可执行修复建议；
5. 使用负向 conformance 和 property tests 覆盖分支、循环、match、闭包和并发边界。

#### G3.2 Managed runtime

只实现被 Explore/Native 支持矩阵需要的最小 Managed 能力。GC 算法是实现选择，但 root、可观察 finalization、OOM/Fault 和 FFI pinning 必须有规范边界；Critical Profile 不得因此引入通用 GC。

#### G3.3 Native Lowering

1. 定义 Typed Core 到 backend-neutral Native IR 的可验证映射；
2. 先交付一个基线 backend，闭合调用、控制流、ADT、资源和 Effect ABI；
3. 再按支持矩阵接入 Cranelift/LLVM 中尚缺的 backend，不重复实现前端语义；
4. 优化前后运行同一 differential suite；
5. 生成可复现产物或记录所有导致差异的非确定输入。

#### G3.4 Typed FFI

1. FFI 声明显式包含 ABI、ownership、threading、reentrancy、Error/Fault 与 Capability；
2. 不安全代码隔离在可审计 Target Primitive Package；
3. 自动生成边界检查和必要 shim；
4. 对错误签名、生命周期、布局和目标不匹配给出编译期诊断；
5. 建立 C ABI 最小互操作套件，其他 ABI 仅在支持矩阵承诺后纳入。

#### G3.5 多执行引擎一致性

建立 Interpreter ↔ VM ↔ Native 三方 differential 测试；对浮点、Fault、I/O、资源释放和并发可观察点明确允许差异，禁止用快照掩盖未解释差异。

### 7.4 出口标准

- Ownership/Region checker 覆盖规范的全部稳定规则；
- Resource 在正常返回、Error、Fault、取消路径上都有确定清理证据；
- 至少一个 Native backend 覆盖稳定语言核心，其他 backend 状态如实标记；
- FFI 边界可审计，unsafe/TCB 清单完整；
- Interpreter、VM、Native 在支持范围内通过 differential conformance；
- sanitizer、fuzz 和跨目标测试未发现未分类内存安全问题。

## 8. G4：`v0.4 Heterogeneous`

### 8.1 目标

为数据并行 Kernel 建立受限、可验证的语义子集，从 CPU 参考实现逐步扩展到 SIMD 和被支持的 GPU/TPU backend，并让 Placement 成为显式约束。

### 8.2 规范门禁

- Kernel 允许的类型、控制流、Effect、递归、分配和数值模式；
- Device Buffer 的 ownership、address space、同步与传输语义；
- 浮点精度、reduction 顺序和 determinism class；
- Placement 约束、fallback、设备缺失和成本信息；
- backend feature/capability discovery 与缓存身份。

### 8.3 分步工作

#### G4.1 Kernel 子集检查器

在通用 Typed Core 之后增加独立验证 pass，拒绝不受支持的 Effect、动态分配、递归、别名和不可界定控制流；不得通过 backend 编译失败代替语言诊断。

#### G4.2 CPU 参考执行

先实现标量 CPU reference backend，作为所有设备 Lowering 的可比较语义基线；覆盖索引、边界、buffer 读写、map/reduce 和 Fault。

#### G4.3 CPU SIMD

实现合法向量化与标量 fallback，验证对齐、尾部处理、溢出和浮点模式；优化不得改变声明的 determinism class。

#### G4.4 GPU Lowering

1. 定义 backend-neutral device IR 或明确复用的中间层；
2. 先接入一个受支持 GPU backend；
3. 实现 buffer transfer、launch、synchronization 和 device Fault 映射；
4. 与 CPU reference 做精确或容差受规范约束的 differential tests；
5. 设备/驱动不满足要求时明确拒绝或执行被规范允许的 fallback。

#### G4.5 TPU/加速器扩展

通过窄接口复用 Kernel 验证、shape/layout 和 Placement，不在核心编译器复制设备语义。只有进入支持矩阵的 backend 才是 1.0 发布门禁；其余保持 Experimental 插件。

#### G4.6 Placement 与缓存

Placement 决策必须可解释、可记录、可重放；设备二进制缓存键包含 Program ID、backend、target、driver/toolchain、数值模式和 Profile，禁止错误复用。

### 8.4 出口标准

- Kernel verifier 在 Lowering 前拒绝全部越界语言能力；
- CPU reference、SIMD 与稳定设备 backend 通过差分测试；
- Device Buffer 生命周期和传输 Effect 可在 Semantic Graph/Audit 中追踪；
- unsupported target 不静默改变数值或 Effect 语义；
- 支持矩阵明确列出已验证硬件/软件组合和 Experimental backend。

## 9. G5：`v0.5 Critical`

### 9.1 目标

提供可受限、可分析、可重复构建的 Critical Profile：同步 Node、Contract、有限模型检查和机器可验证的证据包形成闭环。

### 9.2 规范门禁

- Critical 最小可验证 Core 和禁止能力清单；
- Node tick、state、deadline、overrun 和 Fault 语义；
- Contract 的前置/后置/不变量、证明状态和运行时检查边界；
- bounded allocation、递归/循环界、并发和 FFI 限制；
- 模型检查状态空间、假设、结论和“不证明什么”；
- evidence bundle Schema、身份、来源、升级与验证规则。

### 9.3 分步工作

#### G5.1 Critical Profile checker

实现独立、可组合的 Profile 验证 pass，至少检查：

- 无通用 GC 和未界定分配；
- 无隐式 I/O、时间、随机数、网络或设备访问；
- 递归、循环、队列、任务和资源有静态或配置上界；
- FFI 仅来自经审计的 Target Primitive Package；
- 所有 fallback 和 Fault 路径显式。

#### G5.2 Node

1. 实现同步 tick、显式输入/输出和持久 state；
2. 定义 deadline 与 overrun 的检查/运行行为；
3. 禁止 Node 内出现未受控 suspension 或动态 topology；
4. 建立虚拟时钟测试和 worst-case path fixture；
5. 将周期、预算和失败证据写入 Semantic Graph。

#### G5.3 Contract

1. 在 parser、checker、Typed Core 和 Graph 中统一表示 Contract；
2. 先实现运行时检查，再接入静态 discharge；
3. 区分 `Proved / RuntimeChecked / Assumed / Unknown`；
4. 优化器只能使用满足规范可信条件的已证明事实；
5. 失败报告包含原始条款、实例化值和 provenance。

#### G5.4 有界模型检查

对 Task/Actor/Node 的有限状态投影提供边界明确的探索。工具输出必须包含搜索界、未覆盖状态和假设，禁止把“界内未发现反例”表述为完全证明。

#### G5.5 Evidence bundle

聚合并版本化：

- Program/Semantic ID 与规范版本；
- 源、依赖、锁文件和 hermetic build identity；
- conformance、property、fuzz、模型检查和 Contract 结果；
- target、Profile、编译器和 TCB 清单；
- 未证明项、豁免、已知限制和 provenance。

bundle 必须可由独立工具验证，签名和远程证明仅在相应 RFC 接受后成为 Stable 要求。

### 9.4 出口标准

- 不符合 Critical 限制的程序在构建前被稳定诊断拒绝；
- Node 周期、deadline、state 和 Fault 有规范与可重复测试；
- Contract 状态不会把 Assumed/Unknown 伪装为 Proved；
- 模型检查报告完整披露边界与假设；
- evidence bundle 可离线、独立验证并复现其引用的构建与测试身份。

## 10. G6：1.0 稳定化与发布

本块不再默认加入新的语言特性。若发现支持面过大，应收缩为 Preview/Experimental，而不是牺牲规范、测试或兼容性质量。

### 10.1 分步工作

#### G6.1 冻结稳定支持面

1. 逐项审查 G0 支持矩阵；
2. 每个 Stable 能力必须存在 Accepted RFC、完整规范和可执行 conformance；
3. 删除或隐藏会暗示未实现语义的占位入口；
4. 为 Preview/Experimental 能力建立显式开关、版本和退出策略；
5. 发布 1.0 feature/profile/target matrix。

#### G6.2 冻结公开协议

完成 Diagnostic、Semantic Graph、Audit Source、Semantic Transaction、包/锁文件、构建元数据、Replay 和 Evidence 的 reader/writer 兼容测试。对每种 Schema 提供：

- 当前版本和稳定字段；
- unknown-field 行为；
- N-1 读取或迁移策略；
- canonical encoding 与 hash 升级规则；
- golden corpus 和损坏输入测试。

#### G6.3 标准库与包生态

1. 冻结最小稳定标准库，避免把尚未成熟的便利 API 永久化；
2. 为每个公开符号记录 Effect、Capability、Fault、复杂度和 Profile 可用性；
3. 完成包发布、校验、锁定、离线缓存和供应链 provenance；
4. 包服务不可用时，锁定项目仍可从已缓存依赖重复构建；
5. 建立恶意 manifest、依赖混淆和构建脚本隔离测试。

#### G6.4 兼容性与迁移套件

1. 建立 1.0 compiler 对历史 Seed～v0.5 corpus 的兼容矩阵；
2. 验证 1.0.x 对 1.0 源码、包、Schema 和产物的承诺；
3. 提供弃用周期、自动迁移和无法自动迁移的诊断；
4. 将 Semantic ID/Hash 变更作为显式协议升级，不静默重算身份；
5. 对不兼容输入给出可操作、双语、稳定错误。

#### G6.5 可靠性、安全与性能基线

- Parser、Unicode、bytecode、Schema、package、replay、FFI 和 device 输入持续 fuzz；
- 完成依赖许可证、供应链、unsafe/TCB、FFI 和沙箱边界审计；
- 三个 Profile 均建立 cold/warm build、增量延迟、内存、启动和运行基线；
- 性能回归阈值由测量数据确定，不以未验证数字作为发布门禁；
- 长时压力、故障注入和缓存损坏测试进入 CI 或周期性验证。

#### G6.6 文档与发行

1. 完成语言参考、工具参考、Profile 指南、包指南、FFI 指南和迁移指南；
2. 所有稳定能力至少有一个最小示例和一个真实工程示例；
3. 发布多平台可验证产物、checksums、SBOM、许可证和 provenance；
4. 发布候选必须从干净 tag、锁定依赖和记录的工具链构建；
5. 建立安全漏洞、兼容性缺陷和 Schema 漏洞的响应流程。

### 10.2 `v1.0` 最终完成定义

只有下列条件全部满足，才能发布 `v1.0`：

- [ ] 1.0 支持矩阵中的每项能力都有 Accepted RFC/规范条款；
- [ ] 规范条款、实现、conformance 和发布证据可双向追踪；
- [ ] 稳定语言核心在所有 Tier 1 平台通过同一 conformance suite；
- [ ] Interpreter、VM、Native 和设备路径在各自承诺范围内完成差分验证；
- [ ] Explore、Native、Critical 的允许能力、拒绝行为和 fallback 均已测试；
- [ ] Semantic Graph Schema 和其他稳定协议通过版本/兼容/损坏输入测试；
- [ ] 公共诊断中英双语、错误码注册完整且位置保持原始 UTF-8 byte span；
- [ ] Unicode 版本仍明确固定；若升级，已有 Accepted RFC、生成表和迁移证据；
- [ ] 正常构建与测试在依赖锁定后可离线复现；
- [ ] 支持平台完成安全、依赖、unsafe/TCB 与许可证审计；
- [ ] 所有已知 P0/P1 正确性、安全和数据损坏问题关闭；
- [ ] Preview/Experimental 能力不会被默认文档或 API 误报为 Stable；
- [ ] 1.x 兼容、弃用、支持周期和安全响应政策已发布；
- [ ] release candidate 经过独立验证，最终 tag 与证据包身份一致。

## 11. 跨版本工作流

### 11.1 规范与语言设计

维护 RFC、勘误、术语、示例和反例；确保 `SEMANTICS`、`LANGUAGE` 与 Accepted RFC 不分叉。任何规范冲突都会阻断对应实现合并。

### 11.2 Compiler Core

保持单向管线和单一 Checked Typed Core。新增 Trait、Effect、Ownership、Task、Kernel 或 Contract 时，优先扩展明确的领域模型与验证 pass，避免形成一个同时负责解析、类型、优化和执行的巨型组件。

### 11.3 Runtime 与 Backend

Interpreter 保留为参考执行路径；VM、Native 和设备 backend 复用相同语义输入。后端只负责合法 Lowering 和执行，不负责修补前端未检查的程序。

### 11.4 Developer Experience

CLI、LSP、Formatter、REPL 和 AI 工具复用 parser、增量查询、诊断与 Semantic Transaction。人类文本和 JSON 协议共享错误分类，但分别优化可读性与机器稳定性。

### 11.5 Quality Engineering

每个版本块至少包含：

- 语法/静态语义/运行语义的正反 conformance；
- parser、decoder、verifier 和公开 Schema 的 fuzz；
- formatter/audit/schema 的 round-trip 与 canonicality；
- 多执行引擎 differential tests；
- 确定性、并行调度和跨进程测试；
- 跨平台、MSRV、offline/locked build；
- 性能基线、故障注入和安全审计。

## 12. 每个里程碑的统一验收模板

每个里程碑或 Pull Request 必须明确回答：

| 类别 | 必须提供的证据 |
| --- | --- |
| 规范 | 覆盖的规范条款、Accepted RFC/decision 链接 |
| 冲突 | 遇到的规范缺口/冲突；若无则明确写“无” |
| 实现 | 受影响的管线层与依赖方向，说明为何没有复制语义 |
| 测试 | 新增/更新的正例、反例、性质、fuzz、差分或兼容测试 |
| 兼容 | Diagnostic、Schema、Semantic ID、CLI、ABI 或包格式影响 |
| 确定性 | 输出顺序、缓存键、调度、路径和主机信息影响 |
| Unicode | Unicode 17.0.0 数据、XID、归一化和 span 影响 |
| 推迟 | 有意不实现的相邻能力与原因 |

版本出口还必须附带该块的追踪矩阵、支持矩阵差异和可重复执行命令。

## 13. 关键风险与控制

| 风险 | 后果 | 控制措施 |
| --- | --- | --- |
| 规范远景被误当作已接受语义 | 后端和工具产生不兼容方言 | RFC 门禁；冲突先停实现 |
| 同时开发过多 runtime/backend | 核心语义重复、验证面失控 | 每阶段先完成一个纵向 reference path |
| Semantic Graph 过早冻结 | 无法表达后续 Trait/Ownership/并发 | Experimental→Preview→Stable；版本化 reader |
| Trait/Effect/Ownership 相互放大复杂度 | 类型检查不可预测、诊断恶化 | 分块接受 RFC；限制首版表达力；性质测试 |
| 并发和远程失败被隐藏 | 程序行为不可审计 | 显式 Effect/Capability/Fault；bounded mailbox |
| GPU/TPU 支持面过宽 | CI 无法覆盖、数值语义漂移 | 受限 Kernel；CPU reference；分层支持矩阵 |
| Critical 宣称超过证据 | 产生错误安全保证 | 明示边界/假设；独立 evidence 验证 |
| 包生态引入供应链执行 | 破坏 hermetic/offline 构建 | 类型化构建步骤；禁止依赖任意 shell |
| 兼容承诺晚于实现 | 1.0 无法稳定升级 | G0 起维护版本策略和历史 corpus |

## 14. 立即开始的执行批次

以下批次直接推进 `v0.1 Living`，不提前实现 `v0.2+` 语义：

1. **建立 1.0 能力/规范缺口台账**：把 `SEMANTICS §31`、RFC-0001 后续 RFC 清单和当前实现缺口归并，标出版本阻断关系；
2. **提交首组 RFC**：包身份与模块图、bytecode/VM 可观察语义、Trait coherence、Semantic Hash/Schema 生命周期；
3. **先写验收 corpus**：多包解析、解释器/VM 差分、formatter 幂等、LSP UTF-8/LSP position、增量失效；
4. **实现 VM 最小纵向切片**：`Hello World → Typed Core → bytecode → verifier → VM`，暂不加入优化器；
5. **实现最小项目图**：本地依赖、锁定身份、循环/冲突诊断和离线重复构建；
6. **接入增量查询基础**：先覆盖 source/parse/resolve/type，验证 clean 与 incremental 等价后再接 LSP；
7. **形成 `v0.1` 追踪矩阵**：每关闭一个能力同步记录规范、测试、兼容、确定性和有意推迟项。

该批次完成后，再根据证据拆分 `v0.1` 的后续实施 issue。不要在首批次同时建设 JIT、完整包注册中心、高级 Trait、分布式 runtime 或多个 Native backend。

## 15. 1.0 后明确推迟的工作

除非后续 Accepted RFC 将其纳入 1.0 支持矩阵，以下内容默认不阻断 1.0：

- Self-hosting 编译器；
- 高级 Trait specialization、隐式动态分派和任意类型级计算；
- 对全部 GPU/TPU 厂商、驱动和设备代际的通用支持；
- 无边界的分布式一致性抽象或虚假 location transparency；
- 对任意程序的完全形式证明；
- 未经支持矩阵验证的 ABI、平台和 package registry 服务；
- 仅为未来可能需求准备、当前没有规范和验收用例的公开 API。

这些推迟项体现 YAGNI：1.0 优先交付小而稳定、可解释、可审计的核心，而不是永久背负未经验证的表面积。
