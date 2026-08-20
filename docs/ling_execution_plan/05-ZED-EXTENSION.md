# Zed 的 Ling 语言扩展详细开发计划

> 扩展名称：Ling  
> 建议扩展仓库：`zed-ling`  
> 建议 grammar 仓库：`tree-sitter-ling`  
> 语言服务器：`zero lsp --stdio`  
> 目标：从 v0.1 开始提供可靠的语法高亮、错误提示、跳转、重命名、补全、格式化和运行任务；后续加入调试器。

## 1. 设计边界

Zed 支持分为两条互补路径：

```text
Tree-sitter
  → 即时 CST、基础高亮、括号、缩进、outline、textobjects、runnables

zero lsp
  → parser/type/effect/capability 错误、hover、definition、references、rename、completion、code action、semantic tokens
```

原则：

- Tree-sitter grammar 不是 Ling 合法性权威；
- Zed extension 不嵌入 checker；
- LSP 挂掉时仍保留基础编辑体验；
- compiler parser、Tree-sitter 和 formatter 共享 corpus；
- 任何新关键字/语法先进入 Accepted RFC，再同步 grammar/query；
- 中文标识符和原始 byte span 是一等测试对象。

## 2. Zed 官方扩展约束对本项目的影响

当前 Zed 语言扩展结构要求：

- 根目录 `extension.toml`；
- `languages/<language>/config.toml`；
- Tree-sitter grammar 在 `extension.toml` 中注册并固定 Git revision；
- 高亮等功能由 `.scm` query 提供；
- 需要启动 language server 时，扩展使用 Rust + Zed Extension API，编译到 `wasm32-wasip2`；
- 发布扩展不应直接打包 language server，而应查找用户环境中的二进制或下载独立 release。

因此采用：

```text
zed-ling extension Wasm
    └─ 查找/下载 zero
          └─ zero lsp --stdio
```

## 3. 仓库结构

### 3.1 `tree-sitter-ling`

```text
tree-sitter-ling/
├── grammar.js
├── package.json
├── tree-sitter.json
├── src/
├── bindings/
├── queries/
│   └── highlights.scm       # 可共享基础版
├── test/
│   ├── corpus/
│   │   ├── declarations.txt
│   │   ├── expressions.txt
│   │   ├── patterns.txt
│   │   ├── unicode.txt
│   │   ├── errors.txt
│   │   └── future-keywords.txt
│   └── highlight/
├── examples/
└── scripts/
    └── sync-compiler-corpus.*
```

### 3.2 `zed-ling`

```text
zed-ling/
├── extension.toml
├── Cargo.toml
├── src/lib.rs
├── languages/ling/
│   ├── config.toml
│   ├── highlights.scm
│   ├── brackets.scm
│   ├── indents.scm
│   ├── outline.scm
│   ├── textobjects.scm
│   ├── runnables.scm
│   ├── overrides.scm
│   ├── injections.scm
│   ├── redactions.scm
│   └── semantic_token_rules.json
├── snippets/ling.json
├── tests/
├── README.md
└── LICENSE
```

## 4. Manifest 草案

`extension.toml`（字段版本以开发时最新 Zed 文档/API 为准）：

```toml
id = "ling"
name = "Ling"
version = "0.0.1"
schema_version = 1
authors = ["Ling Contributors"]
description = "Ling language support for Zed"
repository = "https://github.com/<org>/zed-ling"
snippets = ["./snippets/ling.json"]

[grammars.ling]
repository = "https://github.com/<org>/tree-sitter-ling"
rev = "<PINNED_COMMIT_SHA>"

[language_servers.ling-lsp]
name = "Ling Language Server"
languages = ["Ling"]

[language_servers.ling-lsp.language_ids]
"Ling" = "ling"
```

`languages/ling/config.toml`：

```toml
name = "Ling"
grammar = "ling"
path_suffixes = ["ling"]
line_comments = ["// "]
tab_size = 4
hard_tabs = false
```

若项目最终允许 shebang，可增加 `first_line_pattern`；在语义未接受前不要预设。

## 5. 版本路线

| 扩展阶段 | 对应语言 | 能力 |
| --- | --- | --- |
| Z0 Grammar | Seed/G0 | 文件识别、基础高亮、括号、outline、缩进 |
| Z1 Living | v0.1 | LSP diagnostics、hover、definition、rename、completion、format、tasks |
| Z2 Concurrent | v0.2 | task/actor/supervisor/replay 语法与语义 token、运行命令 |
| Z3 Native | v0.3 | resource/borrow 高亮、Native task、DAP Preview |
| Z4 Heterogeneous | v0.4 | kernel/device/placement 高亮与 device diagnostics |
| Z5 Critical | v0.5 | node/contract/profile、证明状态与 evidence navigation |
| Z6 Stable | v1.0 | 扩展发布、兼容矩阵、自动 LS 获取、稳定 settings |

# 6. Tree-sitter grammar 实施

## TS-3101：Grammar 规范映射表

**规模：S。**

创建 `docs/grammar-map.md`：

| Accepted syntax | Compiler node | Tree-sitter node | Corpus |
| --- | --- | --- | --- |

任何 grammar rule 必须映射到已接受语法或明确的 error-recovery helper。Tree-sitter helper 不进入语言规范。

## TS-3102：宽度优先 grammar skeleton

先建立：

```text
source_file
declaration
type_declaration
function_definition
module/import
expression
pattern
type
identifier
literal
comment
```

保持 CST 直观；避免照搬规范中所有 precedence 层产生深树。表达式用明确 precedence/associativity。

验收：每个顶层语法类别至少一个 corpus test。

## TS-3103：Offside/缩进策略

Ling Author Source 使用 offside rule。Tree-sitter 需要 external scanner 或等价布局 token 方案时：

- scanner 只发 `indent/dedent/newline` 等布局 token；
- tab 的语义错误仍由 compiler diagnostic 权威判定；
- scanner 必须支持错误恢复和增量重解析；
- scanner state 可序列化；
- CRLF、空行、注释行、括号内换行、文件末尾 dedent 全覆盖。

先写 corpus 和 scanner state tests，再实现。

## TS-3104：Unicode identifier

目标是尽量匹配固定 XID/NFC 词法，但 Tree-sitter 不是安全检查器。

实施顺序：

1. 评估 grammar regex 对 Unicode XID 的覆盖；
2. 优先从 Ling 的 Unicode 17.0.0 生成表产生 grammar range 或 external scanner；
3. 建立 compiler lexer 与 Tree-sitter token differential corpus；
4. 对 Tree-sitter 宽松接受、compiler 拒绝的字符，确保 LSP 能显示权威错误；
5. 不在 query 层尝试安全/Confusable 检查。

## TS-3105：Expression precedence

覆盖：function application、member access、unary、arithmetic、comparison、equality、boolean、pipeline、assignment（仅 place context）。

每个 precedence pair 有正向 CST fixture；故意歧义必须显式处理，不依赖生成器偶然选择。

## TS-3106：Pattern 与 Type

ADT pattern、record pattern（若已接受）、tuple、wildcard、literal、guard；type application、generic、function type。未进入 v0.1 的 syntax 只保留 future keyword，不创建假节点。

## TS-3107：Error recovery

必须覆盖：

- 未闭合 string/record/tuple；
- 缺 `=`、`->`、`with`；
- 半输入中文标识符；
- 不完整 pipeline；
- 错误缩进；
- 编辑中间状态。

目标不是判定合法，而是保持周边定义可高亮/outline。

## TS-3108：Grammar differential

对 compiler conformance corpus：

- 合法程序应无 Tree-sitter `ERROR/MISSING`（允许明确白名单）；
- 非法程序的 Tree-sitter 必须终止并生成有限树；
- 随机 edit 不崩溃；
- CST node mapping 稳定。

# 7. Zed Tree-sitter queries

## ZQ-3201：`highlights.scm`

基础 captures：

```text
@keyword                let/type/match/with/if/then/else/module/import/mutable/...
@type.builtin           Unit/Bool/Int/Text/...
@type                   type declaration/reference
@constructor            ADT constructor
@function               function reference
@function.definition    定义（若主题不支持则 fallback）
@variable.parameter
@variable
@property                    record field/member
@string / @string.escape
@number / @boolean
@operator
@comment / @comment.doc
@punctuation.bracket / delimiter
```

要求：

- 中文 identifier 可获得与 ASCII 相同结构高亮；
- 语法层无法区分局部变量/类型引用时使用保守 capture，交给 semantic tokens 精化；
- 每个 capture 有 highlight fixture；
- 不为美观添加无语义依据的 keyword。

## ZQ-3202：`brackets.scm`

覆盖 `() [] {}` 和 string quotes；嵌套块注释是否参与 bracket matching 先测试再决定。对 string quote 可禁用 rainbow。

## ZQ-3203：`indents.scm`

以 CST 节点为单位，保守支持：

- function/type/match/if/module body；
- record/tuple/list；
- match arm；
- pipeline continuation。

规则不应与 formatter 竞争。Zed 自动缩进只为编辑辅助，保存时仍由 `zero fmt` 权威格式化。

## ZQ-3204：`outline.scm`

v0.1：module、type、variant、function、trait、impl。

后续：task、actor、node、kernel、contract。`@name` 只捕获名称，`@item` 捕获完整定义，`@context` 显示 module/type。

## ZQ-3205：`textobjects.scm`

- `@function.around/inside`：顶层函数与 method-like impl member；
- `@class.around/inside`：module/type/trait/actor 等大段结构；
- `@comment.around/inside`。

闭包默认不当作顶层 function text object，除非用户研究证明需要。

## ZQ-3206：`runnables.scm`

v0.1 可检测：

- 文件/项目入口（若语法有明确 main）；
- 测试定义（只有 test syntax 已接受后）；
- example block（若有）。

`@run` capture 生成 gutter action；实际命令由 Zed task 调 `zero run/test`，query 不决定语义。

## ZQ-3207：`overrides.scm`

用于 string/comment scope 的 word/completion behavior 和 bracket auto-close。中文标识符不需要额外 word character hack；若 Zed selection 行为有问题，先写 fixture/issue 再配置。

## ZQ-3208：`injections.scm`

首版保持空或只支持已规范化的 doc Markdown。不要凭设想把 SQL/JSON 注入任意字符串。每种 injection 必须有真实 syntax marker。

## ZQ-3209：`redactions.scm`

默认不对普通 Ling 代码自动隐藏。未来可对显式 secret literal/schema marker 做 redaction，但必须有语言或标准库约定。

# 8. Zed extension Rust 实施

## ZEXT-3301：Grammar-only dev extension

**规模：S；依赖：TS skeleton。**

- 创建 manifest/config/queries；
- 使用 `file://` grammar repo 进行本地开发；
- 安装为 Dev Extension；
- 检查 Zed log；
- 验证 `.ling` 识别、highlight、outline、indent。

此阶段无需 Rust code。

## ZEXT-3302：Extension Wasm 骨架

**规模：S。**

- Rust crate `cdylib`；
- 使用开发时最新兼容 `zed_extension_api`；
- target `wasm32-wasip2`；
- 实现 Extension trait 与注册宏；
- stdout/stderr 只用于调试；
- platform 通过 Zed API 查询，不依赖 `std::env` 假设。

## ZEXT-3303：查找本地 `zero`

优先级：

1. 用户扩展设置显式路径（若 Zed API 支持并接受）；
2. worktree/PATH 中 `zero` / Windows `zero.exe`；
3. extension-managed 下载缓存（发布阶段）；
4. 返回可操作错误，说明安装命令与日志位置。

启动参数：

```text
zero lsp --stdio
```

不要从当前文件路径推导 workspace；LSP 根据 root/workspace folders 处理。

## ZEXT-3304：Language server command

在 `extension.toml` 注册 server，在 Extension trait 的 `language_server_command` 返回 command/args/env。

验收：

- 正确启动一次；
- 退出后 Zed 能重启；
- 找不到 binary 有清晰错误；
- 路径包含空格；
- Windows/macOS/Linux；
- extension log 不泄露源码。

## ZEXT-3305：Release 下载（Z6 前）

Zed 发布规则不允许直接把 language server 二进制打包进扩展，因此：

- 从官方 Ling release 获取平台 artifact；
- 选择 host OS/arch；
- 下载 manifest + checksum；
- 验证 hash/signature（按 Ling release policy）；
- 解压到 extension cache；
- 版本与 extension 兼容矩阵；
- 离线时优先已缓存；
- 网络失败不破坏 grammar-only 能力。

下载 URL 与 release schema 不能散落硬编码，集中在 adapter。

# 9. 错误提示与语义功能

## ZED-3401：Diagnostics smoke

测试：

```ling
let 人物 =
    { 姓名 = "关羽"
      mutable 血量 = 100 }

人物.蓝量 = 30
```

期望：

- 未知字段错误范围只覆盖 `蓝量`；
- code 可见；
-中英文 message 按设置；
- related info 指向 `人物` 类型定义；
- quick fix 只在存在结构化 FixPlan 时出现。

## ZED-3402：Hover/definition/references

- 中文符号 hover；
- 跨模块 definition；
- dependency readonly reference；
- outline 与 LSP symbol 大体一致；
- parser 错误附近仍能提供不误导的部分信息。

## ZED-3403：Rename

- ASCII→中文、中文→ASCII；
- NFC 等价；
- Confusable 拒绝；
- multi-file Workspace Edit；
- stale document version；
- import alias；
- readonly dependency。

## ZED-3404：Completion

- scope、member、variant、type、import；
- 中文候选；
- completion documentation；
- snippets 与 LSP completion 不重复污染；
- large workspace latency baseline。

## ZED-3405：Code action

确认 Zed 展示：missing import、add match cases、rename confusable、format。action 应携带 error code，并在 stale snapshot 时重新计算或拒绝。

## ZED-3406：Semantic tokens

Zed 当前可配置：

```json
{
  "languages": {
    "Ling": {
      "semantic_tokens": "combined"
    }
  }
}
```

扩展提供 `semantic_token_rules.json`，把 Ling 自定义 token 映射到 Zed style。由于 semantic tokens 可能默认关闭，Tree-sitter highlights 必须独立完整。

建议规则概念：

```json
[
  { "token_type": "effect", "style": ["keyword"] },
  { "token_type": "capability", "style": ["type"] },
  { "token_type": "actor", "style": ["type"] },
  { "token_type": "resource", "style": ["type"] },
  { "token_modifiers": ["mutable"], "underline": true }
]
```

具体 style 名称以发布时 Zed 支持列表为准，并有 fallback capture。

# 10. Tasks、Runnables 与 Formatter

## ZED-3501：项目 tasks 模板

提供 `.zed/tasks.json` 示例：

```json
[
  {
    "label": "Ling: check project",
    "command": "zero",
    "args": ["check", "--manifest-path", "$ZED_WORKTREE_ROOT/ling.toml"]
  },
  {
    "label": "Ling: run current file",
    "command": "zero",
    "args": ["run", "$ZED_FILE"]
  },
  {
    "label": "Ling: test project",
    "command": "zero",
    "args": ["test", "--manifest-path", "$ZED_WORKTREE_ROOT/ling.toml"]
  }
]
```

命令和 manifest 名称以最终 decision 为准。参数用数组避免路径空格问题。

## ZED-3502：Runnables query

将 `@run` captures 与 task tag 对齐；测试 gutter action 在 main/test 上出现，不在普通 helper 上出现。

## ZED-3503：Format on save

首选 LSP document formatting；用户可配置 external `zero fmt` fallback。扩展 README 给出设置，不强制覆盖用户全局设置。

## ZED-3504：Snippets

只提供稳定语法的低数量 snippets：function、type/ADT、match、module、test（若接受）。中文变量不硬编码，snippet placeholder 允许用户输入中文。

# 11. Debugger 路线（v0.3 Preview）

Zed 支持 Debug Adapter Protocol 和 debugger extension，但 Ling 只有在以下前置完成后才开始：

- VM/Native source map 稳定；
- runtime breakpoint/step/stack/variables protocol；
- ProgramSnapshot 与 binary identity；
- Fault 与 exception categories；
- Accepted debugger RFC。

## DAP-3601：`zero dap --stdio`

实现独立 DAP adapter；Zed extension 只启动它。先支持 Explore VM，再支持 Native backend。

## DAP-3602：Zed debugger registration

- 在 language config 声明 debugger；
- extension 注册 adapter；
- launch/attach 配置；
- build task 先执行 `zero build`；
- debug locator 把 Ling run task 转为 scenario。

## DAP-3603：能力阶段

1. launch、breakpoint、continue；
2. step in/over/out；
3. stack trace、scopes、variables；
4. conditional breakpoint/logpoint；
5. attach（仅 runtime 支持后）；
6. actor/task views（未来自定义）。

DAP 不阻断 v0.1；在 G3 之前不提供虚假按钮或占位 adapter。

# 12. 测试策略

## 12.1 Grammar/query 自动测试

```bash
tree-sitter generate
tree-sitter test
tree-sitter parse examples/*.ling
```

另外运行共享 corpus differential。

## 12.2 Extension build

```bash
cargo check --target wasm32-wasip2
cargo test
```

具体构建命令随 Zed extension tooling 调整。

## 12.3 Zed 手工 smoke checklist

- [ ] 安装 Dev Extension；
- [ ] `.ling` 自动识别；
- [ ] 中文高亮；
- [ ] bracket/indent/outline/textobject；
- [ ] LSP 启动；
- [ ] parser/type error；
- [ ] hover/definition/references；
- [ ] rename/completion/action；
- [ ] format；
- [ ] tasks/runnables；
- [ ] Zed restart；
- [ ] LSP crash/restart；
- [ ] 无 `zero` 时 grammar-only 与可操作提示。

## 12.4 跨平台矩阵

至少：

```text
Windows x86_64
Linux x86_64
macOS arm64
```

其他平台标 Tier 2/Experimental。测试 PATH、空格路径、非 ASCII 用户目录、CRLF 和 executable permission。

# 13. 发布流程

1. tree-sitter-ling 发布固定 commit/tag；
2. zed-ling manifest 更新 exact rev；
3. Dev Extension 全平台 smoke；
4. license 满足 Zed registry 要求；
5. README 声明所需 `zero` 版本和支持矩阵；
6. 提交 Zed extension registry PR；
7. 版本更新保持 extension 与 LS compatibility；
8. grammar-only 回退和离线已缓存 LS 继续工作。

# 14. Zed 阶段出口

### Z0 出口

- [ ] grammar corpus；
- [ ] highlights/brackets/indents/outline/textobjects；
- [ ] Dev Extension 可安装；
- [ ] 中文和错误恢复；
- [ ] grammar commit pinned。

### Z1 出口

- [ ] `zero lsp` 自动启动；
- [ ] diagnostics/hover/definition/references/rename/completion/action/format；
- [ ] tasks/runnables/snippets；
- [ ] Unicode position fixtures；
- [ ] 无 LS 时基础高亮仍可用。

### Z6 公开稳定出口

- [ ] 官方 registry；
- [ ] LS 查找/下载/checksum/cache；
- [ ] extension/API compatibility matrix；
- [ ] 三平台 CI/smoke；
- [ ] license/release/provenance；
- [ ] DAP 只按真实支持状态标 Preview/Stable。
