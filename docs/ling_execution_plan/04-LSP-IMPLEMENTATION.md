# Ling Language Server（`zero lsp`）详细实施计划

> 目标版本：v0.1 Living  
> 协议基线：LSP 3.17；具体能力以客户端 capability negotiation 为准。  
> 架构原则：`ling-lsp` 只做协议适配，所有语言分析来自 `ling-ide` 与共享 CompilerHost。

## 1. 成功定义

启动：

```bash
zero lsp --stdio
```

最小完成能力：

- 增量打开/修改/关闭文档；
- parser、resolve、type、effect、capability diagnostics；
- document symbols、hover、definition；
- references、prepareRename/rename；
- completion、code action；
- document formatting；
- semantic tokens（可先 Preview）；
- workspace reload、cancellation、stale result 拒绝；
- 中文标识符、emoji、组合字符、CRLF 的位置正确。

## 2. 分层

```text
LSP JSON-RPC
    ↓
ling-lsp
  - initialize/capability negotiation
  - stdio transport
  - request cancellation
  - URI/FileId mapping
  - PositionEncoding conversion
    ↓
ling-ide
  - diagnostics
  - symbols/hover/navigation
  - rename/completion/actions/tokens
    ↓
AnalysisSnapshot
    ↓
ling-db + CompilerHost + Checked Core + Semantic Graph
```

禁止 `ling-lsp`：

- 自己解析 `.ling`；
- 用文本搜索做 definition/rename；
- 复制类型显示逻辑；
- 直接访问磁盘绕过 VFS；
- 用 LSP line/column 作为 compiler span；
- 在 request handler 中修改全局 checker 状态而没有 revision。

## 3. 目标 crate

```text
crates/ling-ide/
  src/
    analysis.rs
    diagnostics.rs
    symbols.rs
    hover.rs
    navigation.rs
    references.rs
    rename.rs
    completion.rs
    actions.rs
    semantic_tokens.rs
    formatting.rs
    line_index.rs

crates/ling-lsp/
  src/
    main_loop.rs
    capabilities.rs
    state.rs
    documents.rs
    conversions.rs
    handlers/
    logging.rs
```

`ling-ide` 不依赖任何 LSP 类型。创建内部 `TextRange/FilePosition/WorkspaceEdit/FixPlan`，在 `ling-lsp` 中转换。

## 4. 位置编码与 Unicode

### 4.1 内部权威

```text
FileId + UTF-8 byte span
```

LSP adapter 根据客户端协商的位置编码转换。最低支持：

- UTF-16 code units；
- UTF-8 code units。

若客户端未明确协商，按 LSP 兼容默认处理，但实现必须记录选定编码并由 fixture 验证。

### 4.2 `LineIndex`

提供：

```rust
byte_to_position(offset, encoding)
position_to_byte(position, encoding)
range_to_lsp(span, encoding)
lsp_to_range(range, encoding)
```

失败条件：

- line 越界；
- column 越界；
- 落在 UTF-8 code point 或 UTF-16 surrogate pair 中间；
- 文档版本不匹配。

不得静默 clamp；对客户端非法 position 返回 protocol error，对内部非法 span 返回 `I` 类内部诊断并记录 bug。

### 4.3 必测文本

```ling
let 人物 = "关羽"
let 表情 = "😀"
let 组合 = "e\u{301}"
人物.血量 = 100
```

测试每个 token 前后位置、诊断 range、rename edit、semantic token delta。

## 5. 文档同步与 VFS

### LSP-2101：初始化与生命周期

**规模：S。**

实现：initialize、initialized、shutdown、exit；server info、capability negotiation、workspace folders。

验收：无 initialize 前 request 明确拒绝；shutdown 后不处理普通请求；stdio 不输出非协议内容。

### LSP-2102：Position encoding negotiation

**规模：S；依赖：LineIndex。**

记录 client/server 共同支持编码；所有 handler 只通过 conversion API。

### LSP-2103：Open document overlay

**规模：M。**

状态：

```text
URI → FileId
FileId → {version, bytes, open/closed}
```

规则：

- `didOpen` 内容覆盖磁盘；
- `didChange` 检查单调 version；
- `didClose` 回到磁盘或移除临时文件；
- dependency/readonly file 不接受编辑；
- 保存不是语义必需事件，内容更新以 change 为准。

### LSP-2104：增量文本变更

先允许 Full sync 作为最小可用；增量 query 稳定后实现 Incremental sync。

实现 Incremental 时：

- edit ranges 按 negotiated encoding 转 bytes；
- 一批 changes 按协议顺序应用；
- edit 后重建/增量更新 LineIndex；
- 越界或版本倒退拒绝，不污染 VFS；
- property test 与 full replacement 结果相同。

### LSP-2105：Workspace reload

监听或响应 manifest/source changes。配置、lock、dependency 改变触发 project graph revision；不把每个文件事件都当全量重建。

## 6. Diagnostics

### LSP-2201：Compiler diagnostic adapter

映射：

| Ling | LSP |
| --- | --- |
| stable code | `Diagnostic.code` |
| bilingual rendered message | `message` |
| primary byte span | `range` |
| secondary labels | `relatedInformation` 或 message labels |
| severity | `severity` |
| retired/deprecated | tags（适用时） |
| structured fix IDs | `data`（版本化 Experimental） |

错误码文档可以通过 `codeDescription` 指向稳定文档页面；在离线环境仍必须有完整 message。

### LSP-2202：Push diagnostics v0

G1 首版使用 `publishDiagnostics`：

- open file edit 后 debounce；
- syntax diagnostics 可快速发布；
- workspace type diagnostics 在新 snapshot 完成后发布；
- 发布携带 document version（客户端支持时）；
- 新结果覆盖旧结果；
- close 时清除临时 diagnostics。

### LSP-2203：Pull diagnostics Preview

若客户端声明支持，再提供 LSP 3.17 pull diagnostics；push/pull 不得产生不同错误集合。未验证 Zed 支持前不把 pull 设为唯一通道。

### LSP-2204：Root-cause 与错误风暴控制

- parser error recovery 限制级联；
- 同一根因合并重复 diagnostics；
- Trait/solver resource limit 产生一个明确诊断；
- workspace error cap 可配置，但必须报告“已省略数量”；
- diagnostics 稳定排序：logical path、byte offset、code、tie-breaker。

### LSP-2205：Diagnostic fixtures

使用纯 JSON-RPC transcript：

```text
initialize
open Chinese source
didChange before emoji
expect code/range/message
fix source
expect cleared diagnostics
```

不依赖 Zed，确保协议层可独立测试。

## 7. IDE 查询能力

### IDE-2301：Document symbols

从 resolved/checked definitions 输出：module、type、variant、function、field、trait、impl；range 与 selection range 分离。嵌套定义按源码顺序稳定。

### IDE-2302：Hover

显示：

- 名称与规范化签名；
- inferred type；
- Effect/Capability；
- selected Trait constraints；
- documentation；
- resource/borrow facts（未来）；
- stability/profile availability。

Hover renderer 是 IDE 公共服务，CLI `zero query hover` 可复用。

### IDE-2303：Go to definition/declaration/type definition

基于 `ResolvedRef → DefinitionId → SourceOrigin`。跨 package dependency 返回只读位置；generated/primitive 定义返回虚拟文档或文档 URI，需 decision。

### IDE-2304：References

建立 definition-reference index；区分 read/write/call/type/implementation relation 供后续 semantic tokens 和 rename 使用。索引可增量更新。

### IDE-2305：Prepare rename

拒绝：

- keyword/builtin 不可重命名；
- generated definition；
- dependency readonly；
- confusable/非法新名称；
- 当前 snapshot 有阻断解析歧义；
- rename 将违反 visibility/coherence。

返回精确 identifier range 与 placeholder。

### IDE-2306：Rename

基于 identity，不做文本全局替换。

步骤：

1. resolve target；
2. 校验新名称 Unicode/NFC/confusable；
3. 收集 definitions/references/import aliases；
4. 模拟 apply 到临时 snapshot；
5. resolve/type/check；
6. 若 identity/behavior preserve 条件失败则拒绝；
7. 返回版本化 Workspace Edit。

必须检查 open documents version；支持 stale edit 拒绝。

### IDE-2307：Completion v0

上下文：

- expression start；
- member access；
- type position；
- pattern/variant；
- import/module；
- keyword。

排序基于作用域接近、类型适配、显式 import、稳定字典序。不得使用 HashMap 顺序。首版不做 AI completion。

### IDE-2308：Completion resolve

延迟加载 documentation、full signature、Effect/Capability。插入文本必须尊重 Unicode 名称与 formatter。

### IDE-2309：Code actions

只消费结构化 `FixPlan`：

- import missing symbol；
- rename confusable；
- make binding/field mutable（必须语义允许）；
- add missing match cases；
- replace stale syntax；
- apply formatter。

每个 action 有 kind、diagnostic code、snapshot/version precondition。禁止从错误 message 文本解析修复。

### IDE-2310：Formatting

调用 `ling-fmt`。支持 document formatting；range formatting 只有在边界语义确定并有 tests 后启用。

### IDE-2311：Workspace symbols

增量 symbol index；结果带 package/module context；限制最大结果并支持 query cancellation。

## 8. Semantic Tokens

### LSP-2401：Token taxonomy RFC/decision

优先使用 LSP 标准 token types：

```text
namespace/type/class/enum/enumMember
function/method/property/variable/parameter
keyword/string/number/operator/comment
```

Ling 自定义 token（仅必要时）：

```text
effect
capability
resource
actor
node
kernel
semanticId
```

modifiers：

```text
declaration/definition/readonly/static/deprecated
mutable/borrowed/unsafeBoundary/generated
```

Zed extension 提供 custom mapping；标准客户端忽略未知自定义类型时仍有 Tree-sitter 基础高亮。

### LSP-2402：Typed token generation

从 Checked Core/Resolved HIR 生成，不依赖 Tree-sitter。对有语法错误区域可回退到已解析 token，但必须标明来源，不伪造解析成功。

### LSP-2403：Full 与 delta

先实现 full；性能证据需要后实现 delta。token 必须按 position 排序、不重叠、在同一 document version 上生成。

### LSP-2404：Semantic token fixtures

测试中文列、emoji 前缀、同名不同 scope、mutable field、variant constructor、Effect/Capability、error recovery。

## 9. Cancellation、并发与快照

### LSP-2501：Request snapshot

每个 request 捕获 immutable `AnalysisSnapshot` 与 document version。长任务不持有 host write lock。

### LSP-2502：Cancellation

- 响应 `$/cancelRequest`；
- solver/index/rename/completion 定期检查 cancellation token；
- 被取消请求不发布部分 Workspace Edit；
- compiler query 可返回 Cancelled，不缓存半成品。

### LSP-2503：Debounce 与优先级

- typing diagnostics 高优先；
- workspace index 低优先；
- 新 revision 取消旧分析；
- definition/hover 不等待无关全 workspace build；
- actor/native 未来 feature 不改变工具调度可观察语义。

### LSP-2504：Memory/resource limits

限制 open document bytes、pending requests、completion results、diagnostic count、solver work。超限返回稳定 tool diagnostic，不 OOM。

## 10. 配置

建议初始化 options 或 workspace config：

```json
{
  "ling": {
    "profile": "Explore",
    "diagnosticLanguage": "zh-CN",
    "semanticTokens": true,
    "checkOnChange": true,
    "checkDependencies": false
  }
}
```

配置必须进入 analysis revision；未知字段按协议策略处理。不得允许配置改变核心语言语义而不进入 ProgramSnapshot。

## 11. 日志与隐私

- stdio 只输出 JSON-RPC；日志写 stderr 或文件；
- 默认不记录源码内容、标识符和完整路径；
- request timing 可记录 method、duration、revision、cancelled；
- crash report 对 source/path 做脱敏；
- `--trace-protocol` 必须显式启用并警告可能含源码。

## 12. 测试矩阵

### 单元

- LineIndex；
- URI/FileId；
- diagnostic adapter；
- rename simulation；
- token encoding；
- completion ordering。

### 协议 transcript

- lifecycle；
- full/incremental sync；
- stale version；
- cancellation；
- workspace reload；
- Unicode positions；
- malformed client request。

### 差分

- CLI `zero check --format json` 与 LSP diagnostics code/span 集合一致；
- `zero fmt` 与 LSP formatting 一致；
- `zero query definition` 与 LSP definition 一致；
- clean vs incremental LSP results 一致。

### 压力

- 连续快速编辑；
- 大 workspace；
- 大错误文件；
- cancellation storm；
- manifest/lock 反复变化；
- solver 病态输入。

## 13. 建议验收命令

```bash
cargo test -p ling-ide
cargo test -p ling-lsp
cargo xtask lsp-fixtures --all-encodings
cargo xtask lsp-differential
cargo xtask incremental-equivalence --consumer lsp
cargo fuzz run lsp_message_decoder
zero lsp --stdio < tests/protocols/smoke.input
```

## 14. 分阶段交付

| 阶段 | 能力 | 稳定性 |
| --- | --- | --- |
| LSP-0 | lifecycle、full sync、push diagnostics | Preview |
| LSP-1 | symbols、hover、definition | Preview |
| LSP-2 | references、rename、completion、actions、format | Preview→Stable 候选 |
| LSP-3 | semantic tokens、workspace symbols、incremental sync | Preview |
| LSP-4 | call hierarchy/inlay hints/advanced query | 1.0 非阻断，按证据纳入 |

G1 出口至少要求 LSP-0～LSP-2；LSP-3 中 semantic tokens 可保持 Preview，但 Zed 必须始终有 Tree-sitter 基础高亮。
