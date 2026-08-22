# Ling 1.0 总实施 Backlog

> 用途：Codex 本地执行的统一任务索引  
> 状态定义：`Ready / In Progress / BlockedSpec / BlockedDependency / Review / Done`  
> 更新规则：任务状态以仓库 `docs/status/implementation-status.toml` 为机器权威；本文用于人类浏览

## 1. 使用规则

1. 每次只领取一个 `Ready` 的 XS/S/M 任务；L/XL 必须先拆分；
2. 领取前阅读来源文档的完整任务段落，不以本表摘要替代；
3. 规范未决时改为 `BlockedSpec`，创建 spec-gap；
4. 同一公共接口只允许一个写任务处于 `In Progress`；
5. 完成后更新状态、traceability、测试证据和下一批依赖；
6. 不提前实现未来阶段，以免倒逼前序语义。

## 2. 第一批可执行队列

下面是从 Seed 基线进入 v0.1 的建议领取顺序。

| 顺序 | Task | 目标 | 前置 | 可并行 | 完成门禁 |
| ---: | --- | --- | --- | --- | --- |
| 0 | `BASE-0001` | 盘点 Seed 基线并接入执行计划 | Seed 文档齐全 | 否 | **Done**；见机器状态与基线盘点 |
| 1 | `GOV-0101` | 建规范权威索引 | Seed 文档齐全 | 与 2–5 | **Done**；33 文档、16 Accepted，检查器与负例通过 |
| 2 | `GOV-0102` | 建规范缺口台账 | 无 | 与 1/3/4 | **Done**；25 Open gaps、6 个 G1 门禁、marker 检查通过 |
| 3 | `GOV-0104` | 盘点全部公开协议 | 无 | 与 1/2 | **Done**；18 项协议边界、版本/稳定级别与 checker 已落地 |
| 4 | `GOV-0105` | 建错误码注册表 | Seed diagnostics | 与 1/2/3 | **Done**；55 active、1 retired，重复/改义/改型/回填/未注册 code CI 拒绝 |
| 5 | `GOV-0106` | 建 Schema 生命周期与 golden corpus | 公开协议清单 | 与 6–8 | **Done**；3 schemas、4 valid、6 invalid、1 canonical、23 corrupt mutations，N-1 明确为 `NoPreviousVersion` |
| 6 | `GOV-0107` | 建 traceability 模板 | 规范索引 | 与 3/4 | **Done**；7 features、32 fixtures、44 evidence，链接/极性/differential/report drift 由 CI 检查 |
| 7 | `GOV-0108` | 建 1.0 支持矩阵草案 | 缺口、协议与追踪注册表 | 与 8 | **Done**；7 features、3 profiles、3 hosts、1 target、6 backends、1 std package、19 protocols、9 unsupported，生成物 drift 由 CI 检查 |
| 8 | `GOV-0109` | Feature state 机器化 | 支持矩阵草案 | 与 7 | **Done**；9 tasks、7 features，docs/release-note/internal CLI views 与 registry drift 由 CI 检查 |
| 9 | `GOV-0110` | 收敛 G0 CI 门禁 | 前述 G0 tasks | 否 | **Done**；8 named gates、19 commands、3 test hosts、Seed 4 surfaces/8 processes |
| 10 | `TS-3101` | 语法→Tree-sitter 映射 | Accepted Seed 语法 | 与 G0 | **Done**；52 syntax rows、8 private helpers、8 deferred groups，均映射 compiler/corpus 边界 |
| 11 | `TS-3102` | Tree-sitter 宽度优先骨架 | TS-3101 | 与架构工作 | **Done**；60 named CST nodes 全部有 corpus evidence，23/23 cases 与本地示例通过，生成物幂等 |
| 12 | `PRJ-1101` | 最小项目 manifest | 包/模块 RFC | 否 | **Done**；`ling.manifest/1` reader、7 个稳定诊断、fixtures/limits/mutation/fuzz evidence |
| 13 | `VM-1201` | bytecode RFC/模型 | VM 语义 RFC | 与 test agent | **Done**；Accepted RFC-0014、未验证 data model、显式 tag/opcode/limits 与 TEST-VM-0001 corpus |
| 14 | `INC-1401` | 增量 query ADR | 编译管线接口 | 与 corpus | query/invalidations 评审通过 |
| 15 | `LSP-2101` | LSP 生命周期骨架 | CompilerSession/VFS | 与 Zed query | initialize/shutdown fixtures |
| 16 | `LSP-2102` | UTF-16 position 协商 | LineIndex | 与 diagnostics | 中文/emoji/CRLF fixtures |
| 17 | `ZEXT-3301` | grammar-only Zed 扩展 | TS skeleton | 与 LSP | 本地 Zed 识别/高亮 |
| 18 | `VM-1202` | Core→bytecode 最小切片 | VM-1201 | 与 verifier | **Done**；Hello Checked Core → 精确 bytes/disassembly golden；decode round-trip 属 VM-1203 |
| 19 | `VM-1203` | 独立 decoder/verifier | VM-1201 | 与 lowering | **Done**；bounded decoder/verifier、22 个 malformed vectors、arbitrary-byte/fuzz boundary；VM-1204 承接执行 |
| 20 | `PRJ-1102` | module discovery | PRJ-1101 | 与 VM | deterministic graph |
| 21 | `INC-1402` | VFS/revision | INC-1401 | 与 VM | overlay/revision tests |
| 22 | `LSP-2201` | Diagnostic adapter | LSP lifecycle + compiler diagnostics | 与 Zed | stable code/span/related info |
| 23 | `ZQ-3201` | 基础 highlights | TS grammar | 与 LSP | highlight fixtures |
| 24 | `ZQ-3202` | brackets | TS grammar | 与 ZQ-3201 | pair fixtures |
| 25 | `ZQ-3204` | outline | TS declarations | 与 query tasks | symbols visible in Zed |
| 26 | `VM-1204` | VM 基础执行 | VM-1202/1203 | 否 | **Done**；verifier-gated `ling-vm`、全 1.0 scalar operator/control flow、Capability preflight、budget/Fault/source-map 与 interpreter differential evidence |
| 27 | `LSP-2202` | push diagnostics | LSP-2201 | 与 hover | stale diagnostics handled |
| 28 | `ZEXT-3302` | Zed Wasm extension | grammar-only stable | 与 LSP | wasm32-wasip2 build |
| 29 | `ZEXT-3303` | 查找本地 zero | extension skeleton | 与 queries | setting/PATH behavior |
| 30 | `ZEXT-3304` | 启动 `zero lsp --stdio` | LSP command stable | 否 | diagnostics in Zed |
| 31 | `IDE-2301` | document symbols | semantic index | 与 IDE-2302 | protocol fixtures |
| 32 | `IDE-2302` | hover | typed core index | 与 symbols | types/effects/capabilities |
| 33 | `IDE-2303` | definition | resolved symbols | 与 hover | Unicode navigation |

## 3. 批次状态建议

```text
G0 Done：规范、协议、错误码、追踪、CI
G1：`TS-3101`～`TS-3108`、`ZQ-3201`～`ZQ-3203`、`PRJ-1101`～`PRJ-1106`、`PRJ-1108` 与 `VM-1201`～`VM-1210` Done；`PRJ-1107` 的 project `test`/`build` CLI 行为仍服从其接口前置
G2 Blocked：需 v0.1 exit + Effect/Task/Actor RFC
G3 Blocked：需资源/ownership/native RFC
G4 Blocked：需 G3 ABI/memory + Kernel RFC
G5 Blocked：需 G2 Replay + G3 Native + G4 restricted lowering
G6 Blocked：只在 G1～G5 完成后稳定化
```

## 4. 全任务索引

> “建议状态”是生成本计划时的初始值，实际执行后必须由仓库状态文件更新。

| Task | 阶段 | 标题 | 规模 | 建议状态 | 来源 |
| --- | --- | --- | --- | --- | --- |
| `BASE-0001` | G0 | 仓库基线盘点与执行计划落位 | S | Done | `14-FIRST-SPRINT-CODEX-TASKS.md` Task A |
| `GOV-0101` | G0 | 建立规范权威索引 | S | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:53`；见机器状态与实施报告 |
| `GOV-0102` | G0 | 规范缺口台账 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:75`；见机器状态与实施报告 |
| `GOV-0103` | G0 | RFC 与 decision 生命周期 | S | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:105`；见机器状态与实施报告 |
| `GOV-0104` | G0 | 公开接口与协议总盘点 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:125`；见机器状态与实施报告 |
| `GOV-0105` | G0 | Diagnostic 错误码注册表 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:159`；见机器状态与实施报告 |
| `GOV-0106` | G0 | Schema 生命周期与 golden corpus | L | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:188`；见机器状态、Schema registry/corpus 与实施报告 |
| `GOV-0107` | G0 | 统一追踪矩阵 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:221`；见机器状态、生成矩阵与实施报告 |
| `GOV-0108` | G0 | 1.0 支持矩阵草案 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:240`；见机器状态、生成矩阵与实施报告 |
| `GOV-0109` | G0 | 发布状态机器可读化 | S | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:267`；见机器状态、生成视图与实施报告 |
| `GOV-0110` | G0 | G0 CI 门禁 | M | Done | `02-G0-GOVERNANCE-AND-COMPATIBILITY.md:281`；见机器状态、CI contract 与实施报告 |
| `PRJ-1101` | G1/Editor | 最小项目 manifest | M | Done | `03-G1-V0.1-LIVING.md:47`；见 `crates/ling-project/`、manifest-v1 fixtures、机器状态与实施报告 |
| `PRJ-1102` | G1/Editor | Module discovery | M | Done | `03-G1-V0.1-LIVING.md:79`；见 `ling-project` discovery、discovery-v1 fixtures、机器状态与实施报告 |
| `PRJ-1103` | G1/Editor | Import 与 visibility | M | Done | `03-G1-V0.1-LIVING.md:89`；见 package-aware resolver、`ling.semantic/0.2`、resolution-v1 fixtures、机器状态与实施报告 |
| `PRJ-1104` | G1/Editor | Dependency graph | L | Done | `03-G1-V0.1-LIVING.md:97`；见 `ling-project` package graph、dependency-v1 fixtures、机器状态与实施报告 |
| `PRJ-1105` | G1/Editor | Lock file | L | Done | `03-G1-V0.1-LIVING.md:107`；见 `ling-project` lockfile reader/writer、`schemas/lock/1` corpus、机器状态与实施报告 |
| `PRJ-1106` | G1/Editor | Project fixtures | M | Done | `03-G1-V0.1-LIVING.md:116`；见七组命名 fixture、expected diagnostics/graph/lock、机器状态与实施报告 |
| `PRJ-1107` | G1/Editor | Project API 与 CLI 接入 | M | BlockedSpec | Accepted RFC-0024 closes only the locked graph-check child; semantic project check/run/test/build/workspace/artifact behavior remains in `GAP-PROJECT-CLI-INTERFACE-001`; see `docs/status/PRJ-1107-AUTHORITY-AUDIT.md` |
| `PRJ-1107-CHECK` | G1/Editor | Locked project graph check Preview | M | Done | Accepted RFC-0024; see `crates/ling-cli/tests/project_check.rs`, `tests/protocols/project-check/README.md`, and `docs/status/PRJ-1107-CHECK-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for the deferred project surface |
| `PRJ-1107-LOAD` | G1/Editor | Locked project snapshot boundary | S | Done | Accepted DEC-0058; see `crates/ling-project/src/workspace.rs`, `crates/ling-project/tests/locked_project.rs`, and `docs/status/PRJ-1107-LOAD-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for the deferred project surface |
| `PRJ-1108` | G1/Editor | Project graph fuzz/property | M | Done | `03-G1-V0.1-LIVING.md:140`；见生成式 cycle/path/order/lock properties、manifest fuzz/CI、机器状态与实施报告 |
| `VM-1201` | G1/Editor | bytecode RFC 与模型 | M | Done | `03-G1-V0.1-LIVING.md:150`；见 RFC-0014、`ling-bytecode` 未验证模型、TEST-VM-0001 corpus、机器状态与实施报告 |
| `VM-1202` | G1/Editor | Checked Core → bytecode 最小 lowering | L | Done | `03-G1-V0.1-LIVING.md:166`；见 deterministic lowering/encoder/disassembler、Hello exact goldens、机器状态与实施报告 |
| `VM-1203` | G1/Editor | 独立 decoder/verifier | L | Done | `03-G1-V0.1-LIVING.md:180`；见 bounded decoder、独立 verifier、双语诊断、malformed/fuzz evidence、机器状态与实施报告 |
| `VM-1204` | G1/Editor | VM 基础执行 | L | Done | `03-G1-V0.1-LIVING.md:196`；见 verifier-gated `ling-vm`、显式 value/host Capability/budget/source-map Fault、differential evidence、机器状态与实施报告 |
| `VM-1205` | G1/Editor | 函数、closure、recursion | L | Done | Accepted RFC-0015；见 `crates/ling-bytecode/src/lower/v1_1.rs`、`crates/ling-vm/src/execute.rs` 与 `docs/status/VM-1205-IMPLEMENTATION-REPORT.md` |
| `VM-1206` | G1/Editor | Record、ADT、match | L | Done | RFC-0016；见 `ling-bytecode` v1.2 lowering/verifier/round-trip tests、`ling-vm` aggregate differential tests、`docs/status/VM-1206-IMPLEMENTATION-REPORT.md` |
| `VM-1207` | G1/Editor | Mutable place 与基础 borrow | L | Done | RFC-0017；见 v1.2 SSA place lowering、CFG join、interpreter/VM differential evidence、机器状态与实施报告 |
| `VM-1208` | G1/Editor | Effect/Capability/Fault | L | Done | RFC-0018；见现有 Effect/Capability 元数据闭包、执行前 capability preflight、稳定 Runtime Fault/source-map 证据及 host panic containment 测试 |
| `VM-1209` | G1/Editor | Interpreter ↔ VM differential | L | Done | RFC-0019；见 `crates/ling-vm/tests/differential.rs` 的事件、Unit、Fault projection、ProgramId 与 round-trip evidence 及机器状态报告 |
| `VM-1210` | G1/Editor | Fuzz 与资源限制 | L | Done | RFC-0020；见 `crates/ling-vm` cancellation/resource tests、bytecode fuzz target、机器状态与实施报告 |
| `TRAIT-1301` | G1/Editor | Trait RFC 收口 | — | Done | Accepted RFC-0005; see `docs/RFC-0005.md` and the authority audit |
| `TRAIT-1302` | G1/Editor | AST/HIR 表示 | — | Done | RFC-0005 §1; see `docs/status/TRAIT-1302-IMPLEMENTATION-REPORT.md` |
| `TRAIT-1303` | G1/Editor | Constraint collection | — | Done | DEC-0024; see `docs/status/TRAIT-1303-IMPLEMENTATION-REPORT.md` |
| `TRAIT-1304` | G1/Editor | Coherence/orphan checker | — | Done | DEC-0025; `03-G1-V0.1-LIVING.md:289`; `docs/status/TRAIT-1304-IMPLEMENTATION-REPORT.md` |
| `TRAIT-1305` | G1/Editor | Solver v0 | — | Done | DEC-0026; `03-G1-V0.1-LIVING.md:296`; `docs/status/TRAIT-1305-IMPLEMENTATION-REPORT.md` |
| `TRAIT-1306` | G1/Editor | Checked Core 显式化 | — | Done | DEC-0027; `03-G1-V0.1-LIVING.md:300`; `docs/status/TRAIT-1306-IMPLEMENTATION-REPORT.md` |
| `TRAIT-1307` | G1/Editor | Interpreter/VM lowering | L | Done | Accepted RFC-0021; see `docs/status/TRAIT-1307-AUTHORITY-AUDIT.md`, `docs/status/TRAIT-1307-IMPLEMENTATION-REPORT.md`, and `03-G1-V0.1-LIVING.md:304` |
| `TRAIT-1308` | G1/Editor | IDE 支持 | — | BlockedSpec | `03-G1-V0.1-LIVING.md:308`, `docs/status/TRAIT-1308-AUTHORITY-AUDIT.md` |
| `TRAIT-1308-PROJECTION` | G1/Editor | Trait Semantic Graph projection | M | Done | Accepted RFC-0022; see `docs/status/TRAIT-1308-PROJECTION-IMPLEMENTATION-REPORT.md`; full LSP/rename/repair surface remains in `TRAIT-1308` |
| `TRAIT-1308-QUERY` | G1/Editor | Trait projection read-only lookups | S | Done | Accepted DEC-0059; see `docs/status/TRAIT-1308-QUERY-IMPLEMENTATION-REPORT.md`; full LSP/rename/repair surface remains in `TRAIT-1308` |
| `TRAIT-1309` | G1/Editor | 性能与终止 | — | BlockedSpec | `03-G1-V0.1-LIVING.md:316`, `docs/status/TRAIT-1309-AUTHORITY-AUDIT.md` |
| `TRAIT-1309-TERMINATION` | G1/Editor | Bounded Trait solver termination evidence | S | Done | Accepted `DEC-0068`; see `docs/status/TRAIT-1309-TERMINATION-IMPLEMENTATION-REPORT.md`; production benchmark/LSP contract remains BlockedSpec |
| `INC-1401` | G1/Editor | Query boundary ADR | M | Done | DEC-0019；见 `docs/decisions/0019-incremental-query-boundary.md` 与实施报告 |
| `INC-1402` | G1/Editor | VFS 与 revision | M | Done | DEC-0019；见 `crates/ling-source/src/vfs.rs` 的 immutable snapshot、overlay、workspace revision 与 change-dedup 测试及实施报告 |
| `INC-1403` | G1/Editor | Parse queries | M | Done | DEC-0019；见 `crates/ling-db/src/lib.rs` 的 source_bytes、line_index、tokens、parse、ast 查询、确定性缓存与 clean/incremental 等价测试及实施报告 |
| `INC-1404` | G1/Editor | Resolve/module queries | M | Done | DEC-0019；见 `crates/ling-db/src/lib.rs` 的 HIR、模块图与 resolve 查询、公共导出失效及私有实现复用测试，以及实施报告 |
| `INC-1405` | G1/Editor | Type/effect queries | M | Done | DEC-0019；见 `crates/ling-db/src/lib.rs` 的模块级 type/effect 投影、public interface/body 失效边界、结构化错误与 clean/incremental 复用测试，以及实施报告 |
| `INC-1406` | G1/Editor | Semantic queries | M | Done | DEC-0019；见 `crates/ling-db/src/lib.rs` 的 canonical semantic snapshot、module fragment、Program/Body ID 失效边界与 JSON round-trip 测试，以及实施报告 |
| `INC-1407` | G1/Editor | Clean ↔ incremental equivalence | M | Done | DEC-0019; see `crates/ling-db/src/lib.rs` clean-rebuild equivalence harness/tests and implementation report |
| `INC-1408` | G1/Editor | Deterministic parallel scheduling | M | Done | DEC-0019 §4, DEC-0021; see `crates/ling-db/src/lib.rs` canonical parallel parse scheduling and implementation report |
| `INC-1409` | G1/Editor | Persistent cache（bounded disposable slice） | L | Done | DEC-0022；见 `crates/ling-cache` 的 versioned envelope、`crates/ling-db/src/lib.rs` 的 checked line-index persistence/corruption fallback、实施报告；dependent-query serialization and migration remain open |
| `INC-1410` | G1/Editor | 增量性能基线 | M | Done | `cargo xtask performance baseline`；见 `tools/xtask/src/performance.rs` 与 `docs/status/INC-1410-PERFORMANCE-BASELINE.json`；门禁采用趋势比较，不冻结绝对时延 |
| `FMT-1501` | G1/Editor | Formatter preservation decision | M | Done | Accepted DEC-0023；见 `docs/decisions/0023-author-source-formatter-preservation.md` 与实施报告；Format IR remains FMT-1502 |
| `FMT-1502` | G1/Editor | Format IR | M | Done | DEC-0023；见 `crates/ling-format/src/format_ir.rs` 的 compiler-CST 投影、原始/词法双 span、精确 token spelling、无效源码保留与实施报告；不含渲染器、CLI/LSP 或第二 parser |
| `FMT-1503` | G1/Editor | 核心语法格式化 | M | Done | DEC-0002、DEC-0006、DEC-0023；见 `crates/ling-format/src/author.rs` 的 compiler-IR 核心格式化、四空格布局、LF/spacing、编译器重解析回退、Seed 语法覆盖与实施报告；incomplete recovery、CLI/LSP deferred |
| `FMT-1504` | G1/Editor | Comment attachment | M | Done | DEC-0002、DEC-0023；见 `crates/ling-format/src/comments.rs` 的 compiler-token/CST attachment、文档/行尾/中文/嵌套块注释覆盖、格式化保留守卫与实施报告；incomplete recovery、property evidence、CLI/LSP/Audit separation deferred |
| `FMT-1505` | G1/Editor | 不完整源码 | M | Done | DEC-0002、DEC-0023；见 `crates/ling-format/src/author.rs` 的 `FormatDisposition`/`FormatResult` 保守回退、完整前缀不部分改写、BOM/CRLF 字节保留测试与实施报告；完整节点局部恢复仍需后续 Accepted 决策，性质测试、CLI/LSP/Audit separation deferred |
| `FMT-1506` | G1/Editor | 性质测试 | M | Done | DEC-0002、DEC-0023；见 `crates/ling-format/src/author.rs` 的固定离线 corpus：幂等、compiler token signature、comment spelling/order、AST→HIR→resolve→types→effects→Semantic Graph 等价测试与实施报告；CLI/LSP、Audit separation deferred |
| `FMT-1507` | G1/Editor | CLI/LSP 接入 | M | BlockedSpec | `GAP-FORMATTER-AUTHOR-SOURCE-001`, `GAP-FORMATTER-CLI-PROTOCOL-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`, `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`; see `docs/status/FMT-1507-AUTHORITY-AUDIT.md` |
| `FMT-1507-CLI` | G1/Editor | Formatter CLI Preview slice | M | Done | DEC-0028; see `crates/ling-cli/src/main.rs`, `schemas/format/0.1/`, and `docs/status/FMT-1507-CLI-IMPLEMENTATION-REPORT.md`; LSP/Semantic Transaction remains in FMT-1507 |
| `FMT-1507-EDIT` | G1/Editor | Deterministic formatter whole-document edit projection | S | Done | Accepted DEC-0057; see `crates/ling-format/src/edit.rs` and `docs/status/FMT-1507-EDIT-IMPLEMENTATION-REPORT.md`; public LSP/Workspace Edit remains in FMT-1507 |
| `FMT-1508` | G1/Editor | Audit 分离 | S | Done | DEC-0015、DEC-0023；见 `crates/ling-format/src/author.rs` 的 canonical Audit Source byte-equivalence property 与实施报告；CLI/LSP protocol decisions remain FMT-1507 blockers |
| `CLI-1701` | G1/Editor | 命令模型统一 | — | BlockedSpec | Accepted DEC-0036 closes only the internal current-command catalog; public command registry, services, and future commands remain open; see `docs/status/CLI-1701-AUTHORITY-AUDIT.md` |
| `CLI-1701-CATALOG` | G1/Editor | Internal current CLI command catalog | S | Done | Accepted DEC-0036; see `crates/ling-cli/src/command_catalog.rs` and `docs/status/CLI-1701-CATALOG-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1702` | G1/Editor | 输出与退出码 | — | BlockedSpec | Accepted DEC-0037 closes only the internal exit catalog; public output/failure/retry contracts remain open; see `docs/status/CLI-1702-AUTHORITY-AUDIT.md` |
| `CLI-1702-EXIT` | G1/Editor | Internal CLI exit-code catalog | S | Done | Accepted DEC-0037; see `crates/ling-cli/src/exit_catalog.rs` and `docs/status/CLI-1702-EXIT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1703` | G1/Editor | `init` | — | BlockedSpec | Accepted DEC-0038 closes only the bounded init child; broader project CLI behavior remains open; see `docs/status/CLI-1703-AUTHORITY-AUDIT.md` |
| `CLI-1703-INIT` | G1/Editor | Offline `ling init` scaffold | M | Done | Accepted DEC-0038; see `crates/ling-cli/src/init.rs`, `schemas/init/0.1/`, and `docs/status/CLI-1703-INIT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1704` | G1/Editor | `test` | — | BlockedSpec | Accepted DEC-0039 closes only the standalone-file child; project/workspace test behavior remains in `GAP-PROJECT-CLI-INTERFACE-001`; see `docs/status/CLI-1704-AUTHORITY-AUDIT.md` |
| `CLI-1704-FILE` | G1/Editor | Explicit standalone test-file runner Preview | M | Done | Accepted DEC-0039; verified/committed as `72d85d7de77f188b0706acde7a559169d4ac149e`; see `crates/ling-cli/src/test_runner.rs`, `schemas/test/0.1/`, and `docs/status/CLI-1704-TEST-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1705` | G1/Editor | `query/patch` | — | BlockedSpec | `03-G1-V0.1-LIVING.md:478`, `docs/status/CLI-1705-AUTHORITY-AUDIT.md` |
| `CLI-1706` | G1/Editor | Shell completion 与 help fixtures | — | BlockedSpec | `03-G1-V0.1-LIVING.md:482`, `docs/status/CLI-1706-AUTHORITY-AUDIT.md` |
| `CLI-1706-HELP` | G1/Editor | Truthful implemented-command help fixture | S | Done | Accepted DEC-0040; see `crates/ling-cli/tests/help.rs` and `docs/status/CLI-1706-HELP-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `LSP-2101` | G1/Editor | 初始化与生命周期 | S | BlockedSpec | `04-LSP-IMPLEMENTATION.md:137`, `docs/status/LSP-2101-AUTHORITY-AUDIT.md` |
| `LSP-2101-LIFECYCLE` | G1/Editor | LSP lifecycle Preview slice | S | Done | `RFC-0004`; verified/committed as `38d95fb7b91c2035bd2b1b4ebf864c1693050925`; see `crates/ling-lsp`, `tests/protocols/lsp-lifecycle`, and `docs/status/LSP-2101-LIFECYCLE-IMPLEMENTATION-REPORT.md`; document, diagnostic, edit, and transaction surfaces remain deferred |
| `LSP-2102` | G1/Editor | Position encoding negotiation | S | BlockedSpec | `04-LSP-IMPLEMENTATION.md:145`, `docs/status/LSP-2102-AUTHORITY-AUDIT.md`; bounded negotiation and SourceMap children are complete |
| `LSP-2102-SOURCE-MAP` | G1/Editor | SourceMap position projection | S | Done | `DEC-0029`; see `crates/ling-source/src/position.rs` and `docs/status/LSP-2102-SOURCE-MAP-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for transaction semantics |
| `LSP-2102-NEGOTIATION` | G1/Editor | LSP initialize position-encoding negotiation | S | Done | `RFC-0004` + `DEC-0029`; see `crates/ling-lsp/tests/position_encoding.rs` and `docs/status/LSP-2102-NEGOTIATION-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for transaction semantics |
| `LSP-2103` | G1/Editor | Open document overlay | M | BlockedSpec | `04-LSP-IMPLEMENTATION.md:151`, `docs/status/LSP-2103-AUTHORITY-AUDIT.md` |
| `LSP-2103-OVERLAY` | G1/Editor | LSP full-text overlay Preview slice | M | Done | Accepted RFC-0023; see `crates/ling-lsp/tests/overlay.rs` and `docs/status/LSP-2103-OVERLAY-IMPLEMENTATION-REPORT.md`; incremental edits and transactions remain in `LSP-2103` |
| `LSP-2104-UTF8-EDITS` | G1/Editor | Bounded internal UTF-8 edit application | S | Done | Accepted `DEC-0069`; see `docs/status/LSP-2104-UTF8-EDITS-IMPLEMENTATION-REPORT.md`; public range/version/VFS/transaction behavior remains BlockedSpec in `LSP-2104` |
| `LSP-2104-POSITION-EDITS` | G1/Editor | Bounded internal position-edit projection | S | Done | Accepted `DEC-0070`; see `docs/status/LSP-2104-POSITION-EDITS-IMPLEMENTATION-REPORT.md`; public LSP range/version/VFS/transaction behavior remains BlockedSpec in `LSP-2104` |
| `LSP-2104` | G1/Editor | 增量文本变更 | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:170`, `docs/status/LSP-2104-AUTHORITY-AUDIT.md` |
| `LSP-2105` | G1/Editor | Workspace reload | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:182`, `docs/status/LSP-2105-AUTHORITY-AUDIT.md` |
| `LSP-2201` | G1/Editor | Compiler diagnostic adapter | — | BlockedSpec | Accepted DEC-0034 closes only the internal ordering child; public adapter, positions, fields, and publication remain open; see `docs/status/LSP-2201-AUTHORITY-AUDIT.md` |
| `LSP-2201-ORDERING` | G1/Editor | Internal canonical diagnostic ordering | S | Done | Accepted DEC-0034; see `crates/ling-lsp/src/diagnostics.rs` and `docs/status/LSP-2201-ORDERING-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `LSP-2202` | G1/Editor | Push diagnostics v0 | — | BlockedSpec | Accepted DEC-0035 closes only the internal diagnostic-batch child; public publish/trigger/version/clear behavior remains open; see `docs/status/LSP-2202-AUTHORITY-AUDIT.md` |
| `LSP-2202-BATCH` | G1/Editor | Internal immutable diagnostic batch | S | Done | Accepted DEC-0035; see `crates/ling-lsp/src/diagnostic_batch.rs` and `docs/status/LSP-2202-BATCH-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `LSP-2203` | G1/Editor | Pull diagnostics Preview | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:215`, `docs/status/LSP-2203-AUTHORITY-AUDIT.md` |
| `LSP-2204` | G1/Editor | Root-cause 与错误风暴控制 | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:219`, `docs/status/LSP-2204-AUTHORITY-AUDIT.md` |
| `LSP-2205` | G1/Editor | Diagnostic fixtures | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:227`, `docs/status/LSP-2205-AUTHORITY-AUDIT.md` |
| `IDE-2301` | G1/Editor | Document symbols | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:244`, `docs/status/IDE-2301-AUTHORITY-AUDIT.md` |
| `IDE-2302` | G1/Editor | Hover | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:248`, `docs/status/IDE-2302-AUTHORITY-AUDIT.md` |
| `IDE-2303` | G1/Editor | Go to definition/declaration/type definition | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:262`, `docs/status/IDE-2303-AUTHORITY-AUDIT.md` |
| `IDE-2304` | G1/Editor | References | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:266`, `docs/status/IDE-2304-AUTHORITY-AUDIT.md` |
| `IDE-2305` | G1/Editor | Prepare rename | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:270`, `docs/status/IDE-2305-AUTHORITY-AUDIT.md` |
| `IDE-2306` | G1/Editor | Rename | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:283`, `docs/status/IDE-2306-AUTHORITY-AUDIT.md` |
| `IDE-2307` | G1/Editor | Completion v0 | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:299`, `docs/status/IDE-2307-AUTHORITY-AUDIT.md` |
| `IDE-2308` | G1/Editor | Completion resolve | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:312`, `docs/status/IDE-2308-AUTHORITY-AUDIT.md` |
| `IDE-2309` | G1/Editor | Code actions | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:316`, `docs/status/IDE-2309-AUTHORITY-AUDIT.md` |
| `IDE-2310` | G1/Editor | Formatting | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:329`, `docs/status/IDE-2310-AUTHORITY-AUDIT.md` |
| `IDE-2311` | G1/Editor | Workspace symbols | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:333`, `docs/status/IDE-2311-AUTHORITY-AUDIT.md` |
| `LSP-2401` | G1/Editor | Token taxonomy RFC/decision | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:339`, `docs/status/LSP-2401-AUTHORITY-AUDIT.md` |
| `LSP-2402` | G1/Editor | Typed token generation | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:370`, `docs/status/LSP-2402-AUTHORITY-AUDIT.md` |
| `LSP-2403` | G1/Editor | Full 与 delta | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:374`, `docs/status/LSP-2403-AUTHORITY-AUDIT.md` |
| `LSP-2404` | G1/Editor | Semantic token fixtures | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:378`, `docs/status/LSP-2404-AUTHORITY-AUDIT.md` |
| `LSP-2501` | G1/Editor | Request snapshot | — | BlockedSpec | Accepted DEC-0030 closes only the internal immutable capture child; public request identity, CompilerHost/query inputs, cancellation, stale publication, and protocol lifecycle remain open; see `docs/status/LSP-2501-AUTHORITY-AUDIT.md` |
| `LSP-2501-SNAPSHOT` | G1/Editor | Internal immutable request snapshot capture | S | Done | Accepted DEC-0030; see `crates/ling-lsp/tests/request_snapshot.rs` and `docs/status/LSP-2501-SNAPSHOT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for the public analysis boundary |
| `LSP-2502` | G1/Editor | Cancellation | — | BlockedSpec | Accepted DEC-0031 closes only the internal cooperative-token child; public `$/cancelRequest`, compiler propagation, and result publication remain open; see `docs/status/LSP-2502-AUTHORITY-AUDIT.md` |
| `LSP-2502-CANCELLATION` | G1/Editor | Internal cooperative cancellation token | S | Done | Accepted DEC-0031; see `crates/ling-lsp/tests/cancellation.rs` and `docs/status/LSP-2502-CANCELLATION-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for public cancellation |
| `LSP-2503` | G1/Editor | Debounce 与优先级 | — | BlockedSpec | Accepted DEC-0032 closes only the internal ordering child; public debounce, fairness, freshness, cancellation, and publication remain open; see `docs/status/LSP-2503-AUTHORITY-AUDIT.md` |
| `LSP-2503-SCHEDULER` | G1/Editor | Internal deterministic LSP work ordering | S | Done | Accepted DEC-0032; see `crates/ling-lsp/src/scheduler.rs` and `docs/status/LSP-2503-SCHEDULER-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `LSP-2504` | G1/Editor | Memory/resource limits | — | BlockedSpec | Accepted DEC-0033 closes only the internal byte-accounting child; public quotas, diagnostics, configuration, and failure precedence remain open; see `docs/status/LSP-2504-AUTHORITY-AUDIT.md` |
| `LSP-2504-BYTE-ACCOUNTING` | G1/Editor | Internal deterministic LSP UTF-8 byte accounting | S | Done | Accepted DEC-0033; see `crates/ling-lsp/src/resource.rs` and `docs/status/LSP-2504-BYTE-ACCOUNTING-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `TS-3101` | G1/Editor | Grammar 规范映射表 | S | Done | `05-ZED-EXTENSION.md:154`；见 `docs/grammar-map.md`、机器状态与实施报告 |
| `TS-3102` | G1/Editor | 宽度优先 grammar skeleton | M | Done | `05-ZED-EXTENSION.md:165`；见 `editors/tree-sitter-ling/`、机器状态与实施报告 |
| `TS-3103` | G1/Editor | Offside/缩进策略 | M | Done | `05-ZED-EXTENSION.md:187`；见 scanner、ADR、layout corpus、机器状态与实施报告 |
| `TS-3104` | G1/Editor | Unicode identifier | M | Done | `05-ZED-EXTENSION.md:199`；见 Unicode 17.0.0 生成范围、共享 lexer differential corpus、ADR、机器状态与实施报告 |
| `TS-3105` | G1/Editor | Expression precedence | M | Done | `05-ZED-EXTENSION.md:211`；Accepted `DEC-0017`；见 compiler/Tree-sitter 共享 precedence corpus、机器状态与实施报告 |
| `TS-3106` | G1/Editor | Pattern 与 Type | M | Done | `05-ZED-EXTENSION.md:217`；见共享 41-case validity corpus、compiler/conformance evidence、机器状态与实施报告 |
| `TS-3107` | G1/Editor | Error recovery | M | Done | `05-ZED-EXTENSION.md:221`；见 10 个静态、9 个增量、64 个定种子 mutation cases、scanner tests、机器状态与实施报告 |
| `TS-3108` | G1/Editor | Grammar differential | M | Done | `05-ZED-EXTENSION.md:234`；见 42 个全程序 compiler/Tree-sitter cases、84 个定种子编辑、43 个稳定 CST/node-type 映射、机器状态与实施报告 |
| `ZQ-3201` | G1/Editor | `highlights.scm` | S | Done | `05-ZED-EXTENSION.md:245`；见 18 个 capture、46 个标准 highlight assertions、机器状态与实施报告 |
| `ZQ-3202` | G1/Editor | `brackets.scm` | S | Done | `05-ZED-EXTENSION.md:273`；见 4 类括号对、20 个正反断言、机器状态与实施报告 |
| `ZQ-3203` | G1/Editor | `indents.scm` | S | Done | `05-ZED-EXTENSION.md:277`；见 15 类 CST 节点、38/14/4 个 indent/end/start ranges、4 个 fixtures、机器状态与实施报告 |
| `ZQ-3204` | G1/Editor | `outline.scm` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:288` |
| `ZQ-3205` | G1/Editor | `textobjects.scm` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:294` |
| `ZQ-3206` | G1/Editor | `runnables.scm` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:302` |
| `ZQ-3207` | G1/Editor | `overrides.scm` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:312` |
| `ZQ-3208` | G1/Editor | `injections.scm` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:316` |
| `ZQ-3209` | G1/Editor | `redactions.scm` | S | Blocked by G0/interface | `05-ZED-EXTENSION.md:320` |
| `ZEXT-3301` | G1/Editor | Grammar-only dev extension | S | Blocked by G0/interface | `05-ZED-EXTENSION.md:326` |
| `ZEXT-3302` | G1/Editor | Extension Wasm 骨架 | S | Blocked by G0/interface | `05-ZED-EXTENSION.md:338` |
| `ZEXT-3303` | G1/Editor | 查找本地 `zero` | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:349` |
| `ZEXT-3304` | G1/Editor | Language server command | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:366` |
| `ZEXT-3305` | G1/Editor | Release 下载（Z6 前） | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:379` |
| `ZED-3401` | G1/Editor | Diagnostics smoke | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:396` |
| `ZED-3402` | G1/Editor | Hover/definition/references | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:416` |
| `ZED-3403` | G1/Editor | Rename | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:424` |
| `ZED-3404` | G1/Editor | Completion | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:434` |
| `ZED-3405` | G1/Editor | Code action | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:442` |
| `ZED-3406` | G1/Editor | Semantic tokens | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:446` |
| `ZED-3501` | G1/Editor | 项目 tasks 模板 | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:478` |
| `ZED-3502` | G1/Editor | Runnables query | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:504` |
| `ZED-3503` | G1/Editor | Format on save | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:508` |
| `ZED-3504` | G1/Editor | Snippets | — | Blocked by G0/interface | `05-ZED-EXTENSION.md:512` |
  | `EFF-2101` | G2 | Effect 核心模型冻结 | M | Done | `06-G2-V0.2-CONCURRENT.md:81`, `docs/RFC-0006.md`, `docs/status/EFF-2101-AUTHORITY-AUDIT.md`, `docs/status/EFF-2101-IMPLEMENTATION-REPORT.md` |
| `EFF-2101-SEED-ROW` | G2 | Seed EffectRow canonical snapshot | S | Done | Accepted DEC-0060; see `docs/status/EFF-2101-SEED-ROW-IMPLEMENTATION-REPORT.md`; v0.2 Effect model remains in `EFF-2101` |
| `EFF-2102` | G2 | Effect 推导和约束求解 | L | Done | `06-G2-V0.2-CONCURRENT.md:105`, `DEC-0062`, `docs/status/EFF-2102-AUTHORITY-AUDIT.md`, `docs/status/EFF-2102-IMPLEMENTATION-REPORT.md` |
| `EFF-2103` | G2 | Handler Typed Core 表示 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:115`, `docs/status/EFF-2103-AUTHORITY-AUDIT.md` |
| `EFF-2103-CORE` | G2 | First-order handler Typed Core projection | S | Done | Accepted `DEC-0063`; see `docs/status/EFF-2103-CORE-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for source syntax/lowering |
| `EFF-2103-SYNTAX` | G2 | Handler source CST projection | S | Done | Accepted `DEC-0064`; see `docs/status/EFF-2103-SYNTAX-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for AST/HIR lowering and checking |
| `EFF-2103-AST` | G2 | Handler unresolved AST projection | S | Done | Accepted `DEC-0065`; see `docs/status/EFF-2103-AST-IMPLEMENTATION-REPORT.md`; HIR/checking and parent remain BlockedSpec |
| `EFF-2103-HIR` | G2 | Handler unresolved HIR projection | S | Done | Accepted `DEC-0066`; see `docs/status/EFF-2103-HIR-IMPLEMENTATION-REPORT.md`; resolver rejects before checked semantics and parent remains BlockedSpec |
| `EFF-2105-MODEL-PROPERTIES` | G2 | Effect model deterministic property corpus | S | Done | Accepted `DEC-0067`; see `docs/status/EFF-2105-MODEL-PROPERTIES-IMPLEMENTATION-REPORT.md`; full EFF-2105 runtime/differential target remains BlockedSpec |
| `EFF-2104` | G2 | 解释器与 VM Handler 执行 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:131`, `docs/status/EFF-2104-AUTHORITY-AUDIT.md` |
| `EFF-2105` | G2 | Effect fuzz/property tests | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:143`, `docs/status/EFF-2105-AUTHORITY-AUDIT.md` |
| `TASK-2201` | G2 | Task 语法与 Checked Core | M | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:158`, `docs/status/TASK-2201-AUTHORITY-AUDIT.md` |
| `TASK-2202` | G2 | Task 状态机 Lowering | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:182`, `docs/status/TASK-2202-AUTHORITY-AUDIT.md` |
| `TASK-2203` | G2 | 结构化生命周期 Runtime | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:195`, `docs/status/TASK-2203-AUTHORITY-AUDIT.md` |
| `TASK-2204` | G2 | 确定性测试调度器 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:207`, `docs/status/TASK-2204-AUTHORITY-AUDIT.md` |
| `TASK-2205` | G2 | 生产本地调度器 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:221`, `docs/status/TASK-2205-AUTHORITY-AUDIT.md` |
| `TASK-2206` | G2 | Task conformance 与压力测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:235`, `docs/status/TASK-2206-AUTHORITY-AUDIT.md` |
| `ACT-2301` | G2 | Actor 身份与状态隔离 | M | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:250`, `docs/status/ACT-2301-AUTHORITY-AUDIT.md` |
| `ACT-2302` | G2 | 消息可发送性检查 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:263`, `docs/status/ACT-2302-AUTHORITY-AUDIT.md` |
| `ACT-2303` | G2 | 有界 Mailbox 与背压 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:273`, `docs/status/ACT-2303-AUTHORITY-AUDIT.md` |
| `ACT-2304` | G2 | Turn 与重入规则 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:290`, `docs/status/ACT-2304-AUTHORITY-AUDIT.md` |
| `ACT-2305` | G2 | Actor Runtime | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:300`, `docs/status/ACT-2305-AUTHORITY-AUDIT.md` |
| `ACT-2306` | G2 | Actor 性质与压力测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:313`, `docs/status/ACT-2306-AUTHORITY-AUDIT.md` |
| `SUP-2401` | G2 | Supervisor 模型 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:326`, `docs/status/SUP-2401-AUTHORITY-AUDIT.md` |
| `SUP-2402` | G2 | 重启预算与熔断 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:340`, `docs/status/SUP-2402-AUTHORITY-AUDIT.md` |
| `SUP-2403` | G2 | 监督测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:355`, `docs/status/SUP-2403-AUTHORITY-AUDIT.md` |
| `REP-2501` | G2 | Determinism Class | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:369`, `docs/status/REP-2501-AUTHORITY-AUDIT.md` |
| `REP-2502` | G2 | Replay Log Schema | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:384`, `docs/status/REP-2502-AUTHORITY-AUDIT.md` |
| `REP-2503` | G2 | Effect Recorder | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:401`, `docs/status/REP-2503-AUTHORITY-AUDIT.md` |
| `REP-2504` | G2 | Replay Player | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:414`, `docs/status/REP-2504-AUTHORITY-AUDIT.md` |
| `REP-2505` | G2 | 隐私、裁剪与损坏 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:429`, `docs/status/REP-2505-AUTHORITY-AUDIT.md` |
| `REP-2506` | G2 | 跨进程重放验收 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:440`, `docs/status/REP-2506-AUTHORITY-AUDIT.md` |
| `REM-2601` | G2 | RemoteRef 与 Endpoint | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:452`, `docs/status/REM-2601-AUTHORITY-AUDIT.md` |
| `REM-2602` | G2 | Transport-neutral Envelope | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:466`, `docs/status/REM-2602-AUTHORITY-AUDIT.md` |
| `REM-2603` | G2 | Delivery 语义 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:482`, `docs/status/REM-2603-AUTHORITY-AUDIT.md` |
| `REM-2604` | G2 | 最小参考传输 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:497`, `docs/status/REM-2604-AUTHORITY-AUDIT.md` |
| `REM-2605` | G2 | 安全与资源限制 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:506`, `docs/status/REM-2605-AUTHORITY-AUDIT.md` |
| `MEM-3101` | G3 | 类型分类模型 | M | BlockedSpec | `07-G3-V0.3-NATIVE.md:67`, `docs/status/MEM-3101-AUTHORITY-AUDIT.md` |
| `MEM-3101-SEED-VALUE` | G3 | Seed completed-type Value classification | S | Done | Accepted DEC-0061; see `docs/status/MEM-3101-SEED-VALUE-IMPLEMENTATION-REPORT.md`; Managed/Resource model remains in `MEM-3101` |
| `MEM-3102` | G3 | Value 布局与 Copy/Move | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:88`, `docs/status/MEM-3102-AUTHORITY-AUDIT.md` |
| `MEM-3103` | G3 | Resource 定义与 Drop 契约 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:99`, `docs/status/MEM-3103-AUTHORITY-AUDIT.md` |
| `MEM-3104` | G3 | Managed 类型和 Island 边界 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:111`, `docs/status/MEM-3104-AUTHORITY-AUDIT.md` |
| `OWN-3201` | G3 | Place 与 Move Analysis | L | BlockedSpec | `07-G3-V0.3-NATIVE.md:125`, `docs/status/OWN-3201-AUTHORITY-AUDIT.md` |
| `OWN-3202` | G3 | Borrow Exclusivity | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:152`, `docs/status/OWN-3202-AUTHORITY-AUDIT.md` |
| `OWN-3203` | G3 | Region Inference | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:172`, `docs/status/OWN-3203-AUTHORITY-AUDIT.md` |
| `OWN-3204` | G3 | 跨 `await` / Actor Turn 的 Borrow | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:186`, `docs/status/OWN-3204-AUTHORITY-AUDIT.md` |
| `OWN-3205` | G3 | Drop 顺序 Lowering | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:196`, `docs/status/OWN-3205-AUTHORITY-AUDIT.md` |
| `OWN-3206` | G3 | Ownership 诊断与修复 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:209`, `docs/status/OWN-3206-AUTHORITY-AUDIT.md` |
| `OWN-3207` | G3 | 负向 corpus 与 property tests | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:224`, `docs/status/OWN-3207-AUTHORITY-AUDIT.md` |
| `GC-3301` | G3 | 最小对象模型 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:241`, `docs/status/GC-3301-AUTHORITY-AUDIT.md` |
| `GC-3302` | G3 | 第一版 Collector | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:256`, `docs/status/GC-3302-AUTHORITY-AUDIT.md` |
| `GC-3303` | G3 | Managed 与 Native/FFI 边界 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:271`, `docs/status/GC-3303-AUTHORITY-AUDIT.md` |
| `GC-3304` | G3 | Profile 检查 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:282`, `docs/status/GC-3304-AUTHORITY-AUDIT.md` |
| `NIR-3401` | G3 | Native IR 设计 | L | BlockedSpec | `07-G3-V0.3-NATIVE.md:291`, `docs/status/NIR-3401-AUTHORITY-AUDIT.md` |
| `NIR-3402` | G3 | Core → Native IR Lowering | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:308`, `docs/status/NIR-3402-AUTHORITY-AUDIT.md` |
| `NIR-3403` | G3 | IR Verifier | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:324`, `docs/status/NIR-3403-AUTHORITY-AUDIT.md` |
| `BACK-3501` | G3 | Backend 选择 Spike | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:341`, `docs/status/BACK-3501-AUTHORITY-AUDIT.md` |
| `BACK-3502` | G3 | Baseline Codegen | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:352`, `docs/status/BACK-3502-AUTHORITY-AUDIT.md` |
| `BACK-3503` | G3 | Runtime ABI | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:365`, `docs/status/BACK-3503-AUTHORITY-AUDIT.md` |
| `BACK-3504` | G3 | 基础优化与验证 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:380`, `docs/status/BACK-3504-AUTHORITY-AUDIT.md` |
| `BACK-3505` | G3 | Reproducible Native Build | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:393`, `docs/status/BACK-3505-AUTHORITY-AUDIT.md` |
| `FFI-3601` | G3 | FFI 声明模型 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:411`, `docs/status/FFI-3601-AUTHORITY-AUDIT.md` |
| `FFI-3602` | G3 | C ABI 最小互操作 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:431`, `docs/status/FFI-3602-AUTHORITY-AUDIT.md` |
| `FFI-3603` | G3 | Shim Generator | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:445`, `docs/status/FFI-3603-AUTHORITY-AUDIT.md` |
| `FFI-3604` | G3 | Target Primitive Package | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:460`, `docs/status/FFI-3604-AUTHORITY-AUDIT.md` |
| `FFI-3605` | G3 | FFI fuzz/sanitizer 套件 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:476`, `docs/status/FFI-3605-AUTHORITY-AUDIT.md` |
| `DIFF-3701` | G3 | 三方 Harness | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:492`, `docs/status/DIFF-3701-AUTHORITY-AUDIT.md` |
| `DIFF-3702` | G3 | 已允许差异登记表 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:512`, `docs/status/DIFF-3702-AUTHORITY-AUDIT.md` |
| `DAP-3601` | G3+ | `zero dap --stdio` | — | BlockedSpec | `05-ZED-EXTENSION.md:526`, `docs/status/DAP-3601-AUTHORITY-AUDIT.md` |
| `DAP-3602` | G3+ | Zed debugger registration | — | BlockedSpec | `05-ZED-EXTENSION.md:530`, `docs/status/DAP-3602-AUTHORITY-AUDIT.md` |
| `DAP-3603` | G3+ | 能力阶段 | — | BlockedSpec | `05-ZED-EXTENSION.md:538`, `docs/status/DAP-3603-AUTHORITY-AUDIT.md` |
| `KCHK-4101` | G4 | Kernel 允许能力矩阵 | M | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:69`, `docs/status/KCHK-4101-AUTHORITY-AUDIT.md` |
| `KCHK-4102` | G4 | Kernel Effect 与 Capability 检查 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:90`, `docs/status/KCHK-4102-AUTHORITY-AUDIT.md` |
| `KCHK-4103` | G4 | Shape、Index 与 Bounds | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:103`, `docs/status/KCHK-4103-AUTHORITY-AUDIT.md` |
| `KCHK-4104` | G4 | Alias 和并行写冲突 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:113`, `docs/status/KCHK-4104-AUTHORITY-AUDIT.md` |
| `KCHK-4105` | G4 | Kernel Core 与 Verifier | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:124`, `docs/status/KCHK-4105-AUTHORITY-AUDIT.md` |
| `CPU-4201` | G4 | Scalar Reference Backend | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:137`, `docs/status/CPU-4201-AUTHORITY-AUDIT.md` |
| `CPU-4202` | G4 | Reference Trace | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:153`, `docs/status/CPU-4202-AUTHORITY-AUDIT.md` |
| `CPU-4203` | G4 | Kernel Corpus | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:167`, `docs/status/CPU-4203-AUTHORITY-AUDIT.md` |
| `SIMD-4301` | G4 | 向量化合法性分析 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:183`, `docs/status/SIMD-4301-AUTHORITY-AUDIT.md` |
| `SIMD-4302` | G4 | Portable SIMD IR | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:198`, `docs/status/SIMD-4302-AUTHORITY-AUDIT.md` |
| `SIMD-4303` | G4 | SIMD Differential | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:210`, `docs/status/SIMD-4303-AUTHORITY-AUDIT.md` |
| `DBUF-4401` | G4 | Device 类型与 Capability | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:223`, `docs/status/DBUF-4401-AUTHORITY-AUDIT.md` |
| `DBUF-4402` | G4 | Buffer Ownership | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:240`, `docs/status/DBUF-4402-AUTHORITY-AUDIT.md` |
| `DBUF-4403` | G4 | Transfer Effect | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:252`, `docs/status/DBUF-4403-AUTHORITY-AUDIT.md` |
| `DBUF-4404` | G4 | 同步模型 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:272`, `docs/status/DBUF-4404-AUTHORITY-AUDIT.md` |
| `DIR-4501` | G4 | Device IR Schema | L | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:287`, `docs/status/DIR-4501-AUTHORITY-AUDIT.md` |
| `DIR-4502` | G4 | Kernel Core → Device IR | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:305`, `docs/status/DIR-4502-AUTHORITY-AUDIT.md` |
| `DIR-4503` | G4 | Device IR Canonicalization | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:317`, `docs/status/DIR-4503-AUTHORITY-AUDIT.md` |
| `GPU-4601` | G4 | Backend Spike 与选择 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:328`, `docs/status/GPU-4601-AUTHORITY-AUDIT.md` |
| `GPU-4602` | G4 | Backend Adapter | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:351`, `docs/status/GPU-4602-AUTHORITY-AUDIT.md` |
| `GPU-4603` | G4 | Launch 与 Runtime | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:365`, `docs/status/GPU-4603-AUTHORITY-AUDIT.md` |
| `GPU-4604` | G4 | 差分和硬件矩阵 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:378`, `docs/status/GPU-4604-AUTHORITY-AUDIT.md` |
| `GPU-4605` | G4 | 错误归一化 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:394`, `docs/status/GPU-4605-AUTHORITY-AUDIT.md` |
| `ACC-4701` | G4 | Accelerator Plugin Interface | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:412`, `docs/status/ACC-4701-AUTHORITY-AUDIT.md` |
| `ACC-4702` | G4 | Experimental 适配器 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:425`, `docs/status/ACC-4702-AUTHORITY-AUDIT.md` |
| `PLC-4801` | G4 | Placement 约束模型 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:431`, `docs/status/PLC-4801-AUTHORITY-AUDIT.md` |
| `PLC-4802` | G4 | 静态候选与运行时选择 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:448`, `docs/status/PLC-4802-AUTHORITY-AUDIT.md` |
| `PLC-4803` | G4 | Cost Model v0 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:466`, `docs/status/PLC-4803-AUTHORITY-AUDIT.md` |
| `PLC-4804` | G4 | `zero explain placement` | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:480`, `docs/status/PLC-4804-AUTHORITY-AUDIT.md` |
| `PLC-4805` | G4 | 设备二进制缓存 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:495`, `docs/status/PLC-4805-AUTHORITY-AUDIT.md` |
| `PROF-5101` | G5 | 机器可读 Profile | M | BlockedSpec | `09-G5-V0.5-CRITICAL.md:77`, `docs/status/PROF-5101-AUTHORITY-AUDIT.md` |
| `PROF-5102` | G5 | 禁止能力检查 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:108`, `docs/status/PROF-5102-AUTHORITY-AUDIT.md` |
| `PROF-5103` | G5 | Profile Composition | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:122`, `docs/status/PROF-5103-AUTHORITY-AUDIT.md` |
| `PROF-5104` | G5 | Profile Audit 与 LSP | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:126`, `docs/status/PROF-5104-AUTHORITY-AUDIT.md` |
| `BND-5201` | G5 | Bound 类型与表达式 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:140`, `docs/status/BND-5201-AUTHORITY-AUDIT.md` |
| `BND-5202` | G5 | 循环和递归检查 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:156`, `docs/status/BND-5202-AUTHORITY-AUDIT.md` |
| `BND-5203` | G5 | 内存预算 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:169`, `docs/status/BND-5203-AUTHORITY-AUDIT.md` |
| `BND-5204` | G5 | 资源预算诊断 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:184`, `docs/status/BND-5204-AUTHORITY-AUDIT.md` |
| `NODE-5301` | G5 | Node 语法与语义 | L | BlockedSpec | `09-G5-V0.5-CRITICAL.md:199`, `docs/status/NODE-5301-AUTHORITY-AUDIT.md` |
| `NODE-5302` | G5 | Node Checked Core | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:226`, `docs/status/NODE-5302-AUTHORITY-AUDIT.md` |
| `NODE-5303` | G5 | 静态调度 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:242`, `docs/status/NODE-5303-AUTHORITY-AUDIT.md` |
| `NODE-5304` | G5 | 虚拟时间参考 Runtime | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:254`, `docs/status/NODE-5304-AUTHORITY-AUDIT.md` |
| `NODE-5305` | G5 | Native Node Runtime | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:266`, `docs/status/NODE-5305-AUTHORITY-AUDIT.md` |
| `NODE-5306` | G5 | Node/Actor 边界 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:277`, `docs/status/NODE-5306-AUTHORITY-AUDIT.md` |
| `NODE-5307` | G5 | Node conformance | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:288`, `docs/status/NODE-5307-AUTHORITY-AUDIT.md` |
| `CTR-5401` | G5 | Contract 语法与 AST/Core | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:304`, `docs/status/CTR-5401-AUTHORITY-AUDIT.md` |
| `CTR-5402` | G5 | Contract 状态模型 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:320`, `docs/status/CTR-5402-AUTHORITY-AUDIT.md` |
| `CTR-5403` | G5 | Runtime Contract Check | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:335`, `docs/status/CTR-5403-AUTHORITY-AUDIT.md` |
| `CTR-5404` | G5 | Verification Condition Generation | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:347`, `docs/status/CTR-5404-AUTHORITY-AUDIT.md` |
| `CTR-5405` | G5 | Solver/Proof Checker Adapter | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:360`, `docs/status/CTR-5405-AUTHORITY-AUDIT.md` |
| `CTR-5406` | G5 | 优化器使用 Contract 的规则 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:371`, `docs/status/CTR-5406-AUTHORITY-AUDIT.md` |
| `CTR-5407` | G5 | Contract LSP/Zed | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:375`, `docs/status/CTR-5407-AUTHORITY-AUDIT.md` |
| `PROOF-5501` | G5 | Proof IR | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:386`, `docs/status/PROOF-5501-AUTHORITY-AUDIT.md` |
| `PROOF-5502` | G5 | 独立 Checker | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:400`, `docs/status/PROOF-5502-AUTHORITY-AUDIT.md` |
| `PROOF-5503` | G5 | 假设注册表 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:412`, `docs/status/PROOF-5503-AUTHORITY-AUDIT.md` |
| `MC-5601` | G5 | 有限状态投影 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:431`, `docs/status/MC-5601-AUTHORITY-AUDIT.md` |
| `MC-5602` | G5 | 探索引擎 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:446`, `docs/status/MC-5602-AUTHORITY-AUDIT.md` |
| `MC-5603` | G5 | 报告语义 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:458`, `docs/status/MC-5603-AUTHORITY-AUDIT.md` |
| `MC-5604` | G5 | Replay Counterexample | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:471`, `docs/status/MC-5604-AUTHORITY-AUDIT.md` |
| `TIM-5701` | G5 | Timing IR 与 Path | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:477`, `docs/status/TIM-5701-AUTHORITY-AUDIT.md` |
| `TIM-5702` | G5 | 测量与静态分析分离 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:489`, `docs/status/TIM-5702-AUTHORITY-AUDIT.md` |
| `TIM-5703` | G5 | Deadline Check | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:503`, `docs/status/TIM-5703-AUTHORITY-AUDIT.md` |
| `EVD-5801` | G5 | Bundle Schema | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:518`, `docs/status/EVD-5801-AUTHORITY-AUDIT.md` |
| `EVD-5802` | G5 | 独立验证器 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:543`, `docs/status/EVD-5802-AUTHORITY-AUDIT.md` |
| `EVD-5803` | G5 | 可重复构建绑定 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:557`, `docs/status/EVD-5803-AUTHORITY-AUDIT.md` |
| `EVD-5804` | G5 | AI Provenance | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:566`, `docs/status/EVD-5804-AUTHORITY-AUDIT.md` |
| `CBK-5901` | G5 | 可信编译路线决策 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:585`, `docs/status/CBK-5901-AUTHORITY-AUDIT.md` |
| `CBK-5902` | G5 | Lowering Validator | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:599`, `docs/status/CBK-5902-AUTHORITY-AUDIT.md` |
| `CBK-5903` | G5 | Critical Runtime/Target Package | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:611`, `docs/status/CBK-5903-AUTHORITY-AUDIT.md` |
| `STAB-6101` | G6 | 逐项支持矩阵审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:32`, `docs/status/STAB-6101-AUTHORITY-AUDIT.md` |
| `STAB-6102` | G6 | 删除虚假入口 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:51`, `docs/status/STAB-6102-AUTHORITY-AUDIT.md` |
| `STAB-6103` | G6 | Feature State 元数据 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:63`, `docs/status/STAB-6103-AUTHORITY-AUDIT.md` |
| `PROTO-6201` | G6 | 协议注册表 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:86`, `docs/status/PROTO-6201-AUTHORITY-AUDIT.md` |
| `PROTO-6202` | G6 | Reader/Writer 兼容测试 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:109`, `docs/status/PROTO-6202-AUTHORITY-AUDIT.md` |
| `PROTO-6203` | G6 | Semantic Hash 升级演练 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:123`, `docs/status/PROTO-6203-AUTHORITY-AUDIT.md` |
| `PROTO-6204` | G6 | CLI 与退出码冻结 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:136`, `docs/status/PROTO-6204-AUTHORITY-AUDIT.md` |
| `STD-6301` | G6 | 稳定标准库审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:164`, `docs/status/STD-6301-AUTHORITY-AUDIT.md` |
| `STD-6302` | G6 | 删除过度便利 API | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:180`, `docs/status/STD-6302-AUTHORITY-AUDIT.md` |
| `STD-6303` | G6 | Unicode 与中文编程稳定性 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:195`, `docs/status/STD-6303-AUTHORITY-AUDIT.md` |
| `PKG-6401` | G6 | 包发布协议 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:210`, `docs/status/PKG-6401-AUTHORITY-AUDIT.md` |
| `PKG-6402` | G6 | Hermetic Build | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:226`, `docs/status/PKG-6402-AUTHORITY-AUDIT.md` |
| `PKG-6403` | G6 | Registry 最小实现或推迟策略 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:239`, `docs/status/PKG-6403-AUTHORITY-AUDIT.md` |
| `PKG-6404` | G6 | 供应链攻击测试 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:248`, `docs/status/PKG-6404-AUTHORITY-AUDIT.md` |
| `COMPAT-6501` | G6 | 历史 Corpus | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:265`, `docs/status/COMPAT-6501-AUTHORITY-AUDIT.md` |
| `COMPAT-6502` | G6 | 1.0 Compiler 兼容矩阵 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:280`, `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md` |
| `COMPAT-6503` | G6 | Language Migration Tool | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:293`, `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |
| `COMPAT-6504` | G6 | 弃用政策 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:307`, `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md` |
| `REL-6601` | G6 | Fuzz 总覆盖盘点 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:321`, `docs/status/REL-6601-AUTHORITY-AUDIT.md`, `docs/testing/FUZZ-COVERAGE.md` |
| `REL-6601-SEED` | G6 | Seed fuzz inventory and corpus drift gate | S | Done | Accepted DEC-0041; see `cargo xtask fuzz verify`, `fuzz/README.md`, and `docs/status/REL-6601-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6602` | G6 | 故障注入 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:340`, `docs/status/REL-6602-AUTHORITY-AUDIT.md`, `docs/testing/FAULT-INJECTION.md` |
| `REL-6602-SEED` | G6 | Seed fault-matrix drift gate | S | Done | Accepted DEC-0042; see `cargo xtask fault verify` and `docs/status/REL-6602-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6603` | G6 | 安全审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:354`, `docs/status/REL-6603-AUTHORITY-AUDIT.md`, `docs/testing/SECURITY-AUDIT.md` |
| `REL-6603-SEED` | G6 | Seed security-audit matrix drift gate | S | Done | Accepted DEC-0043; see `cargo xtask security verify` and `docs/status/REL-6603-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6604` | G6 | 性能基线 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:368`, `docs/status/REL-6604-AUTHORITY-AUDIT.md`, `docs/testing/PERFORMANCE-BASELINE.md` |
| `REL-6604-SEED` | G6 | Seed performance-matrix drift gate | S | Done | Accepted DEC-0044; see `cargo xtask performance verify` and `docs/status/REL-6604-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6701` | G6 | 正式文档集 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:391`, `docs/status/DOC-6701-AUTHORITY-AUDIT.md`, `docs/testing/DOCUMENTATION-INVENTORY.md` |
| `DOC-6701-SEED` | G6 | Seed documentation-inventory drift gate | S | Done | Accepted DEC-0045; see `cargo xtask docs verify` and `docs/status/DOC-6701-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6702` | G6 | 双层示例 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:406`, `docs/status/DOC-6702-AUTHORITY-AUDIT.md`, `docs/testing/EXAMPLE-COVERAGE.md` |
| `DOC-6702-SEED` | G6 | Seed example-matrix drift gate | S | Done | Accepted DEC-0046; see `cargo xtask examples verify` and `docs/status/DOC-6702-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6703-SEED` | G6 | Seed bilingual tutorial coverage drift gate | S | Done | Accepted DEC-0047; see `cargo xtask tutorial verify`, `docs/testing/TUTORIAL-COVERAGE.md`, and `docs/status/DOC-6703-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6703` | G6 | Tutorial 与中文优先样例 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:417`, `docs/status/DOC-6703-AUTHORITY-AUDIT.md`, `docs/TUTORIAL.md` |
| `ZED-6801-SEED` | G6 | Seed Zed compatibility-matrix drift gate | S | Done | Accepted DEC-0048; see `cargo xtask zed verify`, `docs/testing/ZED-COMPATIBILITY-MATRIX.md`, and `docs/status/ZED-6801-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6801` | G6 | 兼容矩阵 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:430`, `docs/status/ZED-6801-AUTHORITY-AUDIT.md`, `docs/testing/ZED-COMPATIBILITY-MATRIX.md` |
| `ZED-6802-SEED` | G6 | Seed language-server discovery inventory drift gate | S | Done | Accepted DEC-0049; see `cargo xtask lsp verify`, `docs/testing/LSP-DISCOVERY-ACQUISITION.md`, and `docs/status/ZED-6802-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6802` | G6 | 语言服务器发现/获取 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:442`, `docs/status/ZED-6802-AUTHORITY-AUDIT.md`, `docs/testing/LSP-DISCOVERY-ACQUISITION.md` |
| `ZED-6803-SEED` | G6 | Seed Zed extension acceptance inventory drift gate | S | Done | Accepted DEC-0050; see `cargo xtask zed-extension verify`, `docs/testing/ZED-EXTENSION-ACCEPTANCE.md`, and `docs/status/ZED-6803-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6803` | G6 | 扩展完整功能验收 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:461`, `docs/status/ZED-6803-AUTHORITY-AUDIT.md`, `docs/testing/ZED-EXTENSION-ACCEPTANCE.md` |
| `ZED-6804-SEED` | G6 | Seed DAP status inventory drift gate | S | Done | Accepted DEC-0051; see `cargo xtask dap verify`, `docs/testing/DAP-STATUS.md`, and `docs/status/ZED-6804-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6804` | G6 | DAP 状态 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:473`, `docs/status/ZED-6804-AUTHORITY-AUDIT.md`, `docs/testing/DAP-STATUS.md` |
| `RC-6901-SEED` | G6 | Seed RC0 internal-freeze inventory drift gate | S | Done | Accepted DEC-0052; see `cargo xtask rc0 verify`, `docs/testing/RC0-INTERNAL-FREEZE.md`, and `docs/status/RC-6901-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6901` | G6 | RC0 内部冻结 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:479`, `docs/status/RC-6901-AUTHORITY-AUDIT.md`, `docs/testing/RC0-INTERNAL-FREEZE.md` |
| `RC-6902-SEED` | G6 | Seed RC1 public-validation inventory drift gate | S | Done | Accepted DEC-0053; see `cargo xtask rc1 verify`, `docs/testing/RC1-PUBLIC-VALIDATION.md`, and `docs/status/RC-6902-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6902` | G6 | RC1 公开验证 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:490`, `docs/status/RC-6902-AUTHORITY-AUDIT.md`, `docs/testing/RC1-PUBLIC-VALIDATION.md` |
| `RC-6903-SEED` | G6 | Seed RC3 independent-verification inventory drift gate | S | Done | Accepted DEC-0054; see `cargo xtask rc3 verify`, `docs/testing/RC3-INDEPENDENT-VERIFICATION.md`, and `docs/status/RC-6903-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6903` | G6 | 独立验证 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:502`, `docs/status/RC-6903-AUTHORITY-AUDIT.md`, `docs/testing/RC3-INDEPENDENT-VERIFICATION.md` |
| `RC-6904-SEED` | G6 | Seed RC2/final change-control inventory drift gate | S | Done | Accepted DEC-0055; see `cargo xtask rc2 verify`, `docs/testing/RC2-FINAL-CHANGE-CONTROL.md`, and `docs/status/RC-6904-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6904` | G6 | RC2 / Final | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:514`, `docs/status/RC-6904-AUTHORITY-AUDIT.md`, `docs/testing/RC2-FINAL-CHANGE-CONTROL.md` |
| `RC-6905-SEED` | G6 | Seed v1 release-artifact inventory drift gate | S | Done | Accepted DEC-0056; see `cargo xtask v1 verify`, `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md`, and `docs/status/RC-6905-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6905` | G6 | v1.0 发布物 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:524`, `docs/status/RC-6905-AUTHORITY-AUDIT.md`, `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md` |

## 5. 状态文件建议

`docs/status/implementation-status.toml`：

```toml
[[task]]
id = "GOV-0101"
state = "Ready"
owner = ""
branch = ""
depends_on = []
spec = ["ROADMAP-1.0#G0"]
updated = "2026-08-21"

[[task]]
id = "VM-1203"
state = "Done"
depends_on = ["VM-1201", "VM-1202"]

[[task]]
id = "VM-1204"
state = "Done"
depends_on = ["TEST-VM-0001", "VM-1201", "VM-1202", "VM-1203"]
```

CI 验证：

- ID 唯一；
- state 合法；
- `Done` 任务有 merged commit、tests、traceability；
- `InProgress` 任务有 owner/branch；
- `Ready` 的依赖均 Done；
- `BlockedSpec` 链接 spec-gap；
- 文档中的 Task ID 在 registry 中存在。

## 6. 每周/每批次更新

1. 合并完成项并记录证据；
2. 重算 Ready 集合；
3. 检查同一核心接口的并行写冲突；
4. 更新风险、规范缺口、协议版本；
5. 只从下一纵向切片选择任务，不按“最有趣功能”跳跃；
6. 每个里程碑出口生成支持矩阵 diff 和 traceability 报告。
