# Ling 错误码注册表 / Ling Error Code Registry

> Registry schema：`ling.diagnostic-registry/0.1`
> Diagnostic wire schema：`ling.diagnostic/0.1`
> 初始版本：`0.0.1-dev`  
> 分配规则：见 [`decisions/0001-error-code-policy.md`](decisions/0001-error-code-policy.md)

本文件是稳定错误码分配的唯一来源。错误码一经进入公开测试或发布版本，不得复用为其他含义；废弃项保留记录。

This file is the single source of truth for stable diagnostic-code allocation. A code that enters public tests or a release must never be reused for another meaning; deprecated entries remain recorded. The generated [`governance/error-code-lock.toml`](governance/error-code-lock.toml) is compatibility evidence, not a second allocation source.

`Phase` is the root-cause domain embedded in the code. `Stability = Preview` describes the current `ling.diagnostic/0.1` container; DEC-0001 still makes an allocated code's root-cause meaning and existing payload field types non-reusable. A payload item is `name:type`; `?` marks an optional field. Allowed current JSON types are `string`, `integer`, `boolean`, and `string[]`.

## Active allocations / 活跃分配

| Code | Phase | Stability | Severity | 中文标题 | English title | 中文模板 | English template | Payload schema | Repair schema | Since |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `L-IO-0001` | `IO` | `Preview` | `Error` | 无法从宿主文件系统读取源码 | Source cannot be read from the host filesystem | 无法读取源码文件“{path}” | failed to read source file `{path}` | `io_kind:string` | — | `0.0.1-dev` |
| `L-LEX-0001` | `LEX` | `Preview` | `Error` | 源码包含非法 UTF-8 | Source contains invalid UTF-8 | 源码不是有效的 UTF-8 | source is not valid UTF-8 | `valid_up_to:integer` | — | `0.0.1-dev` |
| `L-LEX-0002` | `LEX` | `Preview` | `Error` | UTF-8 BOM 出现在文件头以外 | UTF-8 BOM occurs outside the file start | UTF-8 BOM 只能出现在文件开头 | the UTF-8 byte-order mark is only allowed at the start of a file | — | — | `0.0.1-dev` |
| `L-LEX-0003` | `LEX` | `Preview` | `Error` | 源码超过当前 `u32` byte-span 上限 | Source exceeds the current `u32` byte-span limit | 源码文件超过当前实现支持的大小 | source file exceeds the size supported by this implementation | `byte_len:integer, maximum_byte_len:integer` | — | `0.0.1-dev` |
| `L-LEX-0004` | `LEX` | `Preview` | `Error` | 标识符违反 Unicode/XID 安全规则 | Identifier violates Unicode/XID security rules | 标识符包含不允许的 Unicode 字符 | identifier contains a disallowed Unicode character | `reason:string` | — | `0.0.1-dev` |
| `L-LEX-0005` | `LEX` | `Preview` | `Error` | 源码包含无法识别的字符 | Source contains an unrecognized character | 无法识别字符 `{codepoint}` | unrecognized character `{codepoint}` | `codepoint:string` | — | `0.0.1-dev` |
| `L-LEX-0006` | `LEX` | `Preview` | `Error` | 嵌套块注释在 EOF 前未闭合 | Nested block comment is not closed before EOF | 块注释未闭合 | unterminated block comment | — | — | `0.0.1-dev` |
| `L-LEX-0007` | `LEX` | `Preview` | `Error` | 块注释嵌套超过实现上限 | Block-comment nesting exceeds the implementation limit | 块注释嵌套超过 256 层 | block-comment nesting exceeds 256 levels | `maximum_depth:integer` | — | `0.0.1-dev` |
| `L-LEX-0008` | `LEX` | `Preview` | `Error` | Text 在换行或 EOF 前未闭合 | Text is not closed before newline or EOF | Text 字面量未闭合 | unterminated Text literal | — | — | `0.0.1-dev` |
| `L-LEX-0009` | `LEX` | `Preview` | `Error` | Text 包含未知或残缺的普通转义 | Text contains an unknown or incomplete ordinary escape | Text 字面量包含无效转义 | Text literal contains an invalid escape | — | — | `0.0.1-dev` |
| `L-LEX-0010` | `LEX` | `Preview` | `Error` | `\u{...}` 不是有效 Unicode scalar escape | `\u{...}` is not a valid Unicode scalar escape | Text 字面量包含无效 Unicode 转义 | Text literal contains an invalid Unicode escape | — | — | `0.0.1-dev` |
| `L-LEX-0011` | `LEX` | `Preview` | `Error` | 数字不符合 DEC-0005 | Number does not conform to DEC-0005 | 数字字面量格式无效 | invalid numeric literal | — | — | `0.0.1-dev` |
| `L-LEX-0012` | `LEX` | `Preview` | `Error` | Seed 源码使用不支持的 Char 字面量 | Seed source uses an unsupported Char literal | Ling Seed 不支持 Char 字面量；请使用 Text | Ling Seed does not support Char literals; use Text instead | — | — | `0.0.1-dev` |
| `L-SYNTAX-0001` | `SYNTAX` | `Preview` | `Error` | 行首语义缩进包含 Tab | Leading semantic indentation contains a tab | 语义缩进不能使用 Tab | tabs are not allowed in semantic indentation | — | — | `0.0.1-dev` |
| `L-SYNTAX-0002` | `SYNTAX` | `Preview` | `Error` | Dedent 未匹配已有 layout 层级 | Dedent does not match an existing layout level | Dedent 未对齐到已有缩进层级 | dedent does not align with an existing indentation level | `actual_column:integer, recovered_column:integer` | — | `0.0.1-dev` |
| `L-SYNTAX-0003` | `SYNTAX` | `Preview` | `Error` | Layout/delimiter 超过 256 层 | Layout or delimiter depth exceeds 256 | Layout 或 delimiter 嵌套超过 256 层 | layout or delimiter nesting exceeds 256 levels | `maximum_depth:integer` | — | `0.0.1-dev` |
| `L-SYNTAX-0004` | `SYNTAX` | `Preview` | `Error` | 结束 delimiter 没有开始项 | Closing delimiter has no opening delimiter | 存在没有对应开始符号的结束 delimiter | closing delimiter has no matching opening delimiter | `found:string` | — | `0.0.1-dev` |
| `L-SYNTAX-0005` | `SYNTAX` | `Preview` | `Error` | 开始/结束 delimiter 种类不匹配 | Opening and closing delimiter kinds differ | 结束 delimiter 与开始 delimiter 不匹配 | closing delimiter does not match the opening delimiter | `expected:string, found:string` | — | `0.0.1-dev` |
| `L-SYNTAX-0006` | `SYNTAX` | `Preview` | `Error` | Delimiter 在 EOF 前未闭合 | Delimiter is not closed before EOF | delimiter 在文件结尾前未闭合 | delimiter is not closed before the end of the file | `expected:string` | — | `0.0.1-dev` |
| `L-SYNTAX-0010` | `SYNTAX` | `Preview` | `Error` | Parser 遇到不符合当前 grammar 的 token | Parser encounters a token outside the current grammar | 语法错误：需要 `{expected}` | syntax error: expected `{expected}` | `context:string, expected:string, found:string` | — | `0.0.1-dev` |
| `L-SYNTAX-0011` | `SYNTAX` | `Preview` | `Error` | Parser recursion 超过 512 层 | Parser recursion exceeds 512 levels | 语法嵌套超过 512 层 | syntax nesting exceeds 512 levels | `maximum_depth:integer` | — | `0.0.1-dev` |
| `L-NAME-0001` | `NAME` | `Preview` | `Error` | 引用无法解析 | Reference cannot be resolved | 未定义名称“{name}” | undefined name `{name}` | — | — | `0.0.1-dev` |
| `L-NAME-0002` | `NAME` | `Preview` | `Error` | 同一作用域重复定义 | Duplicate definition in one scope | 名称“{name}”在同一作用域中重复定义 | name `{name}` is defined more than once in the same scope | `previous_name?:string` | — | `0.0.1-dev` |
| `L-NAME-0003` | `NAME` | `Preview` | `Error` | module 声明或 Seed module 边界非法 | Invalid module declaration or Seed module boundary | module 规则无效 | invalid module rule | `actual_module?:string, committed?:boolean, expected_module?:string` | — | `0.0.1-dev` |
| `L-NAME-0004` | `NAME` | `Preview` | `Error` | import alias 重复 | Duplicate import alias | import 别名“{alias}”重复 | import alias `{alias}` is duplicated | — | — | `0.0.1-dev` |
| `L-NAME-0005` | `NAME` | `Preview` | `Error` | Seed import graph 包含 cycle | Seed import graph contains a cycle | Seed 不允许 import cycle | Ling Seed rejects import cycles | — | — | `0.0.1-dev` |
| `L-NAME-0006` | `NAME` | `Preview` | `Error` | 同一作用域存在 UTS #39 confusable collision | One scope contains a UTS #39 confusable collision | 名称视觉混淆 | names are confusable in the same scope | `first?:string, second?:string` | — | `0.0.1-dev` |
| `L-NAME-0007` | `NAME` | `Preview` | `Error` | module scope 重定义保留名称 | Module scope redefines a reserved name | 模块作用域不能重定义保留名称“{name}” | module scope cannot redefine reserved name `{name}` | — | — | `0.0.1-dev` |
| `L-NAME-0008` | `NAME` | `Preview` | `Error` | import module 或精确大小写路径不存在 | Imported module or exact-case path is absent | 找不到 import 模块“{module}” | imported module `{module}` was not found | `module?:string` | — | `0.0.1-dev` |
| `L-NAME-0009` | `NAME` | `Preview` | `Error` | 标识符包含 Seed 默认禁止的 Latin/Cyrillic 或 Latin/Greek 可疑混写 | Identifier contains a suspicious Latin/Cyrillic or Latin/Greek mix rejected by Seed | 标识符包含可疑混合文字 | identifier contains a suspicious script mix | `name:string, scripts:string[]` | — | `0.0.1-dev` |
| `L-TYPE-0001` | `TYPE` | `Preview` | `Error` | 类型无法统一或表达式不可调用 | Types cannot unify or an expression is not callable | 类型不匹配 | type mismatch | `generalization?:string, restriction_reason?:string` | — | `0.0.1-dev` |
| `L-TYPE-0002` | `TYPE` | `Preview` | `Error` | occurs check 检测到无限类型 | Occurs check detects an infinite type | 类型推导产生无限类型 | type inference produced an infinite type | — | — | `0.0.1-dev` |
| `L-TYPE-0003` | `TYPE` | `Preview` | `Error` | 函数参数数量不匹配 | Function argument count mismatch | 参数数量不匹配 | argument count mismatch | — | — | `0.0.1-dev` |
| `L-TYPE-0004` | `TYPE` | `Preview` | `Error` | nominal record 字段不存在 | Nominal record field is absent | record 中不存在字段“{field}” | record has no field named `{field}` | `field:string` | — | `0.0.1-dev` |
| `L-TYPE-0005` | `TYPE` | `Preview` | `Error` | 字段集合无法唯一确定 nominal record | Field set does not identify one nominal record | 无法唯一确定 nominal record 类型 | record fields do not identify one nominal record type | — | — | `0.0.1-dev` |
| `L-TYPE-0006` | `TYPE` | `Preview` | `Error` | match 非穷尽 | Match is non-exhaustive | match 非穷尽 | match is non-exhaustive | `witness:string` | — | `0.0.1-dev` |
| `L-TYPE-0007` | `TYPE` | `Preview` | `Warning` | match 分支被前序无 guard 分支覆盖 | Match case is covered by an earlier unguarded case | match 分支不可达 | match case is unreachable | `reason:string` | — | `0.0.1-dev` |
| `L-TYPE-0008` | `TYPE` | `Preview` | `Error` | record 构造或更新包含重复字段 | Record construction or update contains a duplicate field | record 字段重复 | record field is duplicated | `field:string` | — | `0.0.1-dev` |
| `L-TYPE-0009` | `TYPE` | `Preview` | `Error` | record literal 缺少 nominal 类型要求的字段 | Record literal omits fields required by its nominal type | record 缺少字段 | record is missing fields | `fields:string` | — | `0.0.1-dev` |
| `L-TYPE-0010` | `TYPE` | `Preview` | `Error` | constructor pattern 参数数量错误 | Constructor pattern has the wrong arity | constructor 模式参数数量不匹配 | constructor pattern has the wrong arity | `actual_arity:integer, constructor:string, expected_arity:integer` | — | `0.0.1-dev` |
| `L-TYPE-0011` | `TYPE` | `Preview` | `Error` | 类型不支持 Seed 相等性比较 | Type does not support Seed equality | 类型 `{type}` 不支持相等性比较 | type `{type}` does not support equality | `type:string` | — | `0.0.1-dev` |
| `L-MUT-0001` | `MUT` | `Preview` | `Error` | 赋值目标不是合法 mutable Place | Assignment target is not a legal mutable Place | 赋值左侧不可修改 | assignment target is not mutable | `field?:string, mutability?:string, reason?:string` | — | `0.0.1-dev` |
| `L-CAP-0001` | `CAP` | `Preview` | `Error` | module 缺少所需 Capability | Module lacks a required Capability | 模块缺少 Capability 声明“{capability}” | module is missing required capability `{capability}` | — | — | `0.0.1-dev` |
| `L-CAP-0002` | `CAP` | `Preview` | `Error` | Capability 不属于 Seed | Capability is outside Seed | Seed 不支持 Capability“{capability}” | Ling Seed does not support capability `{capability}` | — | — | `0.0.1-dev` |
| `L-CAP-0003` | `CAP` | `Preview` | `Warning` | module 声明了未使用的 Capability | Module declares an unused Capability | 模块声明了未使用的 Capability | module declares an unused capability | — | — | `0.0.1-dev` |
| `L-ENTRY-0001` | `ENTRY` | `Preview` | `Error` | run 入口 module 不是 Main | Run entry module is not Main | run 入口模块必须是 Main | the run entry module must be `Main` | — | — | `0.0.1-dev` |
| `L-ENTRY-0002` | `ENTRY` | `Preview` | `Error` | Main 缺少 main | Main has no main | Main 模块缺少 main 定义 | module `Main` does not define `main` | — | — | `0.0.1-dev` |
| `L-ENTRY-0003` | `ENTRY` | `Preview` | `Error` | main 签名或参数 pattern 非法 | Invalid main signature or parameter pattern | main 必须具有 Unit -> Unit 和 Unit pattern | `main` must have Unit -> Unit and a Unit pattern | — | — | `0.0.1-dev` |
| `L-AUDIT-0001` | `AUDIT` | `Preview` | `Error` | Audit Source token、字符串或数字语法非法 | Invalid Audit Source token, string, or number syntax | Audit Source 语法无效 | invalid Audit Source syntax | `audit_schema:string` | — | `0.0.1-dev` |
| `L-AUDIT-0002` | `AUDIT` | `Preview` | `Error` | Audit Source 包含未知核心字段或不兼容版本 | Audit Source contains an unknown core field or incompatible version | Audit Source 字段或版本不兼容 | incompatible Audit Source field or version | `audit_schema:string` | — | `0.0.1-dev` |
| `L-AUDIT-0003` | `AUDIT` | `Preview` | `Error` | Audit Source 缺少或重复结构字段 | Audit Source has a missing or duplicate structural field | Audit Source 结构字段缺失或重复 | missing or duplicate Audit Source structural field | `audit_schema:string` | — | `0.0.1-dev` |
| `L-AUDIT-0004` | `AUDIT` | `Preview` | `Error` | Audit model 违反结构或引用不变量 | Audit model violates structural or reference invariants | Audit model 不满足语义不变量 | Audit model violates semantic invariants | `audit_schema:string` | — | `0.0.1-dev` |
| `L-RUNTIME-0001` | `RUNTIME` | `Preview` | `Error` | Checked Core 求值或宿主 Capability 发生 Runtime Fault | Checked Core or host Capability raises a Runtime Fault | 运行时 Fault | runtime Fault | `category:string, committed?:boolean, operation?:string` | — | `0.0.1-dev` |
| `L-INTERNAL-0001` | `INTERNAL` | `Preview` | `Error` | 编译器内部不变量失败；不是用户程序错误 | A compiler invariant failed; this is not a user-program error | 内部编译器错误；事件 ID：`{incident_id}`；重现信息：`{reproduction}` | internal compiler error; incident ID: `{incident_id}`; reproduction: `{reproduction}` | `incident_id:string, reproduction?:string, reproduction_error?:string, stage:string` | — | `0.0.1-dev` |
| `L-SNAPSHOT-0001` | `SNAPSHOT` | `Preview` | `Error` | Canonical Semantic Graph 无法通过独立 reader round-trip | Canonical Semantic Graph fails an independent reader round-trip | Semantic Graph 快照验证失败 | Semantic Graph snapshot validation failed | `committed?:boolean, detail:string` | — | `0.0.1-dev` |

## Retired allocations / 退役分配

退役项保留原 code、根因含义、严重级别和 payload 类型；不得重新出现在 canonical Rust constants 或 emitter 中。

Retired entries preserve their code, root-cause meaning, severity, and payload types. They must not reappear in canonical Rust constants or emitters.

| Code | Phase | Stability | Severity | 中文标题 | English title | 中文模板 | English template | Payload schema | Repair schema | Since |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `L-IMPL-0001` | `IMPL` | `Deprecated` | `Error` | 所请求路径依赖尚未实现的编译阶段（已废弃/保留） | Requested path requires an unimplemented compiler stage (deprecated/reserved) | `{command}` 命令所需的编译阶段尚未实现 | the compiler stage required by `{command}` is not implemented yet | `command:string, completed_stage:string, had_bom?:boolean, source_name?:string, token_count?:integer, unicode_version?:string` | — | `0.0.1-dev` |

## 兼容性边界 / Compatibility boundary

- 稳定：`code` 的含义、`severity` 的错误/警告分类，以及现有 Facts 字段的类型。
- 可兼容扩展：新增可选 Facts、新增 Repair 候选、改进中英文措辞。
- 非兼容变更：改变 code 含义、删除既有 Facts、改变字段类型或把 error 降为 warning。此类变更必须分配新 code，或升级 Diagnostic Schema 并提供迁移说明。

- Stable: the meaning of `code`, error/warning classification, and types of existing Facts.
- Compatible extension: adding optional Facts or Repair candidates and improving localized wording.
- Breaking: changing code meaning, removing existing Facts, changing field types, or downgrading an error to a warning. Such changes require a new code or a Diagnostic Schema version with migration guidance.

## Validation and consumers / 验证与消费方

- 中英文模板的 `{parameter}` 集合必须完全相同。自然语言措辞和标点不是 byte-for-byte 协议，测试只固定 schema、code、severity、payload 类型、结构化 Repair 和原始 UTF-8 byte span 等必要字段。
- 当前 wire type 名为 `Repair`。非空 `Repair schema` 必须至少声明 `kind:string, changes_semantics:boolean`；不得用自然语言建议替代结构化 repair，也不得在没有 Accepted schema 变更时另造 `FixPlan` wire type。
- CLI human、CLI JSON、REPL JSON 共享同一个 `DiagnosticCode`。未来 LSP adapter 必须把同一个字符串原样放入 LSP `Diagnostic.code`；本表不伪造尚未实现的 LSP 能力。
- `error-code-lock.toml` 固定根因标题、phase、severity、首个版本、payload 类型、retired 状态和每个 domain 的 high-water mark。已有 code 只能增加可选 payload 字段或转为 retired；新 code 必须单调超过该 domain 的 high-water mark。
- 运行 `cargo xtask governance check-error-codes` 检查重复分配、翻译/参数、payload/repair schema、Rust constants、源码与 conformance 中的未注册/退役 code，以及 compatibility-lock drift。合法增加 code 或可选 Facts 后，使用 `cargo xtask governance render-error-code-lock` 生成新 lock；该命令拒绝改义、删除/改型既有 Facts、重新激活 retired code 和回填旧号码。

- Chinese and English templates must have exactly the same `{parameter}` set. Natural-language wording and punctuation are not byte-for-byte protocol; tests freeze only necessary fields such as schema, code, severity, payload types, structured Repairs, and original UTF-8 byte spans.
- The current wire type is named `Repair`. A non-empty `Repair schema` must declare at least `kind:string, changes_semantics:boolean`; natural-language advice cannot replace a structured repair, and an unaccepted `FixPlan` wire type must not be invented.
- CLI human output, CLI JSON, and REPL JSON share one `DiagnosticCode`. A future LSP adapter must copy that exact string into LSP `Diagnostic.code`; this registry does not pretend that an LSP implementation already exists.
- `error-code-lock.toml` freezes root-cause titles, phase, severity, first version, payload types, retired state, and each domain high-water mark. An existing code may only gain optional payload fields or become retired; a new code must monotonically advance its domain high-water mark.
- Run `cargo xtask governance check-error-codes` to validate duplicate allocation, translations/parameters, payload/repair schemas, Rust constants, unregistered/retired codes in source and conformance fixtures, and compatibility-lock drift. After a valid new code or optional Fact is added, use `cargo xtask governance render-error-code-lock` to generate the new lock; the renderer rejects changed meanings, removed/retyped existing Facts, retired-code reactivation, and number backfilling.
