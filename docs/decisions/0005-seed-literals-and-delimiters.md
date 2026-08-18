# DEC-0005：Seed 字面量与分隔符

> 状态：Accepted  
> 日期：2026-08-18  
> 关闭缺口：G-03、G-04

## Record 与 List 分隔符

Record type、record literal、record update 和 list literal 的成员可由换行或 `;` 分隔，可有一个尾随 `;`。单行形式必须使用 `;`；逗号不作为这些结构的分隔符。空行和 comment-only 行不产生成员。

```fsharp
{ 姓名 = "关羽"; 血量 = 100 }

{ 姓名 = "关羽"
  血量 = 100 }

[1; 2; 3;]
```

Record 至少包含一个字段。字段名不得在同一构造或类型声明内重复。Record 字段表达式换行时必须比字段起始列更深。花括号内 newline 不产生 block Dedent，但 Parser 保留 soft newline 作为字段分隔信息。

Tuple 使用逗号：`(a, b)`；`()` 是 Unit；`(a)` 是分组。Seed 不提供单元素 tuple。

## Int

`Int` 是任意精度整数。源码支持：

```text
decimal: 0 | [1-9] digit*
binary:  0b binary_digit+
octal:   0o octal_digit+
hex:     0x hex_digit+
```

数字之间可使用 `_`，但不得位于前缀之后、末尾或连续出现。正负号是 unary operator，不属于 literal token。Lexer 保存原始拼写；Parser 去除 `_` 并将数值交给任意精度表示，禁止先经过 `i64`/`i128`。

## f64

浮点必须包含小数点或十进制指数：`1.0`、`1e3`、`1.5e-2`。小数点两侧都必须有十进制数字；指数符号后必须有数字。`_` 只允许在同一数字段的数字之间。

Seed 不提供 `NaN`、`Infinity` 或十六进制浮点源码 literal。需要序列化非有限运行时值时，JSON 协议使用字符串 `"NaN"`、`"+Infinity"`、`"-Infinity"`，不得生成非法 JSON number；Semantic Graph 的最终编码仍受 G-11 决议约束。

## Text

Text 使用双引号，Seed 支持：

```text
\\  \"  \n  \r  \t  \0  \u{HEX}
```

`\u{HEX}` 包含 1–6 个十六进制数字，结果必须是 Unicode scalar value，禁止 surrogate 与大于 `U+10FFFF` 的值。未转义换行、未知 escape 和未闭合字符串是词法错误。Seed 不实现 raw string、插值或多行 Text。

Bool 为 `true` / `false`，Unit 为 `()`。单引号用于类型变量前缀；`Char` literal 不属于 Seed，`'x'` 必须产生明确的未支持诊断。

## 理由

换行/`;` 同时覆盖规范的多行示例和 REPL 单行输入。Tuple 独立使用逗号可避免 record field 与 tuple element 混淆。数字规则保持实现直接、无平台宽度泄漏，并为以后扩展 literal suffix 留出空间。

