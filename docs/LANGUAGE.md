# 零语言设计总纲（LANGUAGE.md）

> 状态：设计草案 0.1  
> 日期：2026-08-17  
> 中文名：**零**  
> 英文名：**Ling**  
> 命令：**`ling`**  
> 发起目标：为人类、AI、编译器与运行时共同设计一门面向未来的系统语言

> [!NOTE]
> 2026 年 5 月，Vercel Labs 已公开发布名为 **Zerolang** 的实验语言，并使用 `zero` 作为命令；它同样强调语义图、AI 语义修改和人类文本投影。  
> 为避免用户、搜索、生态与潜在商标混淆，本项目英文名定为 **Ling**，命令为 `ling`，文件扩展名为 `.ling`；包管理命名空间与域名由后续 RFC 确定。

---

## 1. 语言使命

“零”是一门以函数式表达为默认方式、面向系统编程与异构计算、为 AI Agent 原生设计的通用编程语言。

它希望同时提供：

- 接近 F#/OCaml 的简洁、可读与高信息密度；
- 接近 Rust 的资源安全、可预测性能与零成本抽象；
- GC/VM 驱动的快速原型开发；
- Native/No-GC 驱动的高性能程序；
- Task、Actor、实时 Node、数据并行 Kernel 等原生计算模型；
- CPU、GPU、TPU、NPU 与集群的统一表达和分层编译；
- Type、Effect、Ownership、Capability、Contract 组成的静态语义基础；
- 面向人类审计、AI 修改、编译器证明和运行时追责的统一工程系统。

语言的核心口号：

> **始于零，生于意，成于计算。**

工程口号：

> **人类可读，AI 可操作，编译器可验证，运行时可追责。**

英文表达：

> **Human-readable. AI-operable. Compiler-verifiable. Runtime-accountable.**

---

## 2. 不可妥协的语言宪章

### 2.1 人类表达层必须是一等公民

即使未来绝大多数实现代码由 AI 生成，人类仍必须能够：

- 阅读程序；
- 看见副作用；
- 看见资源和权限边界；
- 审查关键算法；
- 检查 AI 修改前后的语义变化；
- 追踪需求、合同、源码、证明、测试与二进制；
- 在事故后重建“程序究竟被要求做什么”。

F#/OCaml 风格源码不是装饰层，而是正式的作者与审计界面。

### 2.2 简洁可以依靠推导，行为不得依靠隐藏

语言允许编译器推导：

- 类型；
- Effect；
- Ownership；
- Borrow；
- 生命周期；
- Trait 约束；
- 并行安全；
- 数据布局；
- 设备 Placement。

但所有推导结果必须能够通过 `audit` 或 `explain` 展开。

> **可以省略，不可隐瞒。**

### 2.3 AI 不属于可信计算基

AI 可以提出：

- 实现；
- 重构；
- 合同；
- 证明；
- 测试；
- 性能优化；
- 状态迁移。

但只有以下系统可以接受或拒绝修改：

- 类型检查器；
- Effect/Capability 检查器；
- Ownership 检查器；
- Contract/Proof 检查器；
- 模型检查器；
- 测试与基准系统；
- 人类审批流程。

### 2.4 一种语言，多种严格程度

“零”不是“Debug 时一门 GC 语言，Release 时突然变成 Rust”。

它采用：

- 一套统一语义；
- 多种内存区域；
- 多个构建 Profile；
- 不同强度的编译与证明。

核心模型：

```text
Value       -> 内联、寄存器、栈、复制或移动
Managed     -> 受管堆与 GC
Resource    -> 所有权、Borrow、Region 与确定性释放
```

构建模型：

```text
Explore     -> VM/JIT/GC/快速增量开发
Native      -> AOT/优化/可选 Managed Island
Critical    -> AOT/有界内存/无一般 GC/可验证子集
```

---

## 3. 程序的四种正式视图

同一个程序具有四种互相可验证的视图。

### 3.1 Author Source

面向人类作者的简洁源码：

```fsharp
type 人物 =
    { 姓名: Text
      mutable 血量: i32 }

let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }
```

### 3.2 Audit Source

规范化的人类审计源码，展开：

- 完整类型；
- Effect；
- Capability；
- Ownership；
- 数值语义；
- Fault；
- 合同；
- 调度与资源约束。

### 3.3 Semantic Graph

面向 AI 和编译器的结构化程序：

- 定义节点；
- 类型节点；
- 调用边；
- Effect 边；
- Capability 边；
- 所有权与 Borrow 边；
- Actor 消息边；
- Contract 与 Requirement 边；
- Placement 与 Cost 信息；
- 稳定 Semantic ID。

### 3.4 Execution View

面向运行时的执行图：

- CPU/GPU/TPU 节点；
- Task 层级；
- Actor 监督树；
- 实时 Node 调度；
- Kernel 数据流；
- 内存区域；
- 网络位置；
- 运行时健康与资源预算。

四种视图必须满足机械一致性，禁止通过自然语言或 AI 猜测进行转换。

---

## 4. 表层语言风格

“零”采用 **ML/F# 家族的表达式优先风格**，并引入少量经过约束的系统编程扩展。

### 4.1 默认不可变

```fsharp
let 最大生命 = 100
```

不可重新赋值。

```fsharp
let mutable 当前生命 = 100
当前生命 <- 80
```

可变绑定必须显式写出 `mutable`。

### 4.2 `=` 绑定与初始化，`<-` 赋值

`=` 只用于创建值；修改已有 Place 使用 `<-`：

```fsharp
let 血量 = 100
人物.血量 <- 100
```

- `let 名称 = 表达式`：定义绑定；
- record literal 内 `字段 = 表达式`：字段初始化；
- `place <- 表达式`：赋值。

两者词法不同，不存在上下文歧义。

赋值返回 `Unit`，禁止链式赋值。

相等比较使用：

```fsharp
人物.血量 == 100
人物.状态 != 死亡
```

### 4.3 函数定义与调用

```fsharp
let 加 a b =
    a + b

let 结果 =
    加 10 20
```

默认采用空格函数应用，不要求大量括号。

### 4.4 Seed 参数值语义

v0.0.1 Seed 的函数参数按 Value 传递，参数 binding 不可变。需要修改 record 的函数返回新值：

```fsharp
let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }
```

Audit Source 中的类型为：

```text
受到伤害 : Int -> 人物 -> 人物
```

调用方显式写回当前函数内的 mutable local：

```fsharp
let mutable 关羽 = 初始人物
关羽 <- 受到伤害 30 关羽
```

Seed 不实现 Resource、Borrow、`&mut`、隐式引用传递或 Borrow Edge。对参数字段赋值必须拒绝；完整 Borrow 模型保留给后续 RFC。

### 4.5 Pipeline

```fsharp
人物
|> 受到伤害 30
|> 更新状态
|> 保存人物
```

### 4.6 Record

```fsharp
type 位置 =
    { x: f32
      y: f32
      z: f32 }

type 人物 =
    { 编号: 人物编号
      姓名: Text
      mutable 血量: i32
      位置: 位置 }
```

record 默认是名义类型，避免结构相同但业务意义不同的类型被意外混用。

### 4.7 Algebraic Data Type

```fsharp
type 生存状态 =
    | 健康
    | 受伤 of 伤势
    | 濒死 of 剩余时间: Duration
    | 死亡 of 死因
```

### 4.8 模式匹配

```fsharp
let 状态文字 状态 =
    match 状态 with
    | 健康 ->
        "健康"

    | 受伤 伤势 ->
        伤势.名称

    | 濒死 时间 ->
        Text.format "濒死：{}" 时间

    | 死亡 原因 ->
        原因.说明
```

模式匹配必须穷尽。默认不允许静默遗漏 variant。

### 4.9 条件表达式

```fsharp
let 战斗状态 人物 =
    if 人物.血量 <= 0 then
        死亡 失血
    else
        健康
```

`if` 是表达式；各分支必须具有兼容类型。

### 4.10 类型标注

```fsharp
let 距离 (a: 位置) (b: 位置) : f32 =
    ...
```

类型通常推导，公共 API、FFI、Critical 边界和含歧义代码要求显式标注。

### 4.11 泛型

```fsharp
let identity (value: 'a) : 'a =
    value
```

### 4.12 模块

```fsharp
module 战斗.伤害

let 计算 攻击 防御 =
    ...
```

模块是命名与 Capability 边界，不是类。

---

## 5. 中文与多语言编程

### 5.1 中文标识符是正式语言能力

以下代码必须被视为普通、完整、可发布的源码：

```fsharp
type 人物 =
    { 姓名: Text
      mutable 血量: i32
      最大血量: i32 }

let 恢复生命 数值 人物 =
    { 人物 with
        血量 = min 人物.最大血量 (人物.血量 + 数值) }
```

中文标识符不是：

- 转义字符串；
- 宏；
- 编译前替换；
- IDE 显示别名；
- 仅限教学的语法糖。

它们直接进入名称解析、类型检查、Semantic Graph、诊断与调试信息。

### 5.2 为什么支持中文

中文在许多专业领域具有高概念密度，例如：

```text
血量
伤势
人口
粮食
赋税
徭役
军心
郡县
户籍
流民
水文
径流
承载力
```

相较于不准确或冗长的音译/英译，本地语言标识符可能更短、更清楚，也更有利于领域专家直接审查模型。

该设计同时适用于日语、韩语、阿拉伯语等其他书写系统，而不是只为中文建立特例。

### 5.3 源码编码

- 源码必须为 UTF-8；
- 编译器内部按 Unicode Scalar Value 处理；
- 换行在语义哈希前规范化；
- BOM 仅允许出现在文件起始位置，编译器应给出提示；
- 字符串与注释可以包含完整 Unicode。

### 5.4 标识符语法

基础规则采用 Unicode UAX #31 的 `XID_Start` / `XID_Continue`：

```text
IdentifierStart    := XID_Start | "_"
IdentifierContinue := XID_Continue
Identifier         := IdentifierStart IdentifierContinue*
```

因此以下均为合法标识符：

```text
人物
血量
update人物
GPU温度
玩家_编号
```

### 5.5 归一化与比较

- 标识符在名称解析前采用 Unicode NFC 归一化；
- 标识符大小写敏感；
- 两个标识符的 NFC 形式相同，则视为同一名称；
- 不使用 NFKC 作为名称身份归一化，以避免兼容分解改变书写意图；
- Semantic Graph 保存规范化形式，同时保留原始拼写用于审计与诊断。

### 5.6 禁止字符

标识符内禁止：

- ZWJ 与 ZWNJ；
- 双向格式控制字符；
- variation selector；
- 默认不可见格式字符；
- 私用区字符；
- Unicode noncharacter；
- 未分配码位；
- 仅用于版式控制的字符。

这些字符仍可在字符串中出现，但编辑器和审计工具必须能显示其存在。

### 5.7 混合文字与视觉混淆

编译器必须实施基于 Unicode UTS #39/#55 的检查：

- 同一作用域内存在视觉可混淆名称时，默认报错；
- Latin 与 Cyrillic/Greek 的可疑混写默认报错；
- 双向文字显示必须使用安全渲染；
- 隐藏字符必须在诊断和代码审查中显式展示；
- Semantic ID 不以视觉字符串为唯一身份；
- Critical Profile 中启用最严格的标识符安全策略。

允许的常见组合包括：

```text
Hani + ASCII Latin + ASCII digit + "_"
```

例如：

```text
GPU温度
AI决策器
人物ID
```

但包清单应声明主要文字系统，工具可以对未声明的混合文字给出警告。

### 5.8 多语言别名

名称不是定义身份。一个 Semantic Definition 可以拥有多个本地化别名：

```fsharp
alias Character = 人物
alias health = 人物.血量
```

别名是名称元数据，不复制实现，也不改变定义的 Semantic ID。

未来 IDE 可以按团队语言偏好显示：

```text
人物.血量
Character.health
```

它们指向同一个语义节点。

v0.0.1 只实现单一主名称；别名进入 Semantic Graph Schema，但语法和工具在后续 RFC 中启用。

### 5.9 关键字策略

语言核心关键字暂时保持一套小型 ASCII 关键字：

```text
let type match with if then else module import
trait impl resource managed task actor node kernel
requires ensures invariant
```

原因：

- 保持全球文档与工具链一致；
- 避免多套语法分裂；
- 避免关键字翻译产生歧义；
- 让中文主要用于高价值领域名词。

未来可研究“本地化 Author View”，但规范化 Audit Source 只使用一套核心关键字。

---

## 6. 类型系统

### 6.1 基本原则

- 静态类型；
- HM 风格局部与模块内类型推导；
- 默认不可变；
- 支持 let-polymorphism；
- Effect、Capability 与 Ownership 进入函数类型；
- 公共 ABI 与 Critical 边界要求显式类型；
- 不存在 `null`。

### 6.2 基本类型

初始基本类型：

```text
Unit
Bool
Int
Nat
i8 i16 i32 i64
u8 u16 u32 u64
f32 f64
Char
Text
Bytes
```

`Int` 为语义上的任意精度整数，适合 Explore 与普通业务逻辑。

固定宽度整数用于 Native、设备接口、存储协议和性能关键代码。

### 6.3 Option 与 Result

```fsharp
type Option<'a> =
    | Some of 'a
    | None

type Result<'a, 'e> =
    | Ok of 'a
    | Error of 'e
```

不存在无类型异常作为普通控制流。

### 6.4 Trait，而不是继承

```fsharp
trait 可绘制 'a =
    draw: 'a -> RenderContext -> Unit

impl 可绘制 人物 =
    let draw 人物 context =
        ...
```

Trait 只描述能力和约束，不形成对象继承树。

语言不提供：

- class inheritance；
- protected inheritance；
- virtual method hierarchy；
- fragile base class 模型。

---

## 7. 内存与资源模型

### 7.1 Value

普通 record、tuple、variant 和小型数据默认属于 Value 世界。

编译器可以决定：

- 内联；
- 寄存器；
- 栈；
- copy；
- move；
- SIMD；
- 标量替换。

### 7.2 Managed

显式声明允许 GC 管理：

```fsharp
managed type 编辑器节点 =
    { mutable 父节点: Option<编辑器节点>
      mutable 子节点: Vector<编辑器节点> }
```

Native Profile 可以包含 Managed Island。

Critical Profile 默认禁止一般 Managed 对象。

### 7.3 Resource

文件、Socket、GPU Buffer、锁、设备句柄等是 affine resource：

```fsharp
resource 文件 =
    { handle: FileHandle }
```

Resource：

- 不能隐式复制；
- 可以移动；
- 可以 Borrow；
- 离开作用域时确定性释放；
- 跨 Task、Actor、设备和远程边界时必须满足对应约束。

### 7.4 Borrow

Author Source 尽可能推导 Borrow。

需要显式表达时使用：

```fsharp
let 写入 (buffer: &mut Buffer) (data: &[u8]) =
    ...
```

生命周期默认推导；只有复杂公共 API 才允许显式 lifetime 参数。

### 7.5 Unsafe 的位置

普通语言不提供任意 `unsafe { ... }`。

硬件寄存器、汇编、启动代码和外部 ABI 位于独立的 Target Primitive Package：

```fsharp
primitive 读取惯导
    provided_by Target.飞行器X.惯导驱动

    requires DeviceRegister<惯导0>
```

> **Unsafe 是项目边界，不是随手打开的代码块。**

---

## 8. Effect 与 Capability

### 8.1 Effect 描述行为

```text
Pure
Console.Write
File.Read
Network
Clock
Random
State<World>
Actor.Send<City>
GPU
Allocate<Managed>
```

函数类型在 Audit Source 中完整显示：

```text
人物 -> Unit ! { Console.Write }
```

Author Source 中可推导：

```fsharp
let 打印人物 人物 =
    Console.write 人物.姓名
```

### 8.2 Capability 授权行为

Effect 说明“程序会做什么”。

Capability 说明“程序被允许做什么”。

```fsharp
module 战斗日志
    requires Console.Write
```

没有 `Console.Write` Capability 的模块在类型层面无法调用对应 API。

### 8.3 时间、随机数和网络不再隐式

```fsharp
Clock.now ()
Random.next rng
Remote.send actor 消息
```

分别产生明确 Effect。

这为：

- 测试；
- Deterministic Replay；
- 安全分析；
- AI 重构；
- 分布式故障推理

提供可见边界。

### 8.4 Effect Handler

后续版本允许处理和消除 Effect：

```fsharp
handle Random with 固定随机种子 42 in
    模拟世界 初始状态
```

---

## 9. Contract 与验证

公共函数、Actor、实时 Node 与安全关键模块可以声明：

```fsharp
let 设置血量 新值 人物
    requires 0 <= 新值
    requires 新值 <= 人物.最大血量

    ensures result ->
        result.血量 == 新值
=
    { 人物 with 血量 = 新值 }
```

合同状态必须显式区分：

```text
Proved
RuntimeChecked
Tested
Assumed
Unverified
```

不得把“测试通过”显示成“已证明”。

Critical Profile 可以要求：

- 所有安全合同均为 `Proved`；
- 所有 `Assumed` 均进入人工审计清单；
- 无未处理 Fault；
- 终止性、栈上界、内存上界和截止时间具有证据。

---

## 10. 五种原生计算模型

### 10.1 `let` / `fn`：纯函数和值变换

```fsharp
let 计算距离 a b =
    ...
```

适合算法、规则和转换。

### 10.2 `task`：结构化异步

```fsharp
task 载入人物 编号 =
    scope
        async let 基础资料 = 数据库.读取人物 编号
        async let 装备 = 数据库.读取装备 编号

        let! 人物 = 基础资料
        let! 装备 = 装备

        return 组装人物 人物 装备
```

子任务必须在父 Scope 退出前：

- 完成；
- 被取消；
- 或显式移交所有权。

### 10.3 `actor`：长期身份与隔离状态

```fsharp
actor 城市 =
    state
        mutable 人口: i64
        mutable 粮食: i64

    mailbox
        capacity 1024
        backpressure Wait

    receive
        | 收获 数量 ->
            粮食 <- 粮食 + 数量

        | 查询人口 reply ->
            reply 人口
```

Actor 默认：

- 私有状态；
- 类型化消息；
- 有界邮箱；
- 明确 Backpressure；
- 监督策略；
- 故障封闭；
- 可选持久化与状态迁移。

### 10.4 `node`：同步实时节点

```fsharp
node 姿态控制
    every 10.ms
    deadline 2.ms
    no_gc
=
    ...
```

用于：

- 飞控；
- 工业控制；
- 音频；
- 硬实时循环；
- 可静态调度的数据流。

### 10.5 `kernel`：数据并行

```fsharp
kernel 更新位置 dt 位置 速度 =
    parallel for i in 0 .. 位置.length - 1 do
        位置[i] <- 位置[i] + 速度[i] * dt
```

编译器可将 Kernel Lower 到：

- CPU SIMD；
- GPU；
- TPU/NPU；
- FPGA/专用加速器；
- 集群分片。

程序优先描述计算性质，而不是写死某个厂商 API。

---

## 11. Actor、ECS 与数据并行的边界

不鼓励“每个小对象一个 Actor”。

推荐：

```text
长期、有身份、需要故障隔离的实体 -> Actor
大量同构、连续、逐帧更新的数据 -> ECS / Kernel
有限生命周期的等待工作         -> Task
硬实时周期控制                 -> Node
普通业务与算法                 -> 函数
```

例如大型世界：

```text
WorldSupervisor actor
├── Region actor × 1000
│   └── ECS / Kernel
│       ├── 士兵
│       ├── 动物
│       └── 粒子
├── Economy actor
├── Diplomacy actor
└── AI actor
```

---

## 12. 本地、远程与异构设备

### 12.1 不承诺虚假的 Location Transparency

本地 Actor 与远程 Actor 的语法可以接近，但类型必须不同：

```text
ActorRef<'message>
RemoteActorRef<'message, 'failure>
```

远程操作必须面对：

- timeout；
- partition；
- duplicate；
- reorder；
- retry；
- partial failure。

### 12.2 数据位置进入类型或审计信息

```text
Buffer<CPU, f32>
Buffer<GPU, f32>
Tensor<TPU, f16>
```

数据移动不可假装免费：

```fsharp
let gpu数据 =
    数据 |> transfer gpu
```

`explain` 必须展示：

- 传输字节数；
- 设备驻留；
- Kernel 时间；
- 同步点；
- 内存峰值。

### 12.3 Placement 是约束

```fsharp
placement prefers GPU
placement requires TensorCore
placement same_node_as 世界状态
```

不是所有程序都要求程序员手工指定设备。

---

## 13. Build Profile

### 13.1 Explore

目标：快速迭代。

允许：

- 解释器或字节码 VM；
- GC；
- REPL；
- 增量编译；
- 热重载；
- 运行时 Contract；
- 低优化泛型与闭包。

### 13.2 Native

目标：高性能通用程序。

启用：

- AOT；
- Ownership/Region Lowering；
- Monomorphization；
- Escape Analysis；
- Iterator Fusion；
- SIMD；
- CPU/GPU；
- 可选 Managed Island；
- LTO。

### 13.3 Critical

目标：可分析、可审计和安全关键系统。

默认禁止：

- JIT；
- 动态代码；
- 一般 GC；
- 无界分配；
- 无界递归；
- 无界邮箱；
- 隐式时间；
- 隐式随机数；
- 动态反射；
- 未限定网络调用；
- 无证明的算术 Fault；
- 任意 FFI。

Critical Profile 只覆盖冻结的语言子集、标准库、编译器、目标与构建参数。

---

## 14. Semantic Graph

### 14.1 程序身份

原始文件字节不是程序的最终身份。

编译器将 Author Source 解析为规范化 Semantic Graph。

定义身份至少分为：

```text
BodyId       = hash(规范化实现与已解析依赖)
ContractId   = hash(合同、Effect、Capability 与外部保证)
DefinitionId = hash(BodyId, ContractId, 类型与语言版本)
```

名称和格式是元数据，不参与 `BodyId`。

### 14.2 语义图节点

初始节点种类：

```text
Module
Type
Field
Variant
Value
Function
Parameter
Pattern
Expression
Effect
Capability
Contract
Alias
Requirement
Profile
```

未来扩展：

```text
Task
Actor
Message
Supervisor
Node
Kernel
Device
Placement
Migration
Proof
RuntimeEvidence
```

### 14.3 Source 与 Graph 的关系

v0.0.1 采用可实现的过渡方案：

1. `.ling` 文本是主要编辑界面；
2. 编译器生成规范化 Semantic Snapshot；
3. Snapshot 产生稳定 Graph Hash；
4. CI 保存并验证源码与 Snapshot 一致；
5. AI 可以通过语义查询和事务修改 Graph；
6. 修改后重新生成或最小化更新 Author Source；
7. 审计构建同时保存 Source、Graph 与 Binary Manifest。

长期目标是“语义身份优先”，但不牺牲普通工程师阅读文本源码的能力。

---

## 15. AI 原生工具链

### 15.1 结构化诊断

每个诊断同时提供：

- 人类可读消息；
- 稳定错误码；
- JSON Schema；
- 相关 Semantic ID；
- 根因；
- 可验证修复候选；
- 修改前置条件。

示意：

```json
{
  "code": "L-TYPE-0042",
  "severity": "error",
  "message_zh": "不能修改不可变字段“最大血量”",
  "semantic_id": "def:7bd18...",
  "facts": {
    "field": "最大血量",
    "mutability": "immutable"
  },
  "repairs": [
    {
      "kind": "return_record_update",
      "preserves": ["public_type", "effects"]
    }
  ]
}
```

### 15.2 语义事务

AI 修改不是“替换第 173 行”，而是：

```text
Target:
    semantic_id = def:7bd18

Preconditions:
    graph_hash = ...
    field 血量 is mutable

Change:
    replace expression ...

Preserve:
    public contract
    determinism
    effects
```

事务只有通过检查后才可提交。

### 15.3 语义 Diff

默认 Diff 优先展示：

- 行为变化；
- 类型变化；
- Effect 变化；
- Capability 扩张；
- Contract 变化；
- 内存与延迟变化；
- 公开 API 变化；
- 未证明假设。

文本 Diff 始终可展开，但不再是唯一审查方式。

### 15.4 Provenance

AI 生成和修改应记录：

- Agent/工具身份；
- 输入 Graph Hash；
- 任务目标；
- 保留约束；
- 新增假设；
- 验证结果；
- 人工审批。

---

## 16. 明确删除的旧机制

“零”不引入以下机制：

- class inheritance；
- ambient global mutable state；
- untyped exception；
- token/text macro；
- implicit I/O；
- implicit network；
- arbitrary reflection；
- header/source split；
- unrestricted shell build script；
- file path as definition identity；
- name as dependency identity；
- unrestricted raw pointer；
- arbitrary dynamic loading；
- ordinary-code `unsafe` block；
- unbounded actor mailbox；
- implicit numeric conversion；
- `null`；
- undefined integer overflow；
- non-exhaustive match by default；
- invisible time/randomness；
- “能编译即正确”的工程标准。

对应替代：

```text
组合、ADT、Trait、Actor
State Effect / Ownership
Result / typed Effect
Typed AST Transformation
Capability
Remote type
Typed Schema
Single semantic definition
Hermetic Build Graph
Semantic ID
Content-addressed dependency
Borrow / Span / Capability pointer
Signed semantic component
Target Primitive Package
Bounded mailbox + backpressure
Explicit conversion
Option
Defined arithmetic mode
Exhaustive match
Clock / Random Effect
Evidence ladder
```

---

## 17. 包、构建与互操作

### 17.1 Hermetic Build Graph

构建步骤必须是类型化节点：

```text
CompilerStep
CodeGenerator
SchemaCompiler
Linker
AssetProcessor
```

不得允许依赖包在构建期间任意执行 Shell。

### 17.2 依赖

依赖至少锁定：

- 包身份；
- 语义哈希；
- Contract 版本；
- Capability 要求；
- 构建 Profile；
- 目标平台约束。

### 17.3 Typed FFI

外部接口必须通过：

- 明确 ABI；
- 明确内存所有权；
- 明确线程与重入规则；
- 明确 Error/Fault；
- 明确 Capability；
- 独立 Target Primitive 包

建立边界。

---

## 18. 命令行草案

```bash
ling init
ling repl
ling run main.ling
ling check main.ling
ling test
ling fmt
ling semantic main.ling
ling audit main.ling
ling explain L-TYPE-0042
ling query --symbol 人物
ling patch transaction.json
ling build --profile explore
ling build --profile native
ling build --profile critical
```

所有命令应支持：

```bash
--format human
--format json
```

结构化输出不是附加功能，而是稳定工具协议。

---

## 19. v0.0.1 Seed 范围

第一版必须实现：

- UTF-8 源码；
- 中文 Unicode 标识符；
- NFC 名称归一化；
- Confusable/隐藏字符基础诊断；
- `let`；
- 函数；
- 空格函数应用；
- `if`；
- record；
- ADT；
- exhaustive `match`；
- `Unit/Bool/Int/f64/Text`；
- `Option/Result`；
- 局部类型推导；
- `mutable` 与 `place <- value`；
- `Pure` 与 `Console.Write` Effect；
- `Console.Write` Capability；
- Semantic Graph；
- 稳定 Diagnostic JSON；
- Interpreter；
- REPL；
- `run/check/semantic/audit`。

第一版明确不实现：

- GC Runtime；
- Native Backend；
- Ownership/Borrow Checker；
- Trait；
- Effect Handler；
- Task/Actor/Node/Kernel；
- 分布式；
- GPU；
- 形式证明；
- 包管理器。

这些语义在设计中预留，但不得为了“看起来完整”而仓促实现。

---

## 20. 路线图

```text
v0.0.1 Seed
    语法、类型、中文标识符、Semantic Graph、解释器

v0.1 Living
    字节码 VM、模块、增量编译、LSP、Formatter、基础 Trait

v0.2 Concurrent
    Structured Task、Actor、Bounded Mailbox、Supervisor、Replay

v0.3 Native
    Value/Resource、Ownership/Region、Cranelift/LLVM

v0.4 Heterogeneous
    Kernel、CPU SIMD、GPU/TPU Lowering、Placement

v0.5 Critical
    Node、Critical Profile、Contract、模型检查、证据包

v1.0
    稳定语言核心、稳定 Semantic Graph Schema、兼容性承诺
```

---

## 21. 设计判定标准

任何新特性都必须回答：

1. 它能否保持 F#/OCaml 级的人类可读性？
2. 它是否把副作用、权限、资源和失败变得更清楚？
3. 它能否进入 Semantic Graph，而不是只存在于文本技巧中？
4. AI 能否通过结构化接口可靠操作？
5. 编译器能否检查或证明关键性质？
6. Runtime 能否监控、重放和解释实际行为？
7. 它是否在 Explore、Native、Critical Profile 中具有明确语义？
8. 它是否引入了可以由更一般机制替代的历史包袱？
9. 它是否能被中文和其他自然语言用户清楚表达？
10. 它是否值得永久加入语言？

未能回答这些问题的特性不得仅凭“其他语言都有”而加入。

---

## 22. 规范性参考

1. Unicode Standard Annex #31, **Unicode Identifiers and Syntax**  
   https://www.unicode.org/reports/tr31/

2. Unicode Technical Standard #39, **Unicode Security Mechanisms**  
   https://www.unicode.org/reports/tr39/

3. Unicode Technical Standard #55, **Unicode Source Code Handling**  
   https://www.unicode.org/reports/tr55/

4. Rust Reference, **Identifiers**  
   https://doc.rust-lang.org/reference/identifiers.html

5. PEP 3131, **Supporting Non-ASCII Identifiers**  
   https://peps.python.org/pep-3131/

6. Koka, **Effect Types and Handlers**  
   https://koka-lang.github.io/

7. Erlang/OTP, **Design Principles and Supervision Trees**  
   https://www.erlang.org/doc/system/design_principles.html

8. P Language, **Communicating State Machines and Verification**  
   https://p-org.github.io/P/

9. Swift, **Structured Concurrency and Actors**  
   https://docs.swift.org/swift-book/documentation/the-swift-programming-language/concurrency/

10. Unison, **Content-addressed Definitions**  
    https://www.unison-lang.org/docs/the-big-idea/

11. MLIR, **Multi-Level IR for Heterogeneous Hardware**  
    https://mlir.llvm.org/

12. SPARK, **Ravenscar Profile**  
    https://docs.adacore.com/spark2014-docs/html/ug/en/source/concurrency.html

13. WebAssembly Component Model, **WIT Interfaces and Worlds**  
    https://component-model.bytecodealliance.org/design/wit.html

14. Vercel Labs, **Zerolang** — naming and product-space collision reference  
    https://github.com/vercel-labs/zerolang
