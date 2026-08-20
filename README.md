# 零语言（Ling）

> 一门规范优先、面向 AI、支持中文标识符，并以语义图、显式 Effect 与 Capability 为核心的编程语言。
>
> A specification-first, AI-native programming language centered on semantic graphs, explicit effects and capabilities, and first-class Chinese identifiers.

[中文说明](#中文) · [English](#english)

## 中文

### 项目状态

Ling 的 [`v0.0.1 Seed`](https://github.com/PlayerAI/ling/tree/v0.0.1) annotated tag 已发布。其目标提交通过本地门禁，以及同一 SHA 的 Windows、Linux、macOS、真实 Unix TTY、nightly fuzz 和 Rust 1.85 CI；详细证据见 `docs/SEED-RELEASE-REPORT.md`。这仍是实验性实现，不是生产编译器或运行时。

| 项目 | 当前状态 |
| --- | --- |
| 语言名称 | 中文名：零；英文名：Ling |
| CLI | `ling` |
| 源文件扩展名 | `.ling` |
| 当前里程碑 | P8～P11 已在本地实现；正在完成 P12 发布证据与候选 commit 门禁 |
| 实现状态 | Unicode 17、Seed 前端、module/import、generic nominal types、注入式 Prelude、pattern exhaustiveness、Effect/Capability、Semantic/Audit、解释器、事务式 REPL、共享 CLI compiler 与 conformance runner 已落地 |
| 稳定性 | 设计可能变化，接受的 RFC 才能冻结语义 |

### 构建与验证

需要 Rust 1.97.1；项目声明的最低 Rust 版本为 1.85。先按已提交的 `Cargo.lock` 获取依赖，之后的常规构建与测试显式离线：

```bash
cargo fetch --locked
cargo test --workspace --all-features --locked --offline
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo xtask governance check-all
cargo xtask ci verify
cargo xtask traceability verify --release v0.0.1
cargo xtask schema validate-all
cargo xtask schema compatibility --from N-1 --to N
cargo xtask schema corrupt-inputs
cargo xtask seed reproduce
cargo xtask support verify
cargo xtask status verify
cargo run --locked --offline -- --version
```

`governance check-all` 聚合五项治理 registry 检查；`ci verify` 防止八个 G0 jobs、三平台测试、fuzz 和 MSRV 门禁漂移；`seed reproduce` 构建一次并用八个独立进程比较 check/run/Semantic/Audit 输出。

公开 JSON 协议的版本、reader/writer 范围、字段策略和 golden corpus 由 [`SCHEMA-LIFECYCLE.md`](docs/governance/SCHEMA-LIFECYCLE.md)、[`schemas/registry.toml`](schemas/registry.toml) 与 [`schemas/`](schemas/README.md) 共同记录。当前三个协议都是首版，因此兼容门禁明确报告 `NoPreviousVersion`，不宣称已经支持 N-1。

`check`、`run`、`semantic`、`audit` 和 `repl` 复用同一条真实编译路径；`run` 和 REPL 只解释完成名称、类型、Place、Effect 和 Capability 检查的 ProgramSnapshot。`semantic` 输出确定性的 `ling.semantic/0.1` JSON，`audit` 输出可 round-trip 的 `ling.audit/0.1` 文本：

```bash
cargo run --locked --offline -- check examples/hello.ling
cargo run --locked --offline -- run examples/hello.ling
cargo run --locked --offline -- semantic examples/hello.ling
cargo run --locked --offline -- audit examples/人物.ling
cargo run --locked --offline -- run examples/人物.ling
cargo run --locked --offline -- run examples/adt-match.ling
cargo run --locked --offline -- run examples/pipeline.ling
cargo run --locked --offline -- repl --format human
```

Hello World 的运行输出为 `你好，零`。脚本化 REPL 用空行分隔 submission；需要 Console 时显式传入 `--capability Console.Write`。失败 submission 不提交名称、类型或值，JSON 模式输出 `ling.repl/0.1` 事件。Human TTY 模式由 Rustyline 处理 Ctrl-C/EOF；Ctrl-C 只清空尚未提交的 buffer，保留已提交 session state。

内部编译器错误使用 `L-INTERNAL-0001` 和稳定的 experimental BLAKE3 incident ID，并在操作系统临时目录的 `ling-incidents` 子目录保存 `ling.internal-incident/0.1` 最小重现报告；公开诊断只显示不含用户名的逻辑位置。Semantic reader round-trip 失败使用 `L-SNAPSHOT-0001` 与 exit `6`；宿主 I/O Fault 使用 `L-RUNTIME-0001` 与 exit `4`。

### 语言使命

Ling 希望让程序同时适合人类表达、机器分析和 AI 辅助开发。核心原则是：

- **规范优先**：语义先由规范和 RFC 决定，代码实现不能反向偷偷改变语言含义。
- **可以省略，不可隐瞒**：类型、Effect、Capability、Borrow 和 Contract 可以在安全范围内推导，但审计视图必须展开真实语义。
- **人类表达是一等公民**：中文标识符是正式语言能力，不是演示功能；关键字仍保持一套小型 ASCII 集合，以避免语法方言分裂。
- **AI 不属于可信计算基**：AI 可以查询、提出修改和生成证据，但最终语义必须由解析器、类型系统、权限检查、测试和人工治理决定。
- **显式边界**：时间、随机数、网络、文件等行为通过 Effect 描述，通过 Capability 授权。

### 语言轮廓

示例语法：

```fsharp
let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }

let mutable 关羽 = 初始人物
关羽 <- 受到伤害 30 关羽
```

其中：

- `=` 用于创建绑定和初始化；`<-` 只用于修改已有的可变 Place。
- 函数默认使用空格应用；Seed 参数按 Value 传递，更新由返回值和调用方显式写回完成。
- 中文名称经过 Unicode 规则、NFC 归一化和混淆字符诊断。
- 名称与定义身份分离，未来可以为同一个 Semantic Definition 提供中文、英文和历史别名。

### 核心语义模型

- **内存模型**：`Value`、`Managed`、`Resource` 分别覆盖值语义、受管内存和需要 Ownership/Borrow/Region 的资源语义。
- **Effect**：描述程序实际发生了什么，例如 `Console.Write`、`Clock`、`Random` 或分配行为。
- **Capability**：描述程序被授权做什么，例如访问控制台、文件范围或网络边界。
- **Contract 与证据**：前置条件、后置条件和不变量可以处于 `Proved`、`RuntimeChecked`、`Tested`、`Assumed` 或 `Unverified` 等状态，测试通过不能冒充形式证明。
- **Semantic Graph**：将定义、类型、引用、Effect、Capability、Contract、Borrow 和执行信息组织成可查询的语义图。
- **四种视图**：Author Source 面向人类编写，Audit Source 展开隐式语义，Semantic Graph 面向工具和 AI，Execution View 描述运行结果。

### Build Profile

Ling 计划使用同一套核心语义支持不同严格程度，而不是把 Debug 和 Release 变成两门语言：

| Profile | 目标 | 典型约束 |
| --- | --- | --- |
| Explore | 快速探索和 REPL | VM/JIT、GC、增量编译、运行时 Contract |
| Native | 高性能通用程序 | AOT、Ownership/Region Lowering、优化、SIMD、可选 Managed Island |
| Critical | 可分析、可审计和安全关键系统 | 无一般 GC、有界分配、显式输入、受限 FFI、Fault 证据包 |

Profile 的详细语义和跨 Profile 可移植性仍需后续 RFC 与实现验证。

### `v0.0.1 Seed` 范围

当前 Seed 实现覆盖：

- UTF-8、Unicode 标识符、NFC 名称归一化和基础混淆字符诊断；
- `let`、函数、`if`、record、ADT、穷尽 `match`、空格函数应用和短路 `&&` / `||`；
- `Unit`、`Bool`、`Int`、`f64`、`Text`、`Option`、`Result`；
- 局部类型推导、默认不可变、`mutable` 和 `place <- value`；
- `Pure` 与 `Console.Write` Effect，以及 `Console.Write` Capability；
- Semantic Graph、稳定 Diagnostic JSON、解释器、REPL、`run`、`check`、`semantic`、`audit`。

第一阶段明确后置：GC Runtime、Native Backend、Ownership/Borrow Checker、Trait、Effect Handler、Task、Actor、Node、Kernel、分布式、GPU、形式证明和包管理器。设计中预留的能力不得以静默占位的方式运行。

### 文档结构

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   ├── ling-source/
│   ├── ling-unicode/
│   ├── ling-diagnostics/
│   ├── ling-syntax/
│   ├── ling-ast/
│   ├── ling-hir/
│   ├── ling-resolve/
│   ├── ling-types/
│   ├── ling-effects/
│   ├── ling-semantic/
│   ├── ling-format/
│   ├── ling-eval/
│   └── ling-cli/
├── editors/
│   └── tree-sitter-ling/
├── tests/
│   └── conformance/
├── tools/
│   ├── unicode-gen/
│   └── xtask/
└── docs/
    ├── LANGUAGE.md
    ├── SEMANTICS.md
    ├── RFC-0001.md
    ├── IMPLEMENTATION.md
    ├── NEXT-STEPS.md
    ├── NEXT-STEPS-SEED.md
    ├── ROADMAP-1.0.md
    ├── SEED-TRACEABILITY.md
    ├── SEED-RELEASE-REPORT.md
    ├── ERROR-CODES.md
    ├── DEPENDENCIES.md
    ├── grammar-map.md
    ├── decisions/
    ├── governance/
    ├── traceability/
    └── design-review.html
```

- [`docs/LANGUAGE.md`](docs/LANGUAGE.md)：语言使命、设计宪章、表层语法、中文标识符、内存模型、Effect/Capability、计算模型、Profile、Semantic Graph、AI 工具链和路线图。
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md)：词法、名称、类型、求值、赋值、内存、Effect、Capability、Contract、并发模型、Determinism、Audit Source、Semantic Transaction 和诊断语义。
- [`docs/RFC-0001.md`](docs/RFC-0001.md)：`v0.0.1 Seed` 的范围、语法、编译器架构、CLI、测试策略、治理规则、验收标准和风险清单。
- [`docs/IMPLEMENTATION.md`](docs/IMPLEMENTATION.md)：`v0.0.1 Seed` 的实现计划：仓库布局、技术决策、里程碑出口标准、测试与 CI、规范缺口清单和验收清单。
- [`docs/NEXT-STEPS.md`](docs/NEXT-STEPS.md)：从当前 AST 边界推进到可运行 Hello World 的下一步实施顺序、接口边界、决议门禁和验收矩阵。
- [`docs/NEXT-STEPS-SEED.md`](docs/NEXT-STEPS-SEED.md)：从已完成的 Hello World 纵向切片推进到完整 `v0.0.1 Seed` 的里程碑、决议门禁、验收矩阵和发布条件。
- [`docs/ROADMAP-1.0.md`](docs/ROADMAP-1.0.md)：从已发布的 `v0.0.1 Seed` 推进到 `v1.0` 的分块路线图、RFC 门禁、实施步骤、版本出口和兼容性要求。
- [`docs/traceability/v0.0.1.md`](docs/traceability/v0.0.1.md)：由单一机器注册表生成的 feature/spec/Core/实现/正反测试/differential/release artifact 双语追踪矩阵；CI 校验全部链接与 38 个稳定 fixture ID。
- [`docs/SEED-TRACEABILITY.md`](docs/SEED-TRACEABILITY.md)：`v0.0.1` 发布时保存的历史 Seed 证据索引。
- [`docs/SEED-RELEASE-REPORT.md`](docs/SEED-RELEASE-REPORT.md)：本地质量门禁、候选 SHA、跨平台/fuzz/MSRV CI 与已发布 tag 的双语记录。
- [`docs/ERROR-CODES.md`](docs/ERROR-CODES.md)：`ling.diagnostic/0.1` 的唯一错误码注册表，包含 phase、稳定级别、严重度、双语模板、typed Facts、retired 记录与兼容性边界；生成的 [`error-code-lock.toml`](docs/governance/error-code-lock.toml) 由 CI 检查改义、改型、复用和编号回填。
- [`docs/DEPENDENCIES.md`](docs/DEPENDENCIES.md)：Rust 直接/关键传递依赖、许可证、MSRV、`unsafe` 与审查状态。
- [`docs/grammar-map.md`](docs/grammar-map.md)：`v0.0.1 Seed` Author Source 到 compiler CST/AST、拟议 Tree-sitter 节点与 corpus 义务的映射；明确区分 Accepted decision、Draft 基线和纯恢复 helper。
- [`editors/tree-sitter-ling/`](editors/tree-sitter-ling/)：可独立拆分的 Tree-sitter grammar 开发镜像，包含锁定工具链、生成 parser、37 个 corpus cases、29 个共享表达式 differential cases、41 个共享 pattern/type differential cases、示例和已知差异；它不决定 Ling 语义或合法性。
- [`docs/governance/authority.md`](docs/governance/authority.md)：由机器清单确定性生成的规范权威、生命周期、依赖与冲突处理索引。
- [`docs/governance/gap-register.md`](docs/governance/gap-register.md)：按版本和优先级生成的规范缺口、阻断任务与候选 RFC 台账。
- [`docs/governance/lifecycle.md`](docs/governance/lifecycle.md)：RFC 与 decision 的状态机、稳定实现依据、接受证据、替代关系和模板门禁。
- [`docs/governance/protocol-inventory.md`](docs/governance/protocol-inventory.md)：CLI、JSON、Semantic ID、Audit、内部产物及 Future 协议的版本、稳定级别、reader/writer 和迁移边界。
- [`docs/governance/support-matrix.md`](docs/governance/support-matrix.md)：由机器清单生成的 Ling 1.0 支持矩阵草案，明确区分当前证据、候选范围与不支持范围；JSON fixtures 是未实现 CLI 的内部非契约占位。
- [`docs/status/implementation-status.md`](docs/status/implementation-status.md)：由任务与功能状态注册表生成的当前 implemented/tested/documented、稳定化阻断项、Profile/target 声明及完成 commit。
- [`docs/status/release-status.md`](docs/status/release-status.md)：从同一状态注册表生成的发布说明输入片段；它不是发布公告或稳定性承诺。
- [`docs/design-review.html`](docs/design-review.html)：针对上述规范的设计评审记录；它是非规范性意见，不替代已接受的 RFC。

### 规范权威顺序

```text
Accepted RFC
    > docs/SEMANTICS.md
    > docs/LANGUAGE.md
    > conformance tests
    > implementation
```

当实现、示例和规范不一致时，应先提交 RFC 或修订规范，再修改实现。实现不得通过代码行为悄悄创造未被规范声明的语言特性。

### 路线图

```text
v0.0.1 Seed       语法、类型、中文标识符、Semantic Graph、解释器
v0.1 Living       字节码 VM、模块、增量编译、LSP、Formatter、基础 Trait
v0.2 Concurrent   Structured Task、Actor、Bounded Mailbox、Supervisor、Replay
v0.3 Native       Value/Resource、Ownership/Region、Cranelift/LLVM
v0.4 Heterogeneous Kernel、CPU SIMD、GPU/TPU Lowering、Placement
v0.5 Critical     Node、Critical Profile、Contract、模型检查、证据包
v1.0              稳定语言核心、稳定 Semantic Graph Schema、兼容性承诺
```

路线图不是实现承诺；每个阶段都必须通过规范、测试和可审计的验收标准。

### 参与贡献

欢迎通过 RFC、规范勘误、示例、测试和工具建议参与项目：

1. 会改变语言语义、类型规则、Effect、Capability 或 Graph Schema 的提案，应先写 RFC。
2. 规范性修改必须同步更新示例、诊断和 conformance tests。
3. 实现应复用同一套 Parser、Checker、Interpreter 和 REPL Core，避免工具之间产生不同的小语言。
4. 错误码、JSON 字段和 Semantic ID 的行为应保持可追踪，并记录实验性或不稳定状态。
5. 中英文内容可以分别改善，但代码示例、规范术语和语义定义必须保持一致。

### 许可证

当前仓库包含 Apache License 2.0，具体条款请参阅 [`LICENSE`](LICENSE)。除非文件内另有声明，贡献者和使用者都应以该许可证文本为准。

Ling 仍处于早期设计阶段。请在生产使用、标准化承诺或大规模实现之前，先阅读规范、RFC 和风险评审，并以接受的 RFC 为准。

## English

### Project status

The annotated [`v0.0.1 Seed`](https://github.com/PlayerAI/ling/tree/v0.0.1) tag is published. Its target passed the local gates and same-SHA Windows, Linux, macOS, real Unix TTY, nightly fuzz, and Rust 1.85 CI; see `docs/SEED-RELEASE-REPORT.md` for evidence. This remains experimental rather than a production compiler or runtime.

| Item | Current status |
| --- | --- |
| Language name | Chinese: 零; English: Ling |
| CLI | `ling` |
| Source extension | `.ling` |
| Current milestone | P8–P11 are implemented locally; P12 release evidence and candidate-commit gates remain |
| Implementation | Unicode 17, the Seed frontend, modules/imports, generic nominal types, injected Prelude, pattern exhaustiveness, Effects/Capabilities, Semantic/Audit, interpreter, transactional REPL, shared CLI compiler, and conformance runner implemented |
| Stability | The design may change; accepted RFCs are the mechanism for freezing semantics |

### Build and verification

Rust 1.97.1 is the pinned development toolchain; the declared MSRV is Rust 1.85. Fetch the committed lockfile's dependencies once, then run normal builds and tests explicitly offline:

```bash
cargo fetch --locked
cargo test --workspace --all-features --locked --offline
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo xtask governance check-all
cargo xtask ci verify
cargo xtask traceability verify --release v0.0.1
cargo xtask schema validate-all
cargo xtask schema compatibility --from N-1 --to N
cargo xtask schema corrupt-inputs
cargo xtask seed reproduce
cargo xtask support verify
cargo xtask status verify
cargo run --locked --offline -- --version
```

`governance check-all` aggregates the five governance-registry checks; `ci verify` prevents drift in the eight G0 jobs, three-platform tests, fuzz, and MSRV gates; and `seed reproduce` builds once and compares check/run/Semantic/Audit output across eight independent processes.

Public JSON protocol versions, reader/writer ranges, field policies, and golden corpora are recorded together in [`SCHEMA-LIFECYCLE.md`](docs/governance/SCHEMA-LIFECYCLE.md), [`schemas/registry.toml`](schemas/registry.toml), and [`schemas/`](schemas/README.md). All three current protocols are first versions, so the compatibility gate reports `NoPreviousVersion` explicitly and does not claim N-1 support.

`check`, `run`, `semantic`, `audit`, and `repl` share one real compilation path. `run` and the REPL interpret only ProgramSnapshots that passed name, type, Place, Effect, and Capability checks. `semantic` emits deterministic `ling.semantic/0.1` JSON, while `audit` emits round-trippable `ling.audit/0.1` text:

```bash
cargo run --locked --offline -- check examples/hello.ling
cargo run --locked --offline -- run examples/hello.ling
cargo run --locked --offline -- semantic examples/hello.ling
cargo run --locked --offline -- audit examples/人物.ling
cargo run --locked --offline -- run examples/人物.ling
cargo run --locked --offline -- run examples/adt-match.ling
cargo run --locked --offline -- run examples/pipeline.ling
cargo run --locked --offline -- repl --format human
```

Hello World prints `你好，零`. Scripted REPL sessions separate submissions with a blank line; pass `--capability Console.Write` when Console access is required. Failed submissions do not commit names, types, or values, and JSON mode emits `ling.repl/0.1` events. Rustyline handles Ctrl-C/EOF in human TTY mode; Ctrl-C clears only the pending buffer and preserves committed session state.

Internal compiler failures use `L-INTERNAL-0001` and a stable experimental BLAKE3 incident ID, with a minimal `ling.internal-incident/0.1` reproduction report saved under the OS temporary directory's `ling-incidents` folder; public diagnostics expose only a username-free logical location. Semantic reader round-trip failures use `L-SNAPSHOT-0001` and exit `6`; host I/O faults use `L-RUNTIME-0001` and exit `4`.

### Mission

Ling aims to make programs suitable for human expression, machine analysis, and AI-assisted development at the same time. Its core principles are:

- **Specification first**: semantics are defined by specifications and RFCs; implementation code must not silently redefine the language.
- **Omission without concealment**: types, effects, capabilities, borrows, and contracts may be inferred within safe limits, but the audit view must expose the resulting semantics.
- **Human expression as a first-class concern**: Chinese identifiers are a formal language feature, while a small ASCII keyword set avoids dialect fragmentation.
- **AI outside the trusted computing base**: AI may query, propose changes, and produce evidence, but parsers, type checking, authorization checks, tests, and human governance decide semantics.
- **Explicit boundaries**: time, randomness, networking, files, and similar behavior are described by Effects and authorized by Capabilities.

### Language outline

Example syntax:

```fsharp
let 受到伤害 伤害 人物 =
    { 人物 with 血量 = max 0 (人物.血量 - 伤害) }

let mutable 关羽 = 初始人物
关羽 <- 受到伤害 30 关羽
```

In this example:

- `=` creates bindings and initializes values; `<-` modifies an existing mutable Place.
- Functions use space application by default. Seed parameters use value semantics, so updates return a value and the caller writes it back explicitly.
- Chinese names are processed with Unicode rules, NFC normalization, and confusable-character diagnostics.
- Names are separate from definition identity, allowing future Chinese, English, and historical aliases for one Semantic Definition.

### Core semantic model

- **Memory model**: `Value`, `Managed`, and `Resource` cover value semantics, managed memory, and resources governed by Ownership/Borrow/Region rules.
- **Effects**: describe what a program actually does, such as `Console.Write`, `Clock`, `Random`, or allocation.
- **Capabilities**: describe what a program is authorized to do, such as accessing a console, a file scope, or a network boundary.
- **Contracts and evidence**: preconditions, postconditions, and invariants may be `Proved`, `RuntimeChecked`, `Tested`, `Assumed`, or `Unverified`; passing tests must not be presented as a formal proof.
- **Semantic Graph**: organizes definitions, types, references, Effects, Capabilities, Contracts, Borrows, and execution information into a queryable graph.
- **Four views**: Author Source is for humans, Audit Source expands implicit semantics, Semantic Graph is for tools and AI, and Execution View describes runtime behavior.

### Build Profiles

Ling is designed to support different levels of strictness over one core semantic model, rather than turning Debug and Release into different languages:

| Profile | Goal | Typical constraints |
| --- | --- | --- |
| Explore | Fast exploration and REPL workflows | VM/JIT, GC, incremental compilation, runtime Contracts |
| Native | High-performance general-purpose programs | AOT, Ownership/Region lowering, optimization, SIMD, optional Managed Islands |
| Critical | Analyzable, auditable, safety-critical systems | No general GC, bounded allocation, explicit inputs, restricted FFI, Fault evidence bundles |

Detailed Profile semantics and cross-Profile portability still require later RFCs and implementation evidence.

### `v0.0.1 Seed` scope

The current Seed implementation covers:

- UTF-8, Unicode identifiers, NFC normalization, and basic confusable-character diagnostics;
- `let`, functions, `if`, records, ADTs, exhaustive `match`, space application, and short-circuit `&&` / `||`;
- `Unit`, `Bool`, `Int`, `f64`, `Text`, `Option`, and `Result`;
- local type inference, default immutability, `mutable`, and `place <- value`;
- `Pure` and `Console.Write` Effects, plus the `Console.Write` Capability;
- Semantic Graph, stable Diagnostic JSON, an interpreter, a REPL, `run`, `check`, `semantic`, and `audit`.

The first milestone explicitly postpones the GC runtime, native backend, Ownership/Borrow checker, Traits, Effect Handlers, Task, Actor, Node, Kernel, distribution, GPU support, formal proofs, and package management. Reserved features must not silently execute as placeholders.

### Documentation layout

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   ├── ling-source/
│   ├── ling-unicode/
│   ├── ling-diagnostics/
│   ├── ling-syntax/
│   ├── ling-ast/
│   ├── ling-hir/
│   ├── ling-resolve/
│   ├── ling-types/
│   ├── ling-effects/
│   ├── ling-semantic/
│   ├── ling-format/
│   ├── ling-eval/
│   └── ling-cli/
├── editors/
│   └── tree-sitter-ling/
├── tests/
│   └── conformance/
├── tools/
│   ├── unicode-gen/
│   └── xtask/
└── docs/
    ├── LANGUAGE.md
    ├── SEMANTICS.md
    ├── RFC-0001.md
    ├── IMPLEMENTATION.md
    ├── NEXT-STEPS.md
    ├── NEXT-STEPS-SEED.md
    ├── ROADMAP-1.0.md
    ├── SEED-TRACEABILITY.md
    ├── SEED-RELEASE-REPORT.md
    ├── ERROR-CODES.md
    ├── DEPENDENCIES.md
    ├── grammar-map.md
    ├── decisions/
    ├── governance/
    ├── traceability/
    └── design-review.html
```

- [`docs/LANGUAGE.md`](docs/LANGUAGE.md): mission, design charter, surface syntax, Chinese identifiers, memory model, Effects/Capabilities, computation models, Profiles, Semantic Graph, AI tooling, and roadmap.
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md): lexical, naming, type, evaluation, assignment, memory, Effect, Capability, Contract, concurrency, determinism, Audit Source, Semantic Transaction, and diagnostic semantics.
- [`docs/RFC-0001.md`](docs/RFC-0001.md): `v0.0.1 Seed` scope, syntax, compiler architecture, CLI, tests, governance, acceptance criteria, and risks.
- [`docs/IMPLEMENTATION.md`](docs/IMPLEMENTATION.md): the `v0.0.1 Seed` implementation plan — repository layout, technical decisions, milestone exit criteria, testing and CI, the specification-gap list, and the acceptance checklist.
- [`docs/NEXT-STEPS.md`](docs/NEXT-STEPS.md): the next implementation sequence from the current AST boundary to an executable Hello World, including interface boundaries, decision gates, and acceptance cases.
- [`docs/NEXT-STEPS-SEED.md`](docs/NEXT-STEPS-SEED.md): milestones, decision gates, acceptance cases, and release conditions for progressing from the completed Hello World slice to the full `v0.0.1 Seed` scope.
- [`docs/ROADMAP-1.0.md`](docs/ROADMAP-1.0.md): the block-by-block roadmap from the released `v0.0.1 Seed` to `v1.0`, including RFC gates, implementation steps, release exits, and compatibility requirements.
- [`docs/traceability/v0.0.1.md`](docs/traceability/v0.0.1.md): the bilingual feature/spec/Core/implementation/positive/negative/differential/release-artifact matrix generated from one machine registry; CI validates every link and all 38 stable fixture IDs.
- [`docs/SEED-TRACEABILITY.md`](docs/SEED-TRACEABILITY.md): the historical Seed evidence index preserved from the `v0.0.1` release.
- [`docs/SEED-RELEASE-REPORT.md`](docs/SEED-RELEASE-REPORT.md): a bilingual record of local gates, the candidate SHA, platform/fuzz/MSRV CI, and the published tag.
- [`docs/ERROR-CODES.md`](docs/ERROR-CODES.md): the sole `ling.diagnostic/0.1` code registry, including phase, stability, severity, bilingual templates, typed Facts, retired records, and compatibility boundaries; CI checks the generated [`error-code-lock.toml`](docs/governance/error-code-lock.toml) for changed meanings/types, reuse, and number backfilling.
- [`docs/DEPENDENCIES.md`](docs/DEPENDENCIES.md): direct and key transitive Rust dependencies, licenses, MSRV, `unsafe`, and review status.
- [`docs/grammar-map.md`](docs/grammar-map.md): maps `v0.0.1 Seed` Author Source to compiler CST/AST nodes, proposed Tree-sitter nodes, and corpus obligations while distinguishing Accepted decisions, the Draft baseline, and recovery-only helpers.
- [`editors/tree-sitter-ling/`](editors/tree-sitter-ling/): a standalone-ready Tree-sitter grammar development mirror with a locked toolchain, generated parser, 37 corpus cases, 29 shared expression differential cases, 41 shared pattern/type differential cases, an example, and explicit known differences; it does not define Ling semantics or validity.
- [`docs/governance/authority.md`](docs/governance/authority.md): the deterministically generated index of specification authority, lifecycle, dependencies, and conflict handling.
- [`docs/governance/gap-register.md`](docs/governance/gap-register.md): the release- and priority-ordered register of specification gaps, blocked tasks, and candidate RFCs.
- [`docs/governance/lifecycle.md`](docs/governance/lifecycle.md): the RFC/decision state machine, Stable implementation basis, acceptance evidence, supersession rules, and checked templates.
- [`docs/governance/protocol-inventory.md`](docs/governance/protocol-inventory.md): versions, stability levels, reader/writer rules, and migration boundaries for CLI, JSON, Semantic ID, Audit, internal artifacts, and Future protocols.
- [`docs/governance/support-matrix.md`](docs/governance/support-matrix.md): the generated Ling 1.0 support-matrix draft, separating current evidence, candidate scope, and unsupported scope; its JSON fixtures are internal non-contract placeholders for unimplemented CLI commands.
- [`docs/status/implementation-status.md`](docs/status/implementation-status.md): generated current implemented/tested/documented state, stabilization blockers, Profile/target claims, and completion commits for tasks and features.
- [`docs/status/release-status.md`](docs/status/release-status.md): a release-note input fragment generated from the same registry; it is not a release announcement or stability promise.
- [`docs/design-review.html`](docs/design-review.html): a non-normative design review of the specifications; it does not replace an accepted RFC.

### Authority order

```text
Accepted RFC
    > docs/SEMANTICS.md
    > docs/LANGUAGE.md
    > conformance tests
    > implementation
```

When implementation, examples, and specifications disagree, submit or revise an RFC before changing the implementation. Code must not silently create language features that the specifications do not declare.

### Roadmap

```text
v0.0.1 Seed       Syntax, types, Chinese identifiers, Semantic Graph, interpreter
v0.1 Living       Bytecode VM, modules, incremental compilation, LSP, formatter, basic Traits
v0.2 Concurrent   Structured Task, Actor, bounded mailbox, Supervisor, Replay
v0.3 Native       Value/Resource, Ownership/Region, Cranelift/LLVM
v0.4 Heterogeneous Kernel, CPU SIMD, GPU/TPU lowering, Placement
v0.5 Critical     Node, Critical Profile, Contracts, model checking, evidence bundles
v1.0              Stable language core, stable Semantic Graph Schema, compatibility commitments
```

The roadmap is not an implementation promise. Each stage must pass specification, testing, and auditable acceptance criteria.

### Contributing

Contributions through RFCs, specification corrections, examples, tests, and tooling proposals are welcome:

1. Proposals that change language semantics, type rules, Effects, Capabilities, or the Graph Schema should begin as an RFC.
2. Normative changes must update examples, diagnostics, and conformance tests together.
3. Implementations should reuse one Parser, Checker, Interpreter, and REPL Core so tools do not drift into incompatible mini-languages.
4. Error codes, JSON fields, and Semantic ID behavior should remain traceable, with experimental or unstable status recorded explicitly.
5. Chinese and English documentation may evolve independently, but examples, terminology, and semantic definitions must stay aligned.

### License

This repository currently includes the Apache License 2.0; see [`LICENSE`](LICENSE) for the complete terms. Unless a file states otherwise, contributors and users should follow that license text.

Ling is still an early-stage design. Read the specifications, RFCs, and design review before production use, standardization commitments, or large-scale implementation, and treat accepted RFCs as the source of truth.
