# 零语言（Ling）

> 一门规范优先、面向 AI、支持中文标识符，并以语义图、显式 Effect 与 Capability 为核心的编程语言。
>
> A specification-first, AI-native programming language centered on semantic graphs, explicit effects and capabilities, and first-class Chinese identifiers.

[中文说明](#中文) · [English](#english)

## 中文

### 项目状态

Ling 目前处于 `v0.0.1 Seed` 的早期实现阶段。Rust workspace、Source/Unicode/Diagnostic 基础层、Lexer、Offside Layout、Parser/CST、AST Lowering 与最小 CLI 已建立；名称解析、类型系统、Semantic Graph 和解释器仍未实现，因此当前版本不能用于运行 Ling 程序，更不是生产编译器或运行时。

| 项目 | 当前状态 |
| --- | --- |
| 语言名称 | 中文名：零；英文名：Ling |
| CLI | `ling` |
| 源文件扩展名 | `.ling` |
| 当前里程碑 | M2 Parser（Parser/CST/AST 主路径已完成，验收加固进行中） |
| 实现状态 | UTF-8 SourceMap、Unicode 17 生成表/XID/NFC/UTS #39、双语 JSON Diagnostic、Lexer、Offside Layout、Parser/CST、AST Lowering、最小 CLI 与 conformance runner 已落地 |
| 稳定性 | 设计可能变化，接受的 RFC 才能冻结语义 |

### 构建与验证

需要 Rust 1.97.1；项目声明的最低 Rust 版本为 1.85。首次构建解析并锁定依赖后，常规构建与测试使用已提交的 `Cargo.lock`：

```bash
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo run --locked -- --version
```

当前 `check` 已能读取源码、验证 UTF-8、处理 BOM/换行映射，并执行 Unicode 标识符检查、Lexer/Layout、Parser/CST 与 AST Lowering，输出 `ling.diagnostic/0.1`；合法 AST 随后会以 `L-IMPL-0001` 明确拒绝尚未实现的名称解析与语义阶段，而不会伪装检查成功：

```bash
cargo run --locked -- check --format json tests/conformance/m0-source-pipeline/case.ling
```

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
    人物.血量 <- max 0 (人物.血量 - 伤害)
```

其中：

- `=` 用于创建绑定和初始化；`<-` 只用于修改已有的可变 Place。
- 函数默认使用空格应用，例如 `受到伤害 30 关羽`。
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

第一阶段计划覆盖：

- UTF-8、Unicode 标识符、NFC 名称归一化和基础混淆字符诊断；
- `let`、函数、`if`、record、ADT、穷尽 `match` 和空格函数应用；
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
│   └── ling-cli/
├── tests/
│   └── conformance/
├── tools/
│   └── unicode-gen/
└── docs/
    ├── LANGUAGE.md
    ├── SEMANTICS.md
    ├── RFC-0001.md
    ├── IMPLEMENTATION.md
    ├── NEXT-STEPS.md
    ├── ERROR-CODES.md
    ├── DEPENDENCIES.md
    ├── decisions/
    └── design-review.html
```

- [`docs/LANGUAGE.md`](docs/LANGUAGE.md)：语言使命、设计宪章、表层语法、中文标识符、内存模型、Effect/Capability、计算模型、Profile、Semantic Graph、AI 工具链和路线图。
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md)：词法、名称、类型、求值、赋值、内存、Effect、Capability、Contract、并发模型、Determinism、Audit Source、Semantic Transaction 和诊断语义。
- [`docs/RFC-0001.md`](docs/RFC-0001.md)：`v0.0.1 Seed` 的范围、语法、编译器架构、CLI、测试策略、治理规则、验收标准和风险清单。
- [`docs/IMPLEMENTATION.md`](docs/IMPLEMENTATION.md)：`v0.0.1 Seed` 的实现计划：仓库布局、技术决策、里程碑出口标准、测试与 CI、规范缺口清单和验收清单。
- [`docs/NEXT-STEPS.md`](docs/NEXT-STEPS.md)：从当前 AST 边界推进到可运行 Hello World 的下一步实施顺序、接口边界、决议门禁和验收矩阵。
- [`docs/ERROR-CODES.md`](docs/ERROR-CODES.md)：`ling.diagnostic/0.1` 的稳定错误码、双语模板、Facts 与兼容性边界。
- [`docs/DEPENDENCIES.md`](docs/DEPENDENCIES.md)：Rust 直接/关键传递依赖、许可证、MSRV、`unsafe` 与审查状态。
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

Ling is in the early implementation phase of `v0.0.1 Seed`. The Rust workspace, Source/Unicode/Diagnostic foundations, lexer, offside layout, Parser/CST, AST lowering, and minimal CLI now exist. Name resolution, the type system, Semantic Graph, and interpreter remain unimplemented, so this version cannot yet execute Ling programs and is not a production compiler or runtime.

| Item | Current status |
| --- | --- |
| Language name | Chinese: 零; English: Ling |
| CLI | `ling` |
| Source extension | `.ling` |
| Current milestone | M2 Parser (Parser/CST/AST main path complete; acceptance hardening in progress) |
| Implementation | UTF-8 SourceMap, generated Unicode 17 XID/NFC/UTS #39 tables, bilingual JSON diagnostics, Lexer, Offside Layout, Parser/CST, AST lowering, minimal CLI, and conformance runner implemented |
| Stability | The design may change; accepted RFCs are the mechanism for freezing semantics |

### Build and verification

Rust 1.97.1 is the pinned development toolchain; the declared MSRV is Rust 1.85. After the initial dependency resolution, normal builds and tests use the committed `Cargo.lock`:

```bash
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo run --locked -- --version
```

The current `check` path reads the source, validates UTF-8, handles BOM/newline mapping, performs Unicode identifier checks, Lexer/Layout processing, Parser/CST construction, and AST lowering, and emits `ling.diagnostic/0.1`. A valid AST then rejects the missing name-resolution and semantic stages explicitly with `L-IMPL-0001` instead of pretending that checking succeeded:

```bash
cargo run --locked -- check --format json tests/conformance/m0-source-pipeline/case.ling
```

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
    人物.血量 <- max 0 (人物.血量 - 伤害)
```

In this example:

- `=` creates bindings and initializes values; `<-` modifies an existing mutable Place.
- Functions use space application by default, for example `受到伤害 30 关羽`.
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

The first milestone is planned to cover:

- UTF-8, Unicode identifiers, NFC normalization, and basic confusable-character diagnostics;
- `let`, functions, `if`, records, ADTs, exhaustive `match`, and space application;
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
│   └── ling-cli/
├── tests/
│   └── conformance/
├── tools/
│   └── unicode-gen/
└── docs/
    ├── LANGUAGE.md
    ├── SEMANTICS.md
    ├── RFC-0001.md
    ├── IMPLEMENTATION.md
    ├── NEXT-STEPS.md
    ├── ERROR-CODES.md
    ├── DEPENDENCIES.md
    ├── decisions/
    └── design-review.html
```

- [`docs/LANGUAGE.md`](docs/LANGUAGE.md): mission, design charter, surface syntax, Chinese identifiers, memory model, Effects/Capabilities, computation models, Profiles, Semantic Graph, AI tooling, and roadmap.
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md): lexical, naming, type, evaluation, assignment, memory, Effect, Capability, Contract, concurrency, determinism, Audit Source, Semantic Transaction, and diagnostic semantics.
- [`docs/RFC-0001.md`](docs/RFC-0001.md): `v0.0.1 Seed` scope, syntax, compiler architecture, CLI, tests, governance, acceptance criteria, and risks.
- [`docs/IMPLEMENTATION.md`](docs/IMPLEMENTATION.md): the `v0.0.1 Seed` implementation plan — repository layout, technical decisions, milestone exit criteria, testing and CI, the specification-gap list, and the acceptance checklist.
- [`docs/NEXT-STEPS.md`](docs/NEXT-STEPS.md): the next implementation sequence from the current AST boundary to an executable Hello World, including interface boundaries, decision gates, and acceptance cases.
- [`docs/ERROR-CODES.md`](docs/ERROR-CODES.md): stable `ling.diagnostic/0.1` codes, bilingual templates, Facts, and compatibility boundaries.
- [`docs/DEPENDENCIES.md`](docs/DEPENDENCIES.md): direct and key transitive Rust dependencies, licenses, MSRV, `unsafe`, and review status.
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
