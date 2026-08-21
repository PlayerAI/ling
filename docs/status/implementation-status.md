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
| `ACC-4701` | Accelerator Plugin Interface | `G4` | `M` | `BlockedSpec` | — | `` |
| `ACC-4702` | Experimental Accelerator Adapter | `G4` | `M` | `BlockedSpec` | `ACC-4701` | `` |
| `ACT-2301` | Actor identity and state isolation | `G2` | `M` | `BlockedSpec` | `TASK-2203` | `` |
| `ACT-2302` | Actor message sendability checking | `G2` | `L` | `BlockedSpec` | `ACT-2301` | `` |
| `ACT-2303` | Bounded mailbox and backpressure | `G2` | `L` | `BlockedSpec` | `ACT-2302` | `` |
| `ACT-2304` | Actor turn and reentry rules | `G2` | `L` | `BlockedSpec` | `ACT-2303` | `` |
| `ACT-2305` | Actor runtime | `G2` | `L` | `BlockedSpec` | `ACT-2304` | `` |
| `ACT-2306` | Actor properties and stress tests | `G2` | `L` | `BlockedSpec` | `ACT-2305` | `` |
| `BACK-3501` | Native backend selection spike | `G3` | `M` | `BlockedSpec` | `NIR-3403` | `` |
| `BACK-3502` | Baseline Native codegen | `G3` | `L` | `BlockedSpec` | `BACK-3501` | `` |
| `BACK-3503` | Native runtime ABI | `G3` | `L` | `BlockedSpec` | `BACK-3502` | `` |
| `BACK-3504` | Native optimization and verification | `G3` | `L` | `BlockedSpec` | `BACK-3503` | `` |
| `BACK-3505` | Reproducible Native build | `G3` | `M` | `BlockedSpec` | `BACK-3504` | `` |
| `BASE-0001` | 仓库基线盘点与执行计划落位 | `G0` | `S` | `Done` | — | `aa8c02894bd2fdd696ab60c97423d07c0ce9614a` |
| `BND-5201` | Bound Types and Expressions | `G5` | `M` | `BlockedSpec` | `PROF-5104` | `` |
| `BND-5202` | Loop and Recursion Checks | `G5` | `M` | `BlockedSpec` | `BND-5201` | `` |
| `BND-5203` | Memory Budget Analysis | `G5` | `M` | `BlockedSpec` | `BND-5202` | `` |
| `BND-5204` | Resource Budget Diagnostics | `G5` | `M` | `BlockedSpec` | `BND-5203` | `` |
| `CLI-1701` | Unified CLI command model | `G1` | `L` | `BlockedSpec` | `FMT-1507`, `PRJ-1107` | `` |
| `CLI-1702` | CLI output and exit behavior | `G1` | `M` | `BlockedSpec` | `CLI-1701` | `` |
| `CLI-1703` | Project initialization command | `G1` | `M` | `BlockedSpec` | `CLI-1701`, `PRJ-1107` | `` |
| `CLI-1704` | Project test command | `G1` | `L` | `BlockedSpec` | `CLI-1701`, `PRJ-1107` | `` |
| `CLI-1705` | Semantic query and patch commands | `G1` | `L` | `BlockedSpec` | `CLI-1701`, `PRJ-1107` | `` |
| `CLI-1706` | Shell completion and help fixtures | `G1` | `M` | `BlockedSpec` | `CLI-1701`, `CLI-1702` | `` |
| `CPU-4201` | Scalar Reference Backend | `G4` | `L` | `BlockedSpec` | `KCHK-4105` | `` |
| `CPU-4202` | Reference Trace | `G4` | `M` | `BlockedSpec` | `CPU-4201` | `` |
| `CPU-4203` | Kernel Corpus | `G4` | `M` | `BlockedSpec` | `CPU-4202` | `` |
| `CTR-5401` | Contract Syntax and AST/Core | `G5` | `M` | `BlockedSpec` | `NODE-5307` | `` |
| `CTR-5402` | Contract Status Model | `G5` | `M` | `BlockedSpec` | `CTR-5401` | `` |
| `CTR-5403` | Runtime Contract Check | `G5` | `M` | `BlockedSpec` | `CTR-5402` | `` |
| `CTR-5404` | Verification Condition Generation | `G5` | `L` | `BlockedSpec` | `CTR-5403` | `` |
| `CTR-5405` | Solver/Proof Checker Adapter | `G5` | `L` | `BlockedSpec` | `CTR-5404` | `` |
| `CTR-5406` | Contract-aware optimizer rules | `G5` | `M` | `BlockedSpec` | `CTR-5405` | `` |
| `CTR-5407` | Contract LSP/Zed | `G5` | `L` | `BlockedSpec` | `CTR-5406`, `LSP-2205` | `` |
| `DAP-3601` | Debugger stdio adapter | `G3+` | `L` | `BlockedSpec` | `DIFF-3702` | `` |
| `DAP-3602` | Zed debugger registration | `G3+` | `M` | `BlockedSpec` | `DAP-3601` | `` |
| `DAP-3603` | Staged debugger capabilities | `G3+` | `L` | `BlockedSpec` | `DAP-3602` | `` |
| `DBUF-4401` | Device Types and Capability | `G4` | `M` | `BlockedSpec` | `SIMD-4303` | `` |
| `DBUF-4402` | Buffer Ownership | `G4` | `M` | `BlockedSpec` | `DBUF-4401` | `` |
| `DBUF-4403` | Transfer Effect | `G4` | `M` | `BlockedSpec` | `DBUF-4402` | `` |
| `DBUF-4404` | Device Synchronization Model | `G4` | `M` | `BlockedSpec` | `DBUF-4403` | `` |
| `DIFF-3701` | Interpreter/VM/Native differential harness | `G3` | `L` | `BlockedSpec` | `FFI-3605` | `` |
| `DIFF-3702` | Allowed-difference registry | `G3` | `M` | `BlockedSpec` | `DIFF-3701` | `` |
| `DIR-4501` | Device IR Schema | `G4` | `L` | `BlockedSpec` | `DBUF-4404` | `` |
| `DIR-4502` | Kernel Core to Device IR | `G4` | `M` | `BlockedSpec` | `DIR-4501` | `` |
| `DIR-4503` | Device IR Canonicalization | `G4` | `M` | `BlockedSpec` | `DIR-4502` | `` |
| `EFF-2101` | Effect core model freeze | `G2` | `M` | `BlockedSpec` | — | `` |
| `EFF-2102` | Effect inference and constraint solving | `G2` | `L` | `BlockedSpec` | `EFF-2101` | `` |
| `EFF-2103` | Handler Typed Core representation | `G2` | `L` | `BlockedSpec` | `EFF-2101`, `EFF-2102` | `` |
| `EFF-2104` | Interpreter and VM handler execution | `G2` | `L` | `BlockedSpec` | `EFF-2103` | `` |
| `EFF-2105` | Effect fuzz and property tests | `G2` | `L` | `BlockedSpec` | `EFF-2104` | `` |
| `FFI-3601` | FFI declaration model | `G3` | `M` | `BlockedSpec` | `BACK-3505` | `` |
| `FFI-3602` | Minimal C ABI interoperability | `G3` | `M` | `BlockedSpec` | `FFI-3601` | `` |
| `FFI-3603` | FFI shim generator | `G3` | `M` | `BlockedSpec` | `FFI-3602` | `` |
| `FFI-3604` | Target Primitive Package | `G3` | `M` | `BlockedSpec` | `FFI-3603` | `` |
| `FFI-3605` | FFI fuzz and sanitizer suite | `G3` | `M` | `BlockedSpec` | `FFI-3604` | `` |
| `FMT-1501` | Author Source formatter preservation decision | `G1` | `M` | `Done` | `INC-1410` | `fa2560fc09772ed98f8af97a71164ee1f465495f` |
| `FMT-1502` | Compiler-CST Format IR | `G1` | `M` | `Done` | `FMT-1501` | `4d006e0c410c1c59f5717df43b8fac843ed960ec` |
| `FMT-1503` | Core syntax formatting | `G1` | `M` | `Done` | `FMT-1502` | `05fe7fb9827b5e33afd094e99a0908c69d0af972` |
| `FMT-1504` | Comment attachment | `G1` | `M` | `Done` | `FMT-1503` | `4348bfe548d0f62c25efdd5bb5c704e0a46958f9` |
| `FMT-1505` | Incomplete-source recovery | `G1` | `M` | `Done` | `FMT-1504` | `da412442bde5c7624946909996fa40772d14ce77` |
| `FMT-1506` | Formatter property and semantic-equivalence evidence | `G1` | `M` | `Done` | `FMT-1505` | `18e14f0a4f19d668b1b854fb1584ec52f91afec1` |
| `FMT-1507` | Formatter CLI/LSP integration | `G1` | `M` | `BlockedSpec` | `FMT-1506` | `` |
| `FMT-1508` | Audit Source separation | `G1` | `S` | `Done` | `FMT-1506` | `f247dfed98f104ea5227532965e8b579938a213e` |
| `GC-3301` | Minimal Managed object model | `G3` | `M` | `BlockedSpec` | — | `` |
| `GC-3302` | First Managed collector | `G3` | `L` | `BlockedSpec` | `GC-3301` | `` |
| `GC-3303` | Managed Native and FFI boundary | `G3` | `L` | `BlockedSpec` | `GC-3302` | `` |
| `GC-3304` | Managed profile checks | `G3` | `M` | `BlockedSpec` | `GC-3303` | `` |
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
| `GPU-4601` | Backend Spike and Selection | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4602` | Backend Adapter | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4603` | Launch and Runtime | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4604` | Differential and Hardware Matrix | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4605` | Error Normalization | `G4` | `M` | `BlockedSpec` | — | `` |
| `IDE-2301` | IDE document symbols | `G1` | `M` | `BlockedSpec` | `LSP-2101`, `LSP-2102` | `` |
| `IDE-2302` | IDE hover | `G1` | `M` | `BlockedSpec` | `IDE-2301`, `LSP-2201` | `` |
| `IDE-2303` | IDE definition navigation | `G1` | `M` | `BlockedSpec` | `IDE-2301`, `IDE-2302`, `LSP-2101`, `LSP-2102` | `` |
| `IDE-2304` | IDE references | `G1` | `M` | `BlockedSpec` | `IDE-2303`, `LSP-2101`, `LSP-2102` | `` |
| `IDE-2305` | IDE prepare rename | `G1` | `M` | `BlockedSpec` | `IDE-2304`, `LSP-2102` | `` |
| `IDE-2306` | IDE rename | `G1` | `L` | `BlockedSpec` | `IDE-2305`, `LSP-2102`, `LSP-2104` | `` |
| `IDE-2307` | IDE completion v0 | `G1` | `L` | `BlockedSpec` | `IDE-2304`, `LSP-2101`, `LSP-2102` | `` |
| `IDE-2308` | IDE completion resolve | `G1` | `M` | `BlockedSpec` | `IDE-2307`, `LSP-2101`, `LSP-2102` | `` |
| `IDE-2309` | IDE code actions | `G1` | `L` | `BlockedSpec` | `FMT-1507`, `IDE-2308`, `LSP-2201`, `LSP-2202` | `` |
| `IDE-2310` | IDE formatting | `G1` | `M` | `BlockedSpec` | `FMT-1507`, `LSP-2102` | `` |
| `IDE-2311` | IDE workspace symbols | `G1` | `L` | `BlockedSpec` | `IDE-2301`, `LSP-2101`, `LSP-2102` | `` |
| `INC-1401` | Incremental query boundary ADR | `G1` | `M` | `Done` | `GOV-0110` | `dcb3fc5c148d0b90a3481a736113c4551a35cf17` |
| `INC-1402` | VFS and revision boundary | `G1` | `M` | `Done` | `INC-1401` | `5c341078f3430a6fa1585fcaaf30d472dba7512f` |
| `INC-1403` | Parse queries | `G1` | `M` | `Done` | `INC-1402` | `a73e5c7150cce3fb86da5d48508158e7e1637f40` |
| `INC-1404` | Resolve and module queries | `G1` | `M` | `Done` | `INC-1403` | `eb6061ea3c815d3f4f57f2eb36f63ef90eb599b5` |
| `INC-1405` | Type and effect queries | `G1` | `M` | `Done` | `INC-1404` | `f9f2512d653f95955e7861953a8e12c3d38e31cc` |
| `INC-1406` | Semantic queries | `G1` | `M` | `Done` | `INC-1405` | `4a3cbbd8e91da023e9bf9c56e606e23cbbcb7a83` |
| `INC-1407` | Clean/incremental equivalence | `G1` | `M` | `Done` | `INC-1406` | `e79c0a51fa9fe17457e885b327cbe205e8b43778` |
| `INC-1408` | Deterministic parallel scheduling | `G1` | `M` | `Done` | `INC-1407` | `4053acb49de1ffaa270d3520434aa8bc00858d6e` |
| `INC-1409` | Disposable persistent query cache slice | `G1` | `L` | `Done` | `INC-1408` | `897d9c0a5480ff93417a17a1b3e642bd8967c55f` |
| `INC-1410` | Incremental performance baseline | `G1` | `M` | `Done` | `INC-1409` | `bcd58bb53a3f9a71613ba287d5a733b7aecd467f` |
| `KCHK-4101` | Kernel allowed capability matrix | `G4` | `M` | `BlockedSpec` | — | `` |
| `KCHK-4102` | Kernel Effect and Capability checks | `G4` | `L` | `BlockedSpec` | `KCHK-4101` | `` |
| `KCHK-4103` | Shape, index, and bounds | `G4` | `L` | `BlockedSpec` | `KCHK-4102` | `` |
| `KCHK-4104` | Alias and parallel-write conflicts | `G4` | `L` | `BlockedSpec` | `KCHK-4103` | `` |
| `KCHK-4105` | Kernel Core and verifier | `G4` | `L` | `BlockedSpec` | `KCHK-4104` | `` |
| `LSP-2101` | LSP lifecycle skeleton | `G1` | `S` | `BlockedSpec` | `CLI-1701` | `` |
| `LSP-2102` | LSP position-encoding negotiation | `G1` | `S` | `BlockedSpec` | `LSP-2101` | `` |
| `LSP-2103` | LSP open-document overlay | `G1` | `M` | `BlockedSpec` | `LSP-2101`, `LSP-2102` | `` |
| `LSP-2104` | LSP incremental text changes | `G1` | `M` | `BlockedSpec` | `LSP-2102`, `LSP-2103` | `` |
| `LSP-2105` | LSP workspace reload | `G1` | `M` | `BlockedSpec` | `LSP-2101`, `LSP-2103`, `PRJ-1107` | `` |
| `LSP-2201` | LSP compiler diagnostic adapter | `G1` | `M` | `BlockedSpec` | `LSP-2101`, `LSP-2102` | `` |
| `LSP-2202` | LSP push diagnostics v0 | `G1` | `M` | `BlockedSpec` | `LSP-2103`, `LSP-2201` | `` |
| `LSP-2203` | LSP pull diagnostics Preview | `G1` | `M` | `BlockedSpec` | `LSP-2101`, `LSP-2201` | `` |
| `LSP-2204` | LSP root-cause and error-storm control | `G1` | `M` | `BlockedSpec` | `LSP-2201`, `LSP-2202` | `` |
| `LSP-2205` | LSP diagnostic fixtures | `G1` | `M` | `BlockedSpec` | `LSP-2201`, `LSP-2202`, `LSP-2203`, `LSP-2204` | `` |
| `LSP-2401` | Semantic token taxonomy RFC/decision | `G1` | `M` | `BlockedSpec` | `IDE-2311`, `LSP-2101`, `LSP-2102` | `` |
| `LSP-2402` | Typed semantic-token generation | `G1` | `L` | `BlockedSpec` | `LSP-2101`, `LSP-2102`, `LSP-2401` | `` |
| `LSP-2403` | Semantic token full and delta transport | `G1` | `L` | `BlockedSpec` | `LSP-2401`, `LSP-2402` | `` |
| `LSP-2404` | Semantic-token fixture corpus | `G1` | `M` | `BlockedSpec` | `LSP-2401`, `LSP-2402`, `LSP-2403` | `` |
| `LSP-2501` | LSP request snapshot | `G1` | `L` | `BlockedSpec` | `INC-1401`, `INC-1402`, `LSP-2103`, `LSP-2104` | `` |
| `LSP-2502` | LSP request cancellation | `G1` | `L` | `BlockedSpec` | `INC-1401`, `LSP-2501` | `` |
| `LSP-2503` | LSP debounce and priority scheduling | `G1` | `L` | `BlockedSpec` | `LSP-2103`, `LSP-2104`, `LSP-2202`, `LSP-2501`, `LSP-2502` | `` |
| `LSP-2504` | LSP memory and resource limits | `G1` | `L` | `BlockedSpec` | `LSP-2201`, `LSP-2501`, `LSP-2502`, `LSP-2503` | `` |
| `MEM-3101` | Type classification model | `G3` | `M` | `BlockedSpec` | — | `` |
| `MEM-3102` | Value layout and Copy/Move | `G3` | `L` | `BlockedSpec` | `MEM-3101` | `` |
| `MEM-3103` | Resource definition and Drop contract | `G3` | `L` | `BlockedSpec` | `MEM-3102` | `` |
| `MEM-3104` | Managed types and island boundaries | `G3` | `L` | `BlockedSpec` | `MEM-3103` | `` |
| `NIR-3401` | Native IR design | `G3` | `L` | `BlockedSpec` | `GC-3304` | `` |
| `NIR-3402` | Core to Native IR lowering | `G3` | `L` | `BlockedSpec` | `NIR-3401` | `` |
| `NIR-3403` | Native IR verifier | `G3` | `M` | `BlockedSpec` | `NIR-3402` | `` |
| `NODE-5301` | Node Syntax and Semantics | `G5` | `L` | `BlockedSpec` | `BND-5204` | `` |
| `NODE-5302` | Node Checked Core | `G5` | `M` | `BlockedSpec` | `NODE-5301` | `` |
| `NODE-5303` | Static Node Scheduling | `G5` | `M` | `BlockedSpec` | `NODE-5302` | `` |
| `NODE-5304` | Virtual-Time Reference Runtime | `G5` | `M` | `BlockedSpec` | `NODE-5303` | `` |
| `NODE-5305` | Native Node Runtime | `G5` | `L` | `BlockedSpec` | `NODE-5304` | `` |
| `NODE-5306` | Node and Actor Boundary | `G5` | `M` | `BlockedSpec` | `NODE-5305` | `` |
| `NODE-5307` | Node Conformance | `G5` | `M` | `BlockedSpec` | `NODE-5306` | `` |
| `OWN-3201` | Place and Move analysis | `G3` | `L` | `BlockedSpec` | `MEM-3104` | `` |
| `OWN-3202` | Borrow exclusivity | `G3` | `L` | `BlockedSpec` | `OWN-3201` | `` |
| `OWN-3203` | Region inference | `G3` | `L` | `BlockedSpec` | `OWN-3202` | `` |
| `OWN-3204` | Borrow across await and Actor turns | `G3` | `L` | `BlockedSpec` | `ACT-2304`, `OWN-3203` | `` |
| `OWN-3205` | Drop-order lowering | `G3` | `L` | `BlockedSpec` | `MEM-3103`, `OWN-3204` | `` |
| `OWN-3206` | Ownership diagnostics and repairs | `G3` | `L` | `BlockedSpec` | `OWN-3201`, `OWN-3202`, `OWN-3203`, `OWN-3204`, `OWN-3205` | `` |
| `OWN-3207` | Negative corpus and property tests | `G3` | `L` | `BlockedSpec` | `OWN-3201`, `OWN-3202`, `OWN-3203`, `OWN-3204`, `OWN-3205`, `OWN-3206` | `` |
| `PLC-4801` | Placement Constraint Model | `G4` | `M` | `BlockedSpec` | — | `` |
| `PLC-4802` | Static Candidates and Runtime Selection | `G4` | `M` | `BlockedSpec` | `PLC-4801` | `` |
| `PLC-4803` | Cost Model v0 | `G4` | `M` | `BlockedSpec` | `PLC-4802` | `` |
| `PLC-4804` | Placement Explain Output | `G4` | `M` | `BlockedSpec` | `PLC-4803` | `` |
| `PLC-4805` | Device Binary Cache | `G4` | `M` | `BlockedSpec` | `PLC-4804` | `` |
| `PRJ-1101` | Minimal project manifest | `G1` | `M` | `Done` | — | `80f46bcf3d175eeb6402bf6267085cb905a5dbcf` |
| `PRJ-1102` | Deterministic module discovery | `G1` | `M` | `Done` | `PRJ-1101` | `f76f4953070b9ae555fce24c7dcc2fbf08a36f7a` |
| `PRJ-1103` | Package-aware imports and visibility | `G1` | `M` | `Done` | `PRJ-1102`, `PRJ-1104` | `8e98d32f54301dee3f198273cfae3146bbf2846b` |
| `PRJ-1104` | Content-identified local dependency graph | `G1` | `L` | `Done` | `PRJ-1101`, `PRJ-1102` | `66a64c9b57c8bb327599a7463345c9d2fbe77a51` |
| `PRJ-1105` | Canonical project lockfile protocol | `G1` | `L` | `Done` | `PRJ-1104` | `9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082` |
| `PRJ-1106` | End-to-end project fixture matrix | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105` | `0e9c5800411a6f1acd1441068e6ce2fd58f29816` |
| `PRJ-1107` | Project API and CLI integration | `G1` | `M` | `BlockedSpec` | `PRJ-1106` | `` |
| `PRJ-1108` | Project graph property and manifest fuzz coverage | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105`, `PRJ-1106` | `29f9c4465b58c7eff23c227436563d69409b880e` |
| `PROF-5101` | Machine-Readable Critical Profile | `G5` | `M` | `BlockedSpec` | `PLC-4805` | `` |
| `PROF-5102` | Forbidden Capability Checks | `G5` | `M` | `BlockedSpec` | `PROF-5101` | `` |
| `PROF-5103` | Profile Composition | `G5` | `M` | `BlockedSpec` | `PROF-5102` | `` |
| `PROF-5104` | Profile Audit and LSP | `G5` | `M` | `BlockedSpec` | `PROF-5103` | `` |
| `PROOF-5501` | Proof IR | `G5` | `L` | `BlockedSpec` | `CTR-5405` | `` |
| `REM-2601` | RemoteRef and endpoint | `G2` | `L` | `BlockedSpec` | `ACT-2305`, `REP-2506` | `` |
| `REM-2602` | Transport-neutral envelope | `G2` | `L` | `BlockedSpec` | `REM-2601` | `` |
| `REM-2603` | Delivery semantics | `G2` | `L` | `BlockedSpec` | `REM-2602` | `` |
| `REM-2604` | Minimal reference transport | `G2` | `L` | `BlockedSpec` | `REM-2603` | `` |
| `REM-2605` | Security and resource limits | `G2` | `L` | `BlockedSpec` | `REM-2604` | `` |
| `REP-2501` | Determinism class | `G2` | `L` | `BlockedSpec` | `SUP-2403` | `` |
| `REP-2502` | Replay log schema | `G2` | `L` | `BlockedSpec` | `REP-2501` | `` |
| `REP-2503` | Effect recorder | `G2` | `L` | `BlockedSpec` | `EFF-2105`, `REP-2502` | `` |
| `REP-2504` | Replay player | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503` | `` |
| `REP-2505` | Replay privacy, trimming, and corruption | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503`, `REP-2504` | `` |
| `REP-2506` | Cross-process replay acceptance | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503`, `REP-2504`, `REP-2505` | `` |
| `SIMD-4301` | Vectorization Legality Analysis | `G4` | `M` | `BlockedSpec` | `CPU-4203` | `` |
| `SIMD-4302` | Portable SIMD IR | `G4` | `M` | `BlockedSpec` | `SIMD-4301` | `` |
| `SIMD-4303` | SIMD Differential | `G4` | `M` | `BlockedSpec` | `SIMD-4302` | `` |
| `SUP-2401` | Supervisor model | `G2` | `L` | `BlockedSpec` | `ACT-2305` | `` |
| `SUP-2402` | Restart budgets and circuit breakers | `G2` | `L` | `BlockedSpec` | `SUP-2401` | `` |
| `SUP-2403` | Supervision tests | `G2` | `L` | `BlockedSpec` | `SUP-2402` | `` |
| `TASK-2201` | Structured Task syntax and Checked Core | `G2` | `M` | `BlockedSpec` | `EFF-2103` | `` |
| `TASK-2202` | Task state-machine lowering | `G2` | `L` | `BlockedSpec` | `TASK-2201` | `` |
| `TASK-2203` | Structured Task lifecycle runtime | `G2` | `L` | `BlockedSpec` | `TASK-2202` | `` |
| `TASK-2204` | Deterministic Task test scheduler | `G2` | `L` | `BlockedSpec` | `TASK-2203` | `` |
| `TASK-2205` | Production local Task scheduler | `G2` | `L` | `BlockedSpec` | `TASK-2204` | `` |
| `TASK-2206` | Task conformance and stress tests | `G2` | `L` | `BlockedSpec` | `TASK-2205` | `` |
| `TEST-VM-0001` | VM failing-first corpus and differential harness baseline | `G1` | `M` | `Done` | `GOV-0104`, `GOV-0105` | `5bd49583c9160cd2067a7124bc014ebc3b4bcf95` |
| `TRAIT-1301` | Trait RFC closure | `G1` | `M` | `Done` | — | `ccab6ea91e05ed477457cc1ed870d76faaa46e3c` |
| `TRAIT-1302` | Trait AST/HIR representation | `G1` | `M` | `Done` | `TRAIT-1301` | `693b841000c98ca8aae119e3797a737fe0cebc7f` |
| `TRAIT-1303` | Trait constraint collection | `G1` | `M` | `Done` | `TRAIT-1302` | `1dfc52ee4439c43f284fbf384869436a408344d3` |
| `TRAIT-1304` | Trait coherence and orphan index | `G1` | `M` | `Done` | `TRAIT-1303` | `94a8daec579b1c730e51ff37bd3cde63dfd9d046` |
| `TRAIT-1305` | Trait solver v0 | `G1` | `M` | `Done` | `TRAIT-1304` | `530de657bfc63018090426f2e6e47eeeaf710f2c` |
| `TRAIT-1306` | Trait Checked Core dictionary witnesses | `G1` | `M` | `Done` | `TRAIT-1305` | `bfd00305473363c03286c2e0dbd060d7d136a95d` |
| `TRAIT-1307` | Trait interpreter and VM dictionary lowering | `G1` | `L` | `BlockedSpec` | `TRAIT-1306` | `` |
| `TRAIT-1308` | Trait IDE support | `G1` | `L` | `BlockedSpec` | `TRAIT-1307` | `` |
| `TRAIT-1309` | Trait solver performance and termination | `G1` | `M` | `BlockedSpec` | `TRAIT-1308` | `` |
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
| `VM-1205` | First-class functions, lexical closures, and recursion in bytecode | `G1` | `L` | `Done` | `VM-1201`, `VM-1202`, `VM-1203`, `VM-1204` | `9a54775ae0ee48d9fb0c75ce819989a24df27ed2` |
| `VM-1206` | Nominal aggregates, immutable record update, and checked match lowering | `G1` | `L` | `Done` | `VM-1204`, `VM-1205` | `84af63773310b91249a6447ee8b59348c1e80bcd` |
| `VM-1207` | Mutable places and basic borrow-boundary lowering | `G1` | `L` | `Done` | `VM-1204`, `VM-1205`, `VM-1206` | `600e4d5d213f2c055237ab9645a21edca1d0f985` |
| `VM-1208` | Effect, Capability, and Fault boundary | `G1` | `L` | `Done` | `VM-1204`, `VM-1205`, `VM-1206`, `VM-1207` | `4c07a68b42aa86c1b5e52feb856b94f61f99ff01` |
| `VM-1209` | Interpreter–VM differential contract and evidence | `G1` | `L` | `Done` | `VM-1208` | `558625676c0402cf793000297b0f388f87e532cc` |
| `VM-1210` | VM robustness, cancellation, and resource evidence | `G1` | `L` | `Done` | `VM-1209` | `611ab0db31d34219243d74a8ea53898a99296a04` |
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
