# Ling 仓库与编译器架构执行设计

> 目标：为 G0～G6 提供不会重复语义、适合增量/LSP/多后端的 Rust 工程边界。  
> 原则：所有执行和工具路径共享同一个 Checked Typed Core 与 ProgramSnapshot。

## 1. 推荐仓库结构

```text
ling/
├── AGENTS.md
├── Cargo.toml
├── rust-toolchain.toml
├── LANGUAGE.md
├── SEMANTICS.md
├── ROADMAP-1.0.md
├── RFC-0001.md
├── rfcs/
├── decisions/
├── schemas/
│   ├── diagnostic/
│   ├── semantic-graph/
│   ├── audit/
│   ├── bytecode/
│   ├── replay/
│   └── evidence/
├── crates/
│   ├── ling-source/
│   ├── ling-unicode/
│   ├── ling-syntax/
│   ├── ling-ast/
│   ├── ling-hir/
│   ├── ling-resolve/
│   ├── ling-types/
│   ├── ling-effects/
│   ├── ling-core-ir/
│   ├── ling-semantic/
│   ├── ling-diagnostics/
│   ├── ling-project/
│   ├── ling-db/
│   ├── ling-ide/
│   ├── ling-eval/
│   ├── ling-bytecode/
│   ├── ling-vm/
│   ├── ling-fmt/
│   ├── ling-lsp/
│   ├── ling-cli/
│   └── ling-test-support/
├── conformance/
│   ├── source/
│   ├── parser/
│   ├── resolve/
│   ├── type/
│   ├── effects/
│   ├── semantic/
│   ├── runtime/
│   └── diagnostics/
├── tests/
│   ├── differential/
│   ├── incremental/
│   ├── protocols/
│   ├── unicode/
│   └── offline/
├── fuzz/
├── examples/
├── docs/
│   ├── execution/
│   ├── traceability/
│   ├── status/
│   └── compatibility/
├── editors/
│   └── zed/                 # 可作为 git submodule 或独立仓库开发镜像
└── tools/
    ├── schema-check/
    ├── corpus-runner/
    ├── release-manifest/
    └── xtask/
```

Tree-sitter grammar 推荐独立仓库 `tree-sitter-ling`，Zed 扩展推荐独立仓库 `zed-ling`。主仓库通过固定 commit 和共享 corpus 对它们做集成测试。

## 2. 稳定的前端管线

```text
SourceBytes
  ↓ ling-source / ling-unicode
TokenStream + Trivia + raw byte spans
  ↓ ling-syntax
CST
  ↓ ling-ast
AST
  ↓ ling-hir + ling-resolve
Resolved HIR
  ↓ ling-types + ling-effects
Checked Typed Core
  ↓ ling-semantic
ProgramSnapshot + Semantic Graph + Audit model
```

### 2.1 强制不变量

- Source 层保留原始 bytes、换行映射和 BOM 信息；
- CST 保留 trivia 与错误恢复节点；
- AST 只表示语法结构，不含未解析字符串引用；
- HIR 中所有引用均解析为 symbol/definition identity；
- Checked Core 中所有表达式都有已解类型、Effect、Capability 要求与规范化 Fault 信息；
- Semantic Graph 从 Checked Core 派生，不重新推断语义；
- Interpreter、VM、Native 和 IDE 都不读取未检查 AST 来决定行为。

## 3. Crate 职责

### `ling-source`

负责：`FileId`、`SourceFile`、byte span、line endings、VFS entry、source map 基础类型。

禁止：Unicode 名称语义、parser、filesystem 全局访问。

### `ling-unicode`

负责：固定 Unicode 版本表、XID、NFC、安全/混淆检测、Script Set。

输出必须可复现；生成表需要版本 manifest 与测试向量。

### `ling-syntax`

负责：lexer、offside/indent tokens、error-tolerant parser、CST、syntax errors。

必须支持不完整源码；不得做名称解析和类型推断。

### `ling-ast`

负责：typed AST wrapper、trivia-preserving navigation、AST lowering diagnostics。

### `ling-hir`

负责：desugaring、scope-ready HIR、stable local IDs、source origin。

### `ling-resolve`

负责：module/scope/name resolution、visibility、resolved refs、duplicate/ambiguous diagnostics。

### `ling-types`

负责：type representation、unification、schemes、value restriction、Trait constraints（G1 后期）。

### `ling-effects`

负责：Effect row、Capability requirements、Fault/effect facts。G1 前只实现已接受基础集合，G2 扩展 Handler/polymorphism。

### `ling-core-ir`

负责：唯一 Checked Typed Core。包含显式 evaluation order、resolved references、types、effects、places/borrows、match decision、fault path。

这是执行后端与 IDE 的语义边界，变更需要高强度兼容审查。

### `ling-semantic`

负责：Semantic Graph、Canonical Bytes、Semantic ID、Audit model、Semantic Transaction validation。

不得访问 Rust 地址、HashMap iteration 或主机路径生成 canonical 数据。

### `ling-diagnostics`

负责：稳定错误码、双语消息模板、labels、related spans、FixPlan、JSON Schema adapter。

核心错误对象保存结构化参数，不在 checker 内拼最终字符串。

### `ling-project`

负责：manifest、module/package graph、lock、dependency identity、offline resolution、build graph。

### `ling-db`

负责：增量 query、revision、dependency tracking、cache serialization（如接受）、snapshot/cancellation。

不得把增量实现细节泄露为语言可观察行为。

### `ling-ide`

负责：基于 compiler snapshot 的 hover、definition、references、rename、completion、symbols、semantic tokens、code actions、inlay hints。

不包含 JSON-RPC；可以被 LSP、测试工具和未来其他编辑器复用。

### `ling-eval`

参考解释器。优先清晰、可观察语义和 differential oracle，不追求极限性能。

### `ling-bytecode`

负责：版本化 bytecode model、encoder/decoder、verifier、disassembler。bytecode 必须从 Checked Core lowering。

### `ling-vm`

负责：执行已验证 bytecode、Fault 映射、资源上限、调试 hooks。不得修补 verifier 未保证的非法程序。

### `ling-fmt`

负责：Author Source formatter、doc model、comment attachment。Audit canonical renderer 留在 `ling-semantic` 或独立 `ling-audit`。

### `ling-lsp`

只负责 LSP transport、capability negotiation、request cancellation、position conversion 和调用 `ling-ide`。

### `ling-cli`

二进制名 `zero`。只编排服务：`check/run/test/fmt/semantic/audit/query/patch/build/lsp`。

### `ling-test-support`

统一 fixture loader、golden updater（显式开关）、diagnostic matcher、snapshot normalization、differential harness。

## 4. 依赖方向

推荐的核心依赖 DAG：

```text
source ← unicode
  ↑
syntax → ast → hir → resolve → types/effects → core-ir → semantic
                          │                    │
                          └──── diagnostics ───┘

project → db → compiler-service → ide → lsp
                         │
                         ├→ eval
                         ├→ bytecode → vm
                         ├→ fmt
                         └→ cli
```

实际 Cargo 依赖应进一步拆细，但必须满足：

- `syntax` 不依赖 `types`；
- `types` 不依赖 `lsp/cli/vm`；
- `semantic` 不依赖后端；
- `ide` 不依赖具体编辑器；
- `lsp` 不依赖 Zed；
- `vm` 不依赖 parser；
- Zed extension 不依赖 compiler crates，只启动外部 `zero lsp`。

## 5. Compiler Service

建立稳定 facade，避免 CLI、REPL、LSP 分别拼管线：

```rust
pub struct CompilerHost { /* db, vfs, config, toolchain */ }
pub struct AnalysisSnapshot { /* immutable revision */ }
pub struct ProgramSnapshot { /* checked identity */ }

impl CompilerHost {
    pub fn set_file(&mut self, file: FileId, bytes: Arc<[u8]>, version: FileVersion);
    pub fn remove_file(&mut self, file: FileId);
    pub fn check(&self, target: Target) -> CheckResult;
    pub fn snapshot(&self) -> AnalysisSnapshot;
}
```

实际 API 可调整，但必须具备：

- open document overlay；
- immutable snapshot；
- cancellation；
- deterministic diagnostics；
- explicit profile/target/toolchain inputs；
- 无 ambient current directory / environment。

## 6. VFS 与文件身份

文件身份不得直接等于绝对路径字符串。

```text
FileId
  ↔ LogicalPath(package, module, relative_path)
  ↔ OptionalPhysicalPath
```

规则：

- canonical output 使用 logical path 或 content identity；
- diagnostics 可显示用户路径，但 Schema 区分 display path 与 identity；
- LSP 使用 URI ↔ FileId map；
- open buffer bytes 优先于磁盘；
- dependency files 默认只读；
- 路径大小写规则由 target/workspace manifest 明确。

## 7. LineIndex 与位置转换

内部唯一权威：原始 UTF-8 byte offset。

`LineIndex` 至少提供：

```text
byte offset ↔ (line, UTF-8 column)
byte offset ↔ (line, UTF-16 code unit column)
byte span   ↔ LSP Range(position encoding)
```

必须测试：

- 中文 BMP 字符；
- supplementary plane emoji；
- combining marks；
- CRLF；
- 非法 UTF-8（在进入 LSP 前应成为 source diagnostic）；
- 增量编辑后 line index 更新；
- span 落在字符中间时返回内部错误而非截断。

## 8. Diagnostic 架构

核心错误：

```text
Diagnostic {
  code,
  severity,
  primary_span,
  labels[],
  message_key,
  args,
  related[],
  fixes[],
  provenance,
}
```

渲染 adapter：

- CLI human Chinese/English；
- CLI JSON；
- LSP `Diagnostic`；
- Zed 通过 LSP 展示；
- Audit/evidence 引用稳定 code 与结构化字段。

禁止 checker 直接返回只包含字符串的错误。

## 9. Semantic Graph 与 Schema

每个 Schema 都有：

```text
schema_name
schema_version
producer_version
language_version
unicode_version
stability
```

Canonical writer 与普通 JSON writer 分离：

- 普通 JSON 为可读/协议；
- Canonical Bytes 为 identity；
- reader 对 unknown fields 的行为由版本策略定义；
- Hash 算法升级必须创建新 ID scheme，不静默重算。

## 10. Formatter 与 parser 的关系

Formatter 必须使用 compiler CST 和 comment attachment：

```text
Source → CST(error tolerant) → Format IR → text
```

禁止：

- regex formatter；
- 维护第二套 parser；
- 为了格式化而类型检查；
- 丢失文档注释或中文名称；
- 把 Author Source 强制渲染成 Audit Source。

## 11. LSP/IDE 分层

```text
JSON-RPC / LSP
     ↓
ling-lsp adapter
     ↓
ling-ide semantic operations
     ↓
AnalysisSnapshot
     ↓
compiler db / Checked Core / Semantic Graph
```

LSP server 不得直接遍历 parser AST 进行 rename、definition 或类型 hover。

## 12. 编辑器仓库

### `tree-sitter-ling`

```text
grammar.js
src/
queries/                  # 可供多编辑器共享的基础 queries
bindings/
test/corpus/
test/highlight/
examples/
scripts/sync-corpus.*
```

### `zed-ling`

```text
extension.toml
Cargo.toml
src/lib.rs
languages/ling/
  config.toml
  highlights.scm
  brackets.scm
  indents.scm
  outline.scm
  textobjects.scm
  runnables.scm
  overrides.scm
  injections.scm          # 有真实需求后启用
  redactions.scm          # 有真实需求后启用
  semantic_token_rules.json
snippets/ling.json
README.md
LICENSE
```

## 13. 后续扩展 crate（到门禁后才创建）

- G2：`ling-task-runtime`、`ling-actor-runtime`、`ling-replay`；
- G3：`ling-memory-check`、`ling-native-ir`、`ling-backend-<first>`、`ling-ffi`；
- G4：`ling-kernel-ir`、`ling-backend-simd`、`ling-backend-<gpu>`；
- G5：`ling-critical`、`ling-contracts`、`ling-model-check`、`ling-evidence`。

不要在 v0.1 建空 crate 或占位公开 API“预留未来”。

## 14. 架构验收命令（建议）

```bash
cargo xtask check-layering
cargo xtask check-diagnostic-registry
cargo xtask check-schema-versions
cargo xtask conformance --profile seed
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

`check-layering` 应解析 `cargo metadata`，拒绝违反声明 DAG 的依赖。
