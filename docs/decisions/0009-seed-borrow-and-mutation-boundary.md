# DEC-0009：Seed Borrow 与 Mutation 边界

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-08

## 建议决议

v0.0.1 Seed 不实现 Resource、Borrow、`&mut`、隐式引用传递或 Borrow Edge。所有函数参数和 record 都使用 Value semantics；参数 binding 默认不可变，函数内对参数副本的修改不得传播给调用方。

`<-` 的合法 Place 仅为：

1. 当前函数内由 `let mutable name = value` 建立的局部 binding；
2. 以上述 mutable local binding 为根，且 nominal record declaration 中逐段声明为 `mutable` 的字段 projection。

以下情况一律拒绝：

- top-level `let mutable`；
- 函数参数、immutable local 或 import 的赋值；
- immutable field；
- 临时表达式、函数返回值、literal 或 constructor application 的 projection；
- 任何需要调用方别名可见性的修改。

Record copy 创建独立 Value。`mutable field` 是 Place 写入权限，不表示字段内部存放共享 Cell。不可变 record update：

```fsharp
{ old with field = value }
```

创建新 record，不修改 `old`。

因此，原有隐式参数更新示例必须改写为显式返回：

```fsharp
let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }

let mutable 关羽 = ...
关羽 <- 受到伤害 30 关羽
```

## 理由

这使 SEMANTICS §29 的 Seed 边界优先于 RFC §6.6 中尚未定义的受限自动 Borrow，避免用共享 Cell 冒充 Value record 或泄漏 Rust aliasing 行为。

## 验收证据

- mutable local 和 mutable field 写入成功；
- immutable binding/field、parameter 和 temporary assignment 被拒；
- record copy 后更新互不影响；
- HIR/Graph 中不产生 Borrow 类型或 Borrow Edge；
- evaluator 只观察显式 Place Cell。
