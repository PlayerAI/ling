# 参考资料与设计依据

> 本文件记录本执行计划采用的规范基线和外部工程资料。外部资料用于决定实现方式与工具集成，不替代 Ling 自身 Accepted RFC / `SEMANTICS.md`。

## 1. Ling 项目基线

1. `ROADMAP-1.0.md`：G0～G6 发布顺序、目标、门禁与立即执行批次。
2. `LANGUAGE.md`：语言设计总纲、人类表达层和长期能力范围。
3. `SEMANTICS.md`：当前核心语义与待 RFC 问题。
4. `RFC-0001.md`：`v0.0.1 Seed` 实现和验收范围。
5. `decisions/`：已接受的局部决策。

发生冲突时，遵守项目规定的规范权威顺序；本执行计划不新增语义。

## 2. Zed 官方资料

### Language Extensions

- <https://zed.dev/docs/extensions/languages>

用于：

- `languages/<language>/config.toml`；
- `extension.toml` 中固定 Tree-sitter grammar Git revision；
- `highlights.scm`、`brackets.scm`、`indents.scm`、`outline.scm`、`textobjects.scm`、`runnables.scm` 等 query；
- language server registration；
- semantic tokens 的 `off / combined / full` 模式；
- `semantic_token_rules.json`。

### Developing Extensions

- <https://zed.dev/docs/extensions/developing-extensions>

用于：

- Zed extension 仓库/manifest；
- 本地安装开发扩展；
- Rust extension 编译到 `wasm32-wasip2`；
- grammar 开发所需环境。

### Debugger Extensions

- <https://zed.dev/docs/extensions/debugger-extensions>
- <https://zed.dev/docs/debugger>

用于后续 `zero dap --stdio` 与 Zed debugger registration；DAP 在 G3 前不进入基础扩展门禁。

### Tasks and Snippets

- <https://zed.dev/docs/tasks>
- <https://zed.dev/docs/extensions/snippets>

用于 `zero check/run/test/fmt/audit/replay/evidence` 的任务模板和 Ling snippets。

### Publishing Extensions

- <https://zed.dev/docs/extensions/publishing-extensions>
- <https://zed.dev/docs/extensions/extension-capabilities>

用于：

- 扩展发布元数据和许可证；
- language server 不直接嵌入扩展，而是发现本地 binary 或下载独立 release；
- 扩展运行能力边界。

## 3. Tree-sitter 官方资料

- <https://tree-sitter.github.io/tree-sitter/creating-parsers/>
- <https://tree-sitter.github.io/tree-sitter/creating-parsers/3-writing-the-grammar.html>
- <https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests.html>
- <https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html>

实施原则：

- 先建立 declarations/types/patterns/expressions 等宽度优先 grammar skeleton；
- CST 节点与用户可识别语言结构保持直观对应；
- 尽量符合 LR(1)，谨慎使用 conflict/external scanner；
- 每条 rule 都进入 `test/corpus`；
- Tree-sitter 为编辑器容错解析，不成为 Ling 合法性权威。

## 4. Language Server Protocol

- LSP 3.17：<https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/>

本计划以 3.17 能力为基线：

- initialize/capability negotiation；
- text synchronization；
- diagnostics；
- document/workspace symbols；
- hover/definition/references；
- rename/prepareRename；
- completion/resolve；
- code actions；
- formatting；
- semantic tokens；
- cancellation/progress。

内部 span 使用原始 UTF-8 bytes；协议边界集中转换为 LSP position encoding，不在 compiler core 中混用。

## 5. OpenAI Codex 官方资料

### Codex CLI

- <https://developers.openai.com/codex/cli>

用于本地 inspect/edit/run 与项目初始化。

### AGENTS.md

- <https://developers.openai.com/codex/agent-configuration/agents-md>
- <https://developers.openai.com/codex/learn/best-practices>
- <https://developers.openai.com/codex/customization/overview>

实施原则：

- 根 `AGENTS.md` 保持简洁、持久、仓库级；
- 目录级规则靠更近的 `AGENTS.md`；
- build/test/review 命令明确；
- 重复错误转化为可执行规则和 CI，而非不断扩大 prompt。

### Worktrees

- <https://developers.openai.com/codex/environments/git-worktrees>

用于独立分支并行任务，避免多个 agent 同时改写本地 checkout。

### Subagents

- <https://developers.openai.com/codex/subagents>

用于把规范审计、测试设计、实现和只读审查拆成边界明确的子任务；读密集和独立工作适合并行，公共接口写入必须由单一集成者控制。

### Configuration

- <https://developers.openai.com/codex/config-reference>

`templates/codex-config.toml.example` 仅为结构示意；使用前必须按已安装 Codex 版本复核字段。

### Programmatic workflows

- <https://developers.openai.com/codex/mcp-server>

只有在手工任务流稳定后，才考虑 Codex MCP Server / SDK 自动编排；不先自动化不可靠流程。

## 6. 外部资料的使用边界

- Zed、Tree-sitter、LSP 资料决定协议和插件实现形态，不决定 Ling 源语言语义；
- Codex 资料决定工作流，不决定规范优先级；
- 外部工具的新版本可能改变字段/API，开发时应锁定版本并重新检查官方文档；
- 若官方工具行为与本计划不同，修改执行文档或适配层，不应偷偷改变 Ling 语义。
