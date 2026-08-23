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
| 22 | `LSP-2201` | Diagnostic adapter | LSP lifecycle + compiler diagnostics | 与 Zed | **Done**；Accepted `RFC-0031`；Experimental `ling.lsp.diagnostic/0.1`；implementation `9a5310fbe48fd40ae5c3d05c7720656361f0b08f` |
| 23 | `ZQ-3201` | 基础 highlights | TS grammar | 与 LSP | highlight fixtures |
| 24 | `ZQ-3202` | brackets | TS grammar | 与 ZQ-3201 | pair fixtures |
| 25 | `ZQ-3204` | outline | TS declarations | 与 query tasks | symbols visible in Zed |
| 26 | `VM-1204` | VM 基础执行 | VM-1202/1203 | 否 | **Done**；verifier-gated `ling-vm`、全 1.0 scalar operator/control flow、Capability preflight、budget/Fault/source-map 与 interpreter differential evidence |
| 27 | `LSP-2202` | push diagnostics | LSP-2201 | 与 hover | **Done**；Accepted RFC-0032；确定性 message-boundary debounce、完整 snapshot freshness、版本化 replace/clear 与 locked-offline compiler diagnostics |
| 28 | `ZEXT-3302` | Zed Wasm extension | grammar-only stable | 与 LSP | wasm32-wasip2 build |
| 29 | `ZEXT-3303` | 查找本地 zero | extension skeleton | 与 queries | setting/PATH behavior |
| 30 | `ZEXT-3304` | 启动 `zero lsp --stdio` | LSP command stable | 否 | diagnostics in Zed |
| 31 | `IDE-2301` | document symbols | semantic index | 与 IDE-2302 | protocol fixtures |
| 32 | `IDE-2302` | hover | typed core index | 与 symbols | types/effects/capabilities |
| 33 | `IDE-2303` | definition | resolved symbols | 与 hover | Unicode navigation |

## 3. 批次状态建议

```text
G0 Done：规范、协议、错误码、追踪、CI
G1：`TS-3101`～`TS-3108`、`ZQ-3201`～`ZQ-3203`、`PRJ-1101`～`PRJ-1108` 与 `VM-1201`～`VM-1210` Done；`PRJ-1107` 由 Accepted RFC-0025 完成显式 locked/offline 语义工程命令与 checked semantic artifact
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
| `PRJ-1107` | G1/Editor | Project API 与 CLI 接入 | M | Done | Accepted RFC-0025；显式 `ling.toml` selection、locked/offline semantic check/run/test/build、`ling.project.command/0.1` 与 canonical checked semantic artifact；见实现、集成测试、权威审计与实施报告 |
| `PRJ-1107-CHECK` | G1/Editor | Locked project graph check Preview | M | Done | Accepted RFC-0024; see `crates/ling-cli/tests/project_check.rs`, `tests/protocols/project-check/README.md`, and `docs/status/PRJ-1107-CHECK-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for the deferred project surface |
| `PRJ-1107-LOAD` | G1/Editor | Locked project snapshot boundary | S | Done | Accepted DEC-0058; see `crates/ling-project/src/workspace.rs`, `crates/ling-project/tests/locked_project.rs`, and `docs/status/PRJ-1107-LOAD-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for the deferred project surface |
| `PRJ-1107-SEMANTIC-SNAPSHOT` | G1/Editor | Internal locked-project semantic snapshot | M | Done | Accepted DEC-0083; see `crates/ling-db/src/project_snapshot.rs`, `CompilerDb::project_semantic_snapshot`, and `docs/status/PRJ-1107-SEMANTIC-SNAPSHOT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec for public project behavior |
| `PRJ-1107-CURRENT-EVIDENCE` | G1/Editor | Current project CLI/API boundary evidence | S | Done | Accepted DEC-0250; `cargo xtask project verify` composes the three bounded children and enforces five `BlockedSpec` public surfaces; parent remains BlockedSpec |
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
| `TRAIT-1308-CURRENT-EVIDENCE` | G1/Editor | Current Trait IDE boundary evidence | S | Done | Accepted DEC-0251; `cargo xtask trait-ide verify` composes the two bounded children and enforces six `BlockedSpec` public editor surfaces; parent remains BlockedSpec |
| `TRAIT-1309` | G1/Editor | 性能与终止 | — | BlockedSpec | `03-G1-V0.1-LIVING.md:316`, `docs/status/TRAIT-1309-AUTHORITY-AUDIT.md` |
| `TRAIT-1309-TERMINATION` | G1/Editor | Bounded Trait solver termination evidence | S | Done | Accepted `DEC-0068`; see `docs/status/TRAIT-1309-TERMINATION-IMPLEMENTATION-REPORT.md`; production benchmark/LSP contract remains BlockedSpec |
| `TRAIT-1309-CURRENT-EVIDENCE` | G1/Editor | Current Trait performance/termination evidence | S | Done | Accepted DEC-0252; `cargo xtask trait-performance verify` composes three accepted termination facts and enforces five `BlockedSpec` production surfaces; parent remains BlockedSpec |
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
| `FMT-1507` | G1/Editor | CLI/LSP 接入 | M | Done | Accepted DEC-0028, DEC-0057, and RFC-0026; Preview CLI plus bounded synchronous whole-document `textDocument/formatting`; range/on-type formatting, format-on-save, Workspace Edit, and Semantic Transaction remain deferred |
| `FMT-1507-CLI` | G1/Editor | Formatter CLI Preview slice | M | Done | DEC-0028; see `crates/ling-cli/src/main.rs`, `schemas/format/0.1/`, and `docs/status/FMT-1507-CLI-IMPLEMENTATION-REPORT.md`; bounded LSP formatting is completed by RFC-0026 |
| `FMT-1507-EDIT` | G1/Editor | Deterministic formatter whole-document edit projection | S | Done | Accepted DEC-0057; see `crates/ling-format/src/edit.rs` and `docs/status/FMT-1507-EDIT-IMPLEMENTATION-REPORT.md`; RFC-0026 reuses this value for whole-document LSP formatting while Workspace Edit remains deferred |
| `FMT-1508` | G1/Editor | Audit 分离 | S | Done | DEC-0015、DEC-0023；见 `crates/ling-format/src/author.rs` 的 canonical Audit Source byte-equivalence property 与实施报告；RFC-0026 retains the same Audit separation for LSP formatting |
| `CLI-1701` | G1/Editor | 命令模型统一 | — | Done | Accepted DEC-0253 composes the exact implemented command catalog, single parser/dispatcher, and existing checked service boundaries; planned commands and CLI-1702 through CLI-1706 remain separate |
| `CLI-1701-CATALOG` | G1/Editor | Internal current CLI command catalog | S | Done | Accepted DEC-0036; see `crates/ling-cli/src/command_catalog.rs` and `docs/status/CLI-1701-CATALOG-IMPLEMENTATION-REPORT.md`; DEC-0253 composes it into the completed current command model |
| `CLI-1702` | G1/Editor | 输出与退出码 | M | Done | Accepted DEC-0254; one non-LSP `OutputPolicy` now governs format, bilingual language order, diagnostic color, quiet/verbose behavior, channels, and unchanged exit classes; see the audit and implementation report |
| `CLI-1702-EXIT` | G1/Editor | Internal CLI exit-code catalog | S | Done | Accepted DEC-0037; see `crates/ling-cli/src/exit_catalog.rs` and `docs/status/CLI-1702-EXIT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1703` | G1/Editor | `init` | M | Done | Accepted DEC-0255 composes the completed DEC-0038 offline scaffold as the current full initializer; RFC-0002 forbids the stale manifest template field and unregistered optional editor files remain deferred |
| `CLI-1703-INIT` | G1/Editor | Offline `ling init` scaffold | M | Done | Accepted DEC-0038; see `crates/ling-cli/src/init.rs`, `schemas/init/0.1/`, and `docs/status/CLI-1703-INIT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1704` | G1/Editor | `test` | L | Done | Accepted DEC-0256 composes DEC-0039 standalone file/directory execution with RFC-0025's locked/offline single project-entry smoke test; test syntax and expanded runners remain deferred |
| `CLI-1704-FILE` | G1/Editor | Explicit standalone test-file runner Preview | M | Done | Accepted DEC-0039; verified/committed as `72d85d7de77f188b0706acde7a559169d4ac149e`; see `crates/ling-cli/src/test_runner.rs`, `schemas/test/0.1/`, and `docs/status/CLI-1704-TEST-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `CLI-1705` | G1/Editor | `query/patch` | L | Done | Accepted RFC-0027; exact-NFC read-only query plus stale-base/preserve-checked proposal-only patch with `committed: false`; see `crates/ling-cli/src/semantic_commands.rs`, protocol schemas, authority audit, and implementation report |
| `CLI-1706` | G1/Editor | Shell completion 与 help fixtures | M | Done | Accepted RFC-0028; deterministic static Bash/Zsh/Fish/PowerShell generators, canonical `ling.cli-completion/0.1` fixtures, parser-catalog parity, truthful help, and implementation report |
| `CLI-1706-HELP` | G1/Editor | Truthful implemented-command help fixture | S | Done | Accepted DEC-0040; see `crates/ling-cli/tests/help.rs` and `docs/status/CLI-1706-HELP-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `LSP-2101` | G1/Editor | 初始化与生命周期 | S | Done | Accepted RFC-0004 + DEC-0257; existing `ling lsp --stdio` framing/lifecycle/server-info/position/workspace implementation and fixtures complete the bounded parent without duplicating later editor surfaces |
| `LSP-2101-LIFECYCLE` | G1/Editor | LSP lifecycle Preview slice | S | Done | `RFC-0004`; verified/committed as `38d95fb7b91c2035bd2b1b4ebf864c1693050925`; see `crates/ling-lsp`, `tests/protocols/lsp-lifecycle`, and `docs/status/LSP-2101-LIFECYCLE-IMPLEMENTATION-REPORT.md`; document, diagnostic, edit, and transaction surfaces remain deferred |
| `LSP-2102` | G1/Editor | Position encoding negotiation | S | Done | Accepted DEC-0258 composes DEC-0029 strict SourceMap projection and RFC-0004 initialize negotiation; every implemented position path uses the shared conversion API, while downstream transaction features remain separate |
| `LSP-2102-SOURCE-MAP` | G1/Editor | SourceMap position projection | S | Done | `DEC-0029`; see `crates/ling-source/src/position.rs` and `docs/status/LSP-2102-SOURCE-MAP-IMPLEMENTATION-REPORT.md`; downstream transaction semantics remain separately governed |
| `LSP-2102-NEGOTIATION` | G1/Editor | LSP initialize position-encoding negotiation | S | Done | `RFC-0004` + `DEC-0029`; see `crates/ling-lsp/tests/position_encoding.rs` and `docs/status/LSP-2102-NEGOTIATION-IMPLEMENTATION-REPORT.md`; DEC-0258 closes the bounded parent |
| `LSP-2103` | G1/Editor | Open document overlay | M | Done | Accepted DEC-0259 composes RFC-0023 path-free URI/file state, full-text overlay precedence, monotonic versions, close fallback/removal, and dependency read-only enforcement; later incremental and transaction work remains separate |
| `LSP-2103-OVERLAY` | G1/Editor | LSP full-text overlay Preview slice | M | Done | Accepted RFC-0023; see `crates/ling-lsp/tests/overlay.rs` and `docs/status/LSP-2103-OVERLAY-IMPLEMENTATION-REPORT.md`; DEC-0259 closes the bounded parent while incremental edits and transactions remain downstream |
| `LSP-2104-UTF8-EDITS` | G1/Editor | Bounded internal UTF-8 edit application | S | Done | Accepted `DEC-0069`; consumed by Accepted `RFC-0029`; see `docs/status/LSP-2104-UTF8-EDITS-IMPLEMENTATION-REPORT.md` |
| `LSP-2104-POSITION-EDITS` | G1/Editor | Bounded internal position-edit projection | S | Done | Accepted `DEC-0070`; consumed by Accepted `RFC-0029`; see `docs/status/LSP-2104-POSITION-EDITS-IMPLEMENTATION-REPORT.md` |
| `LSP-2104` | G1/Editor | 增量文本变更 | — | Done | Accepted `RFC-0029`; implementation `492754b066da11e4ae2fe58774e5c7096e3703a5`; see `docs/status/LSP-2104-IMPLEMENTATION-REPORT.md` |
| `LSP-2105` | G1/Editor | Workspace reload | — | Done | Accepted `RFC-0030`; implementation `49994b9132ff22ae3fd17ab172476d020a79febe`; see `docs/status/LSP-2105-IMPLEMENTATION-REPORT.md` |
| `LSP-2105-WORKSPACE-SNAPSHOT` | G1/Editor | Bounded internal workspace-state snapshot | S | Done | Accepted `DEC-0071`; consumed by Accepted `RFC-0030`; see `docs/status/LSP-2105-WORKSPACE-SNAPSHOT-IMPLEMENTATION-REPORT.md` |
| `LSP-2201` | G1/Editor | Compiler diagnostic adapter | — | Done | Accepted `RFC-0031`; implementation `9a5310fbe48fd40ae5c3d05c7720656361f0b08f`; see `docs/status/LSP-2201-IMPLEMENTATION-REPORT.md` |
| `LSP-2201-DIAGNOSTIC-POSITION` | G1/Editor | Bounded internal diagnostic span projection | S | Done | Accepted `DEC-0072`; consumed by Accepted `RFC-0031`; see `docs/status/LSP-2201-DIAGNOSTIC-POSITION-IMPLEMENTATION-REPORT.md` |
| `LSP-2201-ORDERING` | G1/Editor | Internal canonical diagnostic ordering | S | Done | Accepted DEC-0034; consumed by Accepted `RFC-0031`; see `docs/status/LSP-2201-ORDERING-IMPLEMENTATION-REPORT.md` |
| `LSP-2202` | G1/Editor | Push diagnostics v0 | — | Done | Accepted RFC-0032; implementation `4914d2346f5647f2cdfad85ef4f1335bd44b9f12`; see `docs/status/LSP-2202-IMPLEMENTATION-REPORT.md` |
| `LSP-2202-BATCH` | G1/Editor | Internal immutable diagnostic batch | S | Done | Accepted DEC-0035; see `crates/ling-lsp/src/diagnostic_batch.rs` and `docs/status/LSP-2202-BATCH-IMPLEMENTATION-REPORT.md`; RFC-0032 separately implements the parent without broadening this child |
| `LSP-2203` | G1/Editor | Pull diagnostics Preview | — | Done | `04-LSP-IMPLEMENTATION.md:215`, `docs/RFC-0033.md`, `docs/status/LSP-2203-IMPLEMENTATION-REPORT.md` |
| `LSP-2204` | G1/Editor | Root-cause 与错误风暴控制 | — | Done | `RFC-0034`, `docs/status/LSP-2204-IMPLEMENTATION-REPORT.md`, `b70308c1e215fd2f4a4736aa56d7372c368af599` |
| `LSP-2205` | G1/Editor | Diagnostic fixtures | — | Done | `RFC-0035`, `docs/status/LSP-2205-IMPLEMENTATION-REPORT.md`, `93a58e9090ce5a3be17bcfb8569d7246ce7d71ec` |
| `IDE-2301` | G1/Editor | Document symbols | — | Done | Accepted `RFC-0036`; `docs/status/IDE-2301-IMPLEMENTATION-REPORT.md`; `7ab847af0336d5c3de32d55e66cc3d8a932f1080` |
| `IDE-2301-INDEX` | G1/Editor | Internal resolved-definition source-order index | S | Done | Accepted `DEC-0073`; see `crates/ling-db/src/definition_index.rs` and `docs/status/IDE-2301-INDEX-IMPLEMENTATION-REPORT.md`; public Document Symbols are separately complete under `RFC-0036` |
| `IDE-2302` | G1/Editor | Hover | — | Done | Accepted `RFC-0037`; `docs/status/IDE-2302-IMPLEMENTATION-REPORT.md`; `81116951f9203f8374e59ae4ef6e5cd155e5d5e6` |
| `IDE-2302-TYPED-INDEX` | G1/Editor | Internal typed-definition observation | S | Done | Accepted `DEC-0074`; see `crates/ling-db/src/typed_definition_index.rs` and `docs/status/IDE-2302-TYPED-INDEX-IMPLEMENTATION-REPORT.md`; public hover is separately complete under `RFC-0037` |
| `IDE-2303` | G1/Editor | Go to definition/declaration/type definition | M | Done | Accepted `RFC-0038`; Preview `ling.lsp.navigation/0.1`; implementation `5abd8034dfeac3ca4b3a7b25cb18c22bfb885ec6` |
| `IDE-2303-REFERENCE-INDEX` | G1/Editor | Internal resolved-reference target index | S | Done | Accepted `DEC-0075`; see `crates/ling-db/src/reference_index.rs` and `docs/status/IDE-2303-REFERENCE-INDEX-IMPLEMENTATION-REPORT.md`; consumed by completed `IDE-2303` under Accepted `RFC-0038` |
| `IDE-2304` | G1/Editor | References | M | Done | Accepted `RFC-0039`; Preview `ling.lsp.references/0.1`; implementation `a109de62480d70c2d0d0a48b1604c8a5e04d7307` |
| `IDE-2304-REVERSE-INDEX` | G1/Editor | Internal resolved-reference reverse index | S | Done | Accepted `DEC-0076`; see `crates/ling-db/src/reference_index.rs` and `docs/status/IDE-2304-REVERSE-INDEX-IMPLEMENTATION-REPORT.md`; consumed by completed `IDE-2304` under Accepted `RFC-0039` |
| `IDE-2305` | G1/Editor | Prepare rename | M | Done | Accepted `RFC-0040`; Preview `ling.lsp.prepare-rename/0.1`; implementation `9619693d5e2ae5c9ffd4ec05ef578606b87fcce9` |
| `IDE-2305-IDENTIFIER-OBSERVATION` | G1/Editor | Internal rename-identifier Unicode observation | S | Done | Accepted `DEC-0077`; see `crates/ling-db/src/rename_identifier.rs` and `docs/status/IDE-2305-IDENTIFIER-OBSERVATION-IMPLEMENTATION-REPORT.md`; consumed by completed `IDE-2305` under Accepted `RFC-0040` |
| `IDE-2306` | G1/Editor | Rename | L | Done | Accepted `RFC-0041`; Preview `ling.lsp.rename/0.1`; implementation `ecb6545fec5fa1f457ee9abf69c7354306ea1bb0` |
| `IDE-2306-REFERENCE-SPANS` | G1/Editor | Internal resolved-reference source-span observation | S | Done | Accepted `DEC-0078`; see `crates/ling-db/src/reference_span_index.rs` and `docs/status/IDE-2306-REFERENCE-SPANS-IMPLEMENTATION-REPORT.md`; consumed by completed `IDE-2306` under Accepted `RFC-0041` |
| `IDE-2307` | G1/Editor | Completion v0 | M | Done | Accepted `RFC-0042`; Preview `ling.lsp.completion/0.1`; implementation `360315e4ec52b7e19ecdb475629d0fe71c1594e4` |
| `IDE-2307-SOURCE-INDEX` | G1/Editor | Internal resolver completion-source inventory | S | Done | Accepted `DEC-0079`; see `crates/ling-db/src/completion_source_index.rs` and `docs/status/IDE-2307-SOURCE-INDEX-IMPLEMENTATION-REPORT.md`; consumed by completed `IDE-2307` under Accepted `RFC-0042` |
| `IDE-2308` | G1/Editor | Completion resolve | M | Done | Accepted `RFC-0043`; Public Preview `ling.lsp.completion/0.2` and `ling.lsp.completion-resolve/0.1`; implementation commit `523c9de626c4a320028e6457676e526bfa53f247`; see `crates/ling-lsp/src/completion_resolve.rs`, `crates/ling-lsp/tests/completion_resolve.rs`, and `docs/status/IDE-2308-IMPLEMENTATION-REPORT.md` |
| `IDE-2308-METADATA` | G1/Editor | Internal completion checked-metadata observation | S | Done | Accepted `DEC-0080`; consumed by completed `IDE-2308`; see `crates/ling-db/src/completion_metadata_index.rs` and `docs/status/IDE-2308-METADATA-IMPLEMENTATION-REPORT.md` |
| `IDE-2309` | G1/Editor | Code actions | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:316`, `docs/status/IDE-2309-AUTHORITY-AUDIT.md` |
| `IDE-2309-REPAIR-INDEX` | G1/Editor | Internal structured diagnostic repair index | S | Done | Accepted `DEC-0081`; see `crates/ling-diagnostics/src/repair_index.rs` and `docs/status/IDE-2309-REPAIR-INDEX-IMPLEMENTATION-REPORT.md`; public code actions remain BlockedSpec |
| `IDE-2310` | G1/Editor | Formatting | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:329`, `docs/status/IDE-2310-AUTHORITY-AUDIT.md` |
| `IDE-2311` | G1/Editor | Workspace symbols | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:333`, `docs/status/IDE-2311-AUTHORITY-AUDIT.md` |
| `IDE-2311-SOURCE-LOOKUPS` | G1/Editor | Internal exact workspace-symbol source lookups | S | Done | Accepted `DEC-0082`; see `crates/ling-db/src/definition_index.rs` and `docs/status/IDE-2311-SOURCE-LOOKUPS-IMPLEMENTATION-REPORT.md`; public workspace symbols remain BlockedSpec |
| `LSP-2401` | G1/Editor | Token taxonomy RFC/decision | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:339`, `docs/status/LSP-2401-AUTHORITY-AUDIT.md` |
| `LSP-2401-LEXICAL-SOURCE` | G1/Editor | Internal lexical token source index | S | Done | Accepted `DEC-0084`; see `crates/ling-db/src/token_source_index.rs` and `docs/status/LSP-2401-LEXICAL-SOURCE-IMPLEMENTATION-REPORT.md`; public semantic-token taxonomy remains BlockedSpec |
| `LSP-2402` | G1/Editor | Typed token generation | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:370`, `docs/status/LSP-2402-AUTHORITY-AUDIT.md` |
| `LSP-2402-CHECKED-IDENTITY` | G1/Editor | Internal checked-token identity observation | S | Done | Accepted `DEC-0085`; see `crates/ling-db/src/checked_token_source_index.rs` and `docs/status/LSP-2402-CHECKED-IDENTITY-IMPLEMENTATION-REPORT.md`; public typed semantic-token generation remains BlockedSpec |
| `LSP-2403` | G1/Editor | Full 与 delta | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:374`, `docs/status/LSP-2403-AUTHORITY-AUDIT.md` |
| `LSP-2403-SNAPSHOT-IDENTITY` | G1/Editor | Internal checked-token snapshot identity | S | Done | Accepted `DEC-0086`; see `crates/ling-db/src/checked_token_source_index.rs` and `docs/status/LSP-2403-SNAPSHOT-IDENTITY-IMPLEMENTATION-REPORT.md`; public full/delta transport remains BlockedSpec |
| `LSP-2404` | G1/Editor | Semantic token fixtures | — | BlockedSpec | `04-LSP-IMPLEMENTATION.md:378`, `docs/status/LSP-2404-AUTHORITY-AUDIT.md` |
| `LSP-2404-CHECKED-SOURCE-FIXTURES` | G1/Editor | Internal checked-token source fixture corpus | S | Done | Accepted `DEC-0087`; see `crates/ling-db/tests/checked_token_source.rs` and `docs/status/LSP-2404-CHECKED-SOURCE-FIXTURES-IMPLEMENTATION-REPORT.md`; public semantic-token fixtures remain BlockedSpec |
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
| `EFF-2104-REJECTION-GATE` | G2 | Internal unresolved-handler execution rejection gate | S | Done | Accepted `DEC-0088`; see `crates/ling-cli/tests/handler_boundary.rs` and `docs/status/EFF-2104-REJECTION-GATE-IMPLEMENTATION-REPORT.md`; handler runtime remains BlockedSpec |
| `EFF-2105` | G2 | Effect fuzz/property tests | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:143`, `docs/status/EFF-2105-AUTHORITY-AUDIT.md` |
| `TASK-2201` | G2 | Task 语法与 Checked Core | M | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:158`, `docs/status/TASK-2201-AUTHORITY-AUDIT.md` |
| `TASK-2201-TASK-SYNTAX-REJECTION` | G2 | Internal Task-shaped syntax rejection gate | S | Done | Accepted `DEC-0089`; see `crates/ling-cli/tests/task_boundary.rs` and `docs/status/TASK-2201-REJECTION-GATE-IMPLEMENTATION-REPORT.md`; Task semantics remain BlockedSpec |
| `TASK-2201-CORE-MODEL` | G2 | Internal Structured Task Checked-Core identity model | S | Done | Accepted `DEC-0091`; see `crates/ling-concurrency/src/lib.rs` and `docs/status/TASK-2201-CORE-MODEL-IMPLEMENTATION-REPORT.md`; source/Core and runtime semantics remain BlockedSpec |
| `TASK-2202` | G2 | Task 状态机 Lowering | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:182`, `docs/status/TASK-2202-AUTHORITY-AUDIT.md` |
| `TASK-2202-STATE-MACHINE-MODEL` | G2 | Internal Task state-machine identity model | S | Done | Accepted `DEC-0092`; see `crates/ling-concurrency/src/state_machine.rs` and `docs/status/TASK-2202-STATE-MACHINE-MODEL-IMPLEMENTATION-REPORT.md`; lowering/runtime semantics remain BlockedSpec |
| `TASK-2203` | G2 | 结构化生命周期 Runtime | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:195`, `docs/status/TASK-2203-AUTHORITY-AUDIT.md` |
| `TASK-2203-LIFECYCLE-OBSERVATION` | G2 | Internal Task lifecycle observation trace | S | Done | Accepted `DEC-0093`; see `crates/ling-concurrency/src/lifecycle.rs` and `docs/status/TASK-2203-LIFECYCLE-OBSERVATION-IMPLEMENTATION-REPORT.md`; runtime semantics remain BlockedSpec |
| `TASK-2204` | G2 | 确定性测试调度器 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:207`, `docs/status/TASK-2204-AUTHORITY-AUDIT.md` |
| `TASK-2204-SCHEDULER-OBSERVATION` | G2 | Internal Task scheduler observation trace | S | Done | Accepted `DEC-0094`; see `crates/ling-concurrency/src/scheduler.rs` and `docs/status/TASK-2204-SCHEDULER-OBSERVATION-IMPLEMENTATION-REPORT.md`; scheduler semantics remain BlockedSpec |
| `TASK-2205` | G2 | 生产本地调度器 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:221`, `docs/status/TASK-2205-AUTHORITY-AUDIT.md` |
| `TASK-2206` | G2 | Task conformance 与压力测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:235`, `docs/status/TASK-2206-AUTHORITY-AUDIT.md` |
| `ACT-2301` | G2 | Actor 身份与状态隔离 | M | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:250`, `docs/status/ACT-2301-AUTHORITY-AUDIT.md` |
| `ACT-2301-ACTOR-SYNTAX-REJECTION` | G2 | Internal Actor-shaped syntax rejection gate | S | Done | Accepted `DEC-0090`; see `crates/ling-cli/tests/actor_boundary.rs` and `docs/status/ACT-2301-ACTOR-SYNTAX-REJECTION-IMPLEMENTATION-REPORT.md`; Actor semantics remain BlockedSpec |
| `ACT-2301-IDENTITY-MODEL` | G2 | Internal Actor identity/reference model | S | Done | Accepted `DEC-0095`; see `crates/ling-concurrency/src/actor.rs` and `docs/status/ACT-2301-IDENTITY-MODEL-IMPLEMENTATION-REPORT.md`; turn/state/runtime semantics remain BlockedSpec |
| `ACT-2302` | G2 | 消息可发送性检查 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:263`, `docs/status/ACT-2302-AUTHORITY-AUDIT.md` |
| `ACT-2302-MESSAGE-SCHEMA-MODEL` | G2 | Internal Actor message-schema identity model | S | Done | Accepted `DEC-0096`; see `crates/ling-concurrency/src/message.rs` and `docs/status/ACT-2302-MESSAGE-SCHEMA-MODEL-IMPLEMENTATION-REPORT.md`; Sendable/ownership semantics remain BlockedSpec |
| `ACT-2303` | G2 | 有界 Mailbox 与背压 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:273`, `docs/status/ACT-2303-AUTHORITY-AUDIT.md` |
| `ACT-2303-MAILBOX-OBSERVATION` | G2 | Internal Actor mailbox observation | S | Done | Accepted `DEC-0097`; see `crates/ling-concurrency/src/mailbox.rs` and `docs/status/ACT-2303-MAILBOX-OBSERVATION-IMPLEMENTATION-REPORT.md`; capacity/backpressure/runtime semantics remain BlockedSpec |
| `ACT-2304` | G2 | Turn 与重入规则 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:290`, `docs/status/ACT-2304-AUTHORITY-AUDIT.md` |
| `ACT-2304-TURN-OBSERVATION` | G2 | Internal Actor turn observation | S | Done | Accepted `DEC-0098`; see `crates/ling-concurrency/src/turn.rs` and `docs/status/ACT-2304-TURN-OBSERVATION-IMPLEMENTATION-REPORT.md`; await/reentry/runtime semantics remain BlockedSpec |
| `ACT-2305` | G2 | Actor Runtime | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:300`, `docs/status/ACT-2305-AUTHORITY-AUDIT.md` |
| `ACT-2305-RUNTIME-OBSERVATION` | G2 | Internal Actor runtime observation | S | Done | Accepted `DEC-0099`; see `crates/ling-concurrency/src/runtime.rs` and `docs/status/ACT-2305-RUNTIME-OBSERVATION-IMPLEMENTATION-REPORT.md`; runtime/ABI semantics remain BlockedSpec |
| `ACT-2306` | G2 | Actor 性质与压力测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:313`, `docs/status/ACT-2306-AUTHORITY-AUDIT.md` |
| `ACT-2306-PROPERTY-OBSERVATION` | G2 | Internal Actor property observation | S | Done | Accepted `DEC-0100`; see `crates/ling-concurrency/src/property.rs` and `docs/status/ACT-2306-PROPERTY-OBSERVATION-IMPLEMENTATION-REPORT.md`; property/stress/runtime semantics remain BlockedSpec |
| `SUP-2401` | G2 | Supervisor 模型 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:326`, `docs/status/SUP-2401-AUTHORITY-AUDIT.md` |
| `SUP-2401-OBSERVATION` | G2 | Internal Supervisor observation | S | Done | Accepted `DEC-0101`; see `crates/ling-concurrency/src/supervisor.rs` and `docs/status/SUP-2401-OBSERVATION-IMPLEMENTATION-REPORT.md`; supervision/restart/Fault semantics remain BlockedSpec |
| `SUP-2402` | G2 | 重启预算与熔断 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:340`, `docs/status/SUP-2402-AUTHORITY-AUDIT.md` |
| `SUP-2402-OBSERVATION` | G2 | Internal restart-budget observation | S | Done | Accepted `DEC-0102`; see `crates/ling-concurrency/src/budget.rs` and `docs/status/SUP-2402-OBSERVATION-IMPLEMENTATION-REPORT.md`; budget/circuit/runtime semantics remain BlockedSpec |
| `SUP-2403` | G2 | 监督测试 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:355`, `docs/status/SUP-2403-AUTHORITY-AUDIT.md` |
| `SUP-2403-OBSERVATION` | G2 | Internal supervision test evidence | S | Done | Accepted `DEC-0103`; see `crates/ling-concurrency/tests/supervision_evidence.rs` and `docs/status/SUP-2403-OBSERVATION-IMPLEMENTATION-REPORT.md`; execution/fixture/outcome semantics remain BlockedSpec |
| `REP-2501` | G2 | Determinism Class | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:369`, `docs/status/REP-2501-AUTHORITY-AUDIT.md` |
| `REP-2501-OBSERVATION` | G2 | Internal determinism-class evidence | S | Done | Accepted `DEC-0104`; see `crates/ling-effects/tests/determinism_evidence.rs` and `docs/status/REP-2501-OBSERVATION-IMPLEMENTATION-REPORT.md`; class/replay/runtime semantics remain BlockedSpec |
| `REP-2502` | G2 | Replay Log Schema | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:384`, `docs/status/REP-2502-AUTHORITY-AUDIT.md` |
| `REP-2502-OBSERVATION` | G2 | Internal replay-schema field evidence | S | Done | Accepted `DEC-0105`; see `crates/ling-concurrency/tests/replay_schema_evidence.rs` and `docs/status/REP-2502-OBSERVATION-IMPLEMENTATION-REPORT.md`; wire/payload/replay semantics remain BlockedSpec |
| `REP-2503` | G2 | Effect Recorder | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:401`, `docs/status/REP-2503-AUTHORITY-AUDIT.md` |
| `REP-2503-OBSERVATION` | G2 | Internal effect-recorder boundary evidence | S | Done | Accepted `DEC-0106`; see `crates/ling-effects/tests/effect_recorder_evidence.rs` and `docs/status/REP-2503-OBSERVATION-IMPLEMENTATION-REPORT.md`; recording/payload/runtime semantics remain BlockedSpec |
| `REP-2504` | G2 | Replay Player | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:414`, `docs/status/REP-2504-AUTHORITY-AUDIT.md` |
| `REP-2504-OBSERVATION` | G2 | Internal replay-player boundary evidence | S | Done | Accepted `DEC-0107`; see `crates/ling-concurrency/tests/replay_player_evidence.rs` and `docs/status/REP-2504-OBSERVATION-IMPLEMENTATION-REPORT.md`; playback/checkpoint/runtime semantics remain BlockedSpec |
| `REP-2505` | G2 | 隐私、裁剪与损坏 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:429`, `docs/status/REP-2505-AUTHORITY-AUDIT.md` |
| `REP-2505-OBSERVATION` | G2 | Internal replay privacy/integrity boundary evidence | S | Done | Accepted `DEC-0108`; see `crates/ling-concurrency/tests/replay_privacy_evidence.rs` and `docs/status/REP-2505-OBSERVATION-IMPLEMENTATION-REPORT.md`; privacy/redaction/trimming/checksum/corruption/offline semantics remain BlockedSpec |
| `REP-2506` | G2 | 跨进程重放验收 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:440`, `docs/status/REP-2506-AUTHORITY-AUDIT.md` |
| `REP-2506-OBSERVATION` | G2 | Internal cross-process replay acceptance boundary evidence | S | Done | Accepted `DEC-0109`; see `crates/ling-concurrency/tests/replay_cross_process_evidence.rs` and `docs/status/REP-2506-OBSERVATION-IMPLEMENTATION-REPORT.md`; process/replay/equivalence/CI semantics remain BlockedSpec |
| `REM-2601` | G2 | RemoteRef 与 Endpoint | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:452`, `docs/status/REM-2601-AUTHORITY-AUDIT.md` |
| `REM-2601-OBSERVATION` | G2 | Internal RemoteRef and endpoint boundary evidence | S | Done | Accepted `DEC-0110`; see `crates/ling-concurrency/tests/remote_ref_evidence.rs` and `docs/status/REM-2601-OBSERVATION-IMPLEMENTATION-REPORT.md`; remote identity/protocol/delivery semantics remain BlockedSpec |
| `REM-2602` | G2 | Transport-neutral Envelope | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:466`, `docs/status/REM-2602-AUTHORITY-AUDIT.md` |
| `REM-2602-OBSERVATION` | G2 | Internal transport-neutral envelope boundary evidence | S | Done | Accepted `DEC-0111`; see `crates/ling-concurrency/tests/remote_envelope_evidence.rs` and `docs/status/REM-2602-OBSERVATION-IMPLEMENTATION-REPORT.md`; wire/transport/schema semantics remain BlockedSpec |
| `REM-2603` | G2 | Delivery 语义 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:482`, `docs/status/REM-2603-AUTHORITY-AUDIT.md` |
| `REM-2603-OBSERVATION` | G2 | Internal remote-delivery boundary evidence | S | Done | Accepted `DEC-0112`; see `crates/ling-concurrency/tests/remote_delivery_evidence.rs` and `docs/status/REM-2603-OBSERVATION-IMPLEMENTATION-REPORT.md`; delivery/retry/ordering/Fault semantics remain BlockedSpec |
| `REM-2604` | G2 | 最小参考传输 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:497`, `docs/status/REM-2604-AUTHORITY-AUDIT.md` |
| `REM-2604-OBSERVATION` | G2 | Internal reference-transport boundary evidence | S | Done | Accepted `DEC-0113`; see `crates/ling-concurrency/tests/remote_transport_evidence.rs` and `docs/status/REM-2604-OBSERVATION-IMPLEMENTATION-REPORT.md`; transport/codec/Capability/Fault semantics remain BlockedSpec |
| `REM-2605` | G2 | 安全与资源限制 | — | BlockedSpec | `06-G2-V0.2-CONCURRENT.md:506`, `docs/status/REM-2605-AUTHORITY-AUDIT.md` |
| `REM-2605-OBSERVATION` | G2 | Internal security and resource boundary evidence | S | Done | Accepted `DEC-0114`; see `crates/ling-concurrency/tests/remote_security_resource_evidence.rs` and `docs/status/REM-2605-OBSERVATION-IMPLEMENTATION-REPORT.md`; quota/authentication/authorization/replay/schema/runtime semantics remain BlockedSpec |
| `MEM-3101` | G3 | 类型分类模型 | M | BlockedSpec | `07-G3-V0.3-NATIVE.md:67`, `docs/status/MEM-3101-AUTHORITY-AUDIT.md` |
| `MEM-3101-SEED-VALUE` | G3 | Seed completed-type Value classification | S | Done | Accepted DEC-0061; see `docs/status/MEM-3101-SEED-VALUE-IMPLEMENTATION-REPORT.md`; Managed/Resource model remains in `MEM-3101` |
| `MEM-3102` | G3 | Value 布局与 Copy/Move | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:88`, `docs/status/MEM-3102-AUTHORITY-AUDIT.md` |
| `MEM-3102-OBSERVATION` | G3 | Internal Value-layout and Copy/Move boundary evidence | S | Done | Accepted `DEC-0115`; see `crates/ling-types/tests/memory_layout_evidence.rs` and `docs/status/MEM-3102-OBSERVATION-IMPLEMENTATION-REPORT.md`; layout/ownership/ABI semantics remain BlockedSpec |
| `MEM-3103` | G3 | Resource 定义与 Drop 契约 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:99`, `docs/status/MEM-3103-AUTHORITY-AUDIT.md` |
| `MEM-3103-OBSERVATION` | G3 | Internal Resource and Drop boundary evidence | S | Done | Accepted `DEC-0116`; see `crates/ling-effects/tests/resource_drop_evidence.rs` and `docs/status/MEM-3103-OBSERVATION-IMPLEMENTATION-REPORT.md`; ownership/Drop/cleanup/FFI semantics remain BlockedSpec |
| `MEM-3104` | G3 | Managed 类型和 Island 边界 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:111`, `docs/status/MEM-3104-AUTHORITY-AUDIT.md` |
| `MEM-3104-OBSERVATION` | G3 | Internal Managed-graph and island boundary evidence | S | Done | Accepted `DEC-0117`; see `crates/ling-types/tests/managed_island_evidence.rs` and `docs/status/MEM-3104-OBSERVATION-IMPLEMENTATION-REPORT.md`; graph/collection/isolation semantics remain BlockedSpec |
| `OWN-3201` | G3 | Place 与 Move Analysis | L | BlockedSpec | `07-G3-V0.3-NATIVE.md:125`, `docs/status/OWN-3201-AUTHORITY-AUDIT.md` |
| `OWN-3201-OBSERVATION` | G3 | Internal Place and Move-analysis boundary evidence | S | Done | Accepted `DEC-0118`; see `crates/ling-types/tests/place_move_evidence.rs` and `docs/status/OWN-3201-OBSERVATION-IMPLEMENTATION-REPORT.md`; ownership/dataflow/lifetime semantics remain BlockedSpec |
| `OWN-3202` | G3 | Borrow Exclusivity | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:152`, `docs/status/OWN-3202-AUTHORITY-AUDIT.md` |
| `OWN-3202-OBSERVATION` | G3 | Internal borrow-exclusivity boundary evidence | S | Done | Accepted `DEC-0119`; see `crates/ling-types/tests/borrow_exclusivity_evidence.rs` and `docs/status/OWN-3202-OBSERVATION-IMPLEMENTATION-REPORT.md`; borrow/lifetime/exclusivity semantics remain BlockedSpec |
| `OWN-3203` | G3 | Region Inference | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:172`, `docs/status/OWN-3203-AUTHORITY-AUDIT.md` |
| `OWN-3203-OBSERVATION` | G3 | Internal region-inference boundary evidence | S | Done | Accepted `DEC-0120`; see `crates/ling-types/tests/region_inference_evidence.rs` and `docs/status/OWN-3203-OBSERVATION-IMPLEMENTATION-REPORT.md`; region/lifetime semantics remain BlockedSpec |
| `OWN-3204` | G3 | 跨 `await` / Actor Turn 的 Borrow | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:186`, `docs/status/OWN-3204-AUTHORITY-AUDIT.md` |
| `OWN-3204-OBSERVATION` | G3 | Internal cross-suspension and Actor-turn boundary evidence | S | Done | Accepted `DEC-0121`; see `crates/ling-concurrency/tests/borrow_await_turn_evidence.rs` and `docs/status/OWN-3204-OBSERVATION-IMPLEMENTATION-REPORT.md`; await/Actor/borrow semantics remain BlockedSpec |
| `OWN-3205` | G3 | Drop 顺序 Lowering | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:196`, `docs/status/OWN-3205-AUTHORITY-AUDIT.md` |
| `OWN-3205-OBSERVATION` | G3 | Internal Drop-order and cleanup boundary evidence | S | Done | Accepted `DEC-0122`; see `crates/ling-effects/tests/drop_order_evidence.rs` and `docs/status/OWN-3205-OBSERVATION-IMPLEMENTATION-REPORT.md`; Resource/Drop/cleanup semantics remain BlockedSpec |
| `OWN-3206` | G3 | Ownership 诊断与修复 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:209`, `docs/status/OWN-3206-AUTHORITY-AUDIT.md` |
| `OWN-3206-OBSERVATION` | G3 | Internal ownership-diagnostic and repair boundary evidence | S | Done | Accepted `DEC-0123`; see `crates/ling-diagnostics/tests/ownership_diagnostic_evidence.rs` and `docs/status/OWN-3206-OBSERVATION-IMPLEMENTATION-REPORT.md`; ownership diagnostics/repairs remain BlockedSpec |
| `OWN-3207` | G3 | 负向 corpus 与 property tests | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:224`, `docs/status/OWN-3207-AUTHORITY-AUDIT.md` |
| `OWN-3207-OBSERVATION` | G3 | Internal ownership corpus and property boundary evidence | S | Done | Accepted `DEC-0124`; see `crates/ling-types/tests/ownership_corpus_evidence.rs` and `docs/status/OWN-3207-OBSERVATION-IMPLEMENTATION-REPORT.md`; ownership corpus/property semantics remain BlockedSpec |
| `GC-3301` | G3 | 最小对象模型 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:241`, `docs/status/GC-3301-AUTHORITY-AUDIT.md` |
| `GC-3301-OBSERVATION` | G3 | Internal Managed object-model boundary evidence | S | Done | Accepted `DEC-0125`; see `crates/ling-types/tests/managed_object_model_evidence.rs` and `docs/status/GC-3301-OBSERVATION-IMPLEMENTATION-REPORT.md`; object-model/runtime semantics remain BlockedSpec |
| `GC-3302` | G3 | 第一版 Collector | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:256`, `docs/status/GC-3302-AUTHORITY-AUDIT.md` |
| `GC-3302-OBSERVATION` | G3 | Internal Managed collector boundary evidence | S | Done | Accepted `DEC-0126`; see `crates/ling-concurrency/tests/managed_collector_evidence.rs` and `docs/status/GC-3302-OBSERVATION-IMPLEMENTATION-REPORT.md`; collector/heap/scheduler semantics remain BlockedSpec |
| `GC-3303` | G3 | Managed 与 Native/FFI 边界 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:271`, `docs/status/GC-3303-AUTHORITY-AUDIT.md` |
| `GC-3303-OBSERVATION` | G3 | Internal Managed/Native/FFI boundary evidence | S | Done | Accepted `DEC-0127`; see `crates/ling-concurrency/tests/managed_ffi_boundary_evidence.rs` and `docs/status/GC-3303-OBSERVATION-IMPLEMENTATION-REPORT.md`; interop/ABI/FFI semantics remain BlockedSpec |
| `GC-3304` | G3 | Profile 检查 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:282`, `docs/status/GC-3304-AUTHORITY-AUDIT.md` |
| `GC-3304-OBSERVATION` | G3 | Internal Managed Profile boundary evidence | S | Done | Accepted `DEC-0128`; see `crates/ling-types/tests/managed_profile_evidence.rs` and `docs/status/GC-3304-OBSERVATION-IMPLEMENTATION-REPORT.md`; Profile/no_gc semantics remain BlockedSpec |
| `NIR-3401` | G3 | Native IR 设计 | L | BlockedSpec | `07-G3-V0.3-NATIVE.md:291`, `docs/status/NIR-3401-AUTHORITY-AUDIT.md` |
| `NIR-3401-OBSERVATION` | G3 | Internal Native IR design boundary evidence | S | Done | Accepted `DEC-0129`; see `crates/ling-types/tests/native_ir_design_evidence.rs` and `docs/status/NIR-3401-OBSERVATION-IMPLEMENTATION-REPORT.md`; IR/ABI/lowering semantics remain BlockedSpec |
| `NIR-3402` | G3 | Core → Native IR Lowering | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:308`, `docs/status/NIR-3402-AUTHORITY-AUDIT.md` |
| `NIR-3402-OBSERVATION` | G3 | Internal Native IR lowering boundary evidence | S | Done | Accepted `DEC-0130`; see `crates/ling-types/tests/native_ir_lowering_evidence.rs` and `docs/status/NIR-3402-OBSERVATION-IMPLEMENTATION-REPORT.md`; lowering/ABI/differential semantics remain BlockedSpec |
| `NIR-3403` | G3 | IR Verifier | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:324`, `docs/status/NIR-3403-AUTHORITY-AUDIT.md` |
| `NIR-3403-OBSERVATION` | G3 | Internal Native IR verifier boundary evidence | S | Done | Accepted `DEC-0131`; see `crates/ling-types/tests/native_ir_verifier_evidence.rs` and `docs/status/NIR-3403-OBSERVATION-IMPLEMENTATION-REPORT.md`; verifier/NIR/diagnostic semantics remain BlockedSpec |
| `BACK-3501` | G3 | Backend 选择 Spike | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:341`, `docs/status/BACK-3501-AUTHORITY-AUDIT.md` |
| `BACK-3501-OBSERVATION` | G3 | Internal Native backend selection boundary evidence | S | Done | Accepted `DEC-0132`; see `crates/ling-types/tests/native_backend_selection_evidence.rs` and `docs/status/BACK-3501-OBSERVATION-IMPLEMENTATION-REPORT.md`; backend/toolchain/benchmark/support semantics remain BlockedSpec |
| `BACK-3502` | G3 | Baseline Codegen | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:352`, `docs/status/BACK-3502-AUTHORITY-AUDIT.md` |
| `BACK-3502-OBSERVATION` | G3 | Internal Native codegen boundary evidence | S | Done | Accepted `DEC-0133`; see `crates/ling-types/tests/native_codegen_evidence.rs` and `docs/status/BACK-3502-OBSERVATION-IMPLEMENTATION-REPORT.md`; emission/artifact/diagnostic/build semantics remain BlockedSpec |
| `BACK-3503` | G3 | Runtime ABI | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:365`, `docs/status/BACK-3503-AUTHORITY-AUDIT.md` |
| `BACK-3503-OBSERVATION` | G3 | Internal Native runtime ABI boundary evidence | S | Done | Accepted `DEC-0134`; see `crates/ling-types/tests/native_runtime_abi_evidence.rs` and `docs/status/BACK-3503-OBSERVATION-IMPLEMENTATION-REPORT.md`; ABI/layout/runtime/public-ABI semantics remain BlockedSpec |
| `BACK-3504` | G3 | 基础优化与验证 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:380`, `docs/status/BACK-3504-AUTHORITY-AUDIT.md` |
| `BACK-3504-OBSERVATION` | G3 | Internal Native optimization boundary evidence | S | Done | Accepted `DEC-0135`; see `crates/ling-types/tests/native_optimization_evidence.rs` and `docs/status/BACK-3504-OBSERVATION-IMPLEMENTATION-REPORT.md`; optimization/proof/diagnostic semantics remain BlockedSpec |
| `BACK-3505` | G3 | Reproducible Native Build | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:393`, `docs/status/BACK-3505-AUTHORITY-AUDIT.md` |
| `BACK-3505-OBSERVATION` | G3 | Internal Native reproducible-build boundary evidence | S | Done | Accepted `DEC-0136`; see `crates/ling-types/tests/native_reproducible_build_evidence.rs` and `docs/status/BACK-3505-OBSERVATION-IMPLEMENTATION-REPORT.md`; build/artifact/provenance/release semantics remain BlockedSpec |
| `FFI-3601` | G3 | FFI 声明模型 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:411`, `docs/status/FFI-3601-AUTHORITY-AUDIT.md` |
| `FFI-3601-OBSERVATION` | G3 | Internal FFI declaration boundary evidence | S | Done | Accepted `DEC-0137`; see `crates/ling-types/tests/ffi_declaration_evidence.rs` and `docs/status/FFI-3601-OBSERVATION-IMPLEMENTATION-REPORT.md`; declaration/ABI/ownership semantics remain BlockedSpec |
| `FFI-3602` | G3 | C ABI 最小互操作 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:431`, `docs/status/FFI-3602-AUTHORITY-AUDIT.md` |
| `FFI-3602-OBSERVATION` | G3 | Internal C ABI interoperability boundary evidence | S | Done | Accepted `DEC-0138`; see `crates/ling-types/tests/ffi_c_abi_evidence.rs` and `docs/status/FFI-3602-OBSERVATION-IMPLEMENTATION-REPORT.md`; C layout/linker/ownership semantics remain BlockedSpec |
| `FFI-3603` | G3 | Shim Generator | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:445`, `docs/status/FFI-3603-AUTHORITY-AUDIT.md` |
| `FFI-3603-OBSERVATION` | G3 | Internal FFI shim-generator boundary evidence | S | Done | Accepted `DEC-0139`; see `crates/ling-types/tests/ffi_shim_generator_evidence.rs` and `docs/status/FFI-3603-OBSERVATION-IMPLEMENTATION-REPORT.md`; generator/artifact/provenance semantics remain BlockedSpec |
| `FFI-3604` | G3 | Target Primitive Package | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:460`, `docs/status/FFI-3604-AUTHORITY-AUDIT.md` |
| `FFI-3604-OBSERVATION` | G3 | Internal Target Primitive Package boundary evidence | S | Done | Accepted `DEC-0140`; see `crates/ling-types/tests/target_primitive_package_evidence.rs` and `docs/status/FFI-3604-OBSERVATION-IMPLEMENTATION-REPORT.md`; package/capability/TCB semantics remain BlockedSpec |
| `FFI-3605` | G3 | FFI fuzz/sanitizer 套件 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:476`, `docs/status/FFI-3605-AUTHORITY-AUDIT.md` |
| `FFI-3605-OBSERVATION` | G3 | Internal FFI fuzz and sanitizer boundary evidence | S | Done | Accepted `DEC-0141`; see `crates/ling-types/tests/ffi_fuzz_sanitizer_evidence.rs` and `docs/status/FFI-3605-OBSERVATION-IMPLEMENTATION-REPORT.md`; fuzz/sanitizer/security semantics remain BlockedSpec |
| `DIFF-3701` | G3 | 三方 Harness | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:492`, `docs/status/DIFF-3701-AUTHORITY-AUDIT.md` |
| `DIFF-3701-OBSERVATION` | G3 | Internal differential-harness boundary evidence | S | Done | Accepted `DEC-0142`; see `crates/ling-types/tests/differential_harness_evidence.rs` and `docs/status/DIFF-3701-OBSERVATION-IMPLEMENTATION-REPORT.md`; engine/equivalence semantics remain BlockedSpec |
| `DIFF-3702` | G3 | 已允许差异登记表 | — | BlockedSpec | `07-G3-V0.3-NATIVE.md:512`, `docs/status/DIFF-3702-AUTHORITY-AUDIT.md` |
| `DIFF-3702-OBSERVATION` | G3 | Internal allowed-difference registry boundary evidence | S | Done | Accepted `DEC-0143`; see `crates/ling-types/tests/allowed_difference_registry_evidence.rs` and `docs/status/DIFF-3702-OBSERVATION-IMPLEMENTATION-REPORT.md`; registry/equivalence semantics remain BlockedSpec |
| `DAP-3601` | G3+ | `zero dap --stdio` | — | BlockedSpec | `05-ZED-EXTENSION.md:526`, `docs/status/DAP-3601-AUTHORITY-AUDIT.md` |
| `DAP-3601-OBSERVATION` | G3+ | Internal DAP debugger boundary evidence | S | Done | Accepted `DEC-0144`; see `crates/ling-types/tests/dap_debugger_boundary_evidence.rs` and `docs/status/DAP-3601-OBSERVATION-IMPLEMENTATION-REPORT.md`; debugger protocol/runtime semantics remain BlockedSpec |
| `DAP-3602` | G3+ | Zed debugger registration | — | BlockedSpec | `05-ZED-EXTENSION.md:530`, `docs/status/DAP-3602-AUTHORITY-AUDIT.md` |
| `DAP-3602-OBSERVATION` | G3+ | Internal Zed debugger registration boundary evidence | S | Done | Accepted `DEC-0145`; see `crates/ling-types/tests/zed_debugger_registration_evidence.rs` and `docs/status/DAP-3602-OBSERVATION-IMPLEMENTATION-REPORT.md`; extension/registration semantics remain BlockedSpec |
| `DAP-3603` | G3+ | 能力阶段 | — | BlockedSpec | `05-ZED-EXTENSION.md:538`, `docs/status/DAP-3603-AUTHORITY-AUDIT.md` |
| `DAP-3603-OBSERVATION` | G3+ | Internal staged debugger capability boundary evidence | S | Done | Accepted `DEC-0146`; see `crates/ling-types/tests/staged_debugger_capability_evidence.rs` and `docs/status/DAP-3603-OBSERVATION-IMPLEMENTATION-REPORT.md`; debugger/Task/Actor semantics remain BlockedSpec |
| `KCHK-4101` | G4 | Kernel 允许能力矩阵 | M | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:69`, `docs/status/KCHK-4101-AUTHORITY-AUDIT.md` |
| `KCHK-4101-OBSERVATION` | G4 | Internal Kernel capability-matrix boundary evidence | S | Done | Accepted `DEC-0147`; see `crates/ling-types/tests/kernel_capability_matrix_evidence.rs` and `docs/status/KCHK-4101-OBSERVATION-IMPLEMENTATION-REPORT.md`; Kernel/checker/device semantics remain BlockedSpec |
| `KCHK-4102` | G4 | Kernel Effect 与 Capability 检查 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:90`, `docs/status/KCHK-4102-AUTHORITY-AUDIT.md` |
| `KCHK-4102-OBSERVATION` | G4 | Internal Kernel Effect and Capability boundary evidence | S | Done | Accepted `DEC-0148`; see `crates/ling-types/tests/kernel_effect_capability_evidence.rs` and `docs/status/KCHK-4102-OBSERVATION-IMPLEMENTATION-REPORT.md`; Kernel checker/admission semantics remain BlockedSpec |
| `KCHK-4103` | G4 | Shape、Index 与 Bounds | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:103`, `docs/status/KCHK-4103-AUTHORITY-AUDIT.md` |
| `KCHK-4103-OBSERVATION` | G4 | Internal Kernel shape/index/bounds boundary evidence | S | Done | Accepted `DEC-0149`; see `crates/ling-types/tests/kernel_shape_index_bounds_evidence.rs` and `docs/status/KCHK-4103-OBSERVATION-IMPLEMENTATION-REPORT.md`; shape/index/bounds/device semantics remain BlockedSpec |
| `KCHK-4104` | G4 | Alias 和并行写冲突 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:113`, `docs/status/KCHK-4104-AUTHORITY-AUDIT.md` |
| `KCHK-4104-OBSERVATION` | G4 | Internal Kernel alias/parallel-write boundary evidence | S | Done | Accepted `DEC-0150`; see `crates/ling-types/tests/kernel_alias_parallel_write_evidence.rs` and `docs/status/KCHK-4104-OBSERVATION-IMPLEMENTATION-REPORT.md`; alias/race/ownership semantics remain BlockedSpec |
| `KCHK-4105` | G4 | Kernel Core 与 Verifier | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:124`, `docs/status/KCHK-4105-AUTHORITY-AUDIT.md` |
| `KCHK-4105-OBSERVATION` | G4 | Internal Kernel Core/verifier boundary evidence | S | Done | Accepted `DEC-0151`; see `crates/ling-types/tests/kernel_core_verifier_evidence.rs` and `docs/status/KCHK-4105-OBSERVATION-IMPLEMENTATION-REPORT.md`; Kernel Core/verifier semantics remain BlockedSpec |
| `CPU-4201` | G4 | Scalar Reference Backend | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:137`, `docs/status/CPU-4201-AUTHORITY-AUDIT.md` |
| `CPU-4201-OBSERVATION` | G4 | Internal CPU scalar-reference boundary evidence | S | Done | Accepted `DEC-0152`; see `crates/ling-types/tests/cpu_scalar_reference_evidence.rs` and `docs/status/CPU-4201-OBSERVATION-IMPLEMENTATION-REPORT.md`; scalar Kernel execution remains BlockedSpec |
| `CPU-4202` | G4 | Reference Trace | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:153`, `docs/status/CPU-4202-AUTHORITY-AUDIT.md` |
| `CPU-4202-OBSERVATION` | G4 | Internal CPU reference-trace boundary evidence | S | Done | Accepted `DEC-0153`; see `crates/ling-types/tests/cpu_reference_trace_evidence.rs` and `docs/status/CPU-4202-OBSERVATION-IMPLEMENTATION-REPORT.md`; trace semantics remain BlockedSpec |
| `CPU-4203` | G4 | Kernel Corpus | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:167`, `docs/status/CPU-4203-AUTHORITY-AUDIT.md` |
| `CPU-4203-OBSERVATION` | G4 | Internal Kernel corpus boundary evidence | S | Done | Accepted `DEC-0154`; see `crates/ling-types/tests/kernel_corpus_evidence.rs` and `docs/status/CPU-4203-OBSERVATION-IMPLEMENTATION-REPORT.md`; Kernel corpus semantics remain BlockedSpec |
| `SIMD-4301` | G4 | 向量化合法性分析 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:183`, `docs/status/SIMD-4301-AUTHORITY-AUDIT.md` |
| `SIMD-4301-OBSERVATION` | G4 | Internal SIMD legality boundary evidence | S | Done | Accepted `DEC-0155`; see `crates/ling-types/tests/simd_legality_evidence.rs` and `docs/status/SIMD-4301-OBSERVATION-IMPLEMENTATION-REPORT.md`; SIMD legality/fallback semantics remain BlockedSpec |
| `SIMD-4302` | G4 | Portable SIMD IR | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:198`, `docs/status/SIMD-4302-AUTHORITY-AUDIT.md` |
| `SIMD-4302-OBSERVATION` | G4 | Internal Portable SIMD IR boundary evidence | S | Done | Accepted `DEC-0156`; see `crates/ling-types/tests/portable_simd_ir_evidence.rs` and `docs/status/SIMD-4302-OBSERVATION-IMPLEMENTATION-REPORT.md`; Portable SIMD IR semantics remain BlockedSpec |
| `SIMD-4303` | G4 | SIMD Differential | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:210`, `docs/status/SIMD-4303-AUTHORITY-AUDIT.md` |
| `SIMD-4303-OBSERVATION` | G4 | Internal SIMD differential boundary evidence | S | Done | Accepted `DEC-0157`; see `crates/ling-types/tests/simd_differential_evidence.rs` and `docs/status/SIMD-4303-OBSERVATION-IMPLEMENTATION-REPORT.md`; differential semantics remain BlockedSpec |
| `DBUF-4401` | G4 | Device 类型与 Capability | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:223`, `docs/status/DBUF-4401-AUTHORITY-AUDIT.md` |
| `DBUF-4401-OBSERVATION` | G4 | Internal Device capability boundary evidence | S | Done | Accepted `DEC-0158`; see `crates/ling-types/tests/device_capability_evidence.rs` and `docs/status/DBUF-4401-OBSERVATION-IMPLEMENTATION-REPORT.md`; Device/capability semantics remain BlockedSpec |
| `DBUF-4402` | G4 | Buffer Ownership | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:240`, `docs/status/DBUF-4402-AUTHORITY-AUDIT.md` |
| `DBUF-4402-OBSERVATION` | G4 | Internal Buffer ownership boundary evidence | S | Done | Accepted `DEC-0159`; see `crates/ling-types/tests/buffer_ownership_evidence.rs` and `docs/status/DBUF-4402-OBSERVATION-IMPLEMENTATION-REPORT.md`; Buffer ownership semantics remain BlockedSpec |
| `DBUF-4403` | G4 | Transfer Effect | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:252`, `docs/status/DBUF-4403-AUTHORITY-AUDIT.md` |
| `DBUF-4403-OBSERVATION` | G4 | Internal Transfer Effect boundary evidence | S | Done | Accepted `DEC-0160`; see `crates/ling-types/tests/transfer_effect_evidence.rs` and `docs/status/DBUF-4403-OBSERVATION-IMPLEMENTATION-REPORT.md`; Transfer Effect semantics remain BlockedSpec |
| `DBUF-4404` | G4 | 同步模型 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:272`, `docs/status/DBUF-4404-AUTHORITY-AUDIT.md` |
| `DBUF-4404-OBSERVATION` | G4 | Internal Device synchronization boundary evidence | S | Done | Accepted `DEC-0161`; see `crates/ling-types/tests/device_synchronization_evidence.rs` and `docs/status/DBUF-4404-OBSERVATION-IMPLEMENTATION-REPORT.md`; synchronization semantics remain BlockedSpec |
| `DIR-4501` | G4 | Device IR Schema | L | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:287`, `docs/status/DIR-4501-AUTHORITY-AUDIT.md` |
| `DIR-4501-OBSERVATION` | G4 | Internal Device IR schema boundary evidence | S | Done | Accepted `DEC-0162`; see `crates/ling-types/tests/device_ir_schema_evidence.rs` and `docs/status/DIR-4501-OBSERVATION-IMPLEMENTATION-REPORT.md`; Device IR schema semantics remain BlockedSpec |
| `DIR-4502` | G4 | Kernel Core → Device IR | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:305`, `docs/status/DIR-4502-AUTHORITY-AUDIT.md` |
| `DIR-4502-OBSERVATION` | G4 | Internal Kernel-to-Device lowering boundary evidence | S | Done | Accepted `DEC-0163`; see `crates/ling-types/tests/kernel_device_lowering_evidence.rs` and `docs/status/DIR-4502-OBSERVATION-IMPLEMENTATION-REPORT.md`; lowering semantics remain BlockedSpec |
| `DIR-4503` | G4 | Device IR Canonicalization | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:317`, `docs/status/DIR-4503-AUTHORITY-AUDIT.md` |
| `DIR-4503-OBSERVATION` | G4 | Internal Device IR canonicalization boundary evidence | S | Done | Accepted `DEC-0164`; see `crates/ling-types/tests/device_ir_canonicalization_evidence.rs` and `docs/status/DIR-4503-OBSERVATION-IMPLEMENTATION-REPORT.md`; canonicalization semantics remain BlockedSpec |
| `GPU-4601` | G4 | Backend Spike 与选择 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:328`, `docs/status/GPU-4601-AUTHORITY-AUDIT.md` |
| `GPU-4601-OBSERVATION` | G4 | Internal backend spike and selection boundary evidence | S | Done | Accepted `DEC-0165`; see `crates/ling-types/tests/backend_spike_selection_evidence.rs` and `docs/status/GPU-4601-OBSERVATION-IMPLEMENTATION-REPORT.md`; backend selection remains BlockedSpec |
| `GPU-4602` | G4 | Backend Adapter | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:351`, `docs/status/GPU-4602-AUTHORITY-AUDIT.md` |
| `GPU-4602-OBSERVATION` | G4 | Internal backend adapter boundary evidence | S | Done | Accepted `DEC-0166`; see `crates/ling-types/tests/backend_adapter_evidence.rs` and `docs/status/GPU-4602-OBSERVATION-IMPLEMENTATION-REPORT.md`; adapter ABI and runtime semantics remain BlockedSpec |
| `GPU-4603` | G4 | Launch 与 Runtime | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:365`, `docs/status/GPU-4603-AUTHORITY-AUDIT.md` |
| `GPU-4603-OBSERVATION` | G4 | Internal launch and runtime boundary evidence | S | Done | Accepted `DEC-0167`; see `crates/ling-types/tests/launch_runtime_evidence.rs` and `docs/status/GPU-4603-OBSERVATION-IMPLEMENTATION-REPORT.md`; runtime semantics remain BlockedSpec |
| `GPU-4604` | G4 | 差分和硬件矩阵 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:378`, `docs/status/GPU-4604-AUTHORITY-AUDIT.md` |
| `GPU-4604-OBSERVATION` | G4 | Internal differential and hardware-matrix boundary evidence | S | Done | Accepted `DEC-0168`; see `crates/ling-types/tests/differential_hardware_matrix_evidence.rs` and `docs/status/GPU-4604-OBSERVATION-IMPLEMENTATION-REPORT.md`; differential and matrix semantics remain BlockedSpec |
| `GPU-4605` | G4 | 错误归一化 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:394`, `docs/status/GPU-4605-AUTHORITY-AUDIT.md` |
| `GPU-4605-OBSERVATION` | G4 | Internal error-normalization boundary evidence | S | Done | Accepted `DEC-0169`; see `crates/ling-types/tests/error_normalization_evidence.rs` and `docs/status/GPU-4605-OBSERVATION-IMPLEMENTATION-REPORT.md`; error normalization remains BlockedSpec |
| `ACC-4701` | G4 | Accelerator Plugin Interface | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:412`, `docs/status/ACC-4701-AUTHORITY-AUDIT.md` |
| `ACC-4701-OBSERVATION` | G4 | Internal accelerator-plugin interface boundary evidence | S | Done | Accepted `DEC-0170`; see `crates/ling-types/tests/accelerator_plugin_interface_evidence.rs` and `docs/status/ACC-4701-OBSERVATION-IMPLEMENTATION-REPORT.md`; plugin semantics remain BlockedSpec |
| `ACC-4702` | G4 | Experimental 适配器 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:425`, `docs/status/ACC-4702-AUTHORITY-AUDIT.md` |
| `ACC-4702-OBSERVATION` | G4 | Internal Experimental accelerator-adapter boundary evidence | S | Done | Accepted `DEC-0171`; see `crates/ling-types/tests/experimental_accelerator_adapter_evidence.rs` and `docs/status/ACC-4702-OBSERVATION-IMPLEMENTATION-REPORT.md`; Experimental adapter semantics remain BlockedSpec |
| `PLC-4801` | G4 | Placement 约束模型 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:431`, `docs/status/PLC-4801-AUTHORITY-AUDIT.md` |
| `PLC-4801-OBSERVATION` | G4 | Internal Placement-constraint boundary evidence | S | Done | Accepted `DEC-0172`; see `crates/ling-types/tests/placement_constraint_evidence.rs` and `docs/status/PLC-4801-OBSERVATION-IMPLEMENTATION-REPORT.md`; Placement syntax and solver remain BlockedSpec |
| `PLC-4802` | G4 | 静态候选与运行时选择 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:448`, `docs/status/PLC-4802-AUTHORITY-AUDIT.md` |
| `PLC-4802-OBSERVATION` | G4 | Internal Placement-selection boundary evidence | S | Done | Accepted `DEC-0173`; see `crates/ling-types/tests/placement_selection_evidence.rs` and `docs/status/PLC-4802-OBSERVATION-IMPLEMENTATION-REPORT.md`; selector/replay semantics remain BlockedSpec |
| `PLC-4803` | G4 | Cost Model v0 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:466`, `docs/status/PLC-4803-AUTHORITY-AUDIT.md` |
| `PLC-4803-OBSERVATION` | G4 | Internal Cost Model boundary evidence | S | Done | Accepted `DEC-0174`; see `crates/ling-types/tests/cost_model_evidence.rs` and `docs/status/PLC-4803-OBSERVATION-IMPLEMENTATION-REPORT.md`; estimator/benchmark semantics remain BlockedSpec |
| `PLC-4804` | G4 | `zero explain placement` | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:480`, `docs/status/PLC-4804-AUTHORITY-AUDIT.md` |
| `PLC-4804-OBSERVATION` | G4 | Internal Placement-explain boundary evidence | S | Done | Accepted `DEC-0175`; see `crates/ling-types/tests/placement_explain_evidence.rs` and `docs/status/PLC-4804-OBSERVATION-IMPLEMENTATION-REPORT.md`; explain command/schema semantics remain BlockedSpec |
| `PLC-4805` | G4 | 设备二进制缓存 | — | BlockedSpec | `08-G4-V0.4-HETEROGENEOUS.md:495`, `docs/status/PLC-4805-AUTHORITY-AUDIT.md` |
| `PLC-4805-OBSERVATION` | G4 | Internal Device Binary Cache boundary evidence | S | Done | Accepted `DEC-0176`; see `crates/ling-types/tests/device_binary_cache_evidence.rs` and `docs/status/PLC-4805-OBSERVATION-IMPLEMENTATION-REPORT.md`; cache/Device IR semantics remain BlockedSpec |
| `PROF-5101` | G5 | 机器可读 Profile | M | BlockedSpec | `09-G5-V0.5-CRITICAL.md:77`, `docs/status/PROF-5101-AUTHORITY-AUDIT.md` |
| `PROF-5101-OBSERVATION` | G5 | Internal Critical Profile boundary evidence | S | Done | Accepted `DEC-0177`; see `crates/ling-types/tests/critical_profile_evidence.rs` and `docs/status/PROF-5101-OBSERVATION-IMPLEMENTATION-REPORT.md`; profile schema/proof semantics remain BlockedSpec |
| `PROF-5102` | G5 | 禁止能力检查 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:108`, `docs/status/PROF-5102-AUTHORITY-AUDIT.md` |
| `PROF-5102-OBSERVATION` | G5 | Internal forbidden-capability boundary evidence | S | Done | Accepted `DEC-0178`; see `crates/ling-types/tests/forbidden_capability_evidence.rs` and `docs/status/PROF-5102-OBSERVATION-IMPLEMENTATION-REPORT.md`; checker/policy semantics remain BlockedSpec |
| `PROF-5103` | G5 | Profile Composition | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:122`, `docs/status/PROF-5103-AUTHORITY-AUDIT.md` |
| `PROF-5103-OBSERVATION` | G5 | Internal Profile Composition boundary evidence | S | Done | Accepted `DEC-0179`; see `crates/ling-types/tests/profile_composition_evidence.rs` and `docs/status/PROF-5103-OBSERVATION-IMPLEMENTATION-REPORT.md`; schema/algebra/identity semantics remain BlockedSpec |
| `PROF-5104` | G5 | Profile Audit 与 LSP | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:126`, `docs/status/PROF-5104-AUTHORITY-AUDIT.md` |
| `PROF-5104-OBSERVATION` | G5 | Internal Profile Audit/LSP boundary evidence | S | Done | Accepted `DEC-0180`; see `crates/ling-types/tests/profile_audit_lsp_evidence.rs` and `docs/status/PROF-5104-OBSERVATION-IMPLEMENTATION-REPORT.md`; checker/CLI/LSP semantics remain BlockedSpec |
| `BND-5201` | G5 | Bound 类型与表达式 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:140`, `docs/status/BND-5201-AUTHORITY-AUDIT.md` |
| `BND-5201-OBSERVATION` | G5 | Internal Bound types/expressions boundary evidence | S | Done | Accepted `DEC-0181`; see `crates/ling-types/tests/bound_types_expressions_evidence.rs` and `docs/status/BND-5201-OBSERVATION-IMPLEMENTATION-REPORT.md`; syntax/solver/resource semantics remain BlockedSpec |
| `BND-5202` | G5 | 循环和递归检查 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:156`, `docs/status/BND-5202-AUTHORITY-AUDIT.md` |
| `BND-5202-OBSERVATION` | G5 | Internal loop/recursion checks boundary evidence | S | Done | Accepted `DEC-0182`; see `crates/ling-types/tests/loop_recursion_checks_evidence.rs` and `docs/status/BND-5202-OBSERVATION-IMPLEMENTATION-REPORT.md`; termination/proof/transform semantics remain BlockedSpec |
| `BND-5203` | G5 | 内存预算 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:169`, `docs/status/BND-5203-AUTHORITY-AUDIT.md` |
| `BND-5203-OBSERVATION` | G5 | Internal memory-budget boundary evidence | S | Done | Accepted `DEC-0183`; see `crates/ling-types/tests/memory_budgets_evidence.rs` and `docs/status/BND-5203-OBSERVATION-IMPLEMENTATION-REPORT.md`; analyzer/model/target semantics remain BlockedSpec |
| `BND-5204` | G5 | 资源预算诊断 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:184`, `docs/status/BND-5204-AUTHORITY-AUDIT.md` |
| `BND-5204-OBSERVATION` | G5 | Internal resource-budget diagnostic boundary evidence | S | Done | Accepted `DEC-0184`; see `crates/ling-types/tests/resource_budget_diagnostics_evidence.rs` and `docs/status/BND-5204-OBSERVATION-IMPLEMENTATION-REPORT.md`; fact/schema/transaction semantics remain BlockedSpec |
| `NODE-5301` | G5 | Node 语法与语义 | L | BlockedSpec | `09-G5-V0.5-CRITICAL.md:199`, `docs/status/NODE-5301-AUTHORITY-AUDIT.md` |
| `NODE-5301-OBSERVATION` | G5 | Internal Node syntax/semantics boundary evidence | S | Done | Accepted `DEC-0185`; see `crates/ling-types/tests/node_syntax_semantics_evidence.rs` and `docs/status/NODE-5301-OBSERVATION-IMPLEMENTATION-REPORT.md`; grammar/Core/runtime/timing semantics remain BlockedSpec |
| `NODE-5302` | G5 | Node Checked Core | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:226`, `docs/status/NODE-5302-AUTHORITY-AUDIT.md` |
| `NODE-5302-OBSERVATION` | G5 | Internal Node Checked Core boundary evidence | S | Done | Accepted `DEC-0186`; see `crates/ling-types/tests/node_checked_core_evidence.rs` and `docs/status/NODE-5302-OBSERVATION-IMPLEMENTATION-REPORT.md`; schema/graph/fixed-point semantics remain BlockedSpec |
| `NODE-5303` | G5 | 静态调度 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:242`, `docs/status/NODE-5303-AUTHORITY-AUDIT.md` |
| `NODE-5303-OBSERVATION` | G5 | Internal Node static-scheduling boundary evidence | S | Done | Accepted `DEC-0187`; see `crates/ling-types/tests/node_static_scheduling_evidence.rs` and `docs/status/NODE-5303-OBSERVATION-IMPLEMENTATION-REPORT.md`; graph/bridge/WCET/manifest semantics remain BlockedSpec |
| `NODE-5304` | G5 | 虚拟时间参考 Runtime | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:254`, `docs/status/NODE-5304-AUTHORITY-AUDIT.md` |
| `NODE-5304-OBSERVATION` | G5 | Internal Node virtual-time runtime boundary evidence | S | Done | Accepted `DEC-0188`; see `crates/ling-types/tests/node_virtual_time_runtime_evidence.rs` and `docs/status/NODE-5304-OBSERVATION-IMPLEMENTATION-REPORT.md`; clock/trace/replay semantics remain BlockedSpec |
| `NODE-5305` | G5 | Native Node Runtime | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:266`, `docs/status/NODE-5305-AUTHORITY-AUDIT.md` |
| `NODE-5305-OBSERVATION` | G5 | Internal Node Native-runtime boundary evidence | S | Done | Accepted `DEC-0189`; see `crates/ling-types/tests/node_native_runtime_evidence.rs` and `docs/status/NODE-5305-OBSERVATION-IMPLEMENTATION-REPORT.md`; ABI/ownership/target/runtime semantics remain BlockedSpec |
| `NODE-5306` | G5 | Node/Actor 边界 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:277`, `docs/status/NODE-5306-AUTHORITY-AUDIT.md` |
| `NODE-5306-OBSERVATION` | G5 | Internal Node/Actor boundary evidence | S | Done | Accepted `DEC-0190`; see `crates/ling-types/tests/node_actor_boundary_evidence.rs` and `docs/status/NODE-5306-OBSERVATION-IMPLEMENTATION-REPORT.md`; envelope/queue/ownership/replay semantics remain BlockedSpec |
| `NODE-5307` | G5 | Node conformance | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:288`, `docs/status/NODE-5307-AUTHORITY-AUDIT.md` |
| `NODE-5307-OBSERVATION` | G5 | Internal Node conformance boundary evidence | S | Done | Accepted `DEC-0191`; see `crates/ling-types/tests/node_conformance_evidence.rs` and `docs/status/NODE-5307-OBSERVATION-IMPLEMENTATION-REPORT.md`; runner/manifest/oracle/runtime semantics remain BlockedSpec |
| `CTR-5401` | G5 | Contract 语法与 AST/Core | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:304`, `docs/status/CTR-5401-AUTHORITY-AUDIT.md` |
| `CTR-5401-OBSERVATION` | G5 | Internal Contract syntax/Core boundary evidence | S | Done | Accepted `DEC-0192`; see `crates/ling-types/tests/contract_syntax_core_evidence.rs` and `docs/status/CTR-5401-OBSERVATION-IMPLEMENTATION-REPORT.md`; grammar/Core/proof/runtime semantics remain BlockedSpec |
| `CTR-5402` | G5 | Contract 状态模型 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:320`, `docs/status/CTR-5402-AUTHORITY-AUDIT.md` |
| `CTR-5402-OBSERVATION` | G5 | Internal Contract status-model boundary evidence | S | Done | Accepted `DEC-0193`; see `crates/ling-types/tests/contract_status_model_evidence.rs` and `docs/status/CTR-5402-OBSERVATION-IMPLEMENTATION-REPORT.md`; lifecycle/schema/propagation semantics remain BlockedSpec |
| `CTR-5403` | G5 | Runtime Contract Check | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:335`, `docs/status/CTR-5403-AUTHORITY-AUDIT.md` |
| `CTR-5403-OBSERVATION` | G5 | Internal Contract runtime-check boundary evidence | S | Done | Accepted `DEC-0194`; see `crates/ling-types/tests/contract_runtime_check_evidence.rs` and `docs/status/CTR-5403-OBSERVATION-IMPLEMENTATION-REPORT.md`; evaluator/Fault/profile semantics remain BlockedSpec |
| `CTR-5404` | G5 | Verification Condition Generation | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:347`, `docs/status/CTR-5404-AUTHORITY-AUDIT.md` |
| `CTR-5404-OBSERVATION` | G5 | Internal Contract VC boundary evidence | S | Done | Accepted `DEC-0195`; see `crates/ling-types/tests/contract_vc_evidence.rs` and `docs/status/CTR-5404-OBSERVATION-IMPLEMENTATION-REPORT.md`; proof grammar/translation/soundness remain BlockedSpec |
| `CTR-5405` | G5 | Solver/Proof Checker Adapter | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:360`, `docs/status/CTR-5405-AUTHORITY-AUDIT.md` |
| `CTR-5405-OBSERVATION` | G5 | Internal Solver/Proof Checker boundary evidence | S | Done | Accepted `DEC-0196`; see `crates/ling-types/tests/solver_proof_checker_evidence.rs` and `docs/status/CTR-5405-OBSERVATION-IMPLEMENTATION-REPORT.md`; query/certificate/checker/TCB semantics remain BlockedSpec |
| `CTR-5406` | G5 | 优化器使用 Contract 的规则 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:371`, `docs/status/CTR-5406-AUTHORITY-AUDIT.md` |
| `CTR-5406-OBSERVATION` | G5 | Internal Contract optimizer boundary evidence | S | Done | Accepted `DEC-0197`; see `crates/ling-types/tests/contract_optimizer_evidence.rs` and `docs/status/CTR-5406-OBSERVATION-IMPLEMENTATION-REPORT.md`; trust/admission/transformation semantics remain BlockedSpec |
| `CTR-5407` | G5 | Contract LSP/Zed | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:375`, `docs/status/CTR-5407-AUTHORITY-AUDIT.md` |
| `CTR-5407-OBSERVATION` | G5 | Internal Contract LSP/Zed boundary evidence | S | Done | Accepted `DEC-0198`; see `crates/ling-types/tests/contract_lsp_zed_evidence.rs` and `docs/status/CTR-5407-OBSERVATION-IMPLEMENTATION-REPORT.md`; editor/projection/transaction semantics remain BlockedSpec |
| `PROOF-5501` | G5 | Proof IR | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:386`, `docs/status/PROOF-5501-AUTHORITY-AUDIT.md` |
| `PROOF-5501-OBSERVATION` | G5 | Internal Proof IR boundary evidence | S | Done | Accepted `DEC-0199`; see `crates/ling-types/tests/proof_ir_evidence.rs` and `docs/status/PROOF-5501-OBSERVATION-IMPLEMENTATION-REPORT.md`; grammar/kernel/translation semantics remain BlockedSpec |
| `PROOF-5502` | G5 | 独立 Checker | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:400`, `docs/status/PROOF-5502-AUTHORITY-AUDIT.md` |
| `PROOF-5502-OBSERVATION` | G5 | Internal Independent Checker boundary evidence | S | Done | Accepted `DEC-0200`; see `crates/ling-types/tests/independent_checker_evidence.rs` and `docs/status/PROOF-5502-OBSERVATION-IMPLEMENTATION-REPORT.md`; checker/kernel/result semantics remain BlockedSpec |
| `PROOF-5503` | G5 | 假设注册表 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:412`, `docs/status/PROOF-5503-AUTHORITY-AUDIT.md` |
| `PROOF-5503-OBSERVATION` | G5 | Internal Assumption Registry boundary evidence | S | Done | Accepted `DEC-0201`; see `crates/ling-types/tests/assumption_registry_evidence.rs` and `docs/status/PROOF-5503-OBSERVATION-IMPLEMENTATION-REPORT.md`; schema/lifecycle/proof semantics remain BlockedSpec |
| `MC-5601` | G5 | 有限状态投影 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:431`, `docs/status/MC-5601-AUTHORITY-AUDIT.md` |
| `MC-5601-OBSERVATION` | G5 | Internal Finite-State Projection boundary evidence | S | Done | Accepted `DEC-0202`; see `crates/ling-concurrency/tests/finite_state_projection_evidence.rs` and `docs/status/MC-5601-OBSERVATION-IMPLEMENTATION-REPORT.md`; concurrency/projection/model-check semantics remain BlockedSpec |
| `MC-5602` | G5 | 探索引擎 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:446`, `docs/status/MC-5602-AUTHORITY-AUDIT.md` |
| `MC-5602-OBSERVATION` | G5 | Internal Exploration Engine boundary evidence | S | Done | Accepted `DEC-0203`; see `crates/ling-concurrency/tests/exploration_engine_evidence.rs` and `docs/status/MC-5602-OBSERVATION-IMPLEMENTATION-REPORT.md`; state-hash/reduction/result semantics remain BlockedSpec |
| `MC-5603` | G5 | 报告语义 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:458`, `docs/status/MC-5603-AUTHORITY-AUDIT.md` |
| `MC-5603-OBSERVATION` | G5 | Internal Model-Check Report boundary evidence | S | Done | Accepted `DEC-0204`; see `crates/ling-concurrency/tests/model_check_report_evidence.rs` and `docs/status/MC-5603-OBSERVATION-IMPLEMENTATION-REPORT.md`; result/counterexample/protocol semantics remain BlockedSpec |
| `MC-5604` | G5 | Replay Counterexample | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:471`, `docs/status/MC-5604-AUTHORITY-AUDIT.md` |
| `MC-5604-OBSERVATION` | G5 | Internal Replay Counterexample boundary evidence | S | Done | Accepted `DEC-0205`; see `crates/ling-concurrency/tests/replay_counterexample_evidence.rs` and `docs/status/MC-5604-OBSERVATION-IMPLEMENTATION-REPORT.md`; converter/scheduler/runtime semantics remain BlockedSpec |
| `TIM-5701` | G5 | Timing IR 与 Path | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:477`, `docs/status/TIM-5701-AUTHORITY-AUDIT.md` |
| `TIM-5701-OBSERVATION` | G5 | Internal Timing IR and Path boundary evidence | S | Done | Accepted `DEC-0206`; see `crates/ling-types/tests/timing_ir_path_evidence.rs` and `docs/status/TIM-5701-OBSERVATION-IMPLEMENTATION-REPORT.md`; IR/cost/WCET semantics remain BlockedSpec |
| `TIM-5702` | G5 | 测量与静态分析分离 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:489`, `docs/status/TIM-5702-AUTHORITY-AUDIT.md` |
| `TIM-5702-OBSERVATION` | G5 | Internal timing-analysis separation boundary evidence | S | Done | Accepted `DEC-0207`; see `crates/ling-types/tests/timing_analysis_separation_evidence.rs` and `docs/status/TIM-5702-OBSERVATION-IMPLEMENTATION-REPORT.md`; result/WCET semantics remain BlockedSpec |
| `TIM-5703` | G5 | Deadline Check | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:503`, `docs/status/TIM-5703-AUTHORITY-AUDIT.md` |
| `TIM-5703-OBSERVATION` | G5 | Internal Deadline Check boundary evidence | S | Done | Accepted `DEC-0208`; see `crates/ling-types/tests/deadline_check_evidence.rs` and `docs/status/TIM-5703-OBSERVATION-IMPLEMENTATION-REPORT.md`; Node/schedulability/WCET semantics remain BlockedSpec |
| `EVD-5801` | G5 | Bundle Schema | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:518`, `docs/status/EVD-5801-AUTHORITY-AUDIT.md` |
| `EVD-5801-OBSERVATION` | G5 | Internal Evidence Bundle Schema boundary evidence | S | Done | Accepted `DEC-0209`; see `crates/ling-types/tests/evidence_bundle_schema_evidence.rs` and `docs/status/EVD-5801-OBSERVATION-IMPLEMENTATION-REPORT.md`; schema/verifier/trust semantics remain BlockedSpec |
| `EVD-5802` | G5 | 独立验证器 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:543`, `docs/status/EVD-5802-AUTHORITY-AUDIT.md` |
| `EVD-5802-OBSERVATION` | G5 | Internal Independent Evidence Verifier boundary evidence | S | Done | Accepted `DEC-0210`; see `crates/ling-types/tests/independent_evidence_verifier_evidence.rs` and `docs/status/EVD-5802-OBSERVATION-IMPLEMENTATION-REPORT.md`; bundle/trust/result semantics remain BlockedSpec |
| `EVD-5803` | G5 | 可重复构建绑定 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:557`, `docs/status/EVD-5803-AUTHORITY-AUDIT.md` |
| `EVD-5803-OBSERVATION` | G5 | Internal Reproducible Build Binding boundary evidence | S | Done | Accepted `DEC-0211`; see `crates/ling-types/tests/reproducible_build_binding_evidence.rs` and `docs/status/EVD-5803-OBSERVATION-IMPLEMENTATION-REPORT.md`; manifest/artifact/equivalence semantics remain BlockedSpec |
| `EVD-5804` | G5 | AI Provenance | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:566`, `docs/status/EVD-5804-AUTHORITY-AUDIT.md` |
| `EVD-5804-OBSERVATION` | G5 | Internal AI Provenance boundary evidence | S | Done | Accepted `DEC-0212`; see `crates/ling-types/tests/ai_provenance_evidence.rs` and `docs/status/EVD-5804-OBSERVATION-IMPLEMENTATION-REPORT.md`; provenance/privacy/approval semantics remain BlockedSpec |
| `CBK-5901` | G5 | 可信编译路线决策 | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:585`, `docs/status/CBK-5901-AUTHORITY-AUDIT.md` |
| `CBK-5901-OBSERVATION` | G5 | Internal Trusted Compiler Route boundary evidence | S | Done | Accepted `DEC-0213`; see `crates/ling-types/tests/trusted_compiler_route_evidence.rs` and `docs/status/CBK-5901-OBSERVATION-IMPLEMENTATION-REPORT.md`; Native/ABI/target/route semantics remain BlockedSpec |
| `CBK-5902` | G5 | Lowering Validator | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:599`, `docs/status/CBK-5902-AUTHORITY-AUDIT.md` |
| `CBK-5902-OBSERVATION` | G5 | Internal Lowering Validator boundary evidence | S | Done | Accepted `DEC-0214`; see `crates/ling-types/tests/lowering_validator_evidence.rs` and `docs/status/CBK-5902-OBSERVATION-IMPLEMENTATION-REPORT.md`; Native/IR/correspondence semantics remain BlockedSpec |
| `CBK-5903` | G5 | Critical Runtime/Target Package | — | BlockedSpec | `09-G5-V0.5-CRITICAL.md:611`, `docs/status/CBK-5903-AUTHORITY-AUDIT.md` |
| `CBK-5903-OBSERVATION` | G5 | Internal Critical Runtime/Target Package boundary evidence | S | Done | Accepted `DEC-0215`; see `crates/ling-types/tests/critical_runtime_target_package_evidence.rs` and `docs/status/CBK-5903-OBSERVATION-IMPLEMENTATION-REPORT.md`; schedule/resource/target/ABI semantics remain BlockedSpec |
| `STAB-6101` | G6 | 逐项支持矩阵审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:32`, `docs/status/STAB-6101-AUTHORITY-AUDIT.md` |
| `STAB-6101-OBSERVATION` | G6 | Internal Support-Matrix Item Audit boundary evidence | S | Done | Accepted `DEC-0216`; see `crates/ling-types/tests/support_matrix_item_audit_evidence.rs` and `docs/status/STAB-6101-OBSERVATION-IMPLEMENTATION-REPORT.md`; Stable candidate/promotion semantics remain BlockedSpec |
| `STAB-6102` | G6 | 删除虚假入口 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:51`, `docs/status/STAB-6102-AUTHORITY-AUDIT.md` |
| `STAB-6102-OBSERVATION` | G6 | Internal False-Entry-Point Audit boundary evidence | S | Done | Accepted `DEC-0217`; see `crates/ling-types/tests/false_entry_point_audit_evidence.rs`, `crates/ling-cli/tests/help.rs`, and `docs/status/STAB-6102-OBSERVATION-IMPLEMENTATION-REPORT.md`; deletion/migration authority remains BlockedSpec |
| `STAB-6103` | G6 | Feature State 元数据 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:63`, `docs/status/STAB-6103-AUTHORITY-AUDIT.md` |
| `STAB-6103-OBSERVATION` | G6 | Internal Feature-State Metadata boundary evidence | S | Done | Accepted `DEC-0218`; see `crates/ling-types/tests/feature_state_metadata_evidence.rs`, `tools/xtask/src/status.rs`, and `docs/status/STAB-6103-OBSERVATION-IMPLEMENTATION-REPORT.md`; public schema/lifecycle/consumer semantics remain BlockedSpec |
| `PROTO-6201` | G6 | 协议注册表 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:86`, `docs/status/PROTO-6201-AUTHORITY-AUDIT.md` |
| `PROTO-6201-OBSERVATION` | G6 | Internal Protocol Registry boundary evidence | S | Done | Accepted `DEC-0219`; see `crates/ling-types/tests/protocol_registry_evidence.rs`, `tools/xtask/src/protocols.rs`, and `docs/status/PROTO-6201-OBSERVATION-IMPLEMENTATION-REPORT.md`; Stable lifecycle/owner/universal compatibility semantics remain BlockedSpec |
| `PROTO-6202` | G6 | Reader/Writer 兼容测试 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:109`, `docs/status/PROTO-6202-AUTHORITY-AUDIT.md` |
| `PROTO-6202-OBSERVATION` | G6 | Internal Reader/Writer Compatibility boundary evidence | S | Done | Accepted `DEC-0220`; see `crates/ling-types/tests/reader_writer_compatibility_evidence.rs`, `tools/xtask/src/schema.rs`, and `docs/status/PROTO-6202-OBSERVATION-IMPLEMENTATION-REPORT.md`; N-1/migration/limit semantics remain BlockedSpec |
| `PROTO-6203` | G6 | Semantic Hash 升级演练 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:123`, `docs/status/PROTO-6203-AUTHORITY-AUDIT.md` |
| `PROTO-6203-OBSERVATION` | G6 | Internal Semantic Hash Upgrade Rehearsal boundary evidence | S | Done | Accepted `DEC-0221`; see `crates/ling-types/tests/semantic_hash_upgrade_rehearsal_evidence.rs`, `tools/xtask/src/schema.rs`, and `docs/status/PROTO-6203-OBSERVATION-IMPLEMENTATION-REPORT.md`; real algorithm migration, dual-reader, cache/lock, and replay/evidence semantics remain BlockedSpec |
| `PROTO-6204` | G6 | CLI 与退出码冻结 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:136`, `docs/status/PROTO-6204-AUTHORITY-AUDIT.md` |
| `PROTO-6204-OBSERVATION` | G6 | Internal CLI and Exit-Code Freeze boundary evidence | S | Done | Accepted `DEC-0222`; see `crates/ling-types/tests/cli_exit_freeze_evidence.rs`, `crates/ling-cli/src/command_catalog.rs`, `crates/ling-cli/src/exit_catalog.rs`, and `docs/status/PROTO-6204-OBSERVATION-IMPLEMENTATION-REPORT.md`; Stable 1.0 matrix and plan-only commands remain BlockedSpec |
| `STD-6301` | G6 | 稳定标准库审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:164`, `docs/status/STD-6301-AUTHORITY-AUDIT.md` |
| `STD-6301-OBSERVATION` | G6 | Internal Stable Standard Library Audit boundary evidence | S | Done | Accepted `DEC-0223`; see `crates/ling-types/tests/stable_standard_library_audit_evidence.rs`, `crates/ling-resolve/src/lib.rs`, `tools/xtask/src/support.rs`, and `docs/status/STD-6301-OBSERVATION-IMPLEMENTATION-REPORT.md`; packaged/Profile/Stable library semantics remain BlockedSpec |
| `STD-6302` | G6 | 删除过度便利 API | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:180`, `docs/status/STD-6302-AUTHORITY-AUDIT.md` |
| `STD-6302-OBSERVATION` | G6 | Internal Convenience API Removal Audit boundary evidence | S | Done | Accepted `DEC-0224`; see `crates/ling-types/tests/convenience_api_removal_audit_evidence.rs`, `crates/ling-resolve/src/lib.rs`, and `docs/status/STD-6302-OBSERVATION-IMPLEMENTATION-REPORT.md`; current removal set is empty and real lifecycle/migration semantics remain BlockedSpec |
| `STD-6303` | G6 | Unicode 与中文编程稳定性 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:195`, `docs/status/STD-6303-AUTHORITY-AUDIT.md` |
| `STD-6303-OBSERVATION` | G6 | Internal Unicode and Chinese-programming stability boundary evidence | S | Done | Accepted `DEC-0225`; see `crates/ling-unicode/tests/unicode_chinese_stability.rs`, `crates/ling-types/tests/unicode_chinese_stability_evidence.rs`, and `docs/status/STD-6303-OBSERVATION-IMPLEMENTATION-REPORT.md`; Unicode upgrade/localization/tool protocol semantics remain BlockedSpec |
| `PKG-6401` | G6 | 包发布协议 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:210`, `docs/status/PKG-6401-AUTHORITY-AUDIT.md` |
| `PKG-6401-OBSERVATION` | G6 | Local package and publication-exclusion boundary evidence | S | Done | Accepted `DEC-0226`; see `crates/ling-project/tests/package_publication_boundary.rs`, `crates/ling-types/tests/package_publication_boundary_evidence.rs`, and `docs/status/PKG-6401-OBSERVATION-IMPLEMENTATION-REPORT.md`; publication/registry/supply-chain semantics remain BlockedSpec |
| `PKG-6402` | G6 | Hermetic Build | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:226`, `docs/status/PKG-6402-AUTHORITY-AUDIT.md` |
| `PKG-6402-OBSERVATION` | G6 | Hermetic-build exclusion boundary evidence | S | Done | Accepted `DEC-0227`; see `crates/ling-project/tests/hermetic_build_boundary.rs`, `crates/ling-types/tests/hermetic_build_boundary_evidence.rs`, and `docs/status/PKG-6402-OBSERVATION-IMPLEMENTATION-REPORT.md`; build graph/executor/sandbox/artifact semantics remain BlockedSpec |
| `PKG-6403` | G6 | Registry 最小实现或推迟策略 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:239`, `docs/status/PKG-6403-AUTHORITY-AUDIT.md` |
| `PKG-6403-DEFERMENT` | G6 | Registry deferment strategy evidence | S | Done | Accepted `DEC-0228` selects registry Unsupported through Ling 1.0; see `tools/xtask/src/support.rs`, `crates/ling-types/tests/registry_deferment_evidence.rs`, and `docs/status/PKG-6403-DEFERMENT-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec because PKG-6402 is blocked and no registry is implemented |
| `PKG-6404` | G6 | 供应链攻击测试 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:248`, `docs/status/PKG-6404-AUTHORITY-AUDIT.md` |
| `PKG-6404-LOCAL` | G6 | Local supply-chain attack-boundary evidence | S | Done | Accepted `DEC-0229`; see `crates/ling-project/tests/supply_chain_boundary.rs`, `crates/ling-cache/src/lib.rs`, `crates/ling-types/tests/supply_chain_boundary_evidence.rs`, and `docs/status/PKG-6404-LOCAL-IMPLEMENTATION-REPORT.md`; registry/archive/signing/package-cache/build-sandbox attack semantics remain BlockedSpec |
| `COMPAT-6501` | G6 | 历史 Corpus | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:265`, `docs/status/COMPAT-6501-AUTHORITY-AUDIT.md` |
| `COMPAT-6501-SEED` | G6 | Seed historical-corpus freeze evidence | S | Done | Accepted `DEC-0230`; see `cargo xtask corpus verify`, `docs/governance/seed-corpus-freeze.toml`, and `docs/status/COMPAT-6501-SEED-IMPLEMENTATION-REPORT.md`; v0.1-v0.5 history, compatibility, and migration semantics remain BlockedSpec |
| `COMPAT-6502` | G6 | 1.0 Compiler 兼容矩阵 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:280`, `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md` |
| `COMPAT-6502-CURRENT` | G6 | Current compiler compatibility-boundary evidence | S | Done | Accepted `DEC-0231`; see `cargo xtask compatibility verify`, `docs/governance/compiler-compatibility-boundary.toml`, and `docs/status/COMPAT-6502-CURRENT-IMPLEMENTATION-REPORT.md`; actual 1.0, v0.1-v0.5, warning, migration, rejection, and N-1 semantics remain BlockedSpec |
| `COMPAT-6503` | G6 | Language Migration Tool | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:293`, `docs/status/COMPAT-6503-AUTHORITY-AUDIT.md` |
| `COMPAT-6503-READINESS` | G6 | Migration-tool readiness and absence evidence | S | Done | Accepted `DEC-0232`; see `cargo xtask migration verify`, `crates/ling-cli/src/command_catalog.rs`, and `docs/status/COMPAT-6503-READINESS-IMPLEMENTATION-REPORT.md`; version-pair, transformation, transaction, report, diagnostic, and CLI semantics remain BlockedSpec |
| `COMPAT-6504` | G6 | 弃用政策 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:307`, `docs/status/COMPAT-6504-AUTHORITY-AUDIT.md` |
| `COMPAT-6504-READINESS` | G6 | Deprecation-policy readiness boundary evidence | S | Done | Accepted `DEC-0233`; see `cargo xtask deprecation verify`, `docs/governance/deprecation-readiness.toml`, and `docs/status/COMPAT-6504-READINESS-IMPLEMENTATION-REPORT.md`; the public policy and six lifecycle commitments remain BlockedSpec |
| `REL-6601` | G6 | Fuzz 总覆盖盘点 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:321`, `docs/status/REL-6601-AUTHORITY-AUDIT.md`, `docs/testing/FUZZ-COVERAGE.md` |
| `REL-6601-SEED` | G6 | Seed fuzz inventory and corpus drift gate | S | Done | Accepted DEC-0041; see `cargo xtask fuzz verify`, `fuzz/README.md`, and `docs/status/REL-6601-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6601-SEMANTIC-SCHEMA` | G6 | Semantic Graph reader fuzz coverage | S | Done | Accepted `DEC-0234`; see `semantic_schema_bytes`, `cargo xtask fuzz verify`, and `docs/status/REL-6601-SEMANTIC-SCHEMA-IMPLEMENTATION-REPORT.md`; future-protocol harnesses and the G6 gate remain BlockedSpec |
| `REL-6602` | G6 | 故障注入 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:340`, `docs/status/REL-6602-AUTHORITY-AUDIT.md`, `docs/testing/FAULT-INJECTION.md` |
| `REL-6602-SEED` | G6 | Seed fault-matrix drift gate | S | Done | Accepted DEC-0042; see `cargo xtask fault verify` and `docs/status/REL-6602-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6602-LOCK-PERSISTENCE` | G6 | Lock-persistence fault injection | S | Done | Accepted `DEC-0235`; see private `LockPersistence`, `cargo xtask fault verify`, and `docs/status/REL-6602-LOCK-PERSISTENCE-IMPLEMENTATION-REPORT.md`; OS crash durability and eight future/process scenarios remain BlockedSpec |
| `REL-6603` | G6 | 安全审计 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:354`, `docs/status/REL-6603-AUTHORITY-AUDIT.md`, `docs/testing/SECURITY-AUDIT.md` |
| `REL-6603-SEED` | G6 | Seed security-audit matrix drift gate | S | Done | Accepted DEC-0043; see `cargo xtask security verify` and `docs/status/REL-6603-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6603-UNSAFE-POLICY` | G6 | Workspace unsafe-policy drift gate | S | Done | Accepted `DEC-0236`; see `cargo xtask security verify` and `docs/status/REL-6603-UNSAFE-POLICY-IMPLEMENTATION-REPORT.md`; dependency/generated-code/cross-target audits and the parent gate remain BlockedSpec |
| `REL-6604` | G6 | 性能基线 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:368`, `docs/status/REL-6604-AUTHORITY-AUDIT.md`, `docs/testing/PERFORMANCE-BASELINE.md` |
| `REL-6604-SEED` | G6 | Seed performance-matrix drift gate | S | Done | Accepted DEC-0044; see `cargo xtask performance verify` and `docs/status/REL-6604-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `REL-6604-ARTIFACT` | G6 | Performance-baseline artifact integrity gate | S | Done | Accepted `DEC-0237`; see `cargo xtask performance verify`, `INC-1410-PERFORMANCE-BASELINE.json`, and `docs/status/REL-6604-ARTIFACT-IMPLEMENTATION-REPORT.md`; timing thresholds and the parent gate remain BlockedSpec |
| `DOC-6701` | G6 | 正式文档集 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:391`, `docs/status/DOC-6701-AUTHORITY-AUDIT.md`, `docs/testing/DOCUMENTATION-INVENTORY.md` |
| `DOC-6701-SEED` | G6 | Seed documentation-inventory drift gate | S | Done | Accepted DEC-0045; see `cargo xtask docs verify` and `docs/status/DOC-6701-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6701-EVIDENCE-PATHS` | G6 | Formal inventory evidence-path gate | S | Done | Accepted `DEC-0238`; see `cargo xtask docs verify` and `docs/status/DOC-6701-EVIDENCE-PATHS-IMPLEMENTATION-REPORT.md`; content completeness and the parent gate remain BlockedSpec |
| `DOC-6702` | G6 | 双层示例 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:406`, `docs/status/DOC-6702-AUTHORITY-AUDIT.md`, `docs/testing/EXAMPLE-COVERAGE.md` |
| `DOC-6702-SEED` | G6 | Seed example-matrix drift gate | S | Done | Accepted DEC-0046; see `cargo xtask examples verify` and `docs/status/DOC-6702-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6702-EXECUTION-MANIFEST` | G6 | Seed example execution manifest | S | Done | Accepted `DEC-0239`; see `tests/examples/seed-cases.toml`, the shared CLI process test, and `docs/status/DOC-6702-EXECUTION-MANIFEST-IMPLEMENTATION-REPORT.md`; Stable 1.0 examples and the parent remain BlockedSpec |
| `DOC-6703-SEED` | G6 | Seed bilingual tutorial coverage drift gate | S | Done | Accepted DEC-0047; see `cargo xtask tutorial verify`, `docs/testing/TUTORIAL-COVERAGE.md`, and `docs/status/DOC-6703-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `DOC-6703-SEMANTIC-EQUIVALENCE` | G6 | Bilingual tutorial Semantic-shape equivalence | S | Done | Accepted `DEC-0240`; see the shared CLI process test and `docs/status/DOC-6703-SEMANTIC-EQUIVALENCE-IMPLEMENTATION-REPORT.md`; localization policy, Stable tutorial evidence, and the parent remain BlockedSpec |
| `DOC-6703` | G6 | Tutorial 与中文优先样例 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:417`, `docs/status/DOC-6703-AUTHORITY-AUDIT.md`, `docs/TUTORIAL.md` |
| `ZED-6801-SEED` | G6 | Seed Zed compatibility-matrix drift gate | S | Done | Accepted DEC-0048; see `cargo xtask zed verify`, `docs/testing/ZED-COMPATIBILITY-MATRIX.md`, and `docs/status/ZED-6801-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6801-CURRENT-EVIDENCE` | G6 | Current LSP/grammar/package compatibility evidence | S | Done | Accepted `DEC-0241`; see the locked Windows grammar run, structured package checks, and `docs/status/ZED-6801-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; Zed/cross-host support and the parent remain BlockedSpec |
| `ZED-6801` | G6 | 兼容矩阵 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:430`, `docs/status/ZED-6801-AUTHORITY-AUDIT.md`, `docs/testing/ZED-COMPATIBILITY-MATRIX.md` |
| `ZED-6802-SEED` | G6 | Seed language-server discovery inventory drift gate | S | Done | Accepted DEC-0049; see `cargo xtask lsp verify`, `docs/testing/LSP-DISCOVERY-ACQUISITION.md`, and `docs/status/ZED-6802-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6802-CURRENT-EVIDENCE` | G6 | Current Preview server/discovery boundary evidence | S | Done | Accepted `DEC-0242`; see the six-file structured/current evidence gate and `docs/status/ZED-6802-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; discovery/acquisition and the parent remain BlockedSpec |
| `ZED-6802` | G6 | 语言服务器发现/获取 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:442`, `docs/status/ZED-6802-AUTHORITY-AUDIT.md`, `docs/testing/LSP-DISCOVERY-ACQUISITION.md` |
| `ZED-6803-SEED` | G6 | Seed Zed extension acceptance inventory drift gate | S | Done | Accepted DEC-0050; see `cargo xtask zed-extension verify`, `docs/testing/ZED-EXTENSION-ACCEPTANCE.md`, and `docs/status/ZED-6803-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6803-CURRENT-EVIDENCE` | G6 | Current grammar/LSP/position acceptance evidence | S | Done | Accepted `DEC-0243`; see the composed Zed/discovery/position gate, passing locked Windows grammar suite, and `docs/status/ZED-6803-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; public editor features and the parent remain BlockedSpec |
| `ZED-6803` | G6 | 扩展完整功能验收 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:461`, `docs/status/ZED-6803-AUTHORITY-AUDIT.md`, `docs/testing/ZED-EXTENSION-ACCEPTANCE.md` |
| `ZED-6804-SEED` | G6 | Seed DAP status inventory drift gate | S | Done | Accepted DEC-0051; see `cargo xtask dap verify`, `docs/testing/DAP-STATUS.md`, and `docs/status/ZED-6804-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `ZED-6804-CURRENT-EVIDENCE` | G6 | Current DAP observation evidence | S | Done | Accepted `DEC-0244`; see the 180-boundary observation gate and `docs/status/ZED-6804-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; DAP implementation and the parent remain BlockedSpec |
| `ZED-6804` | G6 | DAP 状态 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:473`, `docs/status/ZED-6804-AUTHORITY-AUDIT.md`, `docs/testing/DAP-STATUS.md` |
| `RC-6901-SEED` | G6 | Seed RC0 internal-freeze inventory drift gate | S | Done | Accepted DEC-0052; see `cargo xtask rc0 verify`, `docs/testing/RC0-INTERNAL-FREEZE.md`, and `docs/status/RC-6901-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6901-CURRENT-EVIDENCE` | G6 | Current RC0 status/protocol evidence | S | Done | Accepted `DEC-0245`; see the composed status/protocol registry gate and `docs/status/RC-6901-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; no RC0 freeze or parent promotion |
| `RC-6901` | G6 | RC0 内部冻结 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:479`, `docs/status/RC-6901-AUTHORITY-AUDIT.md`, `docs/testing/RC0-INTERNAL-FREEZE.md` |
| `RC-6902-SEED` | G6 | Seed RC1 public-validation inventory drift gate | S | Done | Accepted DEC-0053; see `cargo xtask rc1 verify`, `docs/testing/RC1-PUBLIC-VALIDATION.md`, and `docs/status/RC-6902-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6902-CURRENT-EVIDENCE` | G6 | Current RC1 RC0/Zed boundary evidence | S | Done | Accepted `DEC-0246`; see the composed RC0/Zed acceptance gates and `docs/status/RC-6902-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; no public validation or parent promotion |
| `RC-6902` | G6 | RC1 公开验证 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:490`, `docs/status/RC-6902-AUTHORITY-AUDIT.md`, `docs/testing/RC1-PUBLIC-VALIDATION.md` |
| `RC-6903-SEED` | G6 | Seed RC3 independent-verification inventory drift gate | S | Done | Accepted DEC-0054; see `cargo xtask rc3 verify`, `docs/testing/RC3-INDEPENDENT-VERIFICATION.md`, and `docs/status/RC-6903-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6903-CURRENT-EVIDENCE` | G6 | Current RC3 upstream-boundary evidence | S | Done | Accepted `DEC-0247`; see the composed RC1→RC0 inventory gate and `docs/status/RC-6903-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; no independent review or parent promotion |
| `RC-6903` | G6 | 独立验证 | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:502`, `docs/status/RC-6903-AUTHORITY-AUDIT.md`, `docs/testing/RC3-INDEPENDENT-VERIFICATION.md` |
| `RC-6904-SEED` | G6 | Seed RC2/final change-control inventory drift gate | S | Done | Accepted DEC-0055; see `cargo xtask rc2 verify`, `docs/testing/RC2-FINAL-CHANGE-CONTROL.md`, and `docs/status/RC-6904-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6904-CURRENT-EVIDENCE` | G6 | Current RC2 upstream/protocol evidence | S | Done | Accepted `DEC-0248`; see the composed RC3→RC1→RC0 gate and `docs/status/RC-6904-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; no blocker approval or parent promotion |
| `RC-6904` | G6 | RC2 / Final | — | BlockedSpec | `10-G6-V1.0-STABILIZATION.md:514`, `docs/status/RC-6904-AUTHORITY-AUDIT.md`, `docs/testing/RC2-FINAL-CHANGE-CONTROL.md` |
| `RC-6905-SEED` | G6 | Seed v1 release-artifact inventory drift gate | S | Done | Accepted DEC-0056; see `cargo xtask v1 verify`, `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md`, and `docs/status/RC-6905-SEED-IMPLEMENTATION-REPORT.md`; parent remains BlockedSpec |
| `RC-6905-CURRENT-EVIDENCE` | G6 | Current v1 upstream/LSP/protocol evidence | S | Done | Accepted `DEC-0249`; see the composed RC2→RC0 gate and `docs/status/RC-6905-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md`; no artifact publication or parent promotion |
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
