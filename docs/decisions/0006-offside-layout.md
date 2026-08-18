# DEC-0006：Seed Offside/Layout 规则

> 状态：Accepted  
> 日期：2026-08-18  
> 关闭缺口：G-05

## 列与缩进

- 逻辑行以 DEC-0002 规范化后的 LF 划分。
- 缩进只由行首 ASCII space 组成；行首 Tab 是错误。Parser 不要求缩进为四的倍数，Formatter 固定输出四个 space。
- Layout column 是行首 space 数量，零基；human diagnostic 仍使用 DEC-0002 的一基 Unicode-scalar column。
- blank line 和 comment-only line 不改变 layout stack。

## Layout token

Lexer/Layout 层产生 `Newline`、`Indent`、`Dedent` 与 `SoftNewline`：

- 非空行缩进大于 stack top 时产生 `Indent`；
- 等于 stack top 时只产生 `Newline`；
- 小于 stack top 时产生足够的 `Dedent`；目标列必须匹配既有 stack entry，否则产生 inconsistent-dedent 错误，并以最近较小层级作为恢复点；
- `()`、`[]`、`{}` 内 newline 是 `SoftNewline`，不改变 layout stack。Record/List Parser 可以把它当成员分隔符；其他表达式位置把它当空白。

文件末尾隐式产生一个 `Newline`（若需要）和全部剩余 `Dedent`。

## Block 与续行

`=`、`then`、`else`、`->`、module `requires` 等在行尾引入 block body；下一非空行必须比引入者所在 layout column 更深。Block body 中同列的表达式形成 Sequence，Dedent 结束 block。

`match value with` 的 case `|` 与 `match` 所在表达式列对齐，case 的 `->` body 再进一步缩进。这一特例与 RFC-0001 §7.3/SEMANTICS §11 的规范示例保持一致。

以下情况是 continuation，不开始新的 Sequence：

- 前一行以需要右操作数的 operator 结束，下一行必须进一步缩进；
- DEC-0004 定义的行首 `|>`，与当前 pipeline 起始列相同；
- delimiter 内的 soft newline。

任意隐式 continuation 必须由 token/缩进规则唯一确定；Parser 不根据类型或名称解析结果猜测换行含义。

## 注释与恢复上限

- `//` 与 `///` 到逻辑行末结束；它们所在的 comment-only line 忽略缩进。
- `/* ... */` 可嵌套并跨行；完全被块注释覆盖的逻辑行视为 blank line。
- delimiter depth 与 layout depth 上限均为 256，Parser recursion depth 上限为 512。超过上限产生结构化诊断，不得 panic 或栈溢出。
- 未闭合 delimiter/comment 在 EOF 报根因；恢复不得伪造成功 AST。

## 理由

相对缩进允许 2-space、4-space 或其他团队风格，同时保持 Parser 语义一致。Soft newline 解决“delimiter 内不触发 Dedent”与“record 可用换行分隔字段”的共同需求。
