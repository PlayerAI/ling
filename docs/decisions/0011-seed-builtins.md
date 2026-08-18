# DEC-0011：Seed 内置项

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-10

## 建议决议

Seed 只提供当前规范示例所需的以下内置定义：

| 规范名称 | 类型 | Effect | Capability | 行为 |
| --- | --- | --- | --- | --- |
| `Console.write` | `Text -> Unit` | `{Console.Write}` | `Console.Write` | 写入完整 Text，并追加一个 UTF-8 LF；不使用平台换行转换 |
| `Text.format` | `Text -> Int -> Text` | `{}` | — | 将模板中唯一的 `{}` 替换为十进制 Int；占位符数量不为一产生 Runtime Fault |
| `max` | `Int -> Int -> Int` | `{}` | — | 返回较大 Int |
| `min` | `Int -> Int -> Int` | `{}` | — | 返回较小 Int |
| `map` | `('a -> 'b ! ε) -> List<'a> -> List<'b> ! ε` | 参数函数的 `ε` | 由 `ε` 推导 | 严格按输入顺序调用参数函数 |
| `sum` | `List<Int> -> Int` | `{}` | — | 任意精度加法；空 List 返回 `0` |

`max/min` 在 Seed 不重载到 f64；`Text.format` 在 Seed 不引入 Trait、反射或通用字符串化。扩展这些签名必须另行 RFC。

内置项以 Resolver 中的规范 prelude definitions 注入，而不是 Lexer keyword、CLI 分支或 AST 特例。`Console`、`Text` 是保留的 built-in namespace；用户不能在 module scope 重定义它们。未限定的 `max/min/map/sum` 可被局部 lexical binding shadow，但不能在同一 module scope 重复定义。

所有 application 严格 call-by-value、从左到右求值。内置定义拥有与用户定义相同类别的稳定 DefinitionId 和 Graph node，只在 origin 字段标记 `builtin`。

## 验收证据

- Hello World 精确输出 `你好，零\n`；
- `Console.write 1` 是 Type error；
- 无 `Console.Write` 声明时不调用 host；
- 超过 i128 的 `max/min/sum` 正确；
- `map` 保持元素和 Effect 顺序；
- built-in shadow/duplicate 规则一致通过 Resolver。
