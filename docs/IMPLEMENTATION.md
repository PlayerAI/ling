# 实现计划：v0.0.1 Seed（IMPLEMENTATION.md）

> 状态：执行计划草案 0.2  
> 日期：2026-08-18  
> 规范基线：`docs/RFC-0001.md`、`docs/SEMANTICS.md` §29、`docs/LANGUAGE.md` §19  
> 执行前置：相关里程碑开始前，必须关闭第 6 节列出的阻断级规范缺口，并在本文件中记录决议链接  
> 地位：本计划从属于 RFC-0001。冲突时以 Accepted RFC 为准；本文件只安排工程秩序，不创造语言语义。

---

## 1. 目标与边界

交付 `v0.0.1 Seed`：一个由 Rust 实现的解释执行内核，提供 `run / check / repl / semantic / audit` 五个子命令、中文 Unicode 标识符、局部类型推导、record/ADT/穷尽 match、`<-` 赋值、`Pure` 与 `Console.Write` 的 Effect/Capability 检查、确定性 Semantic Graph 输出，以及 human/JSON 双格式诊断。

“确定性”仅指在固定语言版本、Unicode 版本、输入和编译参数下输出可复现；v0.0.1 的 Semantic ID 仍是 `experimental`，不承诺跨语言版本稳定。

非目标完全沿用 RFC-0001 §5 与 SEMANTICS §29：不做 GC、JIT、Native Backend、Resource/Borrow Checker、Trait、Effect Handler、Task/Actor/Node/Kernel、GPU、证明器、包管理器。**为通过验收而伪造上述能力视为失败。**

RFC-0001 §6.6 的“受限自动 Borrow”与 SEMANTICS §29 的“v0.0.1 不实现 Resource/Borrow”存在规范冲突。在正式勘误或新 RFC 接受前，本计划不得实现自动 Borrow，也不得把调用方可见的参数别名语义作为既定事实。该问题列为 G-08 阻断项。

---

## 2. 交付物

1. `ling` CLI：单一二进制，由 `crates/ling-cli` 产出；
2. RFC §9 所列职责边界对应的 Rust workspace。crate 数量是建议架构，不是验收指标；只有出现清晰依赖边界时才建立 crate，禁止为了凑数量创建无行为的公共 API；
3. `ling.diagnostic/0.1` 与 `ling.semantic/0.1` 的版本化 Schema、示例和兼容性测试；
4. `tests/` 下的 conformance、diagnostics、unicode、snapshots 测试数据，以及一个被 `cargo test --workspace` 实际调用的 runner；
5. `examples/` 下至少包含人物、ADT/match、pipeline 三类可运行示例；其中 pipeline 示例须等待 G-02 关闭；
6. 错误码注册表，作为稳定 code 分配的唯一来源；
7. Windows、Linux、macOS 三平台 CI 构建验证。发布产物的目标三元组、签名和打包格式不在 RFC 验收范围内，须另行决策后才能宣称交付。

---

## 3. 仓库与技术决策

### 3.1 仓库布局

实现与规范同仓，workspace 根即仓库根：

```text
.
├── Cargo.toml            # workspace 定义、统一 lints
├── crates/
│   ├── ling-source/      # SourceId、UTF-8、Span、行列映射
│   ├── ling-unicode/     # XID、NFC、Script Set、UTS #39 skeleton
│   ├── ling-syntax/      # Token、Lexer、offside layout、Parser、CST
│   ├── ling-ast/         # AST（保 Span、去语法噪音）
│   ├── ling-hir/         # 名称空间、糖 Lowering、Place 分类
│   ├── ling-resolve/     # 作用域、名称解析、confusable 碰撞
│   ├── ling-types/       # Type Arena、统一、泛化、穷尽性、Value Restriction
│   ├── ling-effects/     # Effect Row、Capability 需求与检查
│   ├── ling-semantic/    # Semantic Graph、Semantic ID、序列化、audit 投影
│   ├── ling-eval/        # Core Value、环境、闭包、Place Cell、解释器
│   ├── ling-diagnostics/ # 稳定 code、human/JSON 渲染、Facts/Repair
│   ├── ling-format/      # 确定性格式化（v0.0.1 仅内部使用）
│   └── ling-cli/         # 命令解析、REPL、编译编排、退出码
├── tests/
│   ├── conformance/      # 正/负语义用例（.ling + 期望）
│   ├── diagnostics/      # 诊断快照
│   ├── unicode/          # Unicode 安全用例
│   └── snapshots/        # semantic graph / audit 快照
├── tools/
│   └── unicode-gen/      # 维护者显式运行的 Unicode 表生成器
├── examples/
└── docs/                 # 规范、RFC、错误码注册表与本计划
```

上图是目标职责分解，不要求 M0 一次性填充所有 crate。`tests/` 是测试数据目录；runner 放在 `crates/ling-cli/tests/conformance.rs`（或经 M0 ADR 选定的等价入口），保证根目录 fixture 确实进入 `cargo test --workspace`。

依赖必须形成有向无环图。以下记号中 `A → B` 表示“A 依赖 B”，不是编译 Pipeline 的数据流：

```text
ling-source
ling-unicode
ling-diagnostics → ling-source
ling-syntax      → ling-source, ling-unicode, ling-diagnostics
ling-ast         → ling-source, ling-syntax
ling-hir         → ling-source, ling-ast
ling-resolve     → ling-hir, ling-unicode, ling-diagnostics
ling-types       → ling-hir, ling-resolve, ling-diagnostics
ling-effects     → ling-hir, ling-types, ling-diagnostics
ling-semantic    → ling-source, ling-hir, ling-resolve, ling-types, ling-effects
ling-eval        → ling-hir, ling-types, ling-effects, ling-diagnostics
ling-format      → ling-source, ling-syntax, ling-semantic
ling-cli         → ling-source, ling-syntax, ling-resolve, ling-types,
                   ling-effects, ling-semantic, ling-eval, ling-format,
                   ling-diagnostics
```

这是允许的层级边界，不是必须逐边照抄的 Cargo 清单。Cargo 自身会拒绝 crate 循环；CI 另外通过 `cargo metadata` 检查禁止的反向依赖，例如基础层依赖 CLI 或 evaluator。

### 3.2 第三方依赖候选

所有第三方依赖必须记录版本、许可证、MSRV、是否含 `unsafe`、维护状态、直接及关键传递依赖。应用型 workspace 提交 `Cargo.lock`；依赖升级必须经过 CI 和 Unicode/Schema 回归测试。表中条目均为候选，不代表已经接受。

| 用途 | 候选 | 备注 |
| --- | --- | --- |
| XID_Start/Continue | `unicode-ident` 或自建表 | 必须用测试确认数据版本为 Unicode 17.0.0；不能只依据 crate 版本号推断 |
| NFC | `unicode-normalization` 或同版本自建表 | 必须通过 Unicode 17.0.0 `NormalizationTest.txt`；不得与 XID 使用不同 Unicode 数据版本 |
| Script Set / UTS #39 | 固定官方数据并生成静态表 | 至少需要 `Scripts.txt`、`ScriptExtensions.txt`、相关 UCD 属性文件，以及 UTS #39 的 `confusables.txt`、`IdentifierStatus.txt`、`IdentifierType.txt` |
| 数学整数 `Int` | `num-bigint` 或等价实现 | `Int` 是数学整数；不得以 `i64`/`i128` 冒充，需评审许可证、性能与序列化边界 |
| Semantic ID | `blake3` | RFC §6.12 允许 BLAKE3/SHA-256；**本计划选定 BLAKE3**，ID 带 `experimental:blake3:` 前缀 |
| JSON | `serde` + `serde_json` | 字段顺序不作为语义（§12） |
| CLI 参数 | `clap`（derive）或手写 | 子命令少，允许手写以减少依赖；决策点在 M0 关闭 |
| REPL 行编辑 | `rustyline` 或纯标准输入 | 仅 `ling-cli` 可依赖；脚本化 REPL 测试不得依赖终端能力 |
| 性质测试 | `proptest` | §14.4 |
| Fuzz | `cargo-fuzz`（libFuzzer）+ `arbitrary` | §14.3，M7 接入 |

Unicode 17.0.0 的权威输入来自 Unicode Consortium 的版本化目录：

- UCD：<https://www.unicode.org/Public/17.0.0/ucd/>
- UTS #39 数据：<https://www.unicode.org/Public/17.0.0/security/>

维护者工具下载或读取这些固定版本数据，校验仓库记录的 SHA-256 与许可证后生成 Rust 静态表；生成结果和生成器测试提交仓库。普通 `cargo build` 不访问网络，也不在 `build.rs` 中重新解释外部 Unicode 数据，从而保证离线、可复现构建。

### 3.3 命名集中定义

- `ling-cli` 内定义唯一常量 `CLI_NAME = "ling"`，所有帮助文本、诊断、错误消息引用该常量，禁止散落硬编码（RFC §2）；
- Cargo package 名即 crate 名（`ling-source` 等），`[[bin]] name = "ling"`；
- Schema 字符串按 RFC §12/§13 固定为 `ling.diagnostic/0.1`、`ling.semantic/0.1`，集中在 `ling-diagnostics` / `ling-semantic` 各一个常量中；
- `LANGUAGE_VERSION`、`UNICODE_VERSION`、Schema ID 与哈希算法使用有类型封装，禁止在业务逻辑中散落字符串；
- 错误码前缀 `L-`，domain 在错误码注册表分配（见 M0）；
- 改变公开 Schema、错误码含义或 ID 编码必须更新版本、兼容性测试和迁移说明，不能只更新快照。

---

## 4. 里程碑计划

里程碑表示依赖关系，不要求所有工作机械串行。下游技术尖峰可以并行，但只有在上游接口评审通过、相关阻断项关闭、且下游不宣称完成上游语义时才能开始。每个出口标准必须由 CI 或可归档的人工审查记录验证；未满足出口标准不得标记该里程碑完成。

### M0：仓库骨架（对应 RFC §17 Milestone 0）

任务：

- 建立 workspace manifest 和 RFC §9 的目标职责边界；crate 可以只有最小私有骨架，但不得暴露伪造能力或占位语义；
- 固定 Rust toolchain/MSRV 策略，并在 workspace manifest 中集中 edition、`rust-version` 和 lints；`unsafe_code` 默认 deny，任何例外必须有局部范围、理由和测试；
- CI 执行 `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets --all-features -- -D warnings`、`cargo test --workspace --all-features`，并在 Windows/Linux/macOS 上至少完成构建（§18.7）；
- 先关闭 G-01，再建立 `docs/ERROR-CODES.md`。每个已分配 code 包含稳定含义、zh/en 消息模板、Facts Schema、Repair 候选和首次引入版本；不为未来特性预分配具体编号；
- `AGENTS.md` 写入完整路径下的权威顺序、RFC §16.2 禁止事项和 §16.4 PR 必需信息；
- 建立 conformance runner：单个 `.ling` 文件 + `expect.toml`，可声明 exit code、stdout、诊断 code、诊断/graph/audit 快照；runner 必须由 `cargo test --workspace` 发现；
- 用短 ADR 关闭 CLI 解析方案、REPL 行编辑方案、测试 runner 位置和依赖准入流程。

出口标准：三平台 CI 全绿；`ling --version` 输出合法 SemVer 预发布版本 `0.0.1-dev`；最小 pipeline 能读取文件，并通过统一诊断层返回明确的“功能尚未实现”错误；至少一个 fixture 可证明 conformance runner 已被执行，而不是仅存在于目录中。

### M1：Source + Unicode（ling-source、ling-unicode）

任务：

- UTF-8 解码；非法 UTF-8 是编译错误。BOM 仅允许在文件头，词法视图忽略它，原始 Source 记录其存在，供 audit/formatter 使用（§3.1）；
- LF/CRLF/CR 在词法视图中规范化，但 Source Span 仍以原始 UTF-8 字节的半开区间表示。若规范化改变偏移，SourceMap 必须保存原始字节与词法视图之间的映射；
- 在 G-13 关闭后定义 line/column 的计量单位。line/column 从 byte span 派生，不作为身份或唯一位置表示；
- 标识符管线（§3.4）：XID 校验 → 禁止字符拒绝 → NFC → Script Set → UTS #39 skeleton。skeleton 只用于安全诊断，不参与名称相等、名称归一化或 Semantic ID。M1 只产出名称元数据；“同一作用域碰撞”由拥有作用域信息的 M3 Resolver 判定；
- Unicode 17.0.0 数据生成器、固定输入校验和、生成表及 conformance tests 入库；编译器版本信息与 ProgramSnapshot 记录 Unicode 版本（§3.3）。

出口标准：UTF-8/BOM/三种换行与 SourceMap 测试通过；Unicode 版本一致性测试通过；NFC、XID、禁止属性、Script Set、UTS #39 skeleton 的单元与官方数据回归通过。RFC §14.2 中依赖作用域或多文件解析的用例在 M3 完成，并在 M7 做全链路复验。

### M2：Parser（ling-syntax、ling-ast）

前置条件：关闭 G-02（`|>`）、G-03（record 分隔符）、G-04（字面量）和 G-05（offside 子集）。Parser 不得自行补齐这些语法。

任务：

- Lexer：关键字（§6.3）、标识符（走 M1 管线）、已决议的数字/Text 字面量、行注释/可嵌套块注释/文档注释（§3.8）、`<-`，以及 G-02 接受后才加入的 `|>` token；
- offside layout 遵循相对列规则：子表达式必须比块起始列更深；Tab 不得参与语义缩进；formatter 输出四个空格，但 Parser 不要求缩进必须是四的倍数；括号、方括号、花括号内部换行不结束块；
- Parser 覆盖已接受的 v0.0.1 grammar；CST 保留源码信息并支持有限 error recovery，恢复点和最大递归/嵌套策略必须有测试；
- AST 去除纯语法噪音但保留 Source Span；`=`/`<-` 在词法层区分，不延后消歧（§8.1）。

出口标准：所有已解锁示例可解析；Source/Unicode fuzz target 接受任意字节，Lexer/Parser fuzz target 接受已解码输入且在 CI 预算内无 panic、越界或非预期超时；CST/AST 快照入库；非法输入产生有界、可定位的诊断。pipeline 示例只有在 G-02 关闭后才进入必过集合。

### M3：Resolver + Type（ling-hir、ling-resolve、ling-types）

前置条件：关闭 G-06（模块/多文件）、G-07（Value Restriction）和 G-08（Borrow 冲突）。

任务：

- 最近词法作用域解析、局部阴影规则（§4.2）；在解析后的同一作用域内，对不同规范化名称的相同 UTS #39 skeleton 报错；
- HIR：已接受语法糖的 Lowering、Place 分类（§9.2）、pattern matrix 输入；
- HM 推导：Unit/Bool/Int/f64/Text、tuple、函数、nominal record、nominal ADT、Option/Result、类型变量；let 多态和已决议的简化 Value Restriction；
- `Int` 字面量和常量语义使用任意精度表示，Parser/Type Checker 不得先截断为 Rust 固定宽度整数；`Char` 不属于 SEMANTICS §29 的 Seed 子集，Parser 不接受 Char literal，若遇到未来保留形式则返回明确的未实现诊断；
- 穷尽性检查覆盖 v0.0.1 的 nominal variant 与 Bool；不可达分支给出警告。不得暗示已支持无限整数区间、守卫完备性或未来 pattern 类型；
- 可变性检查：只有已决议规则下的 `let mutable` 与 `mutable` 字段可成为 `<-` 左侧，提供 `L-MUT-0001` 结构化诊断；
- G-08 决议若维持 SEMANTICS §29 的 Seed 边界，则禁止推导 `&mut`、生成 Borrow Edge 或让参数修改隐式传播到调用方；只有 Accepted 决议明确纳入并限定受限 Borrow 时，才能增加相应实现和验收用例。

出口标准：§18.4 全部通过；名称从 Lexer 到 Resolver/Type Checker 不损坏；作用域内 confusable 与多文件解析测试通过；HM 性质测试在“代换结果按类型变量重命名等价”的意义下验证，不把内部 union-find 顺序当作语义。§18.3 中依赖 Graph、REPL、JSON 的全链路部分分别在 M5/M6 收口。

### M4：Effect + Capability（ling-effects）

前置条件：关闭 G-09（`State<T>` 与 Capability 模型）和 G-10（Seed 内置项签名）。

任务：

- Effect Row 使用无序、去重的规范化表示；Seed 对外只实现空 Row（`Pure`）和 `Console.Write`。`Pure` 是空 Row，不是普通标签；
- 函数类型保存 Effect Row，组合时取已决议的并集；v0.0.1 不实现用户 Effect Handler，也不把高阶 Effect Row 多态作为验收要求；
- 按 G-09 决议表示局部赋值所需的 `State<T>`，明确它是否只存在于 Typed Core/Audit，避免“普通输出隐藏、语义输出又缺失”的双重标准；
- Capability 检查 `module Main requires Console.Write`；缺少声明是编译错误。最小 Capability 闭包和 unused 提示必须基于同一套已解析调用图；
- Graph/Audit 使用同一个规范化 Effect/Capability 数据结构，不允许渲染层重新推断。

出口标准：§18.5 全过；Effect 顺序变化不影响语义或哈希输入；未声明 Capability 的路径在进入 evaluator 前被拒绝；`State<T>` 的 human、JSON、Graph、Audit 可见性符合 G-09 决议。

### M5：Semantic Graph（ling-semantic）

前置条件：关闭 G-11（身份/哈希模型）和 G-12（Audit grammar/round-trip）。

任务：

- 实现 `ling.semantic/0.1`：节点种类按 RFC §6.11，引用边保存已解析 ID；所有 ID、Hash 和 SourceId 使用不同 Rust newtype，禁止裸字符串互换；
- 实现 alpha 归一化及 G-11 接受的 `BodyId / ContractId / DefinitionId` 模型；互递归 CycleId 若未被 Seed 用例需要，只保留 Schema 可扩展点，不伪造可用实现；
- BLAKE3 输入使用带 domain separator、语言版本、Schema 版本的规范化字节编码。不得直接哈希非规范 JSON、HashMap 迭代顺序、Source Span、文件路径或 Rust `Debug` 输出；
- JSON writer 输出确定性顺序，reader 按 Schema 规则容忍未知扩展字段；“字段顺序非语义”和“writer 输出稳定”同时成立；
- 由 `ling-semantic` 提供规范化 Audit model，由 `ling-format` 提供 G-12 接受的 grammar、renderer 和 parser；验证 `parse_audit(render_audit(graph)) = graph`（忽略已明确定义的显示元数据），避免在两个 crate 中各自定义语义。

出口标准：§18.6 全过；Graph Schema 正/负兼容性测试通过；同一输入在两个独立进程中产生逐字节相同的 Graph JSON 与 Audit 文本；Audit round-trip 性质测试通过；依赖实现变化敏感性有最小调用图用例，避免无意的全图或不充分失效。

### M6：Interpreter + REPL（ling-eval、ling-cli）

前置条件：关闭 G-10（内置项）、G-14（REPL）和 G-15（入口与运行错误）。

任务：

- Core Value / 环境 / 闭包 / Record / Variant / Place Cell；`Int` 使用任意精度值，`f64` 遵循 SEMANTICS §13.4。内部可用 `Arc`、arena、`HashMap`，但不得泄漏为语言语义（RFC §20.6）；
- **禁止在未解析 AST 上解释**（RFC §10）：Interpreter 只接受 Type、Effect、Capability 和 Place 检查完成后的 Typed Core；
- 严格 call-by-value、左到右求值（RFC §6.7、SEMANTICS §8.2）；
- `Console.write` 通过注入的 Console Capability 接口执行，测试使用内存实现。编译期权限缺失必须在 evaluator 前拒绝；宿主能力失败按 G-09/G-15 决议映射，不得随意变成 panic；
- 子命令：`run`（check → snapshot → interpret main）、`check`、`repl`、`semantic`、`audit`；统一输出协议与 RFC §15 退出码（0/1/2/3/4/5/6）；
- REPL 复用同一 Parser/Resolver/Type Checker/Interpreter。跨行状态、重定义、多行输入和 Capability 环境按 G-14 决议实现；脚本模式不依赖 TTY。

出口标准：§18.1、§18.2 通过；数学整数大于 `i128` 的计算用例通过；严格求值顺序和 Console 注入测试通过；REPL 脚本化会话与文件执行共享相同 Typed Core 路径；运行期错误和 Fault 使用已决议的退出码及结构化输出。

### M7：硬化与发布

任务：

- cargo-fuzz 覆盖 Source decoder、Lexer、Parser；PR CI 使用明确记录的短预算，nightly/定期任务使用长预算并保存 corpus 与崩溃样本；
- proptest 覆盖 RFC §14.4，并明确每个性质的等价关系，避免把内部编号、迭代顺序或诊断措辞当作语义；
- 建立规范条款到 conformance case 的追踪矩阵；每条 v0.0.1 规范性规则至少有正例、反例，并按适用性包含诊断、Graph、Audit 快照；
- Unicode 17.0.0 安全回归、示例库、Schema 兼容性和独立进程确定性测试完善；
- Windows/Linux/macOS release profile 构建通过；若生成可下载产物，另行记录目标三元组、压缩格式、校验和与签名策略。

出口标准：第 6 节阻断项全部关闭，RFC §18 与本文件第 10 节发布门禁全部通过，方可打 `v0.0.1` 标签。

---

## 5. 测试与 CI 落地

- **条款追踪**：维护“规范文件 + 章节 + 规范句 → conformance case”矩阵。快照数量不能代替条款覆盖；无测试的规范句必须有明确豁免和理由；
- **conformance runner**：每个用例一个目录，含 `case.ling` 与 `expect.toml`，可声明 exit code、stdout、诊断 code、快照和所覆盖的规范条款；runner 比较实际输出。bless 只能由显式本地命令触发，CI 禁止自动更新期望；
- **快照边界**：诊断 JSON、Audit 文本、Graph JSON 使用确定性 writer。只快照公开协议；内部 arena index、HashMap 顺序、临时 ID 和调试字符串不得进入期望；
- **Unicode 测试**：除项目用例外，运行固定 Unicode 17.0.0 数据的 conformance/回归测试，并断言所有 Unicode 子系统报告同一版本；
- **fuzz**：Source decoder target 接受任意字节；Lexer/Parser target 接受合法 UTF-8 或结构化 token 输入。PR 与定期任务的时间预算写入 CI 配置，文档不写易失真的固定分钟数；
- **性质测试**：unification 在主统一子存在性及变量重命名等价意义下检查对称性；覆盖 substitution、generalization/instantiation、occurs check、alpha-renaming。Semantic ID 的 alpha 属性在 M5 对 Typed Core 测试，不混入 M3 的纯类型测试；
- **确定性测试**：同一 fixture 在两个独立进程中运行，比较 Graph/Audit/diagnostic bytes；测试固定语言版本、Unicode 版本和编译参数，不依赖时区、locale、当前目录或随机 HashMap seed；
- **CI 门禁**：使用 `--locked` 执行 fmt、clippy、test、doc build、层级依赖检查和快照确定性检查；普通构建与测试不得访问网络。

---

## 6. 规范缺口清单

以下问题尚无足够规范。对应“最迟关闭”里程碑开始前，必须按 RFC §16.1 形成 Accepted RFC、规范勘误或明确批准的协议决议，并在“决议”栏补链接；不得由实现代码或测试快照反向决定语义。

| ID | 最迟关闭 | 问题 | 必须得到的决议 | 决议 |
| --- | --- | --- | --- | --- |
| G-01 | M0 | 错误码 domain 与编号分配 | domain 名称、编号所有权、废弃规则、消息与 code 的稳定边界 | [DEC-0001](decisions/0001-error-code-policy.md) |
| G-02 | M2 | `|>` 缺 EBNF | 优先级、结合性、与函数应用/换行的关系、Lowering | [DEC-0004](decisions/0004-pipeline-syntax.md) |
| G-03 | M2 | record 字段分隔符不一致 | 换行、`;`、尾分隔符及 REPL 单行形式 | [DEC-0005](decisions/0005-seed-literals-and-delimiters.md) |
| G-04 | M2 | Text/Int/f64 字面量不完整 | 转义、进制、分隔符、指数、溢出、NaN/Infinity 的源码与 JSON 表示；`Char` 明确不进入 Seed | [DEC-0005](decisions/0005-seed-literals-and-delimiters.md) |
| G-05 | M2 | offside 子集不精确 | 列计算、空行/注释、续行、错误恢复、嵌套容器规则 | [DEC-0006](decisions/0006-offside-layout.md) |
| G-06 | M3 | module/import 与多文件边界 | 文件到模块映射、import 解析、循环、重复模块、入口模块发现 | [DEC-0007](decisions/0007-module-and-file-boundaries.md) |
| G-07 | M3 | 简化 Value Restriction | 可泛化表达式集合、mutable/Effect 交互、诊断 | [DEC-0008](decisions/0008-seed-value-restriction.md) |
| G-08 | M3 | Borrow 范围冲突 | RFC §6.6 与 SEMANTICS §29 的优先解释；参数字段赋值是否影响调用方；若后置，示例如何改写 | [DEC-0009](decisions/0009-seed-borrow-and-mutation-boundary.md) |
| G-09 | M4 | `State<T>` 与 Capability 本体 | `State<T>` 可见性；Capability 是静态要求、运行时值或二者；宿主能力失败如何分类 | [DEC-0010](decisions/0010-state-and-capability-model.md) |
| G-10 | M4 | Seed 内置项 | `max`、`min`、`Console.write`、格式化及示例所需集合的规范名称、类型、Effect/Capability | [DEC-0011](decisions/0011-seed-builtins.md) |
| G-11 | M5 | Semantic ID 身份模型 | 逻辑身份与内容哈希是否分离、依赖变化传播、互递归、canonical bytes 与版本迁移 | [DEC-0012](decisions/0012-semantic-identity-and-canonical-bytes.md) |
| G-12 | M5 | Audit Source grammar | 可解析语法、显示元数据边界、唯一性及 round-trip 等价关系 | 待定 |
| G-13 | M1 | Source 位置单位 | byte span 的半开区间、line 基数、column 使用 byte/scalar/UTF-16/grapheme 中哪一种、换行规范化映射 | [DEC-0002](decisions/0002-source-position-units.md) |
| G-14 | M6 | REPL 会话语义 | 多行输入、重定义、阴影、失败事务、会话 Capability、脚本模式输出 | 待定 |
| G-15 | M6 | `main` 与运行错误 | 允许的入口签名、返回类型、Effect 上限、Result/Fault 到退出码和 JSON 的映射 | [DEC-0013](decisions/0013-main-and-runtime-failures.md) |

---

## 7. 并行工作流

RFC §16.3 的 Agent 列表表示工作流，不是固定人数或日历承诺。实际依赖关系为：

```text
Source/Unicode ──▶ Parser/CST/AST ──▶ Resolver/HIR ──▶ Type
                                                            │
                                                            ▼
                                                   Effect/Capability
                                                            │
                                                            ▼
                                                    Checked Typed Core
                                                       │           │
                                                       ▼           ▼
                                           Semantic Graph/Audit  Interpreter/REPL

Diagnostics、Conformance、Fuzz、CI 横跨全部阶段
```

每个并行工作项在开始前必须记录：规范引用、未关闭缺口、负责路径、基础 commit、输入/输出接口、测试义务和预期合并顺序。共享接口先以最小版本合并评审；“冻结”表示变更必须协调和迁移，不表示 M0 永久决定所有 AST/HIR/Graph 细节。

每个里程碑结束做规范符合性自查：实现行为与规范条款追踪矩阵逐项对照。存在差异时，只能修改实现以符合规范，或停止相关实现并提交 RFC/勘误；不得用新增快照把差异合法化。

---

## 8. 工作量与节奏

RFC §19 给出的估计是未校准的早期范围，本计划引用它但不提高其可信度：

```text
总量：80–160 个有效 Agent-hours
原 RFC 日历估计：技术尖峰 1 天；可运行内核 2–4 天；达到验收 5–10 天
```

Agent-hours 不能直接换算为日历时间；原估计依赖规范稳定、并行资源充足且接口返工有限。G-01 至 G-15 尚未关闭时，不应把该范围作为交付承诺。

M1 和 M2 各完成后，根据实际吞吐、缺陷率、测试规模和接口返工重新估算剩余工作，并保留原始估计与偏差原因。范围包含实现、测试、诊断、文档、Unicode 安全和集成返工；不包含品牌/商标处理、发布基础设施及 v0.0.1 之外能力。

---

## 9. 风险与缓解

| 风险 | 缓解 |
| --- | --- |
| 规范缺口或内部冲突 | G-01 至 G-15 作为显式阻断项；无决议不实现相关语义 |
| 范围膨胀（RFC §20.2） | 非目标进入 PR 模板；超出当前出口标准的功能不合并，不用占位实现伪装支持 |
| Unicode 子系统版本不一致 | 固定 17.0.0 官方数据、校验和、生成器和 conformance tests；所有子系统在运行时报告同一版本 |
| 换行规范化破坏 Source Span | 原始字节为位置真值，词法视图维护映射；跨 LF/CRLF/CR 做属性测试 |
| 数学 `Int` 被固定宽度实现泄漏 | 使用任意精度值并加入超出 `i128` 的 conformance 用例 |
| Semantic Hash 过早冻结或级联失效异常 | ID 标记 `experimental`；关闭 G-11；对最小依赖图测试“应变化/不应变化” |
| Audit round-trip 无可实现语法 | G-12 在 M5 前阻断；renderer/parser 同时交付并做性质测试 |
| 解释器泄漏 Rust 行为（RFC §20.6） | evaluator 只接收 Typed Core；conformance 不观察 `Arc`、arena、HashMap 或 Rust panic |
| 快照过度约束实现细节 | 只快照公开协议；性质测试负责语义不变量；更新快照必须说明语义影响 |
| 多工作流规范漂移（RFC §20.7） | 共享接口评审、路径所有权、规范追踪矩阵和按依赖顺序合并 |
| 依赖供应链与许可证风险 | 提交 `Cargo.lock`、依赖审查记录和许可证清单；普通构建离线；升级单独评审 |
| 工期估计过度乐观 | M1/M2 后重新估算；以出口证据而非日历日期宣称进度 |
| 别名系统晚于首批代码定型 | 从第一个内置项开始记录规范名称与命名理由，供后续 Alias RFC 使用 |

---

## 10. 验收清单（映射 RFC §18）

打 `v0.0.1` 标签前，以下 RFC §18 条目必须在 CI 与本地同时验证。该清单是必要条件，不替代第 6 节缺口关闭或规范条款追踪矩阵：

- [ ] `ling run examples/人物.ling` 成功输出预期结果（§18.1）
- [ ] `ling check examples/人物.ling` 返回 0；错误程序返回稳定 code 与 JSON（§18.2）
- [ ] `人物 / 血量 / 最大血量 / 受到伤害 / 生存状态` 全链路不损坏（§18.3）
- [ ] 普通函数可推导；错误类型被拒；record 字段检查；非穷尽 match 被拒；不可变赋值被拒（§18.4）
- [ ] Pure 显示空 Effect；Console 调用显示 `Console.Write`；未声明 Capability 被拒（§18.5）
- [ ] 每定义有实验性 ID；引用边已解析；空白不变 hash；参数改名不变 BodyId；行为改变变 BodyId；Graph JSON 过 Schema 测试（§18.6）
- [ ] Parser fuzz 规定时间无 panic；全部 conformance 通过；REPL 与文件执行同一 Core；Windows/Linux/macOS 构建通过（§18.7）

附加发布门禁：

- [ ] G-01 至 G-15 均有已批准的 RFC、勘误或协议决议链接，且实现与决议一致；
- [ ] `Cargo.lock` 已提交，`cargo build/test --workspace --all-features --locked` 在三平台通过；
- [ ] Unicode 数据版本、输入校验和和许可证记录可追溯，普通构建不访问网络；
- [ ] Diagnostic/Graph/Audit Schema 正负兼容性、独立进程确定性和 Audit round-trip 测试通过；
- [ ] 未实现特性产生明确错误，不存在静默占位路径；
- [ ] 发布 commit 工作区干净，标签指向已通过上述门禁的唯一 commit。

---

## 附：启动顺序

1. 合并本计划时记录所依据规范的 commit SHA，避免“同名章节、不同版本”；
2. 先关闭 G-01，再完成 M0；关闭 G-13 后完成 M1；在 M2 开始前关闭 G-02 至 G-05；
3. M3 前关闭 G-06 至 G-08，M4 前关闭 G-09/G-10，M5 前关闭 G-11/G-12，M6 前关闭 G-14/G-15；
4. 按第 4 节推进。每个里程碑通过出口标准后，在里程碑记录中保存 commit SHA、CI 链接、条款覆盖报告和遗留风险；是否创建内部 Git tag 由独立发布策略决定。
