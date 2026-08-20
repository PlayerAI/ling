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
| 13 | `VM-1201` | bytecode RFC/模型 | VM 语义 RFC | 与 test agent | opcode/encoding/verifier contract |
| 14 | `INC-1401` | 增量 query ADR | 编译管线接口 | 与 corpus | query/invalidations 评审通过 |
| 15 | `LSP-2101` | LSP 生命周期骨架 | CompilerSession/VFS | 与 Zed query | initialize/shutdown fixtures |
| 16 | `LSP-2102` | UTF-16 position 协商 | LineIndex | 与 diagnostics | 中文/emoji/CRLF fixtures |
| 17 | `ZEXT-3301` | grammar-only Zed 扩展 | TS skeleton | 与 LSP | 本地 Zed 识别/高亮 |
| 18 | `VM-1202` | Core→bytecode 最小切片 | VM-1201 | 与 verifier | Hello World Core round-trip |
| 19 | `VM-1203` | 独立 decoder/verifier | VM-1201 | 与 lowering | malformed fuzz 无 panic |
| 20 | `PRJ-1102` | module discovery | PRJ-1101 | 与 VM | deterministic graph |
| 21 | `INC-1402` | VFS/revision | INC-1401 | 与 VM | overlay/revision tests |
| 22 | `LSP-2201` | Diagnostic adapter | LSP lifecycle + compiler diagnostics | 与 Zed | stable code/span/related info |
| 23 | `ZQ-3201` | 基础 highlights | TS grammar | 与 LSP | highlight fixtures |
| 24 | `ZQ-3202` | brackets | TS grammar | 与 ZQ-3201 | pair fixtures |
| 25 | `ZQ-3204` | outline | TS declarations | 与 query tasks | symbols visible in Zed |
| 26 | `VM-1204` | VM 基础执行 | VM-1202/1203 | 否 | interpreter differential |
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
G1：`TS-3101`～`TS-3108`、`ZQ-3201`～`ZQ-3203`、`PRJ-1101`～`PRJ-1106` Done；`PRJ-1108` Ready，可基于已冻结的 project/graph/lock 接口开展 property 与 fuzz；`PRJ-1107` 的 project `test`/`build` CLI 行为仍需先冻结，后续任务继续服从各自接口与 RFC 前置
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
| `PRJ-1107` | G1/Editor | Project API 与 CLI 接入 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:132` |
| `PRJ-1108` | G1/Editor | Project graph fuzz/property | — | Ready | `03-G1-V0.1-LIVING.md:140`；PRJ-1101～PRJ-1106 已冻结 manifest、path、graph、cycle 与 lock 测试接口 |
| `VM-1201` | G1/Editor | bytecode RFC 与模型 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:150` |
| `VM-1202` | G1/Editor | Checked Core → bytecode 最小 lowering | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:166` |
| `VM-1203` | G1/Editor | 独立 decoder/verifier | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:180` |
| `VM-1204` | G1/Editor | VM 基础执行 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:196` |
| `VM-1205` | G1/Editor | 函数、closure、recursion | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:205` |
| `VM-1206` | G1/Editor | Record、ADT、match | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:209` |
| `VM-1207` | G1/Editor | Mutable place 与基础 borrow | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:218` |
| `VM-1208` | G1/Editor | Effect/Capability/Fault | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:225` |
| `VM-1209` | G1/Editor | Interpreter ↔ VM differential | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:233` |
| `VM-1210` | G1/Editor | Fuzz 与资源限制 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:246` |
| `TRAIT-1301` | G1/Editor | Trait RFC 收口 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:260` |
| `TRAIT-1302` | G1/Editor | AST/HIR 表示 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:277` |
| `TRAIT-1303` | G1/Editor | Constraint collection | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:285` |
| `TRAIT-1304` | G1/Editor | Coherence/orphan checker | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:289` |
| `TRAIT-1305` | G1/Editor | Solver v0 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:296` |
| `TRAIT-1306` | G1/Editor | Checked Core 显式化 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:300` |
| `TRAIT-1307` | G1/Editor | Interpreter/VM lowering | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:304` |
| `TRAIT-1308` | G1/Editor | IDE 支持 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:308` |
| `TRAIT-1309` | G1/Editor | 性能与终止 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:316` |
| `INC-1401` | G1/Editor | Query boundary ADR | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:322` |
| `INC-1402` | G1/Editor | VFS 与 revision | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:326` |
| `INC-1403` | G1/Editor | Parse queries | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:335` |
| `INC-1404` | G1/Editor | Resolve/module queries | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:347` |
| `INC-1405` | G1/Editor | Type/effect queries | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:356` |
| `INC-1406` | G1/Editor | Semantic queries | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:360` |
| `INC-1407` | G1/Editor | Clean ↔ incremental equivalence | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:368` |
| `INC-1408` | G1/Editor | Deterministic parallel scheduling | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:379` |
| `INC-1409` | G1/Editor | Persistent cache（若 RFC 接受） | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:383` |
| `INC-1410` | G1/Editor | 增量性能基线 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:390` |
| `FMT-1501` | G1/Editor | Formatter preservation RFC | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:396` |
| `FMT-1502` | G1/Editor | Format IR | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:400` |
| `FMT-1503` | G1/Editor | 核心语法格式化 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:404` |
| `FMT-1504` | G1/Editor | Comment attachment | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:408` |
| `FMT-1505` | G1/Editor | 不完整源码 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:412` |
| `FMT-1506` | G1/Editor | 性质测试 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:416` |
| `FMT-1507` | G1/Editor | CLI/LSP 接入 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:425` |
| `FMT-1508` | G1/Editor | Audit 分离 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:433` |
| `CLI-1701` | G1/Editor | 命令模型统一 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:462` |
| `CLI-1702` | G1/Editor | 输出与退出码 | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:466` |
| `CLI-1703` | G1/Editor | `init` | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:470` |
| `CLI-1704` | G1/Editor | `test` | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:474` |
| `CLI-1705` | G1/Editor | `query/patch` | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:478` |
| `CLI-1706` | G1/Editor | Shell completion 与 help fixtures | — | Blocked by G0/interface | `03-G1-V0.1-LIVING.md:482` |
| `LSP-2101` | G1/Editor | 初始化与生命周期 | S | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:137` |
| `LSP-2102` | G1/Editor | Position encoding negotiation | S | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:145` |
| `LSP-2103` | G1/Editor | Open document overlay | M | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:151` |
| `LSP-2104` | G1/Editor | 增量文本变更 | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:170` |
| `LSP-2105` | G1/Editor | Workspace reload | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:182` |
| `LSP-2201` | G1/Editor | Compiler diagnostic adapter | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:188` |
| `LSP-2202` | G1/Editor | Push diagnostics v0 | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:204` |
| `LSP-2203` | G1/Editor | Pull diagnostics Preview | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:215` |
| `LSP-2204` | G1/Editor | Root-cause 与错误风暴控制 | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:219` |
| `LSP-2205` | G1/Editor | Diagnostic fixtures | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:227` |
| `IDE-2301` | G1/Editor | Document symbols | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:244` |
| `IDE-2302` | G1/Editor | Hover | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:248` |
| `IDE-2303` | G1/Editor | Go to definition/declaration/type definition | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:262` |
| `IDE-2304` | G1/Editor | References | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:266` |
| `IDE-2305` | G1/Editor | Prepare rename | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:270` |
| `IDE-2306` | G1/Editor | Rename | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:283` |
| `IDE-2307` | G1/Editor | Completion v0 | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:299` |
| `IDE-2308` | G1/Editor | Completion resolve | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:312` |
| `IDE-2309` | G1/Editor | Code actions | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:316` |
| `IDE-2310` | G1/Editor | Formatting | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:329` |
| `IDE-2311` | G1/Editor | Workspace symbols | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:333` |
| `LSP-2401` | G1/Editor | Token taxonomy RFC/decision | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:339` |
| `LSP-2402` | G1/Editor | Typed token generation | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:370` |
| `LSP-2403` | G1/Editor | Full 与 delta | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:374` |
| `LSP-2404` | G1/Editor | Semantic token fixtures | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:378` |
| `LSP-2501` | G1/Editor | Request snapshot | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:384` |
| `LSP-2502` | G1/Editor | Cancellation | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:388` |
| `LSP-2503` | G1/Editor | Debounce 与优先级 | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:395` |
| `LSP-2504` | G1/Editor | Memory/resource limits | — | Blocked by G0/interface | `04-LSP-IMPLEMENTATION.md:403` |
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
| `EFF-2101` | G2 | Effect 核心模型冻结 | M | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:81` |
| `EFF-2102` | G2 | Effect 推导和约束求解 | L | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:105` |
| `EFF-2103` | G2 | Handler Typed Core 表示 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:115` |
| `EFF-2104` | G2 | 解释器与 VM Handler 执行 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:131` |
| `EFF-2105` | G2 | Effect fuzz/property tests | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:143` |
| `TASK-2201` | G2 | Task 语法与 Checked Core | M | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:158` |
| `TASK-2202` | G2 | Task 状态机 Lowering | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:182` |
| `TASK-2203` | G2 | 结构化生命周期 Runtime | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:195` |
| `TASK-2204` | G2 | 确定性测试调度器 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:207` |
| `TASK-2205` | G2 | 生产本地调度器 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:221` |
| `TASK-2206` | G2 | Task conformance 与压力测试 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:235` |
| `ACT-2301` | G2 | Actor 身份与状态隔离 | M | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:250` |
| `ACT-2302` | G2 | 消息可发送性检查 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:263` |
| `ACT-2303` | G2 | 有界 Mailbox 与背压 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:273` |
| `ACT-2304` | G2 | Turn 与重入规则 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:290` |
| `ACT-2305` | G2 | Actor Runtime | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:300` |
| `ACT-2306` | G2 | Actor 性质与压力测试 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:313` |
| `SUP-2401` | G2 | Supervisor 模型 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:326` |
| `SUP-2402` | G2 | 重启预算与熔断 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:340` |
| `SUP-2403` | G2 | 监督测试 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:355` |
| `REP-2501` | G2 | Determinism Class | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:369` |
| `REP-2502` | G2 | Replay Log Schema | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:384` |
| `REP-2503` | G2 | Effect Recorder | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:401` |
| `REP-2504` | G2 | Replay Player | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:414` |
| `REP-2505` | G2 | 隐私、裁剪与损坏 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:429` |
| `REP-2506` | G2 | 跨进程重放验收 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:440` |
| `REM-2601` | G2 | RemoteRef 与 Endpoint | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:452` |
| `REM-2602` | G2 | Transport-neutral Envelope | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:466` |
| `REM-2603` | G2 | Delivery 语义 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:482` |
| `REM-2604` | G2 | 最小参考传输 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:497` |
| `REM-2605` | G2 | 安全与资源限制 | — | Blocked by v0.1 + RFC | `06-G2-V0.2-CONCURRENT.md:506` |
| `MEM-3101` | G3 | 类型分类模型 | M | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:67` |
| `MEM-3102` | G3 | Value 布局与 Copy/Move | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:88` |
| `MEM-3103` | G3 | Resource 定义与 Drop 契约 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:99` |
| `MEM-3104` | G3 | Managed 类型和 Island 边界 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:111` |
| `OWN-3201` | G3 | Place 与 Move Analysis | L | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:125` |
| `OWN-3202` | G3 | Borrow Exclusivity | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:152` |
| `OWN-3203` | G3 | Region Inference | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:172` |
| `OWN-3204` | G3 | 跨 `await` / Actor Turn 的 Borrow | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:186` |
| `OWN-3205` | G3 | Drop 顺序 Lowering | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:196` |
| `OWN-3206` | G3 | Ownership 诊断与修复 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:209` |
| `OWN-3207` | G3 | 负向 corpus 与 property tests | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:224` |
| `GC-3301` | G3 | 最小对象模型 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:241` |
| `GC-3302` | G3 | 第一版 Collector | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:256` |
| `GC-3303` | G3 | Managed 与 Native/FFI 边界 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:271` |
| `GC-3304` | G3 | Profile 检查 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:282` |
| `NIR-3401` | G3 | Native IR 设计 | L | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:291` |
| `NIR-3402` | G3 | Core → Native IR Lowering | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:308` |
| `NIR-3403` | G3 | IR Verifier | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:324` |
| `BACK-3501` | G3 | Backend 选择 Spike | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:341` |
| `BACK-3502` | G3 | Baseline Codegen | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:352` |
| `BACK-3503` | G3 | Runtime ABI | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:365` |
| `BACK-3504` | G3 | 基础优化与验证 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:380` |
| `BACK-3505` | G3 | Reproducible Native Build | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:393` |
| `FFI-3601` | G3 | FFI 声明模型 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:411` |
| `FFI-3602` | G3 | C ABI 最小互操作 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:431` |
| `FFI-3603` | G3 | Shim Generator | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:445` |
| `FFI-3604` | G3 | Target Primitive Package | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:460` |
| `FFI-3605` | G3 | FFI fuzz/sanitizer 套件 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:476` |
| `DIFF-3701` | G3 | 三方 Harness | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:492` |
| `DIFF-3702` | G3 | 已允许差异登记表 | — | Blocked by v0.2/G3 RFC | `07-G3-V0.3-NATIVE.md:512` |
| `DAP-3601` | G3+ | `zero dap --stdio` | — | Blocked by v0.2/G3 RFC | `05-ZED-EXTENSION.md:526` |
| `DAP-3602` | G3+ | Zed debugger registration | — | Blocked by v0.2/G3 RFC | `05-ZED-EXTENSION.md:530` |
| `DAP-3603` | G3+ | 能力阶段 | — | Blocked by v0.2/G3 RFC | `05-ZED-EXTENSION.md:538` |
| `KCHK-4101` | G4 | Kernel 允许能力矩阵 | M | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:69` |
| `KCHK-4102` | G4 | Kernel Effect 与 Capability 检查 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:90` |
| `KCHK-4103` | G4 | Shape、Index 与 Bounds | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:103` |
| `KCHK-4104` | G4 | Alias 和并行写冲突 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:113` |
| `KCHK-4105` | G4 | Kernel Core 与 Verifier | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:124` |
| `CPU-4201` | G4 | Scalar Reference Backend | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:137` |
| `CPU-4202` | G4 | Reference Trace | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:153` |
| `CPU-4203` | G4 | Kernel Corpus | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:167` |
| `SIMD-4301` | G4 | 向量化合法性分析 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:183` |
| `SIMD-4302` | G4 | Portable SIMD IR | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:198` |
| `SIMD-4303` | G4 | SIMD Differential | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:210` |
| `DBUF-4401` | G4 | Device 类型与 Capability | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:223` |
| `DBUF-4402` | G4 | Buffer Ownership | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:240` |
| `DBUF-4403` | G4 | Transfer Effect | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:252` |
| `DBUF-4404` | G4 | 同步模型 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:272` |
| `DIR-4501` | G4 | Device IR Schema | L | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:287` |
| `DIR-4502` | G4 | Kernel Core → Device IR | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:305` |
| `DIR-4503` | G4 | Device IR Canonicalization | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:317` |
| `GPU-4601` | G4 | Backend Spike 与选择 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:328` |
| `GPU-4602` | G4 | Backend Adapter | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:351` |
| `GPU-4603` | G4 | Launch 与 Runtime | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:365` |
| `GPU-4604` | G4 | 差分和硬件矩阵 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:378` |
| `GPU-4605` | G4 | 错误归一化 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:394` |
| `ACC-4701` | G4 | Accelerator Plugin Interface | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:412` |
| `ACC-4702` | G4 | Experimental 适配器 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:425` |
| `PLC-4801` | G4 | Placement 约束模型 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:431` |
| `PLC-4802` | G4 | 静态候选与运行时选择 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:448` |
| `PLC-4803` | G4 | Cost Model v0 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:466` |
| `PLC-4804` | G4 | `zero explain placement` | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:480` |
| `PLC-4805` | G4 | 设备二进制缓存 | — | Blocked by v0.3 + RFC | `08-G4-V0.4-HETEROGENEOUS.md:495` |
| `PROF-5101` | G5 | 机器可读 Profile | M | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:77` |
| `PROF-5102` | G5 | 禁止能力检查 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:108` |
| `PROF-5103` | G5 | Profile Composition | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:122` |
| `PROF-5104` | G5 | Profile Audit 与 LSP | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:126` |
| `BND-5201` | G5 | Bound 类型与表达式 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:140` |
| `BND-5202` | G5 | 循环和递归检查 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:156` |
| `BND-5203` | G5 | 内存预算 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:169` |
| `BND-5204` | G5 | 资源预算诊断 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:184` |
| `NODE-5301` | G5 | Node 语法与语义 | L | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:199` |
| `NODE-5302` | G5 | Node Checked Core | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:226` |
| `NODE-5303` | G5 | 静态调度 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:242` |
| `NODE-5304` | G5 | 虚拟时间参考 Runtime | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:254` |
| `NODE-5305` | G5 | Native Node Runtime | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:266` |
| `NODE-5306` | G5 | Node/Actor 边界 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:277` |
| `NODE-5307` | G5 | Node conformance | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:288` |
| `CTR-5401` | G5 | Contract 语法与 AST/Core | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:304` |
| `CTR-5402` | G5 | Contract 状态模型 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:320` |
| `CTR-5403` | G5 | Runtime Contract Check | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:335` |
| `CTR-5404` | G5 | Verification Condition Generation | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:347` |
| `CTR-5405` | G5 | Solver/Proof Checker Adapter | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:360` |
| `CTR-5406` | G5 | 优化器使用 Contract 的规则 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:371` |
| `CTR-5407` | G5 | Contract LSP/Zed | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:375` |
| `PROOF-5501` | G5 | Proof IR | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:386` |
| `PROOF-5502` | G5 | 独立 Checker | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:400` |
| `PROOF-5503` | G5 | 假设注册表 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:412` |
| `MC-5601` | G5 | 有限状态投影 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:431` |
| `MC-5602` | G5 | 探索引擎 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:446` |
| `MC-5603` | G5 | 报告语义 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:458` |
| `MC-5604` | G5 | Replay Counterexample | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:471` |
| `TIM-5701` | G5 | Timing IR 与 Path | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:477` |
| `TIM-5702` | G5 | 测量与静态分析分离 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:489` |
| `TIM-5703` | G5 | Deadline Check | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:503` |
| `EVD-5801` | G5 | Bundle Schema | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:518` |
| `EVD-5802` | G5 | 独立验证器 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:543` |
| `EVD-5803` | G5 | 可重复构建绑定 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:557` |
| `EVD-5804` | G5 | AI Provenance | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:566` |
| `CBK-5901` | G5 | 可信编译路线决策 | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:585` |
| `CBK-5902` | G5 | Lowering Validator | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:599` |
| `CBK-5903` | G5 | Critical Runtime/Target Package | — | Blocked by v0.4 + RFC | `09-G5-V0.5-CRITICAL.md:611` |
| `STAB-6101` | G6 | 逐项支持矩阵审计 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:32` |
| `STAB-6102` | G6 | 删除虚假入口 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:51` |
| `STAB-6103` | G6 | Feature State 元数据 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:63` |
| `PROTO-6201` | G6 | 协议注册表 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:86` |
| `PROTO-6202` | G6 | Reader/Writer 兼容测试 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:109` |
| `PROTO-6203` | G6 | Semantic Hash 升级演练 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:123` |
| `PROTO-6204` | G6 | CLI 与退出码冻结 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:136` |
| `STD-6301` | G6 | 稳定标准库审计 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:164` |
| `STD-6302` | G6 | 删除过度便利 API | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:180` |
| `STD-6303` | G6 | Unicode 与中文编程稳定性 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:195` |
| `PKG-6401` | G6 | 包发布协议 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:210` |
| `PKG-6402` | G6 | Hermetic Build | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:226` |
| `PKG-6403` | G6 | Registry 最小实现或推迟策略 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:239` |
| `PKG-6404` | G6 | 供应链攻击测试 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:248` |
| `COMPAT-6501` | G6 | 历史 Corpus | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:265` |
| `COMPAT-6502` | G6 | 1.0 Compiler 兼容矩阵 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:280` |
| `COMPAT-6503` | G6 | `zero migrate` | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:293` |
| `COMPAT-6504` | G6 | 弃用政策 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:307` |
| `REL-6601` | G6 | Fuzz 总覆盖盘点 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:321` |
| `REL-6602` | G6 | 故障注入 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:340` |
| `REL-6603` | G6 | 安全审计 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:354` |
| `REL-6604` | G6 | 性能基线 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:368` |
| `DOC-6701` | G6 | 正式文档集 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:391` |
| `DOC-6702` | G6 | 双层示例 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:406` |
| `DOC-6703` | G6 | Tutorial 与中文优先样例 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:417` |
| `ZED-6801` | G6 | 兼容矩阵 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:430` |
| `ZED-6802` | G6 | 语言服务器发现/获取 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:442` |
| `ZED-6803` | G6 | 扩展完整功能验收 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:461` |
| `ZED-6804` | G6 | DAP 状态 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:473` |
| `RC-6901` | G6 | RC0 内部冻结 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:479` |
| `RC-6902` | G6 | RC1 公开验证 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:490` |
| `RC-6903` | G6 | 独立验证 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:502` |
| `RC-6904` | G6 | RC2 / Final | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:514` |
| `RC-6905` | G6 | v1.0 发布物 | — | Blocked by G1–G5 exits | `10-G6-V1.0-STABILIZATION.md:524` |

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
updated = "2026-08-20"

[[task]]
id = "VM-1202"
state = "BlockedDependency"
depends_on = ["VM-1201"]
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
