# Ling implementation status / Ling 实现状态

> Generated deterministically from `implementation-status.toml`; do not edit manually.
> 本文由 `implementation-status.toml` 确定性生成；不得手工编辑。

- Registry schema: `2`
- Feature schema: `1`
- Updated: `2026-08-21`
- Feature release: `v0.0.1`
- Baseline release: `v0.0.1`
- Baseline commit: `639790f4c609d137932d8432d9c5be681aa3e3c1`

## Feature state / 功能状态

| Feature | Title / 标题 | Current | Stability | I/T/D | Stabilization blockers | Profiles | Targets | Last verified |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `FTR-SEED-0001` | 检查后执行与入口 / Checked execution and entry point | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0002` | 检查、结构化诊断与稳定退出 / Checking, structured diagnostics, and stable exits | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0003` | Unicode 17 与中文标识符完整性 / Unicode 17 and Chinese identifier integrity | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0004` | Seed 类型、模式与 Place / Seed types, patterns, and Place | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0005` | Effect 与 Capability 检查 / Effect and Capability checking | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0006` | Semantic Graph、Canonical ID 与 Audit / Semantic Graph, canonical identity, and Audit | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001`, `GAP-SEMANTIC-HASH-LIFECYCLE-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `FTR-SEED-0007` | 共享管线、离线构建与稳定性门禁 / Shared pipeline, offline build, and stability gates | `Implemented` | `Experimental` | `yes/yes/yes` | `GAP-GOV-RFC-STATUS-001` | — | — | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |

`I/T/D` means implemented/tested/documented. Empty Profile and target cells are intentional: the current Seed interpreter is unprofiled, and no Ling Native target is supported.
`I/T/D` 表示已实现/已测试/已文档化。Profile 与 target 为空是有意的：当前 Seed 解释器未启用 Profile，且没有受支持的 Ling Native target。

## Task state / 任务状态

| Task | Title | Release | Size | State | Dependencies | Completion commit |
| --- | --- | --- | --- | --- | --- | --- |
| `BASE-0001` | 仓库基线盘点与执行计划落位 | `G0` | `S` | `Done` | — | `aa8c02894bd2fdd696ab60c97423d07c0ce9614a` |
| `GOV-0101` | 建立规范权威索引 | `G0` | `S` | `Done` | `BASE-0001` | `7bba2adf9104d7d7f96c7ef50343647f649e229e` |
| `GOV-0102` | 规范缺口台账 | `G0` | `M` | `Done` | `GOV-0101` | `c147b5c02532b61e23df46f6cb25251d8c94dd7d` |
| `GOV-0103` | RFC 与 decision 生命周期 | `G0` | `S` | `Done` | `GOV-0101` | `4876a0328d994121fb32c10b3f2a25e3ce11e5ff` |
| `GOV-0104` | 公开接口与协议总盘点 | `G0` | `M` | `Done` | `GOV-0101` | `508ae4db327e5815d56093f5b5b107c916732904` |
| `GOV-0105` | Diagnostic 错误码注册表 | `G0` | `M` | `Done` | `GOV-0104` | `7f4452b9c5c629b02f2cfc810529e797dff805b6` |
| `GOV-0106` | Schema 生命周期与 golden corpus | `G0` | `L` | `Done` | `GOV-0104` | `35ba0da126be6b2df8ee3fe2aa9fd3ca27ebbdec` |
| `GOV-0107` | 统一追踪矩阵 | `G0` | `M` | `Done` | `GOV-0102`, `GOV-0104` | `dcfc75a78333bc7ed8e020985d5419b1c21bc789` |
| `GOV-0108` | 1.0 支持矩阵草案 | `G0` | `M` | `Done` | `GOV-0102`, `GOV-0104`, `GOV-0107` | `258e2d1e46c286a9e7e937b0bb65f3d19ed5e8d3` |
| `GOV-0109` | 发布状态机器可读化 | `G0` | `S` | `Done` | `GOV-0108` | `695e40eb6310ba1dcb36580f5feb63b4301ab656` |
| `GOV-0110` | G0 CI 门禁 | `G0` | `M` | `Done` | `GOV-0101`, `GOV-0102`, `GOV-0103`, `GOV-0104`, `GOV-0105`, `GOV-0106`, `GOV-0107`, `GOV-0108`, `GOV-0109` | `a7f2b03270f0e4a163cd8f927cd56475c5062daf` |
| `PRJ-1101` | Minimal project manifest | `G1` | `M` | `Done` | — | `80f46bcf3d175eeb6402bf6267085cb905a5dbcf` |
| `PRJ-1102` | Deterministic module discovery | `G1` | `M` | `Done` | `PRJ-1101` | `f76f4953070b9ae555fce24c7dcc2fbf08a36f7a` |
| `PRJ-1103` | Package-aware imports and visibility | `G1` | `M` | `Done` | `PRJ-1102`, `PRJ-1104` | `8e98d32f54301dee3f198273cfae3146bbf2846b` |
| `PRJ-1104` | Content-identified local dependency graph | `G1` | `L` | `Done` | `PRJ-1101`, `PRJ-1102` | `66a64c9b57c8bb327599a7463345c9d2fbe77a51` |
| `PRJ-1105` | Canonical project lockfile protocol | `G1` | `L` | `Done` | `PRJ-1104` | `9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082` |
| `PRJ-1106` | End-to-end project fixture matrix | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105` | `0e9c5800411a6f1acd1441068e6ce2fd58f29816` |
| `PRJ-1108` | Project graph property and manifest fuzz coverage | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105`, `PRJ-1106` | `29f9c4465b58c7eff23c227436563d69409b880e` |
| `TEST-VM-0001` | VM failing-first corpus and differential harness baseline | `G1` | `M` | `Done` | `GOV-0104`, `GOV-0105` | `5bd49583c9160cd2067a7124bc014ebc3b4bcf95` |
| `TS-3101` | Grammar 规范映射表 | `G1` | `S` | `Done` | `BASE-0001` | `4d1b643bd1a971bcd01d101cd81411557d3c3074` |
| `TS-3102` | 宽度优先 Tree-sitter grammar skeleton | `G1` | `M` | `Done` | `TS-3101` | `14fb7986501abda6eed178b5b7af405fcb0313e9` |
| `TS-3103` | Offside/缩进策略 | `G1` | `M` | `Done` | `TS-3102` | `28750bcbd458322e856cf45842b8241047a8e41b` |
| `TS-3104` | Unicode identifier | `G1` | `M` | `Done` | `TS-3103` | `16e61caf1340611c4752196b47da2973aca6978b` |
| `TS-3105` | Expression precedence | `G1` | `M` | `Done` | `TS-3104` | `cf76a4268b5ec8d5cdd939749709cc0654cff732` |
| `TS-3106` | Pattern 与 Type | `G1` | `M` | `Done` | `TS-3105` | `7948a17a7848c32078b3893b6c9182ab7c41096b` |
| `TS-3107` | Error recovery | `G1` | `M` | `Done` | `TS-3106` | `1debda6d69796182d2b051bd5b5b03992008a1ca` |
| `TS-3108` | Grammar differential | `G1` | `M` | `Done` | `TS-3107` | `c90dc6209ab90a7b7e4c8b0056c164a13821dff0` |
| `VM-1201` | Portable bytecode RFC and unverified model | `G1` | `M` | `Done` | `GOV-0104`, `GOV-0105`, `TEST-VM-0001` | `5bd49583c9160cd2067a7124bc014ebc3b4bcf95` |
| `VM-1202` | Checked Core to deterministic bytecode minimal lowering | `G1` | `L` | `Done` | `TEST-VM-0001`, `VM-1201` | `4fb3f2dc0046cfaa52da6b6db94573044d5ee183` |
| `VM-1203` | Independent bounded bytecode decoder and verifier | `G1` | `L` | `Done` | `TEST-VM-0001`, `VM-1201`, `VM-1202` | `e08940ef511cbcb1416e4b32e0c0805601d5c160` |
| `VM-1204` | Verifier-gated deterministic bytecode execution | `G1` | `L` | `Done` | `TEST-VM-0001`, `VM-1201`, `VM-1202`, `VM-1203` | `dfe2df79bcee020a30e178a568c8921b04aea346` |
| `ZQ-3201` | Syntax highlighting queries | `G1` | `S` | `Done` | `TS-3108` | `77aab24ff8160e1535ea15b67d5302c1a4bb3fc8` |
| `ZQ-3202` | Bracket matching queries | `G1` | `S` | `Done` | `ZQ-3201` | `1106b323685ed4910e6580a4347dce47df466208` |
| `ZQ-3203` | Automatic indentation queries | `G1` | `S` | `Done` | `ZQ-3202` | `a4377450d26374098d95a9bb38520d3e3552dfd7` |

## Generated consumers / 生成视图

- Release-note fragment: [`docs/status/release-status.md`](release-status.md)
- Internal CLI fixture: `tests/fixtures/status/feature-state.governance.json` (`ling.governance.feature-state-fixture/1`, `implemented: false`)
- Proposed command: `ling support --format json` (not implemented)

```text
cargo xtask status verify
cargo xtask status render
cargo xtask status render-release-notes
cargo xtask status render-cli-fixture
```
