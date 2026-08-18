# DEC-0004：Pipeline 语法与 Lowering

> 状态：Accepted  
> 日期：2026-08-18  
> 关闭缺口：G-02

## 决议

`|>` 是左结合的低优先级二元运算符。它低于 equality/comparison/arithmetic/application，高于 `<-` 赋值：

```ebnf
assignment_expression = pipeline_expression, [ "<-", expression ];
pipeline_expression   = equality_expression,
                        { "|>", equality_expression };
```

Lowering 将左侧值插入右侧 application 的**最后一个显式参数**：

```text
x |> f       ≡ f x
x |> f a     ≡ f a x
x |> f a b   ≡ f a b x
x |> f |> g  ≡ g (f x)
```

Parser 先建立 `Pipeline` AST 节点并保存两侧 Span；HIR Lowering 再建立 application。这样诊断和格式化仍能引用 Author Source，而类型检查器只处理统一的 Apply 结构。

Pipeline 可以写在一行，也可以让 `|>` 出现在续行开头：

```fsharp
let 总伤害 人物列表 =
    人物列表
    |> map 计算伤害
    |> sum
```

续行 `|>` 必须位于当前表达式的起始缩进列；其右侧必须在同一行出现，或在下一非空行进一步缩进。`|>` 不跨越 Dedent，不允许缺失右操作数。

## 理由

末参数插入与现有 `受到伤害 30 人物`、`map 计算伤害 人物列表` 示例一致。左结合使多级 pipeline 按源码顺序执行，并保持 SEMANTICS §8.2 的左到右求值。

