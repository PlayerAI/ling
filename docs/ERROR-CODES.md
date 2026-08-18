# Ling 错误码注册表 / Ling Error Code Registry

> Schema：`ling.diagnostic/0.1`  
> 初始版本：`0.0.1-dev`  
> 分配规则：见 [`decisions/0001-error-code-policy.md`](decisions/0001-error-code-policy.md)

本文件是稳定错误码分配的唯一来源。错误码一经进入公开测试或发布版本，不得复用为其他含义；废弃项保留记录。

This file is the single source of truth for stable diagnostic-code allocation. A code that enters public tests or a release must never be reused for another meaning; deprecated entries remain recorded.

| Code | 稳定含义 / Stable meaning | 中文模板 | English template | Facts | Repair | Since |
| --- | --- | --- | --- | --- | --- | --- |
| `L-IO-0001` | 无法从宿主文件系统读取源码 / Source cannot be read from the host filesystem | 无法读取源码文件“{path}” | failed to read source file `{path}` | `io_kind` | — | `0.0.1-dev` |
| `L-LEX-0001` | 源码包含非法 UTF-8 / Source contains invalid UTF-8 | 源码不是有效的 UTF-8 | source is not valid UTF-8 | `valid_up_to` | — | `0.0.1-dev` |
| `L-LEX-0002` | UTF-8 BOM 出现在文件头以外 / UTF-8 BOM occurs outside the file start | UTF-8 BOM 只能出现在文件开头 | the UTF-8 byte-order mark is only allowed at the start of a file | — | — | `0.0.1-dev` |
| `L-LEX-0003` | 源码超过当前 `u32` byte-span 上限 / Source exceeds the current `u32` byte-span limit | 源码文件超过当前实现支持的大小 | source file exceeds the size supported by this implementation | `byte_len`, `maximum_byte_len` | — | `0.0.1-dev` |
| `L-LEX-0004` | 标识符违反 Unicode/XID 安全规则 / Identifier violates Unicode/XID security rules | 标识符包含不允许的 Unicode 字符 | identifier contains a disallowed Unicode character | `reason` | — | `0.0.1-dev` |
| `L-LEX-0005` | 源码包含无法识别的字符 / Source contains an unrecognized character | 无法识别字符 `{codepoint}` | unrecognized character `{codepoint}` | `codepoint` | — | `0.0.1-dev` |
| `L-LEX-0006` | 嵌套块注释在 EOF 前未闭合 / Nested block comment is not closed before EOF | 块注释未闭合 | unterminated block comment | — | — | `0.0.1-dev` |
| `L-LEX-0007` | 块注释嵌套超过实现上限 / Block-comment nesting exceeds the implementation limit | 块注释嵌套超过 256 层 | block-comment nesting exceeds 256 levels | `maximum_depth` | — | `0.0.1-dev` |
| `L-LEX-0008` | Text 在换行或 EOF 前未闭合 / Text is not closed before newline or EOF | Text 字面量未闭合 | unterminated Text literal | — | — | `0.0.1-dev` |
| `L-LEX-0009` | Text 包含未知或残缺的普通转义 / Text contains an unknown or incomplete ordinary escape | Text 字面量包含无效转义 | Text literal contains an invalid escape | — | — | `0.0.1-dev` |
| `L-LEX-0010` | `\u{...}` 不是有效 Unicode scalar escape / `\u{...}` is not a valid Unicode scalar escape | Text 字面量包含无效 Unicode 转义 | Text literal contains an invalid Unicode escape | — | — | `0.0.1-dev` |
| `L-LEX-0011` | 数字不符合 DEC-0005 / Number does not conform to DEC-0005 | 数字字面量格式无效 | invalid numeric literal | — | — | `0.0.1-dev` |
| `L-SYNTAX-0001` | 行首语义缩进包含 Tab / Leading semantic indentation contains a tab | 语义缩进不能使用 Tab | tabs are not allowed in semantic indentation | — | — | `0.0.1-dev` |
| `L-SYNTAX-0002` | Dedent 未匹配已有 layout 层级 / Dedent does not match an existing layout level | Dedent 未对齐到已有缩进层级 | dedent does not align with an existing indentation level | `actual_column`, `recovered_column` | — | `0.0.1-dev` |
| `L-SYNTAX-0003` | Layout/delimiter 超过 256 层 / Layout or delimiter depth exceeds 256 | Layout 或 delimiter 嵌套超过 256 层 | layout or delimiter nesting exceeds 256 levels | `maximum_depth` | — | `0.0.1-dev` |
| `L-SYNTAX-0004` | 结束 delimiter 没有开始项 / Closing delimiter has no opening delimiter | 存在没有对应开始符号的结束 delimiter | closing delimiter has no matching opening delimiter | `found` | — | `0.0.1-dev` |
| `L-SYNTAX-0005` | 开始/结束 delimiter 种类不匹配 / Opening and closing delimiter kinds differ | 结束 delimiter 与开始 delimiter 不匹配 | closing delimiter does not match the opening delimiter | `expected`, `found` | — | `0.0.1-dev` |
| `L-SYNTAX-0006` | Delimiter 在 EOF 前未闭合 / Delimiter is not closed before EOF | delimiter 在文件结尾前未闭合 | delimiter is not closed before the end of the file | `expected` | — | `0.0.1-dev` |
| `L-SYNTAX-0010` | Parser 遇到不符合当前 grammar 的 token / Parser encounters a token outside the current grammar | 语法错误：需要 `{expected}` | syntax error: expected `{expected}` | `context`, `expected`, `found` | — | `0.0.1-dev` |
| `L-SYNTAX-0011` | Parser recursion 超过 512 层 / Parser recursion exceeds 512 levels | 语法嵌套超过 512 层 | syntax nesting exceeds 512 levels | `maximum_depth` | — | `0.0.1-dev` |
| `L-IMPL-0001` | 所请求路径依赖尚未实现的编译阶段 / Requested path requires a compiler stage not yet implemented | `{command}` 命令所需的编译阶段尚未实现 | the compiler stage required by `{command}` is not implemented yet | `command`, `completed_stage`; 可选 / optional: `source_name`, `had_bom`, `unicode_version`, `token_count` | — | `0.0.1-dev` |

## 兼容性边界 / Compatibility boundary

- 稳定：`code` 的含义、`severity` 的错误/警告分类，以及现有 Facts 字段的类型。
- 可兼容扩展：新增可选 Facts、新增 Repair 候选、改进中英文措辞。
- 非兼容变更：改变 code 含义、删除既有 Facts、改变字段类型或把 error 降为 warning。此类变更必须分配新 code，或升级 Diagnostic Schema 并提供迁移说明。

- Stable: the meaning of `code`, error/warning classification, and types of existing Facts.
- Compatible extension: adding optional Facts or Repair candidates and improving localized wording.
- Breaking: changing code meaning, removing existing Facts, changing field types, or downgrading an error to a warning. Such changes require a new code or a Diagnostic Schema version with migration guidance.
