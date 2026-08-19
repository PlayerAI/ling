# DEC-0014：Seed Prelude 中的 `Option` 与 `Result`

> 状态：Accepted
> 提出日期：2026-08-18
> 接受日期：2026-08-19
> 澄清范围：`Option / Result` 的交付模型、名称空间与 Semantic Identity

## 背景

[RFC-0001](../RFC-0001.md) 与 [SEMANTICS](../SEMANTICS.md) 将 `Option`、`Result` 及其 constructor 列入 Seed 类型面，但 [DEC-0011](0011-seed-builtins.md) 只冻结了函数型内置项，没有说明这些 nominal ADT 如何进入每个程序。实现不得通过临时字符串查找或无法追踪的编译器魔法补齐该缺口。

## 建议决议

Seed 定义一个逻辑 module `Ling.Prelude`，由 Resolver 以规范化的预声明注入，不从工作区文件系统加载：

```text
type Option<'a> = Some of 'a | None
type Result<'a, 'e> = Ok of 'a | Error of 'e
```

- `Option / Result` 是 nominal type definition，`Some / None / Ok / Error` 是 constructor definition；
- DefinitionId 使用逻辑 module `Ling.Prelude`、正常的 type/constructor kind 和 DEC-0012 canonical encoding；
- Semantic Graph 的 origin 为 `prelude`，不得标记为普通用户定义或函数型 `builtin`；
- 所有用户 module 隐式获得这些规范名称；module scope 不得重定义，local value binding 可以按普通词法规则 shadow constructor；
- type 与 value/constructor 暂沿用 Seed 的单一名称空间；未来拆分名称空间属于不兼容语义变更；
- prelude definition 参与类型、引用边和 ProgramId，但不产生虚构 Source Span；
- `Ling.Prelude` 不可由磁盘上的同名 module 替换，也不要求 Capability。

## 拒绝的替代方案

- 每次使用时临时构造类型：无法提供稳定定义、引用边和 Graph origin；
- 自动加载用户可覆盖的 `Prelude.ling`：引入未决的标准库发现与版本选择问题；
- 将四个 constructor 作为 Lexer keyword：扩大关键字集合并破坏普通名称解析路径。

## 验收证据

- `Some 1` 与 `Some "x"` 在不同 let binding 中实例化为不同 `Option<T>`；
- `Ok 1` 与 `Error "x"` 可共同约束为 `Result<Int, Text>`；
- constructor pattern 的 payload 类型正确绑定；
- module scope 重定义得到稳定 Resolver 诊断，local shadow 遵循普通词法规则；
- Semantic JSON 中 type/constructor ID、origin 和 reference edge 可验证；
- 两个独立进程生成相同 prelude ID。
