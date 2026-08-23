# 零语言核心语义（SEMANTICS.md）

> 状态：设计草案 0.1  
> 日期：2026-08-17  
> 适用范围：语言核心、Semantic Graph、解释器与未来 Native/Critical 后端  
> 英文名：Ling；命令：`ling`

---

## 0. 文档地位

本文定义“零”语言的核心语义模型。

`LANGUAGE.md` 解释语言目标、表层风格和工程哲学；本文回答：

- 源码如何成为程序；
- 名称如何解析；
- 类型、Effect、Capability 与 Ownership 如何组合；
- 表达式按什么顺序求值；
- 赋值、模式匹配、错误和资源具有何种含义；
- Task、Actor、Node、Kernel 如何进入统一语义；
- Semantic Graph 如何建立稳定身份；
- AI 如何进行可检查的语义事务；
- Explore、Native、Critical Profile 如何共享同一套行为定义。

除明确标注“未来 RFC”的部分外，本文是语言实现的语义权威。

---

## 1. 规范词

本文使用：

- **必须（MUST）**：实现不可违反；
- **禁止（MUST NOT）**：实现不得提供相反行为；
- **应该（SHOULD）**：除非有记录充分的理由，否则应遵守；
- **可以（MAY）**：实现可选择；
- **未指定（UNSPECIFIED）**：不同实现可不同，但实现必须记录；
- **未定义（UNDEFINED）**：本语言原则上尽量避免；Critical Profile 中禁止依赖。

“零”的设计目标之一，是将传统语言中的未定义行为压缩到 Target Primitive 边界。

---

## 2. 语义总模型

一个可执行程序不是若干文本文件的简单集合，而是一个经过检查的语义快照：

```text
ProgramSnapshot =
    LanguageVersion
  × UnicodeVersion
  × SemanticGraph
  × AliasMap
  × ContractSet
  × Profile
  × DependencyLock
  × TargetManifest
```

其中：

- `LanguageVersion` 固定语法和核心语义版本；
- `UnicodeVersion` 固定标识符字符属性和安全算法版本；
- `SemanticGraph` 表示程序的已解析、已类型化结构；
- `AliasMap` 保存人类名称及其本地化别名；
- `ContractSet` 保存前置条件、后置条件、不变量和证明状态；
- `Profile` 规定允许的语言子集和运行时特性；
- `DependencyLock` 内容寻址地固定依赖；
- `TargetManifest` 固定目标平台、后端和 Target Primitive。

源码是正式的人类表达与审计界面；Semantic Graph 是程序的规范化语义身份。

---

## 3. 源码解码与词法语义

### 3.1 编码

源码必须使用 UTF-8。

编译流程：

```text
bytes
  -> UTF-8 decode
  -> newline normalization
  -> lexical scan
  -> identifier normalization
  -> token stream
```

非法 UTF-8 必须是编译错误。

BOM：

- 仅允许在文件开头；
- 解码时忽略；
- Formatter 默认移除；
- Audit 模式记录其存在。

### 3.2 换行

以下换行形式在词法处理后等价：

```text
LF
CRLF
CR
```

Semantic Hash 不得因平台换行差异而改变。

### 3.3 Unicode 版本固定

编译器必须在版本信息和 ProgramSnapshot 中记录所使用的 Unicode 版本。

v0.0.1 目标为：

```text
Unicode 17.0.0
```

升级 Unicode 版本属于可能影响词法和安全诊断的语言工具链升级，必须有迁移报告。

### 3.4 标识符

基础语法：

```ebnf
IdentifierStart    = XID_Start | "_";
IdentifierContinue = XID_Continue;
Identifier         = IdentifierStart, { IdentifierContinue };
```

标识符读取后：

1. 验证字符是否属于允许集合；
2. 拒绝禁止的格式和不可见字符；
3. 转换为 NFC；
4. 计算 Script Set；
5. 运行 Confusable 检查；
6. 将规范化名称交给名称解析器。

名称相等：

```text
equal_name(a, b) iff NFC(a) == NFC(b)
```

比较大小写敏感。

### 3.5 禁止字符集合

标识符不得包含：

```text
U+200C ZWNJ
U+200D ZWJ
Bidi_Control
Variation_Selector
Default_Ignorable_Code_Point（语言白名单除外；v0.0.1 白名单为空）
Private_Use
Noncharacter_Code_Point
Unassigned
Deprecated
Pattern_Syntax
Pattern_White_Space
```

字符串和注释允许其中一部分，但 Source Viewer 必须提供显式显示模式。

### 3.6 混合文字策略

名称解析不以“长得像”作为身份判断，但编译器必须进行视觉混淆诊断。

最低要求：

- 同一作用域内，若两个不同标识符的 UTS #39 skeleton 相同，则报错；
- Latin 与 Cyrillic 或 Greek 可疑混用时默认报错；
- RTL 控制字符不得进入标识符；
- 审计输出显示每个非 ASCII 名称的 Script Set；
- 包可声明主要 Script Policy。

默认允许：

```text
ASCII
单一 Unicode Script
Han + ASCII Latin + ASCII Digit + "_"
Japanese writing system combinations
Korean writing system combinations
```

例如：

```text
人物
人物ID
GPU温度
AI规划器
```

### 3.7 关键字

关键字是固定 ASCII token，大小写敏感。

v0.0.1 关键字：

```text
let mutable rec and
type of
match with
if then else
true false
module import as
```

预留关键字：

```text
trait impl
resource managed
effect handle
requires ensures invariant
task actor node kernel
scope async await
placement profile
primitive
```

用户不能直接使用关键字作为标识符。未来可提供显式 raw identifier 机制，但 v0.0.1 不实现。

### 3.8 注释

```text
// 行注释
/* 块注释，可嵌套 */
/// 文档注释
```

注释不进入 `BodyId`，但文档注释可进入独立 `DocumentationId`。

### 3.9 缩进

Author Source 使用 offside rule：

- 同一语法块的子表达式必须比块起始列缩进；
- Tab 禁止用于语义缩进；
- Formatter 输出四个空格；
- 括号、方括号和花括号内部的换行不触发 offside 结束；
- Audit Source 可使用显式规范化块标记，避免视觉歧义。

缩进只影响解析，不进入 Semantic Hash。

---

## 4. 名称、作用域与身份

### 4.1 名称不是定义身份

人类名称是可修改元数据。

```text
Name -> DefinitionId
```

定义可拥有：

- 主名称；
- 中文别名；
- 英文别名；
- 历史别名；
- 反向兼容别名。

重命名只改变 `AliasMap`，在实现、类型、合同和依赖不变时不改变 `DefinitionId`。

### 4.2 作用域

作用域类型：

```text
package
module
type
trait/impl
function
pattern
block
actor
node
kernel
```

名称解析采用最近词法作用域优先。

阴影：

```fsharp
let x = 1

let f x =
    let x = x + 1
    x
```

允许局部阴影，但：

- 公共 API 阴影给出提示；
- 中文/英文 Confusable 阴影默认报错；
- Critical Profile 可禁止非参数阴影。

### 4.3 已解析引用

Semantic Graph 中的引用不得仅保存文本名称，而必须保存：

```text
ResolvedRef =
    DefinitionId
  × ExpectedKind
  × SourceOccurrence
```

因此后续重命名不需要文本搜索。

### 4.4 Semantic ID

推荐结构：

```text
BodyId =
    Hash(
        LanguageCoreVersion,
        NormalizedTypedCore,
        ResolvedDependencyIds
    )

ContractId =
    Hash(
        PublicType,
        EffectRow,
        CapabilityRequirements,
        Contracts,
        FaultPolicy,
        DeterminismPolicy
    )

DefinitionId =
    Hash(
        DefinitionKind,
        BodyId,
        ContractId
    )
```

局部绑定名称在哈希前转换为位置索引或等价的 alpha-normal form。

源文件路径、空白、注释、局部变量拼写和格式不进入 `BodyId`。

### 4.5 递归定义

互递归组以整体计算 `CycleId`，组内成员获得稳定成员索引：

```text
DefinitionId = CycleId.member_index
```

成员顺序由规范化结构确定，而不是源码排列顺序。

---

## 5. Core IR

表层语法 Lower 到小型 Typed Core。

核心表达式：

```text
Literal
Variable
Let
Lambda
Apply
If
RecordConstruct
RecordProject
RecordUpdate
VariantConstruct
Match
Sequence
PlaceAssign
PrimitiveCall
HandleEffect
CreateTask
AwaitTask
SendActor
ReceiveActor
NodeStep
KernelInvoke
```

v0.0.1 仅实现前十二项与 `Console.Write` Primitive。

所有语法糖必须 Lower 到 Core IR 后再参与类型检查后的规范化与 Semantic Hash。

---

## 6. 类型

### 6.1 类型语法

概念类型：

```text
τ ::=
    Unit
  | Bool
  | Int
  | Nat
  | i8 | i16 | i32 | i64
  | u8 | u16 | u32 | u64
  | f32 | f64
  | Char | Text | Bytes
  | 'a
  | T<τ1, ..., τn>
  | τ1 * ... * τn
  | τ1 -> τ2 ! ε
  | &τ
  | &mut τ
  | Managed<τ>
  | Resource<τ>
  | ActorRef<M>
  | RemoteActorRef<M, F>
  | Task<T, E>
  | Buffer<Device, T>
```

### 6.2 函数类型

函数类型同时携带 Effect Row：

```text
A -> B ! {e1, e2, ...}
```

纯函数：

```text
A -> B ! {}
```

Audit Source 可显示为：

```text
A -> B ! Pure
```

### 6.3 Type Scheme

泛型绑定：

```text
∀ 'a 'b. τ
```

Author Source：

```fsharp
let identity value =
    value
```

推导：

```text
identity : ∀'a. 'a -> 'a ! {}
```

### 6.4 Let-polymorphism 与 Value Restriction

只有满足以下条件的绑定才可完全泛化：

- RHS 为句法值；或
- RHS 被证明为 Pure 且不创建可变引用、Managed identity 或 Resource identity。

这是为了避免多态可变状态导致类型不安全。

### 6.5 Nominal Record

record 是名义类型。

两个字段完全相同但声明不同的 record 不自动兼容：

```fsharp
type 世界坐标 = { x: f64; y: f64 }
type 屏幕坐标 = { x: f64; y: f64 }
```

`世界坐标` 与 `屏幕坐标` 不可隐式互换。

### 6.6 Variant

ADT variant 构造器属于其类型作用域。

```fsharp
type Option<'a> =
    | Some of 'a
    | None
```

构造器值：

```text
Some : 'a -> Option<'a>
None : Option<'a>
```

### 6.7 Null

语言没有 `null` 值。

缺失、失败和延迟初始化分别使用：

```text
Option<T>
Result<T, E>
Late<T>（未来 RFC）
```

### 6.8 Trait

Trait 是约束集合，不是继承。

语义要求：

- impl coherence；
- 同一 ProgramSnapshot 中一个具体类型与一个 Trait 的主 impl 唯一；
- 孤儿规则由后续 RFC 确定；
- 动态派发必须显式进入 existential/trait object 边界；
- 默认采用静态特化或 dictionary passing，由 Profile 与优化器决定。

v0.0.1 不实现 Trait。

---

## 7. 静态判定形式

完整静态判定可表示为：

```text
Γ ; Κ ; Ω ⊢ e : τ ! ε ▷ Ω'
```

含义：

- `Γ`：名称与类型环境；
- `Κ`：可用 Capability 环境；
- `Ω`：输入 Ownership/Region 状态；
- `e`：表达式；
- `τ`：结果类型；
- `ε`：Effect Row；
- `Ω'`：求值后的 Ownership 状态。

示例，纯加法：

```text
Γ ⊢ a : Int ! {}
Γ ⊢ b : Int ! {}
────────────────────────
Γ ⊢ a + b : Int ! {}
```

示例，终端输出：

```text
Console.Write ∈ Κ
Γ ⊢ text : Text ! ε
────────────────────────────────
Γ ; Κ ⊢ Console.write text : Unit
      ! (ε ∪ {Console.Write})
```

示例，修改字段：

```text
Γ ⊢ p : &mut Person
field health : i32 mutable
Γ ⊢ value : i32
────────────────────────────────
Γ ; Ω ⊢ p.health <- value : Unit
      ! {State<Person>} ▷ Ω
```

v0.0.1 可以使用简化判定，但 IR 数据结构必须为未来 `Κ` 与 `Ω` 预留稳定字段。

---

## 8. 求值策略

### 8.1 严格求值

语言采用 strict call-by-value。

### 8.2 求值顺序

除显式 Parallel/Task/Actor/Kernel 结构外，表达式按确定的左到右顺序求值。

函数调用：

```fsharp
f a b c
```

顺序：

1. 求值 `f`；
2. 求值 `a`；
3. 求值 `b`；
4. 求值 `c`；
5. 调用。

Record literal 字段按源码顺序求值。

Tuple、List 和参数同样左到右求值。

编译器可重排纯表达式，但必须保持：

- 结果；
- Fault；
- 可观察 Effect；
- 严格浮点语义；
- Timing Contract（若存在）

不变。

### 8.3 短路布尔

```text
a && b
a || b
```

两个操作数与结果均为 `Bool`。`&&` 比 `||` 结合更紧；两者均低于 equality、高于 pipeline，并按左结合建立语法树。

求值严格从左到右，且左操作数只求值一次：

- `false && b` 返回 `false`，不求值 `b`；
- `true && b` 求值并返回 `b`；
- `true || b` 返回 `true`，不求值 `b`；
- `false || b` 求值并返回 `b`。

未求值右操作数中的 Effect 与 Fault 不可观察。静态检查仍检查两个操作数，并保守地将两侧 Effect 与 Capability 要求纳入结果。完整 precedence 与 Typed Core 边界见 Accepted DEC-0017。

### 8.4 Sequence

块内表达式按顺序执行。

除最后一个表达式外，前置表达式必须：

- 类型为 `Unit`；或
- 结果被显式丢弃。

隐式丢弃 Resource 是错误。

---

## 9. Binding、Place 与赋值

### 9.1 Binding

```fsharp
let x = e
```

创建不可变绑定。

```fsharp
let mutable x = e
```

创建可变 Place。

### 9.2 Place

可赋值 Place 包括：

- `mutable` 局部绑定；
- `mutable` record 字段；
- `&mut` Borrow 下的字段或索引；
- 当前 Actor 的可变 state；
- Kernel 中独占的可写 buffer element。

### 9.3 Seed 参数值语义

v0.0.1 Seed 的函数参数只有 `Value` 使用模式，参数 binding 不可变。Seed 不推导 `BorrowShared`、`BorrowMutable` 或 `Move`，也不生成 Borrow Edge。

更新 record 参数时，函数返回新的 record value，调用方显式写回 mutable local：

```fsharp
let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }

let mutable 关羽 = 初始人物
关羽 <- 受到伤害 30 关羽
```

参数字段赋值、隐式 reborrow、`&mut` 参数和调用方可见的参数别名修改在 Seed 中必须拒绝。完整的 Borrow/Move 参数模式保留给后续 RFC。

### 9.4 Assignment

```fsharp
place <- value
```

规则：

1. 左侧必须解析为 Place；
2. Place 必须可变；
3. 右侧先完整求值；
4. 右侧类型必须可赋给 Place 类型；
5. 旧 Resource 值必须按资源规则处理；
6. 写入发生；
7. 表达式结果为 `Unit`。

禁止：

```fsharp
a <- b <- 10
```

### 9.5 Equality

```text
==
!=
```

Equality 由内建类型或 `Eq` Trait 提供。

函数、ActorRef、Resource 等类型默认不具备结构相等性。

---

## 10. Record 与更新

构造：

```fsharp
{ 姓名 = "关羽"
  血量 = 100 }
```

不可变更新：

```fsharp
{ 人物 with 血量 = 80 }
```

含义是创建一个新 record value；旧值不被修改。

直接赋值：

```fsharp
人物.血量 <- 80
```

只有当：

- `人物` 是可变 Place 或可变 Borrow；
- `血量` 字段声明 `mutable`

时合法。

---

## 11. Pattern Matching

### 11.1 顺序

Pattern clause 自上而下尝试，第一个匹配且 Guard 为真的分支被选择。

### 11.2 Exhaustiveness

编译器必须检查：

- variant 是否穷尽；
- Bool 是否穷尽；
- tuple/record pattern 是否可达；
- 整数范围 pattern 是否存在遗漏（能力允许时）。

非穷尽 match 默认是编译错误。

可以使用显式 wildcard：

```fsharp
| _ -> ...
```

但 Critical Profile 可要求对安全相关 ADT 禁止 wildcard，以避免新增 variant 被静默吸收。

### 11.3 Guard

Guard 只在 Pattern 成功后求值。

Guard Effect 必须计入整个 `match` 的 Effect Row。

### 11.4 不可达分支

明显不可达分支默认警告；Audit/Critical 模式可提升为错误。

---

## 12. 错误、Fault 与终止

### 12.1 可恢复错误

业务错误通过：

```text
Result<T, E>
```

表达。

```fsharp
let 读取配置 path : Result<配置, 配置错误> =
    ...
```

### 12.2 Effect-based abort

未来 Effect Handler 可以表达局部可处理的控制转移，但它必须有类型化 Effect，不存在任意无类型 `throw`.

### 12.3 Fault

Fault 表示违反运行前提或机器级失败，例如：

```text
IntegerOverflow
BoundsViolation
DivisionByZero
OutOfMemory
ContractViolation
DeviceFault
RuntimeCorruption
```

Fault 与普通 `Result` 区分。

Audit Source 必须列出函数可能发生的 Fault 或证明其不存在。

Critical Profile 要求：

- 安全路径上的 Fault 被证明不可达；或
- 转换为显式安全状态；
- 禁止依赖进程崩溃作为普通控制流。

### 12.4 Panic

语言核心不提供可恢复的无类型 `panic`。

开发期断言失败映射为 `ContractViolation` Fault。

---

## 13. 数值语义

### 13.1 `Int` 与 `Nat`

`Int` 是数学整数语义，Explore Runtime 可使用 arbitrary precision。

`Nat` 是非负数学整数。

### 13.2 固定宽度整数

默认算术模式为 checked：

```text
i32 + i32 -> i32 or IntegerOverflow Fault
```

提供显式形式：

```text
checked_add
saturating_add
wrapping_add
try_add
```

具体操作符糖由后续 RFC 决定。

### 13.3 隐式转换

禁止隐式有损数值转换。

```fsharp
let x: i64 = i64.from_i32 y
```

编译器可以消除运行时零成本转换。

### 13.4 浮点

`f32` 与 `f64` 遵循固定版本的 IEEE 754 语义模型。

默认：

- 保留 NaN；
- 保留 signed zero；
- 禁止未声明的 reassociation；
- 禁止默认 fast-math；
- FMA contraction 必须由 Profile/函数策略决定并进入 Audit Source；
- 浮点比较保持标准 NaN 行为。

允许显式：

```text
floating strict
floating fast within error <= ε
```

近似计算必须进入 Contract 与 Cost Model。

### 13.5 量纲类型

未来 RFC 支持：

```text
f64<米>
f64<秒>
f64<米/秒>
```

以消除单位错误；v0.0.1 不实现。

---

## 14. 内存语义

### 14.1 三类内存身份

#### Value

- 没有独立引用身份或身份不应被观察；
- 可复制或移动；
- 编译器可以内联；
- 默认不可变。

#### Managed

- 具有受管身份；
- 可形成图和循环；
- 由 GC 或其他受管策略回收；
- 创建产生 `Allocate<Managed>` Effect；
- Critical Profile 默认禁止。

#### Resource

- 具有唯一资源身份；
- 默认 affine；
- 不可隐式复制；
- 移动后原绑定不可用；
- 支持 Borrow；
- 离开作用域确定性释放。

### 14.2 Copy 与 Move

编译器可自动判定 Value 是否 Copy。

Resource 默认 Move。

Author Source 可显式：

```fsharp
move resource
copy value
borrow value
borrow mut value
```

Audit Source 必须展开实际决策。

### 14.3 Borrow

共享 Borrow：

```text
&'r T
```

可变 Borrow：

```text
&'r mut T
```

核心规则：

- 任意数量共享 Borrow，或一个可变 Borrow；
- 两者不得重叠；
- Borrow 不得超过被借对象生命周期；
- 跨 suspension point 的 Borrow 必须满足 Pin/Region 约束，但实现细节不暴露为普通语法噪音；
- Actor state 的可变 Borrow 不得离开 Actor turn；
- Kernel 可写 slice 必须证明不别名。

### 14.4 Region

Region 可以是：

```text
StackRegion
ArenaRegion
ActorRegion
TaskRegion
ManagedRegion
DeviceRegion
RemoteRegion
```

Region inference 决定对象放置。

程序员只在：

- API 边界；
- 性能约束；
- Critical Profile；
- FFI/设备边界

需要显式说明。

### 14.5 Drop

Resource 离开所有权作用域时运行确定性 Drop。

Drop：

- 不得静默执行任意网络操作；
- 可能失败的清理必须显式；
- Critical Profile 中 Drop 的时间和 Effect 必须有界；
- 循环清理策略属于 Managed，不属于普通 Resource Drop。

---

## 15. Effect System

### 15.1 Effect Row

Effect Row 是无序、去重的标签集合，可包含参数：

```text
{}
{Console.Write}
{File.Read<PathScope>, Clock<Monotonic>}
{State<World>, Random<BattleRng>}
```

### 15.2 Effect 合并

表达式组合的 Effect 为子表达式 Effect 的并集，除非 Effect 被 Handler 消除。

### 15.3 Effect Polymorphism

高阶函数可以对 Effect Row 多态：

```text
map :
    ('a -> 'b ! ε)
    -> List<'a>
    -> List<'b>
    ! ε
```

### 15.4 Pure

`Pure` 表示空 Effect Row，而不是普通标签。

```text
Pure ≡ {}
```

### 15.5 Allocation Effect

内存分配可进入 Effect：

```text
Allocate<Managed>
Allocate<Region r>
Allocate<Device GPU>
```

优化器证明分配被消除时，Audit Source 可以报告：

```text
source effect: Allocate<Region r>
lowered runtime allocation: 0
```

不得混淆语义分配与最终机器分配。

### 15.6 User-defined Effect

未来语法：

```fsharp
effect Random<'rng> =
    next_u64: Unit -> u64
```

Handler：

```fsharp
handle 模拟 with 固定随机 42
```

---

## 16. Capability

### 16.1 定义

Capability 是不可伪造的授权值或静态权限边界。

Effect 是行为事实，Capability 是授权事实。

### 16.2 Capability 环境

模块声明：

```fsharp
module 日志
    requires Console.Write
```

在 Core 中等价于模块或入口获得一个受限 Capability Environment。

### 16.3 最小权限

编译器应计算每个函数和模块的最小 Capability 闭包。

Audit 输出：

```text
declared:
    Console.Write
    File.Read<ConfigDir>

used:
    Console.Write

unused:
    File.Read<ConfigDir>
```

未使用的 Capability 应提示收缩。

### 16.4 Capability 不可伪造

普通代码不能：

- 从整数构造文件能力；
- 从字符串构造网络权限；
- 通过反射访问未导入能力；
- 将受限 Capability 序列化给远程节点。

能力传递必须被类型和边界规则检查。

### 16.5 Component Boundary

包导入/导出显式声明：

```text
imports:
    Clock.Monotonic
    Storage.Read<World>

exports:
    Simulation.tick
```

组件不能通过其他隐藏通道访问外界。

---

## 17. Contract 语义

### 17.1 前置条件

```text
requires P
```

调用者承担证明或检查 `P` 的义务。

### 17.2 后置条件

```text
ensures result -> Q
```

实现承担 `Q`。

### 17.3 不变量

适用于：

- 类型；
- Actor state；
- Node state；
- 循环；
- 模块；
- 持久化 schema。

### 17.4 证明状态

每个 Contract Claim 具有状态：

```text
Proved(kernel, proof_id)
RuntimeChecked(check_id)
ModelChecked(model_id, bound)
Tested(test_set_id)
Assumed(reason, approver)
Unverified
```

状态是证据元数据，不改变逻辑陈述本身。

### 17.5 Contract 与优化

优化必须保持已声明 Contract。

若优化依赖新的假设，必须：

- 产生新 `Assumed`；
- 改变 `ContractId`；
- 在 Semantic Diff 中显著显示；
- Critical Profile 要求审批。

---

## 18. Task 语义

### 18.1 结构化生命周期

`task` 在一个父 Scope 中创建。

规则：

- 子 Task 有且只有一个结构化父节点，除非显式 Detach；
- Scope 退出前必须 Join、Cancel 或 Transfer；
- 父取消向子传播；
- 子 Fault 传播策略由 Scope 明确；
- 未观察的 Task 结果是错误或警告，取决于类型。

### 18.2 Suspension Point

`await` 是显式 suspension point。

Actor turn、Borrow 与事务边界可依赖 suspension point 进行静态检查。

### 18.3 Detach

Detach 不是普通语法捷径。

显式 Detach 必须：

- 转移资源所有权；
- 指定 Supervisor 或 Runtime Owner；
- 指定错误汇报通道；
- 指定取消与关闭策略。

v0.2 以前不实现。

### 18.4 调度不可观察性

除 `Clock`、Deadline、优先级或外部 Effect 外，Task 调度顺序不应成为程序语义。

数据竞争被类型系统排除。

---

## 19. Actor 语义

### 19.1 Actor Identity

Actor 具有稳定运行时身份和私有 state。

外部代码只能通过：

```text
ActorRef<Message>
```

发送消息。

### 19.2 State Isolation

只有当前 Actor turn 可以直接访问其 state。

Actor state 的 `&mut` 不得：

- 返回给调用者；
- 存入消息；
- 跨 `await` 保持，除非编译器证明安全并由 RFC 允许；
- 传给其他 Actor。

### 19.3 Mailbox

Mailbox 必须有界：

```text
capacity: Nat
backpressure:
    Wait
  | Reject
  | DropNewest
  | DropOldest
  | Coalesce<Key>
```

默认推荐 `Wait` 或 `Reject`。

Critical Profile 禁止 Drop 策略，除非安全分析明确允许。

### 19.4 Message

消息必须是 Sendable：

- 不含指向发送方可变状态的 Borrow；
- Resource 必须 Move；
- Managed 引用跨隔离域需满足共享策略；
- Remote 消息必须可序列化且有 schema identity。

### 19.5 顺序

本地 Actor：

- 同一发送者到同一接收者的消息保持发送顺序；
- 不同发送者之间的全局顺序未指定；
- Actor 每次处理一个 turn；
- Actor turn 内无 `await` 时具有原子观察边界。

远程 Actor 顺序和交付策略必须进入通道类型。

### 19.6 Supervision

Actor 必须属于监督树或显式 Runtime Root。

Supervisor 定义：

```text
restart strategy
restart intensity
shutdown order
state restore policy
fault escalation
```

“重启”不能自动等同于“恢复正确状态”；持久化 Actor 必须具有版本化恢复和不变量检查。

### 19.7 Remote Actor

```text
RemoteActorRef<Message, Failure, Delivery>
```

`Delivery` 至少可表达：

```text
AtMostOnce
AtLeastOnce
IdempotentRetry<Key>
```

不承诺真正无条件的 Exactly Once。

---

## 20. Node 语义

### 20.1 同步周期

```fsharp
node 控制
    every 10.ms
    deadline 2.ms
=
    ...
```

语义上，Node 在离散逻辑时钟上反复执行 step。

### 20.2 Node 限制

Critical Node 默认：

- 无一般 GC；
- 无动态分配；
- 无未限定网络；
- 无无界递归；
- 无无界集合增长；
- 固定输入/输出类型；
- 显式 Clock；
- 可计算 WCET 与内存上界；
- 固定 Fault 策略。

### 20.3 状态

Node state 是前一 tick 到后一 tick 的显式状态。

编译器不得把普通全局变量作为隐藏 Node state。

### 20.4 Deadline

Deadline 是 Contract 与 Target 假设的组合，不是纯语言层绝对事实。

证据必须记录：

- 目标处理器；
- 编译器版本；
- 调度器；
- 中断模型；
- 缓存/总线假设；
- 测量或分析方法。

---

## 21. Kernel 语义

### 21.1 数据并行纯度

Kernel 默认要求：

- 迭代之间无未声明依赖；
- 写集合不重叠；
- 数据布局明确；
- 禁止普通 I/O；
- 禁止 Actor/Network Effect；
- 设备 Capability 明确。

### 21.2 Device Buffer

```text
Buffer<Device, T>
ReadBuffer<Device, T>
WriteBuffer<Device, T>
```

数据移动：

```text
transfer : Buffer<A,T> -> Device B -> Task<Buffer<B,T>, TransferError>
```

传输不是隐式零成本转换。

### 21.3 Determinism

并行 reduction 必须声明：

```text
deterministic
associative
approximately_associative(error <= ε)
unordered
```

浮点 reduction 默认不能被当作严格结合。

### 21.4 Lowering

Kernel 可 Lower 到：

- CPU scalar；
- SIMD；
- GPU shader/compute；
- TPU/NPU graph；
- accelerator-specific dialect。

后端选择不得改变声明的数值和确定性 Contract。

---

## 22. Determinism 与 Replay

### 22.1 Determinism Class

函数/组件可以被分析为：

```text
PureDeterministic
SeedDeterministic<RandomSource>
InputDeterministic<EffectLog>
ScheduleDeterministic
Nondeterministic(reason)
```

### 22.2 Effect Log

可记录 Effect：

```text
Clock
Random
Input
NetworkReceive
DeviceInput
ExternalStorage
```

重放由：

```text
Checkpoint + EffectLog + ProgramSnapshot
```

重建。

### 22.3 Actor Replay

Actor 消息日志必须记录：

- Message Schema ID；
- Sender/Receiver ID；
- Delivery ID；
- Logical Time；
- Payload Hash；
- 重试/重复信息。

### 22.4 Replay 不是生产一致性的默认证明

Replay 可以复现给定输入和调度模型，但不能自动证明：

- 外部硬件完全一致；
- 网络故障空间已覆盖；
- Timing Contract 在所有平台成立。

证据状态必须诚实标注。

---

## 23. Profile 语义

### 23.1 Explore

允许：

```text
Managed
GC
Interpreter/VM/JIT
Dynamic module reload
Runtime contract checks
Broad reflection through typed schema
```

但程序行为仍遵循相同核心求值与数值语义。

### 23.2 Native

允许：

```text
Value
Resource
Managed Island
AOT
Region inference
Monomorphization
SIMD
Kernel lowering
```

优化不得改变可观察语义。

### 23.3 Critical

附加静态约束：

```text
no general GC
no JIT
no dynamic code
bounded allocation
bounded recursion
bounded mailbox
explicit Clock/Random/Input
restricted FFI
proved or handled Fault
fixed target manifest
evidence bundle required
```

同一源码若无法进入 Critical，编译器必须输出具体阻碍，而不是模糊地说“不支持”。

---

## 24. Audit Source

### 24.1 目的

Audit Source 是 Semantic Graph 的确定性文本投影。

它不是普通 Formatter 输出，而是：

- 唯一；
- 稳定；
- 无歧义；
- 展开隐式语义；
- 可重新解析；
- 可与 Graph Hash 绑定。

### 24.2 Round-trip

要求：

```text
parse_audit(render_audit(graph)) = graph
```

忽略纯显示元数据。

### 24.3 展开内容

Audit Source 展开：

- 已解析 DefinitionId；
- 完整类型；
- Effect Row；
- Capability；
- Borrow/Move；
- Fault；
- Arithmetic Mode；
- Contract；
- Determinism；
- Profile restriction；
- Target assumption。

### 24.4 Author Source 与 Audit Source

Author Source 可以保留：

- 中文名称；
- 注释；
- 自然布局；
- 类型省略；
- Effect 省略；
- Borrow 省略。

Audit Source 不得丢失这些信息，但可以使用规范化布局和附加语义块。

---

## 25. Semantic Transaction

### 25.1 输入

```text
Transaction {
    base_graph_hash
    target_ids
    preconditions
    operations
    preserve_constraints
    allowed_changes
    provenance
}
```

### 25.2 原子性

事务流程：

```text
load snapshot
-> verify base hash
-> apply to temporary graph
-> resolve names
-> type/effect/capability check
-> ownership check
-> contract/proof check
-> run required tests
-> compute semantic diff
-> commit or rollback
```

任何阶段失败，正式快照不得改变。

### 25.3 Stale Edit

若 `base_graph_hash` 不匹配，事务必须拒绝，而不是自动在新代码上猜测应用。

### 25.4 Preserve

常见约束：

```text
public type
public contract
effect set
capability set
determinism
allocation bound
latency bound
binary ABI
serialization schema
```

### 25.5 AI 权限

AI Agent 的 Capability 可以限制：

```text
Graph.Read
Graph.Propose
Graph.Commit
Test.Run
Benchmark.Run
Proof.Request
Target.Build
```

Proposal 与 Commit 应可分离。

Accepted RFC-0027 defines the first bounded proposal-only realization of this
section. `ling patch` checks one import-free file snapshot, exact
`base_program_id`, authorized target IDs, a checked in-memory full-source
replacement, and definition/type/Effect/Capability preservation. Its result is
always `committed: false`; it cannot mutate source or represent `Graph.Commit`.
`ling query` is the corresponding exact-NFC, checked-graph, read-only surface.
Project transactions, partial edits, tests/proofs, atomic publication, and LSP
projection remain outside that accepted slice.

---

## 26. 诊断语义

### 26.1 稳定错误码

错误码结构：

```text
L-<DOMAIN>-<NUMBER>
```

例如：

```text
L-LEX-0001
L-NAME-0012
L-TYPE-0042
L-EFFECT-0103
L-CAP-0021
L-OWN-0307
L-CONTRACT-0018
```

### 26.2 根因优先

编译器应：

- 报告根因；
- 抑制明显级联错误；
- 允许一次输出多个独立错误；
- 将 Source Span 与 Semantic ID 同时返回。

### 26.3 修复计划

Repair 不是任意文本补丁，而是结构化操作：

```text
AddTypeAnnotation
MakeBindingMutable
ReturnRecordUpdate
ShortenBorrow
SplitResource
AddCapabilityRequirement
HandleEffect
ExpandPatternCases
```

每个 Repair 必须列出可能改变的语义。

### 26.4 本地化

诊断至少可以提供：

```text
message_zh
message_en
```

稳定错误码、Facts 与 Repair Schema 不依赖自然语言。

---

## 27. 构建与依赖语义

### 27.1 Hermetic Build

构建图节点只能通过声明的输入、输出和 Capability 工作。

禁止依赖包在安装或构建时任意执行未声明 Shell。

### 27.2 Dependency Identity

依赖引用：

```text
PackageId
SemanticSnapshotHash
PublicContractHash
Target/Profile constraints
```

名字和版本号是人类标签，不是唯一安全身份。

### 27.3 Code Generation

代码生成必须是 Typed Transformation：

```text
SchemaNode -> GeneratedDefinitionSet
```

生成节点保存：

```text
generated_by
input_ids
generator_version
proof/validation
```

不鼓励将字符串模板生成的代码伪装成人工源码。

---

## 28. FFI 与 Target Primitive

### 28.1 FFI 边界

FFI 声明必须定义：

- 参数与返回 ABI；
- 所有权；
- 生命周期；
- 可变性；
- 线程安全；
- 重入；
- Error/Fault；
- Capability；
- Blocking；
- Target；
- 验证状态。

### 28.2 Primitive

`primitive` 是经过签名的目标适配定义。

普通包不能声明任意 Primitive；只有受信 Target Package 可以提供。

### 28.3 Trusted Computing Base

Critical 构建的 TCB 必须列出：

```text
compiler core
proof kernel
target backend
runtime subset
primitive packages
hardware assumptions
```

AI、IDE 和 Formatter 不应自动进入 TCB。

---

## 29. v0.0.1 的正式语义子集

v0.0.1 实现：

```text
Literal:
    Unit Bool Int Float Text

Expression:
    Variable Let Lambda Apply If
    RecordConstruct RecordProject RecordUpdate
    VariantConstruct Match
    Sequence PlaceAssign
    ConsoleWrite

Type:
    primitive
    function
    tuple
    nominal record
    nominal ADT
    type variable
    Option Result

Effect:
    Pure
    Console.Write

Capability:
    Console.Write

Name:
    Unicode XID
    NFC
    confusable diagnostics

Program:
    module
    Semantic Graph
    Semantic ID
```

v0.0.1 不实现但 Schema 预留：

```text
Trait
Managed GC
Resource/Borrow
Effect Handler
Task
Actor
Node
Kernel
Remote
Contract proof
Native backend
Critical profile enforcement
```

预留不意味着允许占位语法静默运行。未实现特性必须产生清楚错误。

---

## 30. v0.0.1 最小求值示例

源码：

```fsharp
type 人物 =
    { 姓名: Text
      mutable 血量: Int }

let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }

let main () =
    let mutable 关羽 =
        { 姓名 = "关羽"
          血量 = 100 }

    关羽 <- 受到伤害 30 关羽
    Console.write 关羽.姓名
```

推导摘要：

```text
受到伤害:
    Int -> 人物 -> 人物
    effects {}

main:
    Unit -> Unit
    effects {State<人物>, Console.Write}
    requires {Console.Write}
```

v0.0.1 的 Interpreter Cell 只承载当前函数内的 mutable local binding。Record copy 必须产生独立 Value，字段可变性不得实现为调用方可观察的共享 Cell；Semantic Graph 对显式写回使用 `PlaceAssign`。

---

## 31. 待 RFC 决定的问题

1. `State<T>` 是否始终进入 Effect Row，或局部独占可变状态可被 Effect Masking 消除；
2. fixed-width checked arithmetic 的 Fault 是否进入显式函数签名；
3. Trait coherence 与 orphan rule；
4. public lifetime 是否允许完全推导；
5. Actor turn 内 `await` 的精确重入语义；
6. Remote Actor 的交付策略类型；
7. Alias 的源码语法和本地化显示规则；
8. Semantic Hash 算法与升级策略；
9. Author Source 是否可以有多种等价本地化关键字视图；
10. Critical Profile 的最小可验证 Core；
11. 包管理命名空间与域名（英文名 Ling、CLI `ling`、扩展名 `.ling` 已确定）。

在 RFC 接受前，实现不得私自固定不可逆语义。

---

## 32. 规范性与启发性参考

### Unicode 与中文标识符

- Unicode UAX #31 — Unicode Identifiers and Syntax  
  https://www.unicode.org/reports/tr31/

- Unicode UTS #39 — Unicode Security Mechanisms  
  https://www.unicode.org/reports/tr39/

- Unicode UTS #55 — Unicode Source Code Handling  
  https://www.unicode.org/reports/tr55/

- Rust Reference — Identifiers and NFC normalization  
  https://doc.rust-lang.org/reference/identifiers.html

- Python PEP 3131 — Non-ASCII identifiers  
  https://peps.python.org/pep-3131/

### Effect、状态机与并发

- Koka — effect types and handlers  
  https://koka-lang.github.io/

- Erlang/OTP — supervision trees  
  https://www.erlang.org/doc/system/design_principles.html

- P — communicating state machines and automated analysis  
  https://p-org.github.io/P/

- Swift — structured concurrency and actors  
  https://docs.swift.org/swift-book/documentation/the-swift-programming-language/concurrency/

- SPARK — Ravenscar Profile  
  https://docs.adacore.com/spark2014-docs/html/ug/en/source/concurrency.html

### 语义图与异构编译

- Unison — content-addressed definitions  
  https://www.unison-lang.org/docs/the-big-idea/

- MLIR — multi-level IR and heterogeneous lowering  
  https://mlir.llvm.org/

- WebAssembly Component Model — WIT interfaces and worlds  
  https://component-model.bytecodealliance.org/design/wit.html

### 命名冲突

- Vercel Labs Zerolang  
  https://github.com/vercel-labs/zerolang
