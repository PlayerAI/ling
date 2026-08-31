# Ling implementation status / Ling 实现状态

> Generated deterministically from `implementation-status.toml`; do not edit manually.
> 本文由 `implementation-status.toml` 确定性生成；不得手工编辑。

- Registry schema: `2`
- Feature schema: `1`
- Updated: `2026-08-26`
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
| `ACC-4701-OBSERVATION` | Internal accelerator-plugin interface boundary evidence | `G4` | `S` | `Done` | `GPU-4605-OBSERVATION` | `c62cfabb7b17cf46ca85c78c803b30b04fe6bf91` |
| `ACC-4702` | Experimental Accelerator Adapter | `G4` | `M` | `BlockedSpec` | `ACC-4701` | `` |
| `ACC-4702-OBSERVATION` | Internal Experimental accelerator-adapter boundary evidence | `G4` | `S` | `Done` | `ACC-4701-OBSERVATION` | `faaf0c084ae6cb0a9eec08daceaa9e9f06e1928b` |
| `ACT-2301` | Actor identity and state isolation | `G2` | `M` | `Done` | `TASK-2203` | `abb31810ffa8a96e637b83f14659cff1662e0527` |
| `ACT-2301-ACTOR-SYNTAX-REJECTION` | Internal Actor-shaped syntax rejection gate | `G2` | `S` | `Done` | `GOV-0105` | `44bb344db86ad8c1039d8a974f5d608c7c1eff9a` |
| `ACT-2301-IDENTITY-MODEL` | Internal Actor identity/reference model | `G2` | `S` | `Done` | `ACT-2301-ACTOR-SYNTAX-REJECTION` | `4209c9b444b72501731b96fdad2f8832269b406a` |
| `ACT-2302` | Actor message sendability checking | `G2` | `L` | `Done` | `ACT-2301` | `4e925c7ab8bc91f5724a5700d7abd7c3f6623955` |
| `ACT-2302-MESSAGE-SCHEMA-MODEL` | Internal Actor message-schema identity model | `G2` | `S` | `Done` | `ACT-2301-IDENTITY-MODEL` | `dfc5577c80ecf3811014495080aca758df2cf3e9` |
| `ACT-2303` | Bounded mailbox and backpressure | `G2` | `L` | `Ready` | `ACT-2302` | `` |
| `ACT-2303-MAILBOX-OBSERVATION` | Internal Actor mailbox observation | `G2` | `S` | `Done` | `ACT-2302-MESSAGE-SCHEMA-MODEL` | `fb42fb580dde35d09594392f16e5798621fd3785` |
| `ACT-2304` | Actor turn and reentry rules | `G2` | `L` | `BlockedSpec` | `ACT-2303` | `` |
| `ACT-2304-TURN-OBSERVATION` | Internal Actor turn observation | `G2` | `S` | `Done` | `ACT-2303-MAILBOX-OBSERVATION` | `ec279c46f14acb2634fa7783f8e195d457178041` |
| `ACT-2305` | Actor runtime | `G2` | `L` | `BlockedSpec` | `ACT-2304` | `` |
| `ACT-2305-RUNTIME-OBSERVATION` | Internal Actor runtime observation | `G2` | `S` | `Done` | `ACT-2304-TURN-OBSERVATION` | `c8df61e4e0ad01c2f3bd7b30555aca00ab2fd696` |
| `ACT-2306` | Actor properties and stress tests | `G2` | `L` | `BlockedSpec` | `ACT-2305` | `` |
| `ACT-2306-PROPERTY-OBSERVATION` | Internal Actor property observation | `G2` | `S` | `Done` | `ACT-2305-RUNTIME-OBSERVATION` | `09f33d3b1563db59a5e9ad4fb66c556e8725b583` |
| `BACK-3501` | Native backend selection spike | `G3` | `M` | `BlockedSpec` | `NIR-3403` | `` |
| `BACK-3501-OBSERVATION` | Internal Native backend selection boundary evidence | `G3` | `S` | `Done` | `NIR-3403-OBSERVATION` | `1330d19d967d48bbbc95ee3640d75344add66fd7` |
| `BACK-3502` | Baseline Native codegen | `G3` | `L` | `BlockedSpec` | `BACK-3501` | `` |
| `BACK-3502-OBSERVATION` | Internal Native codegen boundary evidence | `G3` | `S` | `Done` | `BACK-3501-OBSERVATION` | `fe1a9477d22015288a05c6cfaf17950f22592d99` |
| `BACK-3503` | Native runtime ABI | `G3` | `L` | `BlockedSpec` | `BACK-3502` | `` |
| `BACK-3503-OBSERVATION` | Internal Native runtime ABI boundary evidence | `G3` | `S` | `Done` | `BACK-3502-OBSERVATION` | `fa8cdb4f16ea22f34fddccd4589d6fb07d2190d1` |
| `BACK-3504` | Native optimization and verification | `G3` | `L` | `BlockedSpec` | `BACK-3503` | `` |
| `BACK-3504-OBSERVATION` | Internal Native optimization boundary evidence | `G3` | `S` | `Done` | `BACK-3503-OBSERVATION` | `66da38f19fd2a75ecc3f5b4c14da9e500d1481e9` |
| `BACK-3505` | Reproducible Native build | `G3` | `M` | `BlockedSpec` | `BACK-3504` | `` |
| `BACK-3505-OBSERVATION` | Internal Native reproducible-build boundary evidence | `G3` | `S` | `Done` | `BACK-3504-OBSERVATION` | `a4ecedcad8d26eb396e0af7ce2be6b2f6e3c8363` |
| `BASE-0001` | 仓库基线盘点与执行计划落位 | `G0` | `S` | `Done` | — | `aa8c02894bd2fdd696ab60c97423d07c0ce9614a` |
| `BND-5201` | Bound Types and Expressions | `G5` | `M` | `BlockedSpec` | `PROF-5104` | `` |
| `BND-5201-OBSERVATION` | Internal Bound types/expressions boundary evidence | `G5` | `S` | `Done` | `PROF-5104-OBSERVATION` | `a41323f755cdb2283aed9d7197f0dfe46ab51c77` |
| `BND-5202` | Loop and Recursion Checks | `G5` | `M` | `BlockedSpec` | `BND-5201` | `` |
| `BND-5202-OBSERVATION` | Internal loop/recursion checks boundary evidence | `G5` | `S` | `Done` | `BND-5201-OBSERVATION` | `5961d9b7e97882333646020b27c8c3b0b7461243` |
| `BND-5203` | Memory Budget Analysis | `G5` | `M` | `BlockedSpec` | `BND-5202` | `` |
| `BND-5203-OBSERVATION` | Internal memory-budget boundary evidence | `G5` | `S` | `Done` | `BND-5202-OBSERVATION` | `a4a63c8443db52946029b9cee3a742216831038b` |
| `BND-5204` | Resource Budget Diagnostics | `G5` | `M` | `BlockedSpec` | `BND-5203` | `` |
| `BND-5204-OBSERVATION` | Internal resource-budget diagnostic boundary evidence | `G5` | `S` | `Done` | `BND-5203-OBSERVATION` | `375ae71c51129051e7049e2379cd489833f0e6fd` |
| `CBK-5901` | Trusted Compiler Route Decision | `G5` | `L` | `BlockedSpec` | `EVD-5804` | `` |
| `CBK-5901-OBSERVATION` | Internal Trusted Compiler Route boundary evidence | `G5` | `S` | `Done` | `EVD-5804-OBSERVATION` | `9caebb42d8022fc8fdb6f1f7a06f128cd64d1a39` |
| `CBK-5902` | Lowering Validator | `G5` | `L` | `BlockedSpec` | `CBK-5901` | `` |
| `CBK-5902-OBSERVATION` | Internal Lowering Validator boundary evidence | `G5` | `S` | `Done` | `CBK-5901-OBSERVATION` | `85bd46dca8c6e7c059816ed9b47b100c4840f816` |
| `CBK-5903` | Critical Runtime/Target Package | `G5` | `L` | `BlockedSpec` | `CBK-5902` | `` |
| `CBK-5903-OBSERVATION` | Internal Critical Runtime/Target Package boundary evidence | `G5` | `S` | `Done` | `CBK-5902-OBSERVATION` | `728edd4a58646c146e792d39ec346648db850971` |
| `CLI-1701` | Unified CLI command model | `G1` | `L` | `Done` | `FMT-1507`, `PRJ-1107` | `160979a422cb521d2987cc0e276161f0536a71fe` |
| `CLI-1701-CATALOG` | Internal current CLI command catalog | `G1` | `S` | `Done` | `FMT-1507-CLI`, `PRJ-1107-CHECK` | `8410aaa4b4ae7508b84a6736015d80ea73444706` |
| `CLI-1702` | CLI output and exit behavior | `G1` | `M` | `Done` | `CLI-1701` | `f0e45f3c880a85492be351f4f6d2f53b48f0f05d` |
| `CLI-1702-EXIT` | Internal CLI exit-code catalog | `G1` | `S` | `Done` | `CLI-1701-CATALOG` | `b55db2ca22e597da4266f1af3deb664f610cd1ea` |
| `CLI-1703` | Project initialization command | `G1` | `M` | `Done` | `CLI-1701`, `PRJ-1107` | `3feaca220e308ebd6a1505c02d2c0229419655ee` |
| `CLI-1703-INIT` | Offline `ling init` scaffold | `G1` | `M` | `Done` | `CLI-1701-CATALOG`, `PRJ-1101` | `8c2ef94e58da75d6c530a4ac60cfbe2eeea11bbf` |
| `CLI-1704` | Project test command | `G1` | `L` | `Done` | `CLI-1701`, `PRJ-1107` | `69beaefd41c452ae25b698d15a4b1e5820519d79` |
| `CLI-1704-FILE` | Explicit standalone test-file runner Preview | `G1` | `M` | `Done` | `CLI-1701-CATALOG`, `CLI-1702-EXIT` | `72d85d7de77f188b0706acde7a559169d4ac149e` |
| `CLI-1705` | Semantic query and patch commands | `G1` | `L` | `Done` | `CLI-1701`, `PRJ-1107` | `d72cdbf3134b960ca07888404d55ea27eadb38ca` |
| `CLI-1706` | Shell completion and help fixtures | `G1` | `M` | `Done` | `CLI-1701`, `CLI-1702` | `a56ca37f1d5686f691f195a39dbe798fd2f96132` |
| `CLI-1706-HELP` | Truthful implemented-command help fixture | `G1` | `S` | `Done` | `CLI-1701-CATALOG`, `CLI-1702-EXIT` | `27c5f1d58a3add59abc87b0cf4dc29128d7be226` |
| `COMPAT-6501` | Historical Corpus | `G6` | `L` | `BlockedSpec` | `PKG-6404` | `` |
| `COMPAT-6501-SEED` | Seed historical-corpus freeze evidence | `G6` | `S` | `Done` | `PKG-6404-LOCAL` | `80ea137740a23bd924fcf40234a5f9c95b21b940` |
| `COMPAT-6502` | 1.0 Compiler Compatibility Matrix | `G6` | `L` | `BlockedSpec` | `COMPAT-6501` | `` |
| `COMPAT-6502-CURRENT` | Current compiler compatibility-boundary evidence | `G6` | `S` | `Done` | `COMPAT-6501-SEED` | `9a56e37933fd78c27fda3c30149f1ec2619855d9` |
| `COMPAT-6503` | Language Migration Tool | `G6` | `L` | `BlockedSpec` | `COMPAT-6502` | `` |
| `COMPAT-6503-READINESS` | Migration Tool Readiness Boundary | `G6` | `S` | `Done` | `COMPAT-6502-CURRENT` | `3ccbc058cd6de3ad447712ffb77bfeb1e2e7cb10` |
| `COMPAT-6504` | Deprecation Policy | `G6` | `L` | `BlockedSpec` | `COMPAT-6503` | `` |
| `COMPAT-6504-READINESS` | Deprecation-policy Readiness Boundary Evidence | `G6` | `S` | `Done` | `COMPAT-6503-READINESS` | `1b2390344b788e0abf6d299afaabc420a27b2d98` |
| `CPU-4201` | Scalar Reference Backend | `G4` | `L` | `BlockedSpec` | `KCHK-4105` | `` |
| `CPU-4201-OBSERVATION` | Internal CPU scalar-reference boundary evidence | `G4` | `S` | `Done` | `KCHK-4105-OBSERVATION` | `9f723650d65177833a34d623b5b0123e57cb9ecf` |
| `CPU-4202` | Reference Trace | `G4` | `M` | `BlockedSpec` | `CPU-4201` | `` |
| `CPU-4202-OBSERVATION` | Internal CPU reference-trace boundary evidence | `G4` | `S` | `Done` | `CPU-4201-OBSERVATION` | `cbdf4d64da02f41d6f7ef21df528b17e77136cfb` |
| `CPU-4203` | Kernel Corpus | `G4` | `M` | `BlockedSpec` | `CPU-4202` | `` |
| `CPU-4203-OBSERVATION` | Internal Kernel corpus boundary evidence | `G4` | `S` | `Done` | `CPU-4202-OBSERVATION` | `3c1d02592f40ecf9273721062419f9852484d0f8` |
| `CTR-5401` | Contract Syntax and AST/Core | `G5` | `M` | `BlockedSpec` | `NODE-5307` | `` |
| `CTR-5401-OBSERVATION` | Internal Contract syntax/Core boundary evidence | `G5` | `S` | `Done` | `NODE-5307-OBSERVATION` | `f26ca7329a439ac29f60372b2dfe87da0c102461` |
| `CTR-5402` | Contract Status Model | `G5` | `M` | `BlockedSpec` | `CTR-5401` | `` |
| `CTR-5402-OBSERVATION` | Internal Contract status-model boundary evidence | `G5` | `S` | `Done` | `CTR-5401-OBSERVATION` | `32eaee6e2e15857fb8ba5a9d0fc99b251e45bab8` |
| `CTR-5403` | Runtime Contract Check | `G5` | `M` | `BlockedSpec` | `CTR-5402` | `` |
| `CTR-5403-OBSERVATION` | Internal Contract runtime-check boundary evidence | `G5` | `S` | `Done` | `CTR-5402-OBSERVATION` | `69e594c9d10f56fe397453e0c79cbd4479101f55` |
| `CTR-5404` | Verification Condition Generation | `G5` | `L` | `BlockedSpec` | `CTR-5403` | `` |
| `CTR-5404-OBSERVATION` | Internal Contract VC boundary evidence | `G5` | `S` | `Done` | `CTR-5403-OBSERVATION` | `e8c3a94a4b85fe3ce4b55816f109b99672591d32` |
| `CTR-5405` | Solver/Proof Checker Adapter | `G5` | `L` | `BlockedSpec` | `CTR-5404` | `` |
| `CTR-5405-OBSERVATION` | Internal Solver/Proof Checker boundary evidence | `G5` | `S` | `Done` | `CTR-5404-OBSERVATION` | `f1bfe47d79e7a34b3fa46e2e61f31993009f28a0` |
| `CTR-5406` | Contract-aware optimizer rules | `G5` | `M` | `BlockedSpec` | `CTR-5405` | `` |
| `CTR-5406-OBSERVATION` | Internal Contract optimizer boundary evidence | `G5` | `S` | `Done` | `CTR-5405-OBSERVATION` | `d80ddc0aaf2dd13152c407085b81f70e10a52b19` |
| `CTR-5407` | Contract LSP/Zed | `G5` | `L` | `BlockedSpec` | `CTR-5406`, `LSP-2205` | `` |
| `CTR-5407-OBSERVATION` | Internal Contract LSP/Zed boundary evidence | `G5` | `S` | `Done` | `CTR-5406-OBSERVATION` | `83ac76284258b83e905f2384d17d33b251db3bb5` |
| `DAP-3601` | Debugger stdio adapter | `G3+` | `L` | `BlockedSpec` | `DIFF-3702` | `` |
| `DAP-3601-OBSERVATION` | Internal DAP debugger boundary evidence | `G3+` | `S` | `Done` | `DIFF-3702-OBSERVATION` | `1626b0dad645ae10436e06146c08ceb8a94d1155` |
| `DAP-3602` | Zed debugger registration | `G3+` | `M` | `BlockedSpec` | `DAP-3601` | `` |
| `DAP-3602-OBSERVATION` | Internal Zed debugger registration boundary evidence | `G3+` | `S` | `Done` | `DAP-3601-OBSERVATION` | `581f4c3b0537ba2de8c289129c8e2c6a53ac49da` |
| `DAP-3603` | Staged debugger capabilities | `G3+` | `L` | `BlockedSpec` | `DAP-3602` | `` |
| `DAP-3603-OBSERVATION` | Internal staged debugger capability boundary evidence | `G3+` | `S` | `Done` | `DAP-3602-OBSERVATION` | `b24c9771a6ea19959bd036a4fc5f6475c1b98d3f` |
| `DBUF-4401` | Device Types and Capability | `G4` | `M` | `BlockedSpec` | `SIMD-4303` | `` |
| `DBUF-4401-OBSERVATION` | Internal Device capability boundary evidence | `G4` | `S` | `Done` | `SIMD-4303-OBSERVATION` | `70f1246ace41008ffa355e9f29adf5d76f04c735` |
| `DBUF-4402` | Buffer Ownership | `G4` | `M` | `BlockedSpec` | `DBUF-4401` | `` |
| `DBUF-4402-OBSERVATION` | Internal Buffer ownership boundary evidence | `G4` | `S` | `Done` | `DBUF-4401-OBSERVATION` | `c38e5bf24ec017fd4c15ac9bf443947a10b8e57c` |
| `DBUF-4403` | Transfer Effect | `G4` | `M` | `BlockedSpec` | `DBUF-4402` | `` |
| `DBUF-4403-OBSERVATION` | Internal Transfer Effect boundary evidence | `G4` | `S` | `Done` | `DBUF-4402-OBSERVATION` | `97b93d7352f4c0e4c32a439eb26768bf6aaeaaf4` |
| `DBUF-4404` | Device Synchronization Model | `G4` | `M` | `BlockedSpec` | `DBUF-4403` | `` |
| `DBUF-4404-OBSERVATION` | Internal Device synchronization boundary evidence | `G4` | `S` | `Done` | `DBUF-4403-OBSERVATION` | `dfcec89a334e2818771efe174e0db62c339bd2c3` |
| `DIFF-3701` | Interpreter/VM/Native differential harness | `G3` | `L` | `BlockedSpec` | `FFI-3605` | `` |
| `DIFF-3701-OBSERVATION` | Internal differential-harness boundary evidence | `G3` | `S` | `Done` | `FFI-3605-OBSERVATION` | `4d43fa0a3ed9144494b82bdc0dbb3e15ab2daf47` |
| `DIFF-3702` | Allowed-difference registry | `G3` | `M` | `BlockedSpec` | `DIFF-3701` | `` |
| `DIFF-3702-OBSERVATION` | Internal allowed-difference registry boundary evidence | `G3` | `S` | `Done` | `DIFF-3701-OBSERVATION` | `c85e5a8b7f874e1cde3306f7b8a098f7919f1101` |
| `DIR-4501` | Device IR Schema | `G4` | `L` | `BlockedSpec` | `DBUF-4404` | `` |
| `DIR-4501-OBSERVATION` | Internal Device IR schema boundary evidence | `G4` | `S` | `Done` | `DBUF-4404-OBSERVATION` | `aab0610cc5f6157e59f9e168cd1e777c3e8b5100` |
| `DIR-4502` | Kernel Core to Device IR | `G4` | `M` | `BlockedSpec` | `DIR-4501` | `` |
| `DIR-4502-OBSERVATION` | Internal Kernel-to-Device lowering boundary evidence | `G4` | `S` | `Done` | `DIR-4501-OBSERVATION` | `a1d7301bb7773242369d077a6c473fcdd78cccf0` |
| `DIR-4503` | Device IR Canonicalization | `G4` | `M` | `BlockedSpec` | `DIR-4502` | `` |
| `DIR-4503-OBSERVATION` | Internal Device IR canonicalization boundary evidence | `G4` | `S` | `Done` | `DIR-4502-OBSERVATION` | `de0d14b41b1c77e4eb81379d181cafcafbdf63f5` |
| `DOC-6701` | Formal Documentation Set | `G6` | `L` | `BlockedSpec` | `REL-6604` | `` |
| `DOC-6701-EVIDENCE-PATHS` | Formal inventory evidence-path gate | `G6` | `S` | `Done` | `DOC-6701-SEED` | `6d04b188c7ddd7be1de6e5ac61bfa51d98b4b36b` |
| `DOC-6701-SEED` | Seed documentation-inventory drift gate | `G6` | `S` | `Done` | `REL-6604-SEED` | `77a905bbac64aaa7e9b6a56f8952a045d2760d87` |
| `DOC-6702` | Two-layer Examples | `G6` | `M` | `BlockedSpec` | `DOC-6701` | `` |
| `DOC-6702-EXECUTION-MANIFEST` | Seed example execution manifest | `G6` | `S` | `Done` | `DOC-6702-SEED` | `b228a382505964705596f8e797084f4d131f5431` |
| `DOC-6702-SEED` | Seed example-matrix drift gate | `G6` | `S` | `Done` | `DOC-6701-SEED` | `dab9a240c3f83c3e671449b318ca9425df57df32` |
| `DOC-6703` | Bilingual Chinese-first Tutorial | `G6` | `M` | `BlockedSpec` | `DOC-6702` | `` |
| `DOC-6703-SEED` | Seed bilingual tutorial coverage drift gate | `G6` | `S` | `Done` | `DOC-6702-SEED` | `9f3386bab5c8f06c7ce76ac0ab5c66f8b8695ca4` |
| `DOC-6703-SEMANTIC-EQUIVALENCE` | Bilingual tutorial Semantic-shape equivalence | `G6` | `S` | `Done` | `DOC-6702-EXECUTION-MANIFEST`, `DOC-6703-SEED` | `b5f3a3538a56f58652acc4e891ed58fcf57344d6` |
| `EFF-2101` | Effect core model freeze | `G2` | `M` | `Done` | — | `61f68a93844431265eca725ee66aab894394c982` |
| `EFF-2101-SEED-ROW` | Seed EffectRow canonical snapshot | `G2` | `S` | `Done` | — | `fb949ce2b28fc73a1668806952c3f8e790cd6d7a` |
| `EFF-2102` | Effect inference and constraint solving | `G2` | `L` | `Done` | `EFF-2101` | `e1827e7ceee6ffba9d40d882119a949d4af65a00` |
| `EFF-2103` | Handler Typed Core representation | `G2` | `L` | `Done` | `EFF-2101`, `EFF-2102` | `004538deec049446740a1d17ca57e5915c6d2777` |
| `EFF-2103-AST` | Handler unresolved AST projection | `G2` | `S` | `Done` | `EFF-2103-SYNTAX` | `225cc9fecabcdb1a3274ee6dc576f9120143584c` |
| `EFF-2103-CORE` | First-order handler Typed Core projection | `G2` | `S` | `Done` | `EFF-2101`, `EFF-2102` | `e1dc5334d15e25e959fa0da6e3462a90210c6fdf` |
| `EFF-2103-HIR` | Handler unresolved HIR projection | `G2` | `S` | `Done` | `EFF-2103-AST` | `fa988aab85a96d3e7c257c630b84acb985639064` |
| `EFF-2103-SYNTAX` | Handler source CST projection | `G2` | `S` | `Done` | `EFF-2103-CORE` | `73807f551753620b52b685d7aca25c2032ec1a77` |
| `EFF-2104` | Interpreter and VM handler execution | `G2` | `L` | `Done` | `EFF-2103` | `1188b2472ff0a61ac3d96c4ae21bbe9b6bd7eaba` |
| `EFF-2104-REJECTION-GATE` | Internal unresolved-handler execution rejection gate | `G2` | `S` | `Done` | `EFF-2103-HIR` | `e265b451985a2afac071da6862c4bf9451faf9a0` |
| `EFF-2105` | Effect fuzz and property tests | `G2` | `L` | `Done` | `EFF-2104` | `3517ffcccc8204a528c9768b0642aface4fcec29` |
| `EFF-2105-MODEL-PROPERTIES` | Effect model deterministic property corpus | `G2` | `S` | `Done` | `EFF-2102`, `EFF-2103-CORE` | `f40c69f0311d3bd970e9613772ddf7135d17772c` |
| `EVD-5801` | Evidence Bundle Schema | `G5` | `L` | `BlockedSpec` | `TIM-5703` | `` |
| `EVD-5801-OBSERVATION` | Internal Evidence Bundle Schema boundary evidence | `G5` | `S` | `Done` | `TIM-5703-OBSERVATION` | `2c27d97a4de390d49eba648830883bf50963f668` |
| `EVD-5802` | Independent Verifier | `G5` | `L` | `BlockedSpec` | `EVD-5801` | `` |
| `EVD-5802-OBSERVATION` | Internal Independent Evidence Verifier boundary evidence | `G5` | `S` | `Done` | `EVD-5801-OBSERVATION` | `4da432f6e872d3a66f387108d8e672e826ecf10b` |
| `EVD-5803` | Reproducible Build Binding | `G5` | `M` | `BlockedSpec` | `EVD-5802` | `` |
| `EVD-5803-OBSERVATION` | Internal Reproducible Build Binding boundary evidence | `G5` | `S` | `Done` | `EVD-5802-OBSERVATION` | `cb4ff8a861327d616aef6cd3b210e41da347800e` |
| `EVD-5804` | AI Provenance | `G5` | `M` | `BlockedSpec` | `EVD-5803` | `` |
| `EVD-5804-OBSERVATION` | Internal AI Provenance boundary evidence | `G5` | `S` | `Done` | `EVD-5803-OBSERVATION` | `bc0cf44884443d1b34cf4e47643c82125a8f633a` |
| `FFI-3601` | FFI declaration model | `G3` | `M` | `BlockedSpec` | `BACK-3505` | `` |
| `FFI-3601-OBSERVATION` | Internal FFI declaration boundary evidence | `G3` | `S` | `Done` | `BACK-3505-OBSERVATION` | `d254184a0f8a3e123e746c1c2f34973756ff6ba0` |
| `FFI-3602` | Minimal C ABI interoperability | `G3` | `M` | `BlockedSpec` | `FFI-3601` | `` |
| `FFI-3602-OBSERVATION` | Internal C ABI interoperability boundary evidence | `G3` | `S` | `Done` | `FFI-3601-OBSERVATION` | `a632900b083762a745a6138629074be182ab2a2d` |
| `FFI-3603` | FFI shim generator | `G3` | `M` | `BlockedSpec` | `FFI-3602` | `` |
| `FFI-3603-OBSERVATION` | Internal FFI shim-generator boundary evidence | `G3` | `S` | `Done` | `FFI-3602-OBSERVATION` | `015c016c6d557bc3ae66d44b7f0a5aaf0981fcc8` |
| `FFI-3604` | Target Primitive Package | `G3` | `M` | `BlockedSpec` | `FFI-3603` | `` |
| `FFI-3604-OBSERVATION` | Internal Target Primitive Package boundary evidence | `G3` | `S` | `Done` | `FFI-3603-OBSERVATION` | `bc0da3bce4de7bc8a10d12ffb5019f842c21e157` |
| `FFI-3605` | FFI fuzz and sanitizer suite | `G3` | `M` | `BlockedSpec` | `FFI-3604` | `` |
| `FFI-3605-OBSERVATION` | Internal FFI fuzz and sanitizer boundary evidence | `G3` | `S` | `Done` | `FFI-3604-OBSERVATION` | `6f26f86a4c5281068c57e3a1d242fbcd918f3ac6` |
| `FMT-1501` | Author Source formatter preservation decision | `G1` | `M` | `Done` | `INC-1410` | `fa2560fc09772ed98f8af97a71164ee1f465495f` |
| `FMT-1502` | Compiler-CST Format IR | `G1` | `M` | `Done` | `FMT-1501` | `4d006e0c410c1c59f5717df43b8fac843ed960ec` |
| `FMT-1503` | Core syntax formatting | `G1` | `M` | `Done` | `FMT-1502` | `05fe7fb9827b5e33afd094e99a0908c69d0af972` |
| `FMT-1504` | Comment attachment | `G1` | `M` | `Done` | `FMT-1503` | `4348bfe548d0f62c25efdd5bb5c704e0a46958f9` |
| `FMT-1505` | Incomplete-source recovery | `G1` | `M` | `Done` | `FMT-1504` | `da412442bde5c7624946909996fa40772d14ce77` |
| `FMT-1506` | Formatter property and semantic-equivalence evidence | `G1` | `M` | `Done` | `FMT-1505` | `18e14f0a4f19d668b1b854fb1584ec52f91afec1` |
| `FMT-1507` | Formatter CLI/LSP integration | `G1` | `M` | `Done` | `FMT-1506` | `0421925f3a8e20f6bc951eff546b00523c3f36ff` |
| `FMT-1507-CLI` | Formatter CLI Preview slice | `G1` | `M` | `Done` | `FMT-1506` | `13737c71281f12b266423d855d856fb5f77e2096` |
| `FMT-1507-EDIT` | Deterministic formatter whole-document edit projection | `G1` | `S` | `Done` | `FMT-1506` | `e33f8496de369b8fc6364007a59a0a1fb4ca9e9f` |
| `FMT-1508` | Audit Source separation | `G1` | `S` | `Done` | `FMT-1506` | `f247dfed98f104ea5227532965e8b579938a213e` |
| `GC-3301` | Minimal Managed object model | `G3` | `M` | `BlockedSpec` | — | `` |
| `GC-3301-OBSERVATION` | Internal Managed object-model boundary evidence | `G3` | `S` | `Done` | `OWN-3207-OBSERVATION` | `9b1f037a0b37f781c39c8b61390bbfe942d580d2` |
| `GC-3302` | First Managed collector | `G3` | `L` | `BlockedSpec` | `GC-3301` | `` |
| `GC-3302-OBSERVATION` | Internal Managed collector boundary evidence | `G3` | `S` | `Done` | `GC-3301-OBSERVATION` | `fc0e9a503d33ea89d9a0372745e22d3bb58ea2e1` |
| `GC-3303` | Managed Native and FFI boundary | `G3` | `L` | `BlockedSpec` | `GC-3302` | `` |
| `GC-3303-OBSERVATION` | Internal Managed/Native/FFI boundary evidence | `G3` | `S` | `Done` | `GC-3302-OBSERVATION` | `51c82b24903732216837255389bc97a89d35143b` |
| `GC-3304` | Managed profile checks | `G3` | `M` | `BlockedSpec` | `GC-3303` | `` |
| `GC-3304-OBSERVATION` | Internal Managed Profile boundary evidence | `G3` | `S` | `Done` | `GC-3303-OBSERVATION` | `37363046e2020c6e5e47093edbf2e85133d201fd` |
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
| `GPU-4601-OBSERVATION` | Internal backend spike and selection boundary evidence | `G4` | `S` | `Done` | `DIR-4503-OBSERVATION` | `d67668549aa436367f19792ddaf10e6968d9e9e4` |
| `GPU-4602` | Backend Adapter | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4602-OBSERVATION` | Internal backend adapter boundary evidence | `G4` | `S` | `Done` | `GPU-4601-OBSERVATION` | `e93888bd62fa4dee6ba4f934cd2ab24a0c0ff0a9` |
| `GPU-4603` | Launch and Runtime | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4603-OBSERVATION` | Internal launch and runtime boundary evidence | `G4` | `S` | `Done` | `GPU-4602-OBSERVATION` | `f41c2bc0a5277c7d51fd2c3050b721b9103bcd15` |
| `GPU-4604` | Differential and Hardware Matrix | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4604-OBSERVATION` | Internal differential and hardware-matrix boundary evidence | `G4` | `S` | `Done` | `GPU-4603-OBSERVATION` | `1b718796e69546462fceed0f52012cee36726885` |
| `GPU-4605` | Error Normalization | `G4` | `M` | `BlockedSpec` | — | `` |
| `GPU-4605-OBSERVATION` | Internal error-normalization boundary evidence | `G4` | `S` | `Done` | `GPU-4604-OBSERVATION` | `afb060a9ee8878b84f0c6a652c320ccd3bbdb172` |
| `IDE-2301` | IDE document symbols | `G1` | `M` | `Done` | `LSP-2101`, `LSP-2102` | `7ab847af0336d5c3de32d55e66cc3d8a932f1080` |
| `IDE-2301-INDEX` | Internal resolved-definition source-order index | `G1` | `S` | `Done` | `INC-1403`, `PRJ-1102` | `cdce9928bb84526f2adefbb0e607c84bb4ac2116` |
| `IDE-2302` | IDE hover | `G1` | `M` | `Done` | `IDE-2301`, `LSP-2201` | `81116951f9203f8374e59ae4ef6e5cd155e5d5e6` |
| `IDE-2302-TYPED-INDEX` | Internal typed-definition observation | `G1` | `S` | `Done` | `EFF-2102`, `IDE-2301-INDEX`, `INC-1405` | `d0727d95442309390a68d1af512111a8cd17f919` |
| `IDE-2303` | IDE definition navigation | `G1` | `M` | `Done` | `IDE-2301`, `IDE-2302`, `LSP-2101`, `LSP-2102` | `5abd8034dfeac3ca4b3a7b25cb18c22bfb885ec6` |
| `IDE-2303-REFERENCE-INDEX` | Internal resolved-reference target index | `G1` | `S` | `Done` | `IDE-2301-INDEX`, `INC-1404` | `1f1b10653f2a38f477cf359ac318be8b662e1503` |
| `IDE-2304` | IDE references | `G1` | `M` | `Done` | `IDE-2303`, `LSP-2101`, `LSP-2102` | `a109de62480d70c2d0d0a48b1604c8a5e04d7307` |
| `IDE-2304-REVERSE-INDEX` | Internal resolved-reference reverse index | `G1` | `S` | `Done` | `IDE-2303-REFERENCE-INDEX` | `29854ae695f58c611518190b5fcc58263458c6b0` |
| `IDE-2305` | IDE prepare rename | `G1` | `M` | `Done` | `IDE-2304`, `LSP-2102` | `9619693d5e2ae5c9ffd4ec05ef578606b87fcce9` |
| `IDE-2305-IDENTIFIER-OBSERVATION` | Internal rename-identifier Unicode observation | `G1` | `S` | `Done` | `IDE-2304-REVERSE-INDEX` | `e2681b0fe08ec95805e6a1346b3e8693ba83a3af` |
| `IDE-2306` | IDE rename | `G1` | `L` | `Done` | `IDE-2305`, `LSP-2102`, `LSP-2104` | `ecb6545fec5fa1f457ee9abf69c7354306ea1bb0` |
| `IDE-2306-REFERENCE-SPANS` | Internal resolved-reference source-span observation | `G1` | `S` | `Done` | `IDE-2304-REVERSE-INDEX`, `IDE-2305-IDENTIFIER-OBSERVATION` | `70e0aeb833013a65c73e71b48d195c2d69454bfd` |
| `IDE-2307` | IDE completion v0 | `G1` | `L` | `Done` | `IDE-2304`, `LSP-2101`, `LSP-2102` | `360315e4ec52b7e19ecdb475629d0fe71c1594e4` |
| `IDE-2307-SOURCE-INDEX` | Internal resolver completion-source inventory | `G1` | `S` | `Done` | `IDE-2304-REVERSE-INDEX`, `IDE-2305-IDENTIFIER-OBSERVATION` | `d9183705b1d720abb90100ff93ee251419290764` |
| `IDE-2308` | IDE completion resolve | `G1` | `M` | `Done` | `IDE-2307`, `LSP-2101`, `LSP-2102` | `523c9de626c4a320028e6457676e526bfa53f247` |
| `IDE-2308-METADATA` | Internal completion checked-metadata observation | `G1` | `S` | `Done` | `IDE-2307-SOURCE-INDEX` | `c528bf7bd7292ff681b41bb4e274ba417f84cdbd` |
| `IDE-2309` | IDE code actions | `G1` | `L` | `Done` | `FMT-1507`, `IDE-2308`, `LSP-2201`, `LSP-2202` | `369432ef6146a27e236276ced2ab14e9065ca5af` |
| `IDE-2309-REPAIR-INDEX` | Internal structured diagnostic repair index | `G1` | `S` | `Done` | `IDE-2308-METADATA` | `fa54770a8ab71dd93eb6c0a7239153851d0c9593` |
| `IDE-2310` | IDE formatting | `G1` | `M` | `Done` | `FMT-1507`, `LSP-2102` | `d8e2a377830d0e34f27603036253e649214bdcaf` |
| `IDE-2311` | IDE workspace symbols | `G1` | `L` | `Done` | `IDE-2301`, `LSP-2101`, `LSP-2102` | `58a425fe42e5112071ddeda922e29cbf2f974148` |
| `IDE-2311-SOURCE-LOOKUPS` | Internal exact workspace-symbol source lookups | `G1` | `S` | `Done` | `IDE-2301-INDEX` | `bf44203af7d2548b768b496c5bf09b0eb3452731` |
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
| `KCHK-4101-OBSERVATION` | Internal Kernel capability-matrix boundary evidence | `G4` | `S` | `Done` | `DAP-3603-OBSERVATION` | `dfae551210f494cd67dd2815901de833132db952` |
| `KCHK-4102` | Kernel Effect and Capability checks | `G4` | `L` | `BlockedSpec` | `KCHK-4101` | `` |
| `KCHK-4102-OBSERVATION` | Internal Kernel Effect and Capability boundary evidence | `G4` | `S` | `Done` | `KCHK-4101-OBSERVATION` | `4248facc2fe1fef95e20eda30e1ef6bffc593b4a` |
| `KCHK-4103` | Shape, index, and bounds | `G4` | `L` | `BlockedSpec` | `KCHK-4102` | `` |
| `KCHK-4103-OBSERVATION` | Internal Kernel shape/index/bounds boundary evidence | `G4` | `S` | `Done` | `KCHK-4102-OBSERVATION` | `c85823812120f41b65db093900f96077fbdecf20` |
| `KCHK-4104` | Alias and parallel-write conflicts | `G4` | `L` | `BlockedSpec` | `KCHK-4103` | `` |
| `KCHK-4104-OBSERVATION` | Internal Kernel alias/parallel-write boundary evidence | `G4` | `S` | `Done` | `KCHK-4103-OBSERVATION` | `5a0fe371452fbdf4bfdd87dd31b8030456b3e5b2` |
| `KCHK-4105` | Kernel Core and verifier | `G4` | `L` | `BlockedSpec` | `KCHK-4104` | `` |
| `KCHK-4105-OBSERVATION` | Internal Kernel Core/verifier boundary evidence | `G4` | `S` | `Done` | `KCHK-4104-OBSERVATION` | `b362af10cd5bf5bc872cb46625aa05aa90c09b03` |
| `LSP-2101` | LSP lifecycle skeleton | `G1` | `S` | `Done` | `CLI-1701` | `eb71c9db15263a38b070962efde9efb04144bcee` |
| `LSP-2101-LIFECYCLE` | LSP lifecycle Preview slice | `G1` | `S` | `Done` | — | `38d95fb7b91c2035bd2b1b4ebf864c1693050925` |
| `LSP-2102` | LSP position-encoding negotiation | `G1` | `S` | `Done` | `LSP-2101` | `15cdfe7963ecd272447ff9ac00d1b71df0a63800` |
| `LSP-2102-NEGOTIATION` | LSP initialize position-encoding negotiation | `G1` | `S` | `Done` | `LSP-2101-LIFECYCLE`, `LSP-2102-SOURCE-MAP` | `39755afad13db66b429967fe61f20f66a4aea699` |
| `LSP-2102-SOURCE-MAP` | LSP SourceMap position projection | `G1` | `S` | `Done` | — | `9e917250bff5bb3ebba1ef02a5f2f6b66ab700de` |
| `LSP-2103` | LSP open-document overlay | `G1` | `M` | `Done` | `LSP-2101`, `LSP-2102` | `5f64ab8bf4de8f562d63dca1bcd627808f955dbc` |
| `LSP-2103-OVERLAY` | LSP full-text overlay Preview slice | `G1` | `M` | `Done` | `LSP-2101-LIFECYCLE`, `LSP-2102-SOURCE-MAP` | `6cedec3e09f8112b28cde1c12dca514dae4191e4` |
| `LSP-2104` | LSP incremental text changes | `G1` | `M` | `Done` | `LSP-2102`, `LSP-2103` | `492754b066da11e4ae2fe58774e5c7096e3703a5` |
| `LSP-2104-POSITION-EDITS` | Bounded internal position-edit projection | `G1` | `S` | `Done` | `LSP-2102-SOURCE-MAP`, `LSP-2104-UTF8-EDITS` | `94a990418730a768a7974f3f40629f1d32a5c05e` |
| `LSP-2104-UTF8-EDITS` | Bounded internal UTF-8 edit application | `G1` | `S` | `Done` | `LSP-2102-SOURCE-MAP`, `LSP-2103-OVERLAY` | `4abc84f81bab339c309a1e5a6227fc7f9483e013` |
| `LSP-2105` | LSP workspace reload | `G1` | `M` | `Done` | `LSP-2101`, `LSP-2103`, `PRJ-1107` | `49994b9132ff22ae3fd17ab172476d020a79febe` |
| `LSP-2105-WORKSPACE-SNAPSHOT` | Bounded internal workspace-state snapshot | `G1` | `S` | `Done` | `INC-1402`, `LSP-2103-OVERLAY` | `60566b4cd547c1969c0deec512f11f59069e2e7c` |
| `LSP-2201` | LSP compiler diagnostic adapter | `G1` | `M` | `Done` | `LSP-2101`, `LSP-2102` | `9a5310fbe48fd40ae5c3d05c7720656361f0b08f` |
| `LSP-2201-DIAGNOSTIC-POSITION` | Bounded internal diagnostic span projection | `G1` | `S` | `Done` | `LSP-2102-SOURCE-MAP`, `LSP-2201-ORDERING` | `e66669142f5b8720a39157532edd77b8bc46269a` |
| `LSP-2201-ORDERING` | Internal canonical diagnostic ordering | `G1` | `S` | `Done` | `LSP-2102-NEGOTIATION`, `LSP-2102-SOURCE-MAP` | `9bf092f0fe6fa0c0cabc371c42392d78ad3d3d53` |
| `LSP-2202` | LSP push diagnostics v0 | `G1` | `M` | `Done` | `LSP-2103`, `LSP-2201` | `4914d2346f5647f2cdfad85ef4f1335bd44b9f12` |
| `LSP-2202-BATCH` | Internal immutable diagnostic batch | `G1` | `S` | `Done` | `LSP-2201-ORDERING`, `LSP-2501-SNAPSHOT` | `15f8461582281ae1f9e99250ca6288a8b2d5f3b4` |
| `LSP-2203` | LSP pull diagnostics Preview | `G1` | `M` | `Done` | `LSP-2101`, `LSP-2201` | `da69abff0c74765283d3e52e182a7c0ae2f8dc3a` |
| `LSP-2204` | LSP root-cause and error-storm control | `G1` | `M` | `Done` | `LSP-2201`, `LSP-2202` | `b70308c1e215fd2f4a4736aa56d7372c368af599` |
| `LSP-2205` | LSP diagnostic fixtures | `G1` | `M` | `Done` | `LSP-2201`, `LSP-2202`, `LSP-2203`, `LSP-2204` | `93a58e9090ce5a3be17bcfb8569d7246ce7d71ec` |
| `LSP-2401` | Semantic token taxonomy RFC/decision | `G1` | `M` | `Done` | `IDE-2311`, `LSP-2101`, `LSP-2102` | `cdc70731f75e27fc508402304e628dcc334c20fb` |
| `LSP-2401-LEXICAL-SOURCE` | Internal lexical token source index | `G1` | `S` | `Done` | `LSP-2102-SOURCE-MAP` | `135bfb07bb8f824f1ab119b5e0f765c674d6f294` |
| `LSP-2402` | Typed semantic-token generation | `G1` | `L` | `Done` | `LSP-2101`, `LSP-2102`, `LSP-2401` | `899d00f56d444f43a5128da844e517ef3a85e186` |
| `LSP-2402-CHECKED-IDENTITY` | Internal checked-token identity observation | `G1` | `S` | `Done` | `LSP-2401-LEXICAL-SOURCE` | `bbb0786453b8fefee974b4137b47587ff636c60c` |
| `LSP-2403` | Semantic token full and delta transport | `G1` | `L` | `Done` | `LSP-2401`, `LSP-2402` | `75c2bd0b7d0dcd6e37cdb16e0ec854529f56f97f` |
| `LSP-2403-SNAPSHOT-IDENTITY` | Internal checked-token snapshot identity | `G1` | `S` | `Done` | `LSP-2402-CHECKED-IDENTITY` | `74b3135ef51558a151af2d4a064f907281e4a32e` |
| `LSP-2404` | Semantic-token fixture corpus | `G1` | `M` | `Done` | `LSP-2401`, `LSP-2402`, `LSP-2403` | `9105ff5be4aad29b471d5997594156a923f5cb56` |
| `LSP-2404-CHECKED-SOURCE-FIXTURES` | Internal checked-token source fixture corpus | `G1` | `S` | `Done` | `LSP-2403-SNAPSHOT-IDENTITY` | `cee068af008cbd1e0327799a418078be850db754` |
| `LSP-2501` | LSP request snapshot | `G1` | `L` | `Done` | `INC-1401`, `INC-1402`, `LSP-2103`, `LSP-2104` | `e5434f632963d622834d90168980c9524414d12b` |
| `LSP-2501-SNAPSHOT` | Internal immutable LSP request snapshot capture | `G1` | `S` | `Done` | `INC-1402`, `LSP-2102-NEGOTIATION`, `LSP-2103-OVERLAY` | `64eb8e858f87bc4f5896bbb8ac00f53afc48c97d` |
| `LSP-2502` | LSP request cancellation | `G1` | `L` | `Done` | `INC-1401`, `LSP-2501` | `9d6edd7b2a6fbf751afffaae46b249f0ee6e52e4` |
| `LSP-2502-CANCELLATION` | Internal cooperative LSP cancellation token | `G1` | `S` | `Done` | `INC-1401`, `LSP-2501-SNAPSHOT` | `132fcc9073bebf2a5e49716bc61048248cc4305a` |
| `LSP-2503` | LSP debounce and priority scheduling | `G1` | `L` | `Done` | `LSP-2103`, `LSP-2104`, `LSP-2202`, `LSP-2501`, `LSP-2502` | `5d7ebb6fb41eea4e4a224d42fac20224e78708a2` |
| `LSP-2503-SCHEDULER` | Internal deterministic LSP work ordering | `G1` | `S` | `Done` | `LSP-2501-SNAPSHOT`, `LSP-2502-CANCELLATION` | `bca5b93faeaf582c00cdd0d39b52fb2f741c9ccd` |
| `LSP-2504` | LSP memory and resource limits | `G1` | `L` | `Done` | `LSP-2201`, `LSP-2501`, `LSP-2502`, `LSP-2503` | `b1fa575a533fbe60dfb48d29e0af287972a4a141` |
| `LSP-2504-BYTE-ACCOUNTING` | Internal deterministic LSP UTF-8 byte accounting | `G1` | `S` | `Done` | `LSP-2501-SNAPSHOT`, `LSP-2502-CANCELLATION`, `LSP-2503-SCHEDULER` | `2bf9ea77d87716138920201116c8849f4625c2e3` |
| `MC-5601` | Finite-State Projection | `G5` | `L` | `BlockedSpec` | `BND-5204`, `NODE-5307`, `PROOF-5503` | `` |
| `MC-5601-OBSERVATION` | Internal Finite-State Projection boundary evidence | `G5` | `S` | `Done` | `PROOF-5503-OBSERVATION` | `7ff487b1fe6f886fa664c68a006dbb0a185f73ec` |
| `MC-5602` | Exploration Engine | `G5` | `L` | `BlockedSpec` | `MC-5601` | `` |
| `MC-5602-OBSERVATION` | Internal Exploration Engine boundary evidence | `G5` | `S` | `Done` | `MC-5601-OBSERVATION` | `19a64ffc75edbe47f3e65d6b36f2183575270c95` |
| `MC-5603` | Model-Check Report Semantics | `G5` | `M` | `BlockedSpec` | `MC-5602` | `` |
| `MC-5603-OBSERVATION` | Internal Model-Check Report boundary evidence | `G5` | `S` | `Done` | `MC-5602-OBSERVATION` | `b1abd86442f7f5908961646eee6fd377d4811219` |
| `MC-5604` | Replay Counterexample | `G5` | `L` | `BlockedSpec` | `MC-5603` | `` |
| `MC-5604-OBSERVATION` | Internal Replay Counterexample boundary evidence | `G5` | `S` | `Done` | `MC-5603-OBSERVATION` | `ef57fe3c594229bad9c1ef098c8beac3d7710c72` |
| `MEM-3101` | Type classification model | `G3` | `M` | `BlockedSpec` | — | `` |
| `MEM-3101-SEED-VALUE` | Seed completed-type Value classification | `G3` | `S` | `Done` | — | `c64ceb9101190d630125d3a7b6e1ede150c01488` |
| `MEM-3102` | Value layout and Copy/Move | `G3` | `L` | `BlockedSpec` | `MEM-3101` | `` |
| `MEM-3102-OBSERVATION` | Internal Value-layout and Copy/Move boundary evidence | `G3` | `S` | `Done` | `MEM-3101-SEED-VALUE` | `6f5d2c1da186151ce36c0c67a3f3410424ae48b4` |
| `MEM-3103` | Resource definition and Drop contract | `G3` | `L` | `BlockedSpec` | `MEM-3102` | `` |
| `MEM-3103-OBSERVATION` | Internal Resource and Drop boundary evidence | `G3` | `S` | `Done` | `MEM-3102-OBSERVATION` | `339e7ff12d67f19c2d99a025353103bb809e87db` |
| `MEM-3104` | Managed types and island boundaries | `G3` | `L` | `BlockedSpec` | `MEM-3103` | `` |
| `MEM-3104-OBSERVATION` | Internal Managed-graph and island boundary evidence | `G3` | `S` | `Done` | `MEM-3103-OBSERVATION` | `778228b810ef690b43a7eb7ce3d3b76dafdb4aeb` |
| `NIR-3401` | Native IR design | `G3` | `L` | `BlockedSpec` | `GC-3304` | `` |
| `NIR-3401-OBSERVATION` | Internal Native IR design boundary evidence | `G3` | `S` | `Done` | `GC-3304-OBSERVATION` | `7192361d50163d96946cb7ab2e3f3daf4f670d9d` |
| `NIR-3402` | Core to Native IR lowering | `G3` | `L` | `BlockedSpec` | `NIR-3401` | `` |
| `NIR-3402-OBSERVATION` | Internal Native IR lowering boundary evidence | `G3` | `S` | `Done` | `NIR-3401-OBSERVATION` | `ebe22cf01a670c58686a829b476ff6e106599441` |
| `NIR-3403` | Native IR verifier | `G3` | `M` | `BlockedSpec` | `NIR-3402` | `` |
| `NIR-3403-OBSERVATION` | Internal Native IR verifier boundary evidence | `G3` | `S` | `Done` | `NIR-3402-OBSERVATION` | `138a8671c22a7a8e04376b7b0440f2ab6f532ef7` |
| `NODE-5301` | Node Syntax and Semantics | `G5` | `L` | `BlockedSpec` | `BND-5204` | `` |
| `NODE-5301-OBSERVATION` | Internal Node syntax/semantics boundary evidence | `G5` | `S` | `Done` | `BND-5204-OBSERVATION` | `157e3cf79e6d35af6eae4568b55138f97257c1f4` |
| `NODE-5302` | Node Checked Core | `G5` | `M` | `BlockedSpec` | `NODE-5301` | `` |
| `NODE-5302-OBSERVATION` | Internal Node Checked Core boundary evidence | `G5` | `S` | `Done` | `NODE-5301-OBSERVATION` | `4008b1ca42444f080d40ac71708ba7bebd9de00e` |
| `NODE-5303` | Static Node Scheduling | `G5` | `M` | `BlockedSpec` | `NODE-5302` | `` |
| `NODE-5303-OBSERVATION` | Internal Node static-scheduling boundary evidence | `G5` | `S` | `Done` | `NODE-5302-OBSERVATION` | `5c60fbc0984a37c163edcf79056fa6a7a04418d7` |
| `NODE-5304` | Virtual-Time Reference Runtime | `G5` | `M` | `BlockedSpec` | `NODE-5303` | `` |
| `NODE-5304-OBSERVATION` | Internal Node virtual-time runtime boundary evidence | `G5` | `S` | `Done` | `NODE-5303-OBSERVATION` | `39e00c566f57f4433051e0f8e04db73e1ed8c1b3` |
| `NODE-5305` | Native Node Runtime | `G5` | `L` | `BlockedSpec` | `NODE-5304` | `` |
| `NODE-5305-OBSERVATION` | Internal Node Native-runtime boundary evidence | `G5` | `S` | `Done` | `NODE-5304-OBSERVATION` | `d08d51178f99c511c4a21fa4c4aa97a17b19b02a` |
| `NODE-5306` | Node and Actor Boundary | `G5` | `M` | `BlockedSpec` | `NODE-5305` | `` |
| `NODE-5306-OBSERVATION` | Internal Node/Actor boundary evidence | `G5` | `S` | `Done` | `NODE-5305-OBSERVATION` | `6b3c43226d741b7d834b7e48caf095c35fea6955` |
| `NODE-5307` | Node Conformance | `G5` | `M` | `BlockedSpec` | `NODE-5306` | `` |
| `NODE-5307-OBSERVATION` | Internal Node conformance boundary evidence | `G5` | `S` | `Done` | `NODE-5306-OBSERVATION` | `0a588dbd2c3db17a66e50af18cf640427eb41a2c` |
| `OWN-3201` | Place and Move analysis | `G3` | `L` | `BlockedSpec` | `MEM-3104` | `` |
| `OWN-3201-OBSERVATION` | Internal Place and Move-analysis boundary evidence | `G3` | `S` | `Done` | `MEM-3104-OBSERVATION` | `1c6528e25fd0b8908cc8b3be5db873a3d2132d0b` |
| `OWN-3202` | Borrow exclusivity | `G3` | `L` | `BlockedSpec` | `OWN-3201` | `` |
| `OWN-3202-OBSERVATION` | Internal borrow-exclusivity boundary evidence | `G3` | `S` | `Done` | `OWN-3201-OBSERVATION` | `bd513f54ece16d4d8618f3011cd1607c7a30acd4` |
| `OWN-3203` | Region inference | `G3` | `L` | `BlockedSpec` | `OWN-3202` | `` |
| `OWN-3203-OBSERVATION` | Internal region-inference boundary evidence | `G3` | `S` | `Done` | `OWN-3202-OBSERVATION` | `31f6c7ea06fe727ade85e9c07fce13a3f6174b47` |
| `OWN-3204` | Borrow across await and Actor turns | `G3` | `L` | `BlockedSpec` | `ACT-2304`, `OWN-3203` | `` |
| `OWN-3204-OBSERVATION` | Internal cross-suspension and Actor-turn boundary evidence | `G3` | `S` | `Done` | `OWN-3203-OBSERVATION` | `e32aaca6f789c48a386d6476d9138e1b41267f73` |
| `OWN-3205` | Drop-order lowering | `G3` | `L` | `BlockedSpec` | `MEM-3103`, `OWN-3204` | `` |
| `OWN-3205-OBSERVATION` | Internal Drop-order and cleanup boundary evidence | `G3` | `S` | `Done` | `OWN-3204-OBSERVATION` | `c7c6e1727c9feece56a07c7623c6501d92048615` |
| `OWN-3206` | Ownership diagnostics and repairs | `G3` | `L` | `BlockedSpec` | `OWN-3201`, `OWN-3202`, `OWN-3203`, `OWN-3204`, `OWN-3205` | `` |
| `OWN-3206-OBSERVATION` | Internal ownership-diagnostic and repair boundary evidence | `G3` | `S` | `Done` | `OWN-3205-OBSERVATION` | `6088923b9b8cc1accea7f7283784ac9ef2cd428c` |
| `OWN-3207` | Negative corpus and property tests | `G3` | `L` | `BlockedSpec` | `OWN-3201`, `OWN-3202`, `OWN-3203`, `OWN-3204`, `OWN-3205`, `OWN-3206` | `` |
| `OWN-3207-OBSERVATION` | Internal ownership corpus and property boundary evidence | `G3` | `S` | `Done` | `OWN-3206-OBSERVATION` | `a905fc3d8efd7a04d39767a02c662de2d8bf4a5f` |
| `PKG-6401` | Package Publication Protocol | `G6` | `L` | `BlockedSpec` | `STD-6303` | `` |
| `PKG-6401-OBSERVATION` | Local package and publication-exclusion boundary evidence | `G6` | `S` | `Done` | `STD-6303-OBSERVATION` | `a2b6d478256fe2578ba2ddccf2b38399a3b6b6f0` |
| `PKG-6402` | Hermetic Build | `G6` | `L` | `BlockedSpec` | `PKG-6401` | `` |
| `PKG-6402-OBSERVATION` | Hermetic-build exclusion boundary evidence | `G6` | `S` | `Done` | `PKG-6401-OBSERVATION` | `21b0ea6b75e70d8328c5b9f70ba1acdcbe959fee` |
| `PKG-6403` | Registry Minimum Implementation or Deferment Strategy | `G6` | `L` | `BlockedSpec` | `PKG-6402` | `` |
| `PKG-6403-DEFERMENT` | Registry deferment strategy evidence | `G6` | `S` | `Done` | `PKG-6402-OBSERVATION` | `875926c368068a4790f7b08fb1710ddbdd749afc` |
| `PKG-6404` | Supply-Chain Attack Tests | `G6` | `L` | `BlockedSpec` | `PKG-6403` | `` |
| `PKG-6404-LOCAL` | Local supply-chain attack-boundary evidence | `G6` | `S` | `Done` | `PKG-6403-DEFERMENT` | `7eb48376bc3ee70ad20a74a7a732bb3498b4af52` |
| `PLC-4801` | Placement Constraint Model | `G4` | `M` | `BlockedSpec` | — | `` |
| `PLC-4801-OBSERVATION` | Internal Placement-constraint boundary evidence | `G4` | `S` | `Done` | `ACC-4702-OBSERVATION` | `74011d062b0f63647f902111e24d310d74fb3976` |
| `PLC-4802` | Static Candidates and Runtime Selection | `G4` | `M` | `BlockedSpec` | `PLC-4801` | `` |
| `PLC-4802-OBSERVATION` | Internal Placement-selection boundary evidence | `G4` | `S` | `Done` | `PLC-4801-OBSERVATION` | `f721a38c61befd3e9bc10f5fcabdbd0f723f7e9d` |
| `PLC-4803` | Cost Model v0 | `G4` | `M` | `BlockedSpec` | `PLC-4802` | `` |
| `PLC-4803-OBSERVATION` | Internal Cost Model boundary evidence | `G4` | `S` | `Done` | `PLC-4802-OBSERVATION` | `22b490bbcc3800b8527cc0d85bbc14c9ba34a9d1` |
| `PLC-4804` | Placement Explain Output | `G4` | `M` | `BlockedSpec` | `PLC-4803` | `` |
| `PLC-4804-OBSERVATION` | Internal Placement-explain boundary evidence | `G4` | `S` | `Done` | `PLC-4803-OBSERVATION` | `5845160677d8d01d1e7035b1ff1ad106c5744630` |
| `PLC-4805` | Device Binary Cache | `G4` | `M` | `BlockedSpec` | `PLC-4804` | `` |
| `PLC-4805-OBSERVATION` | Internal Device Binary Cache boundary evidence | `G4` | `S` | `Done` | `PLC-4804-OBSERVATION` | `295350a636fc26d819ea9d2a439c5364be0aa493` |
| `PRJ-1101` | Minimal project manifest | `G1` | `M` | `Done` | — | `80f46bcf3d175eeb6402bf6267085cb905a5dbcf` |
| `PRJ-1102` | Deterministic module discovery | `G1` | `M` | `Done` | `PRJ-1101` | `f76f4953070b9ae555fce24c7dcc2fbf08a36f7a` |
| `PRJ-1103` | Package-aware imports and visibility | `G1` | `M` | `Done` | `PRJ-1102`, `PRJ-1104` | `8e98d32f54301dee3f198273cfae3146bbf2846b` |
| `PRJ-1104` | Content-identified local dependency graph | `G1` | `L` | `Done` | `PRJ-1101`, `PRJ-1102` | `66a64c9b57c8bb327599a7463345c9d2fbe77a51` |
| `PRJ-1105` | Canonical project lockfile protocol | `G1` | `L` | `Done` | `PRJ-1104` | `9ff0fcca0c65b7e9e2fccf3c1df001b4737d3082` |
| `PRJ-1106` | End-to-end project fixture matrix | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105` | `0e9c5800411a6f1acd1441068e6ce2fd58f29816` |
| `PRJ-1107` | Project API and CLI integration | `G1` | `M` | `Done` | `PRJ-1106` | `56790c7c9fc8e58821f9fb829e978c3bf746725b` |
| `PRJ-1107-CHECK` | Locked project graph check Preview | `G1` | `M` | `Done` | `PRJ-1106` | `b37ab47f268e83e8c2c39931f5ddd0a311dbf4aa` |
| `PRJ-1107-CURRENT-EVIDENCE` | Current project CLI/API boundary evidence | `G1` | `S` | `Done` | `PRJ-1107-CHECK`, `PRJ-1107-LOAD`, `PRJ-1107-SEMANTIC-SNAPSHOT` | `0822280f4a2ea2c9e4205e8fbff104a00b522ef3` |
| `PRJ-1107-LOAD` | Locked project snapshot boundary | `G1` | `S` | `Done` | `PRJ-1106` | `23f9eb78b1bb3dfb0c90de88be30f4e16b248196` |
| `PRJ-1107-SEMANTIC-SNAPSHOT` | Internal locked-project semantic snapshot | `G1` | `M` | `Done` | `PRJ-1103`, `PRJ-1107-LOAD` | `026555cad800dd660fc550408d300fffd3c7af68` |
| `PRJ-1108` | Project graph property and manifest fuzz coverage | `G1` | `M` | `Done` | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105`, `PRJ-1106` | `29f9c4465b58c7eff23c227436563d69409b880e` |
| `PROF-5101` | Machine-Readable Critical Profile | `G5` | `M` | `BlockedSpec` | `PLC-4805` | `` |
| `PROF-5101-OBSERVATION` | Internal Critical Profile boundary evidence | `G5` | `S` | `Done` | `PLC-4805-OBSERVATION` | `f01ee509850941a633e6aeff7fb0d0d8fec4e4e4` |
| `PROF-5102` | Forbidden Capability Checks | `G5` | `M` | `BlockedSpec` | `PROF-5101` | `` |
| `PROF-5102-OBSERVATION` | Internal forbidden-capability boundary evidence | `G5` | `S` | `Done` | `PROF-5101-OBSERVATION` | `3c6a27a0afed8a503c312358cff6dc85d7760514` |
| `PROF-5103` | Profile Composition | `G5` | `M` | `BlockedSpec` | `PROF-5102` | `` |
| `PROF-5103-OBSERVATION` | Internal Profile Composition boundary evidence | `G5` | `S` | `Done` | `PROF-5102-OBSERVATION` | `2edc80e7e24e9479ebb2e7c6a9367143c5ecd109` |
| `PROF-5104` | Profile Audit and LSP | `G5` | `M` | `BlockedSpec` | `PROF-5103` | `` |
| `PROF-5104-OBSERVATION` | Internal Profile Audit/LSP boundary evidence | `G5` | `S` | `Done` | `PROF-5103-OBSERVATION` | `7578bdaebbc65043696c4a0cc582258598864f2c` |
| `PROOF-5501` | Proof IR | `G5` | `L` | `BlockedSpec` | `CTR-5405` | `` |
| `PROOF-5501-OBSERVATION` | Internal Proof IR boundary evidence | `G5` | `S` | `Done` | `CTR-5407-OBSERVATION` | `defaa5f97ea9c406c5f7012412dc8e0ec6f0c4a3` |
| `PROOF-5502` | Independent Checker | `G5` | `L` | `BlockedSpec` | `PROOF-5501` | `` |
| `PROOF-5502-OBSERVATION` | Internal Independent Checker boundary evidence | `G5` | `S` | `Done` | `PROOF-5501-OBSERVATION` | `657d3a63eea75b45708aa91aa7806e16ced03e68` |
| `PROOF-5503` | Assumption Registry | `G5` | `M` | `BlockedSpec` | `PROOF-5502` | `` |
| `PROOF-5503-OBSERVATION` | Internal Assumption Registry boundary evidence | `G5` | `S` | `Done` | `PROOF-5502-OBSERVATION` | `2a34482fb99c7a045f99fd456cf68961aefdfcb6` |
| `PROTO-6201` | Protocol Registry | `G6` | `L` | `BlockedSpec` | `STAB-6103` | `` |
| `PROTO-6201-OBSERVATION` | Internal Protocol Registry boundary evidence | `G6` | `S` | `Done` | `STAB-6103-OBSERVATION` | `5c098c927fbae55cb2e984b6ab40cc983df742cd` |
| `PROTO-6202` | Reader/Writer Compatibility Tests | `G6` | `L` | `BlockedSpec` | `PROTO-6201` | `` |
| `PROTO-6202-OBSERVATION` | Internal Reader/Writer Compatibility boundary evidence | `G6` | `S` | `Done` | `PROTO-6201-OBSERVATION` | `64076eaade1f6858718e70e33f131f835b5c71b1` |
| `PROTO-6203` | Semantic Hash Upgrade Rehearsal | `G6` | `L` | `BlockedSpec` | `PROTO-6202` | `` |
| `PROTO-6203-OBSERVATION` | Internal Semantic Hash Upgrade Rehearsal boundary evidence | `G6` | `S` | `Done` | `PROTO-6202-OBSERVATION` | `37aa6b4cc623fe1322e2806c71f7be349373117b` |
| `PROTO-6204` | CLI and Exit-Code Freeze | `G6` | `L` | `BlockedSpec` | `PROTO-6203` | `` |
| `PROTO-6204-OBSERVATION` | Internal CLI and Exit-Code Freeze boundary evidence | `G6` | `S` | `Done` | `PROTO-6203-OBSERVATION` | `dfc288270f58029ddcb454eda5a7936a931b8c83` |
| `RC-6901` | RC0 Internal Freeze | `G6` | `L` | `BlockedSpec` | `RC-6901-CURRENT-EVIDENCE`, `ZED-6804` | `` |
| `RC-6901-CURRENT-EVIDENCE` | Current RC0 status/protocol evidence | `G6` | `S` | `Done` | `RC-6901-SEED`, `ZED-6804-CURRENT-EVIDENCE` | `a72dc4baf5efc2bceabd03e074b36b87cb47b36b` |
| `RC-6901-SEED` | Seed RC0 internal-freeze inventory drift gate | `G6` | `S` | `Done` | `ZED-6804-SEED` | `dc0f2e5ca98b9c705b9932267011c246058db4ec` |
| `RC-6902` | RC1 Public Validation | `G6` | `L` | `BlockedSpec` | `RC-6901`, `RC-6902-CURRENT-EVIDENCE` | `` |
| `RC-6902-CURRENT-EVIDENCE` | Current RC1 RC0/Zed boundary evidence | `G6` | `S` | `Done` | `RC-6901-CURRENT-EVIDENCE`, `RC-6902-SEED`, `ZED-6803-CURRENT-EVIDENCE` | `554005ef1eea6ab738a5b62657fdb9a5c3599ace` |
| `RC-6902-SEED` | Seed RC1 public-validation inventory drift gate | `G6` | `S` | `Done` | `RC-6901-SEED` | `056c6afb17fd1e8656b825e2c2a4f70b4173f2d8` |
| `RC-6903` | Independent Verification | `G6` | `L` | `BlockedSpec` | `RC-6902`, `RC-6903-CURRENT-EVIDENCE` | `` |
| `RC-6903-CURRENT-EVIDENCE` | Current RC3 upstream-boundary evidence | `G6` | `S` | `Done` | `RC-6902-CURRENT-EVIDENCE`, `RC-6903-SEED` | `723ac8bfd3c5360be474ab3049c60749fa12c748` |
| `RC-6903-SEED` | Seed RC3 independent-verification inventory drift gate | `G6` | `S` | `Done` | `RC-6902-SEED` | `c782b771b0c72c8cf7cbdcbfa3ab64a6cd001774` |
| `RC-6904` | RC2 / Final Change Control | `G6` | `L` | `BlockedSpec` | `RC-6903`, `RC-6904-CURRENT-EVIDENCE` | `` |
| `RC-6904-CURRENT-EVIDENCE` | Current RC2 upstream/protocol evidence | `G6` | `S` | `Done` | `RC-6903-CURRENT-EVIDENCE`, `RC-6904-SEED` | `ced71085cd75b24ffc3d9104ade896b1f0c40463` |
| `RC-6904-SEED` | Seed RC2/final change-control inventory drift gate | `G6` | `S` | `Done` | `RC-6903-SEED` | `b8660b524e103b615f9df332b55b4da8b3638c8b` |
| `RC-6905` | v1.0 Release Artifacts | `G6` | `L` | `BlockedSpec` | `RC-6904`, `RC-6905-CURRENT-EVIDENCE` | `` |
| `RC-6905-CURRENT-EVIDENCE` | Current v1 upstream/LSP/protocol evidence | `G6` | `S` | `Done` | `RC-6904-CURRENT-EVIDENCE`, `RC-6905-SEED`, `ZED-6802-CURRENT-EVIDENCE` | `c0aeb02c2f2d0b28dffefdb61c9b19814909e4bb` |
| `RC-6905-SEED` | Seed v1 release-artifact inventory drift gate | `G6` | `S` | `Done` | `RC-6904-SEED` | `7b2803ddce930278b03d43aacdbf1e3883532b1d` |
| `REL-6601` | Fuzz Coverage Inventory | `G6` | `L` | `BlockedSpec` | `COMPAT-6504` | `` |
| `REL-6601-SEED` | Seed fuzz inventory and corpus drift gate | `G6` | `S` | `Done` | `FMT-1506`, `GOV-0110`, `PRJ-1108`, `VM-1210` | `97a628311f50742730ba043131878d205f0f47d2` |
| `REL-6601-SEMANTIC-SCHEMA` | Semantic Graph Reader Fuzz Coverage | `G6` | `S` | `Done` | `REL-6601-SEED` | `be20332ed1bed9becae115e211f1387e25799b17` |
| `REL-6602` | Fault Injection | `G6` | `L` | `BlockedSpec` | `REL-6601` | `` |
| `REL-6602-LOCK-PERSISTENCE` | Lock-persistence Fault Injection | `G6` | `S` | `Done` | `REL-6602-SEED` | `85fcbb3a4cf819560e4fa6663b1f21b089b74c0e` |
| `REL-6602-SEED` | Seed fault-matrix drift gate | `G6` | `S` | `Done` | `REL-6601-SEED` | `201d9f7cfde5e6c2eebaa5b5324297b6a790d226` |
| `REL-6603` | Security Audit | `G6` | `L` | `BlockedSpec` | `REL-6602` | `` |
| `REL-6603-SEED` | Seed security-audit matrix drift gate | `G6` | `S` | `Done` | `REL-6602-SEED` | `d96bea813fb6576ef3212cbffeb478d9829210ec` |
| `REL-6603-UNSAFE-POLICY` | Workspace unsafe-policy drift gate | `G6` | `S` | `Done` | `REL-6603-SEED` | `1b6548eef4fdc3a6e46615bc7d36d4c06f6e2924` |
| `REL-6604` | Performance Baseline | `G6` | `L` | `BlockedSpec` | `REL-6603` | `` |
| `REL-6604-ARTIFACT` | Performance-baseline artifact integrity gate | `G6` | `S` | `Done` | `REL-6604-SEED` | `f6557b9ea238fbaa2a0c6b95d1be7ca43993e5c5` |
| `REL-6604-SEED` | Seed performance-matrix drift gate | `G6` | `S` | `Done` | `REL-6603-SEED` | `1177fdf133de670ad039aac64aa1c8add92ab249` |
| `REM-2601` | RemoteRef and endpoint | `G2` | `L` | `BlockedSpec` | `ACT-2305`, `REP-2506` | `` |
| `REM-2601-OBSERVATION` | Internal RemoteRef and endpoint boundary evidence | `G2` | `S` | `Done` | `REP-2506-OBSERVATION` | `ab2f17e86bbeb105dc7bd35870d053897c7c66cd` |
| `REM-2602` | Transport-neutral envelope | `G2` | `L` | `BlockedSpec` | `REM-2601` | `` |
| `REM-2602-OBSERVATION` | Internal transport-neutral envelope boundary evidence | `G2` | `S` | `Done` | `REM-2601-OBSERVATION` | `a64cadce09b158820ba5c8841d4695778e8a86af` |
| `REM-2603` | Delivery semantics | `G2` | `L` | `BlockedSpec` | `REM-2602` | `` |
| `REM-2603-OBSERVATION` | Internal remote-delivery boundary evidence | `G2` | `S` | `Done` | `REM-2602-OBSERVATION` | `cbfd4592fd499bd09722936caa597cccbd10f172` |
| `REM-2604` | Minimal reference transport | `G2` | `L` | `BlockedSpec` | `REM-2603` | `` |
| `REM-2604-OBSERVATION` | Internal reference-transport boundary evidence | `G2` | `S` | `Done` | `REM-2603-OBSERVATION` | `529e13a8e3e3017346cc0deb816c74a7a3036c30` |
| `REM-2605` | Security and resource limits | `G2` | `L` | `BlockedSpec` | `REM-2604` | `` |
| `REM-2605-OBSERVATION` | Internal security and resource boundary evidence | `G2` | `S` | `Done` | `REM-2604-OBSERVATION` | `84d3ef501c979778ee88e2f5ed19b4a955095044` |
| `REP-2501` | Determinism class | `G2` | `L` | `BlockedSpec` | `SUP-2403` | `` |
| `REP-2501-OBSERVATION` | Internal determinism-class evidence | `G2` | `S` | `Done` | `SUP-2403-OBSERVATION` | `0d3d11d9be50790c35ac014b33318d3966b10464` |
| `REP-2502` | Replay log schema | `G2` | `L` | `BlockedSpec` | `REP-2501` | `` |
| `REP-2502-OBSERVATION` | Internal replay-schema field evidence | `G2` | `S` | `Done` | `REP-2501-OBSERVATION` | `040ef2ca03438fe39df121acc07c2f554d24382f` |
| `REP-2503` | Effect recorder | `G2` | `L` | `BlockedSpec` | `EFF-2105`, `REP-2502` | `` |
| `REP-2503-OBSERVATION` | Internal effect-recorder boundary evidence | `G2` | `S` | `Done` | `REP-2502-OBSERVATION` | `d85ca61b119d9e7ee70ebad6ec9c4eb48be9accf` |
| `REP-2504` | Replay player | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503` | `` |
| `REP-2504-OBSERVATION` | Internal replay-player boundary evidence | `G2` | `S` | `Done` | `REP-2503-OBSERVATION` | `c095d8bd3ff81132a66ae3710c20fe251732748d` |
| `REP-2505` | Replay privacy, trimming, and corruption | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503`, `REP-2504` | `` |
| `REP-2505-OBSERVATION` | Internal replay privacy and integrity boundary evidence | `G2` | `S` | `Done` | `REP-2504-OBSERVATION` | `4d8c485ab390c616369736750680985688f831cb` |
| `REP-2506` | Cross-process replay acceptance | `G2` | `L` | `BlockedSpec` | `REP-2501`, `REP-2502`, `REP-2503`, `REP-2504`, `REP-2505` | `` |
| `REP-2506-OBSERVATION` | Internal cross-process replay acceptance boundary evidence | `G2` | `S` | `Done` | `REP-2505-OBSERVATION` | `b24983f1807307a48b0e2c2a063702a6a43fff72` |
| `SIMD-4301` | Vectorization Legality Analysis | `G4` | `M` | `BlockedSpec` | `CPU-4203` | `` |
| `SIMD-4301-OBSERVATION` | Internal SIMD legality boundary evidence | `G4` | `S` | `Done` | `CPU-4203-OBSERVATION` | `d4a04a70d8ab207d3bb245987abc31bbf439fe16` |
| `SIMD-4302` | Portable SIMD IR | `G4` | `M` | `BlockedSpec` | `SIMD-4301` | `` |
| `SIMD-4302-OBSERVATION` | Internal Portable SIMD IR boundary evidence | `G4` | `S` | `Done` | `SIMD-4301-OBSERVATION` | `917a64ad8bf7521f7455cade79b4fdb2056bd894` |
| `SIMD-4303` | SIMD Differential | `G4` | `M` | `BlockedSpec` | `SIMD-4302` | `` |
| `SIMD-4303-OBSERVATION` | Internal SIMD differential boundary evidence | `G4` | `S` | `Done` | `SIMD-4302-OBSERVATION` | `ae5694c4a134b9d7a8a0ec671835d28a21be8f81` |
| `STAB-6101` | Support-Matrix Item Audit | `G6` | `L` | `BlockedSpec` | `CBK-5903` | `` |
| `STAB-6101-OBSERVATION` | Internal Support-Matrix Item Audit boundary evidence | `G6` | `S` | `Done` | `CBK-5903-OBSERVATION` | `b6694aab307365446387551651545f2698846ad0` |
| `STAB-6102` | Remove False Entry Points | `G6` | `L` | `BlockedSpec` | `STAB-6101` | `` |
| `STAB-6102-OBSERVATION` | Internal False-Entry-Point Audit boundary evidence | `G6` | `S` | `Done` | `STAB-6101-OBSERVATION` | `5e3e93586768b10ad2e11805538a17c5cfcb0e7b` |
| `STAB-6103` | Feature State Metadata | `G6` | `L` | `BlockedSpec` | `STAB-6102` | `` |
| `STAB-6103-OBSERVATION` | Internal Feature-State Metadata boundary evidence | `G6` | `S` | `Done` | `STAB-6102-OBSERVATION` | `ff9d77c8c6deba2b5e524e473d3fc1360c6d3c44` |
| `STD-6301` | Stable Standard Library Audit | `G6` | `L` | `BlockedSpec` | `PROTO-6204` | `` |
| `STD-6301-OBSERVATION` | Internal Stable Standard Library Audit boundary evidence | `G6` | `S` | `Done` | `PROTO-6204-OBSERVATION` | `5c2f2ae0bbb34aad83ad5f7deb53cbc6d8cd5623` |
| `STD-6302` | Remove Convenience APIs | `G6` | `L` | `BlockedSpec` | `STD-6301` | `` |
| `STD-6302-OBSERVATION` | Internal Convenience API Removal Audit boundary evidence | `G6` | `S` | `Done` | `STD-6301-OBSERVATION` | `74eff3886fd9305f1c4bdc3c78025a7289200a81` |
| `STD-6303` | Unicode and Chinese-Programming Stability | `G6` | `L` | `BlockedSpec` | `STD-6302` | `` |
| `STD-6303-OBSERVATION` | Internal Unicode and Chinese-programming stability boundary evidence | `G6` | `S` | `Done` | `STD-6302-OBSERVATION` | `e4d1aa815a6f9da69dfe6036d73e7fa9a1a8ef47` |
| `SUP-2401` | Supervisor model | `G2` | `L` | `BlockedSpec` | `ACT-2305` | `` |
| `SUP-2401-OBSERVATION` | Internal Supervisor observation | `G2` | `S` | `Done` | `ACT-2306-PROPERTY-OBSERVATION` | `c29d95c7bc7b926c92757b2c32fccf960500e421` |
| `SUP-2402` | Restart budgets and circuit breakers | `G2` | `L` | `BlockedSpec` | `SUP-2401` | `` |
| `SUP-2402-OBSERVATION` | Internal restart-budget observation | `G2` | `S` | `Done` | `SUP-2401-OBSERVATION` | `7ee0cf0bf90971bb6844c8bf467f5506e5e5e796` |
| `SUP-2403` | Supervision tests | `G2` | `L` | `BlockedSpec` | `SUP-2402` | `` |
| `SUP-2403-OBSERVATION` | Internal supervision test evidence | `G2` | `S` | `Done` | `SUP-2402-OBSERVATION` | `73046c5049324455ec87d0011e62482de97c07aa` |
| `TASK-2201` | Structured Task syntax and Checked Core | `G2` | `M` | `Done` | `EFF-2103` | `54e4cecb6ad56685f93b155ef7395a1b0a7a7e26` |
| `TASK-2201-CORE-MODEL` | Internal Structured Task Checked-Core identity model | `G2` | `S` | `Done` | `TASK-2201-TASK-SYNTAX-REJECTION` | `6028e78ce4091e58d5df1289c59b8fd6b0f61c4e` |
| `TASK-2201-TASK-SYNTAX-REJECTION` | Internal Task-shaped syntax rejection gate | `G2` | `S` | `Done` | `GOV-0105` | `b3ede69c33d72d4419775650b820f1b5797dc652` |
| `TASK-2202` | Task state-machine lowering | `G2` | `L` | `Done` | `TASK-2201` | `450ec1bad6403a03a702713d80464fa6bbd83172` |
| `TASK-2202-STATE-MACHINE-MODEL` | Internal Task state-machine identity model | `G2` | `S` | `Done` | `TASK-2201-CORE-MODEL` | `80f3a8c7b579c65bd77030de159cfefdc42a5a4b` |
| `TASK-2203` | Structured Task lifecycle runtime | `G2` | `L` | `Done` | `TASK-2202` | `e8765790c421f3437049562d69c9aa6d487b5464` |
| `TASK-2203-LIFECYCLE-OBSERVATION` | Internal Task lifecycle observation trace | `G2` | `S` | `Done` | `TASK-2202-STATE-MACHINE-MODEL` | `e0a411bbef4620cc190de34939bbcc41d8768736` |
| `TASK-2204` | Deterministic Task test scheduler | `G2` | `L` | `Done` | `TASK-2203` | `e550f4fd8d5f348bcb8415f5cbac23b9f852a719` |
| `TASK-2204-SCHEDULER-OBSERVATION` | Internal Task scheduler observation trace | `G2` | `S` | `Done` | `TASK-2203-LIFECYCLE-OBSERVATION` | `078e106fff68a0f28e2f05fd8c8c6bda83637e61` |
| `TASK-2205` | Production local Task scheduler | `G2` | `L` | `Done` | `TASK-2204` | `330e1b2c18ffbb7e59a07297abf59e92d6954fa5` |
| `TASK-2206` | Task conformance and stress tests | `G2` | `L` | `Done` | `TASK-2205` | `d0c542d3b7caa8d8300ab2bb3021d35235773e91` |
| `TEST-VM-0001` | VM failing-first corpus and differential harness baseline | `G1` | `M` | `Done` | `GOV-0104`, `GOV-0105` | `5bd49583c9160cd2067a7124bc014ebc3b4bcf95` |
| `TIM-5701` | Timing IR and Path | `G5` | `L` | `BlockedSpec` | `MC-5604` | `` |
| `TIM-5701-OBSERVATION` | Internal Timing IR and Path boundary evidence | `G5` | `S` | `Done` | `MC-5604-OBSERVATION` | `d6c3d6315c04a3e85db518341b8002cac1076e14` |
| `TIM-5702` | Measurement and Static-Analysis Separation | `G5` | `M` | `BlockedSpec` | `TIM-5701` | `` |
| `TIM-5702-OBSERVATION` | Internal timing-analysis separation boundary evidence | `G5` | `S` | `Done` | `TIM-5701-OBSERVATION` | `002769f816455efca2932eecf5674f62a4cdf415` |
| `TIM-5703` | Deadline Check | `G5` | `M` | `BlockedSpec` | `TIM-5702` | `` |
| `TIM-5703-OBSERVATION` | Internal Deadline Check boundary evidence | `G5` | `S` | `Done` | `TIM-5702-OBSERVATION` | `ac1e7c226350c6694dd2b9997daa0d3cd149e2e4` |
| `TRAIT-1301` | Trait RFC closure | `G1` | `M` | `Done` | — | `ccab6ea91e05ed477457cc1ed870d76faaa46e3c` |
| `TRAIT-1302` | Trait AST/HIR representation | `G1` | `M` | `Done` | `TRAIT-1301` | `693b841000c98ca8aae119e3797a737fe0cebc7f` |
| `TRAIT-1303` | Trait constraint collection | `G1` | `M` | `Done` | `TRAIT-1302` | `1dfc52ee4439c43f284fbf384869436a408344d3` |
| `TRAIT-1304` | Trait coherence and orphan index | `G1` | `M` | `Done` | `TRAIT-1303` | `94a8daec579b1c730e51ff37bd3cde63dfd9d046` |
| `TRAIT-1305` | Trait solver v0 | `G1` | `M` | `Done` | `TRAIT-1304` | `530de657bfc63018090426f2e6e47eeeaf710f2c` |
| `TRAIT-1306` | Trait Checked Core dictionary witnesses | `G1` | `M` | `Done` | `TRAIT-1305` | `bfd00305473363c03286c2e0dbd060d7d136a95d` |
| `TRAIT-1307` | Trait interpreter and VM dictionary lowering | `G1` | `L` | `Done` | `TRAIT-1306` | `0dfb8d5c2d862143b9f91f0f88f2f47635a8cda5` |
| `TRAIT-1308` | Trait IDE support | `G1` | `L` | `BlockedSpec` | `TRAIT-1307` | `` |
| `TRAIT-1308-CURRENT-EVIDENCE` | Current Trait IDE boundary evidence | `G1` | `S` | `Done` | `TRAIT-1308-PROJECTION`, `TRAIT-1308-QUERY` | `bd1a88ced6dddb69e4708f0bc567cdee32963228` |
| `TRAIT-1308-PROJECTION` | Trait Semantic Graph projection | `G1` | `M` | `Done` | `TRAIT-1307` | `61ea00d8c19c6c5caa51461d4cffb8d2993e59d0` |
| `TRAIT-1308-QUERY` | Trait projection read-only lookups | `G1` | `S` | `Done` | `TRAIT-1308-PROJECTION` | `feb2be24fc78abc73010f283e830d3844f49b303` |
| `TRAIT-1309` | Trait solver performance and termination | `G1` | `M` | `BlockedSpec` | `TRAIT-1308` | `` |
| `TRAIT-1309-CURRENT-EVIDENCE` | Current Trait performance and termination evidence | `G1` | `S` | `Done` | `TRAIT-1309-TERMINATION` | `d4c24920d0dc719a517da2e804b4824dcd792633` |
| `TRAIT-1309-TERMINATION` | Bounded Trait solver termination evidence | `G1` | `S` | `Done` | `TRAIT-1307` | `6f84216c796253e51d4877edfbdc65ba7c0e5cad` |
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
| `ZED-6801` | Zed Compatibility Matrix | `G6` | `M` | `BlockedSpec` | `DOC-6703` | `` |
| `ZED-6801-CURRENT-EVIDENCE` | Current LSP, grammar, and package compatibility evidence | `G6` | `S` | `Done` | `ZED-6801-SEED` | `892bcb4287d7c2538495cafa6b34c78c28283a6b` |
| `ZED-6801-SEED` | Seed Zed compatibility-matrix drift gate | `G6` | `S` | `Done` | `DOC-6703-SEED` | `3ed4af01362d2a0e906acd40cff863237e355303` |
| `ZED-6802` | Language-server Discovery and Acquisition | `G6` | `L` | `BlockedSpec` | `ZED-6801` | `` |
| `ZED-6802-CURRENT-EVIDENCE` | Current Preview server and discovery boundary evidence | `G6` | `S` | `Done` | `ZED-6801-CURRENT-EVIDENCE`, `ZED-6802-SEED` | `f2ced28b6863c9694583883fe9daee114a3adab5` |
| `ZED-6802-SEED` | Seed language-server discovery inventory drift gate | `G6` | `S` | `Done` | `ZED-6801-SEED` | `c058d49001fe24b0d6f22c4850025414451c04d3` |
| `ZED-6803` | Full Zed Extension Acceptance | `G6` | `L` | `BlockedSpec` | `ZED-6802` | `` |
| `ZED-6803-CURRENT-EVIDENCE` | Current grammar, LSP, and position acceptance evidence | `G6` | `S` | `Done` | `ZED-6801-CURRENT-EVIDENCE`, `ZED-6802-CURRENT-EVIDENCE`, `ZED-6803-SEED` | `75ac1729356af7ac4af5b0d7b6304d7e6f3adf5b` |
| `ZED-6803-SEED` | Seed Zed extension acceptance inventory drift gate | `G6` | `S` | `Done` | `ZED-6802-SEED` | `75b6dd213dc1b07717f8cf2718dbe939176b76ae` |
| `ZED-6804` | DAP Status | `G6` | `M` | `BlockedSpec` | `ZED-6803` | `` |
| `ZED-6804-CURRENT-EVIDENCE` | Current DAP observation evidence | `G6` | `S` | `Done` | `DAP-3601-OBSERVATION`, `DAP-3602-OBSERVATION`, `DAP-3603-OBSERVATION`, `ZED-6803-CURRENT-EVIDENCE`, `ZED-6804-SEED` | `e6987c928437d5c5d80b92051b6df8f5c0fafbcf` |
| `ZED-6804-SEED` | Seed DAP status inventory drift gate | `G6` | `S` | `Done` | `ZED-6803-SEED` | `cf49269ecee39eeb8f3033a5520127f2837f4755` |
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
