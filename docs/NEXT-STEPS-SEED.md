# 下一步开发计划：从 Hello World 到 `v0.0.1 Seed`

> 状态：P8～P11 已实现；P12 等待候选 commit 与远程三平台 CI
> 日期：2026-08-18
> 基线 commit：`a88e5ef89abc3c26e0910016dc6305ee79c53e3e`
> 当前基线：Hello World 已贯通 Source、Syntax、HIR、Resolver、Type、Effect/Capability、Semantic Snapshot、Interpreter 与 CLI
> 目标版本：`0.0.1-dev` 功能闭合；是否发布和打标签不由本文授权
> 规范依据：[RFC-0001](RFC-0001.md)、[SEMANTICS](SEMANTICS.md)、[IMPLEMENTATION](IMPLEMENTATION.md) 及已接受的 [decisions](decisions/)
> 地位：本文定义工程顺序、接口约束和验收证据，不新增语言语义；与 Accepted RFC 冲突时，以 Accepted RFC 为准。

---

## 1. 阶段目标

上一阶段已证明最小程序可以沿真实语义管线完成检查、快照和执行。本阶段不进入 VM、Native、LSP 或包管理，而是关闭 `v0.0.1 Seed` 已承诺但尚未完成的语言面与工具面：

1. record、泛型 nominal ADT、模式匹配、穷尽性和可变 Place 达到全链路可用；
2. `examples/人物.ling`、ADT/match 和 pipeline 成为可运行的规范示例；
3. `map` 的高阶 Effect/Capability 传播符合 [DEC-0011](decisions/0011-seed-builtins.md)；
4. Semantic JSON 具备 reader、结构验证和兼容性测试；
5. G-12 决议接受后实现确定性 Audit Source 与 round-trip；
6. G-14 决议接受后实现事务式 REPL，并复用文件模式的同一 Checked Core；
7. 完成规范追踪、性质测试、三平台 CI 和发布前审计。

最主要的可观察出口是：

```text
cargo run --locked --offline -- check examples/人物.ling
cargo run --locked --offline -- run examples/人物.ling
cargo run --locked --offline -- audit examples/人物.ling
```

其中 `run` 必须精确输出：

```text
存活
```

输出末尾是单个 UTF-8 LF，进程退出码为 `0`。

## 2. 当前事实与剩余缺口

### 2.1 已完成基线

- Unicode 17.0.0、原始 byte span、换行映射和标识符安全检查；
- Lexer、offside layout、Parser、lossless CST、AST 和稳定语法快照；
- deterministic module/import、Resolved HIR、Place 分类和 pipeline lowering；
- HM 风格推导、任意精度 `Int`、保守 Value Restriction；
- `Pure`、`State<T>`、`Console.Write`、静态 Capability 检查；
- 版本化 canonical bytes、BLAKE3 `DefinitionId / BodyId / ProgramId`；
- 只接受 Checked ProgramSnapshot 的解释器；
- `check`、`run`、`semantic` 及稳定退出码；
- parser fuzz targets、conformance runner 和三平台 CI 配置。
- RFC §6.11 Semantic Graph 全节点类别、resolved ID edge、Audit 投影和严格 reader validation；
- `L-INTERNAL-0001` incident report、`L-SNAPSHOT-0001` reader round-trip 与 host/internal/snapshot 退出码分离。

### 2.2 不得误报为完成的能力

| 能力 | 当前事实 | 本阶段要求 |
| --- | --- | --- |
| record / ADT | HIR、Type、Semantic、Eval 已有表示和部分路径 | 增加成功用例、泛型替换、字段完整性、constructor/pattern 诊断和跨模块测试 |
| `match` | 已检查 scrutinee、guard 与分支结果类型 | 增加 Bool/nominal variant 穷尽性、不可达分支和稳定 witness |
| `Option / Result` | DEC-0014 已 Accepted | 已由 Resolver 注入 `Ling.Prelude`，泛型 constructor、保留名称和稳定 provenance 已验证 |
| 高阶 Effect | 普通调用图传播已存在 | `map` 必须传播 callback Effect，并由结果推导 Capability |
| Semantic JSON | writer/reader、RFC §6.11 全部节点类别、resolved source/owner/target ID、扩展字段兼容、负向结构验证与独立进程确定性均已完成 | 仅剩候选 SHA 的跨平台 CI 证据；不在 Seed 冻结 experimental Schema |
| Audit | DEC-0015 已 Accepted | `AuditModel`、canonical renderer/parser、`L-AUDIT-*`、round-trip 与 CLI 独立进程确定性已验证 |
| REPL | DEC-0016 已 Accepted | 事务会话、多行/EOF、重定义 generation、回滚、Capability 与 human/JSON 脚本模式已验证 |
| 发布门禁 | 本地 workspace 门禁已通过，CI 已配置 | 需要完整 Seed conformance、真实三平台结果和干净发布 commit |

现有 `m2-record-match-parser` fixture 的 `match` 两个分支分别返回 `Text` 与 `Int`，预期 `L-TYPE-0001` 是正确反例，不是 record/ADT 成功验收。不得把该 fixture 当作完整类型和执行证据。

## 3. 范围边界

### 3.1 本阶段包含

- RFC-0001 已承诺的 record、ADT、`Option`、`Result`、`match`、mutable local/field；
- Seed 内置项 `Console.write`、`Text.format`、`max`、`min`、`map`、`sum`；
- `ling.semantic/0.1` reader 与兼容性验证；
- `ling audit` 与 `ling repl`；
- `examples/人物.ling`、ADT/match、pipeline 示例；
- `v0.0.1` 所需 conformance、性质测试、fuzz/CI 和依赖审计。

### 3.2 本阶段不包含

- VM/JIT、AOT、LLVM/Cranelift、优化器或二进制发布器；
- Ownership、Borrow、Resource、Region、GC 或 Managed runtime；
- Trait、Effect Handler、Task、Actor、Node、Kernel；
- LSP、通用 Author Source formatter、包管理和增量编译；
- Contract 求解、形式证明和 Critical Profile；
- 为未来功能预留不可验证的抽象层或静默占位执行路径。

新增需求若不直接服务本阶段验收，应进入后续 RFC 或 issue，不扩张当前实现。

## 4. 规范决议门禁

### 4.1 必须先关闭的现有缺口

| 缺口 | 阻断内容 | 决议至少必须覆盖 |
| --- | --- | --- |
| G-12 | Audit model/grammar、`ling-format`、`ling audit` | 文本版本标记、唯一布局、字符串/Unicode 转义、字段顺序、显示元数据、parser 错误、等价关系、未知扩展字段和版本迁移 |
| G-14 | REPL session、脚本模式 | submission 边界、多行完成判定、默认 module、重定义/阴影、失败回滚、Capability 环境、stdout/stderr、EOF/中断和 JSON 事件协议 |

每个缺口使用下一个可用 decision 编号建立独立文档。状态为 `Proposed` 时只允许原型和测试设计；进入协议实现前必须改为 `Accepted`，并回填 [IMPLEMENTATION §6](IMPLEMENTATION.md#6-规范缺口清单)。

### 4.2 新发现的规范澄清点

`Option / Result` 虽在 Seed 类型集合中，但 [DEC-0011](decisions/0011-seed-builtins.md) 的内置定义集合没有说明它们由 Resolver 注入、由标准 prelude 源码提供，还是仅允许用户声明同构 ADT。P8 开始实现预定义 `Option / Result` 前必须以勘误或 decision 明确：

- 规范 module/name、类型参数顺序和 constructor namespace；
- `Some / None / Ok / Error` 的可见性、shadow/duplicate 规则；
- `DefinitionId` 和 Semantic Graph origin；
- 是否允许用户在 module scope 重定义这些名称。

该澄清不阻断用户自定义的 `人物` 与 `生存状态`，但阻断“Seed 已完整支持 `Option / Result`”的宣称。

## 5. 架构与接口约束

### 5.1 依赖方向

保持当前 core crate 单向依赖。新增 Audit 层只能依赖稳定语义模型，不得反向污染 checker 或 evaluator：

```text
ling-source / ling-unicode / ling-diagnostics
                    │
                    ▼
ling-syntax → ling-ast → ling-hir → ling-resolve → ling-types → ling-effects
                                                                    │
                                                                    ▼
                                                            ling-semantic
                                                               │       │
                                                               ▼       ▼
                                                        ling-format  ling-eval
                                                               \       /
                                                                ▼     ▼
                                                                ling-cli
```

- `ling-types` 内部增加独立的 exhaustiveness 模块；在出现第二个消费者前不新建 pattern crate；
- `ling-semantic` 拥有 `AuditModel` 和 Semantic JSON reader/validator；
- `ling-format` 只负责 Audit grammar、parser 和 renderer，不重新推导 Type/Effect/Capability；
- `ling-eval` 不依赖 `ling-format`，也不执行从 JSON/Audit 直接读入的未验证模型；
- `ling-cli` 只负责编排、文件/终端 I/O 与退出码。

### 5.2 CLI 与 REPL 复用

开始 REPL 前，先把当前 CLI 内的编译编排提取为可测试的 library module，使文件模式和会话模式共享：

```text
Source → Parse → AST → HIR → Resolve → Type → Exhaustiveness/Place
       → Effect/Capability → CheckedProgram → ProgramSnapshot
```

优先在现有 `ling-cli` package 中增加 `src/lib.rs` 与小型内部模块，避免过早增加 driver/session crate。只有出现独立于 CLI 的第二个真实消费者时，才把该 API 提升为新 crate。

### 5.3 不变量

- evaluator 只能接收由 checker 构造的 `ProgramSnapshot`；
- Audit parser 只重建/验证 Audit model，不获得执行权限；
- exhaustiveness 只处理 Seed 明确承诺的 Bool 与 nominal variant；
- 带 guard 的分支不用于证明覆盖，其 body 仍参与类型和 Effect 检查；
- 所有集合、诊断和序列化输出使用确定性顺序；
- Source Span、路径、Rust `Debug`、arena index 和 HashMap 顺序不得进入 Semantic ID；
- 不通过字符串名称特判 record、variant、`map` 或 REPL 结果。

## 6. 实施里程碑

### P8：完成 nominal data 与 pattern 静态语义

目标：让 record、泛型 ADT 和 `match` 从“存在数据结构”升级为“可证明正确的语言能力”。

任务：

1. 完成 type parameter 环境、实例化与 nominal 参数替换，覆盖跨 module 引用和 alias；
2. 检查重复/缺失/未知 record 字段、字段类型、ambiguous literal 和 update base；
3. 检查 variant constructor arity、payload 类型、pattern binder 重复和 constructor 所属类型；
4. 实现 Bool 与 nominal variant 的 pattern matrix；
5. 非穷尽 `match` 产生 error，并给出确定性的最小缺失 witness；
6. 被前序无 guard 分支完全覆盖的 case 产生 warning；guarded case 不贡献静态覆盖；
7. 复验 mutable local、mutable field、immutable field 和参数 value semantics；
8. 按实际新增错误依次分配 `L-TYPE-*` / `L-MUT-*`，同步 [ERROR-CODES](ERROR-CODES.md)。

必要测试：

- 泛型 `Option<'a>` / `Result<'a, 'e>` 或决议规定的等价 prelude 测试；
- record 正常构造、字段乱序、缺字段、重复字段、未知字段、错误类型；
- ADT 无 payload/有 payload constructor，跨 module constructor；
- Bool 完整/不完整、ADT 完整/不完整、wildcard、重复 case、guard；
- immutable root、immutable field、mutable field、参数字段赋值拒绝；
- witness、warning 顺序和 Span 在重复运行中稳定。

出口：新增的正向 record/ADT 程序通过 `check`；所有不完整 match 在进入 Effect/Eval 前被拒；类型内部编号变化不改变公开诊断或 Semantic ID。

### P9：领域示例、内置项与执行闭环

目标：以非特例的真实程序证明 P8 能力贯通 Graph 与 Interpreter。

任务：

1. 新增 RFC §7.2 对应的 `examples/人物.ling`，保持参数 value semantics 和调用方显式写回；
2. 新增自包含 `examples/adt-match.ling`，覆盖三分支 `生存状态` 与 payload；
3. 新增自包含 `examples/pipeline.ling`，覆盖 pipeline → `map` → `sum`；
4. 修正 `map` callback 的 Effect Row 传播，间接 `Console.write` 必须推导 `Console.Write` Capability；
5. 验证 `map` 严格按元素顺序调用，application 保持严格左到右；
6. 为 `max/min/sum` 增加超过 `i128`、负数、空 List 和顺序测试；
7. 让中文名称从 Source 到 HIR、Diagnostic、Semantic JSON、Audit model 和 Eval 全链路可见；
8. 每个成功示例建立 `check`、`run`、`semantic` conformance fixture，并给出精确 stdout/exit code。

预期输出：

| 示例 | 预期 stdout |
| --- | --- |
| `examples/人物.ling` | `存活\n` |
| `examples/adt-match.ling` | `受伤 30\n` |
| `examples/pipeline.ling` | `9\n` |

出口：三个示例不含测试专用语法或 CLI 特判；删除 Capability、漏掉 ADT case、修改 immutable field 时均在执行前得到稳定诊断。

### P10：Semantic Schema 与 Audit Source

前置：G-12 为 `Accepted`。

目标：让 Semantic Graph 成为可验证协议，让 Audit Source 成为它的唯一确定性文本投影。

任务：

1. 为 `ling.semantic/0.1` 增加 reader 和结构验证；
2. 接受规范允许的未知扩展字段，拒绝错误 schema/version、非法 ID、重复 ID、悬空引用和错误 node kind；
3. 增加 alpha rename、依赖 body 变化、Capability/Effect 变化和路径不变性测试；
4. 在 `ling-semantic` 定义只包含已实现 Seed 字段的 `AuditModel`；未来字段不伪造默认语义；
5. 新建 `ling-format`，实现版本化 Audit lexer/parser/renderer；
6. renderer 使用唯一顺序和布局，明确显示完整类型、Effect Row、Capability、resolved ID、Unicode 安全元数据和实现状态；
7. 验证 `parse_audit(render_audit(model)) = model`，只忽略 G-12 明确列出的显示元数据；
8. `ling audit <file>` 复用现有 compile pipeline，成功时只向 stdout 输出 Audit 文本；
9. human/JSON 诊断共享根因 code，Audit parse error 不得变成 panic 或 `L-IMPL-*`；
10. 两个独立进程对同一程序产生逐字节相同的 Semantic JSON 和 Audit 文本。

出口：`audit examples/人物.ling` 返回 `0`；Schema 正/负兼容测试和 Audit round-trip 性质测试通过；解析后的 Audit model 不能绕过 checker 进入 evaluator。

### P11：事务式 REPL

前置：G-14 为 `Accepted`，P8 静态语义完成。

目标：提供可脚本测试的 REPL，同时避免维护第二套语言或半提交状态。

任务：

1. 提取并测试共享 compiler orchestration；
2. 实现 `Session` 状态：已接受 declarations、source identities、resolved definitions、Capability environment 和 evaluator values；
3. 每次 submission 在临时状态中完成完整检查，仅成功后原子提交；失败不得污染名称、类型、Capability 或运行时值；
4. 按 G-14 实现多行完成判定、空行、EOF、中断、重定义与 shadow；
5. confusable 检查必须覆盖历史 session scope；
6. REPL 不允许源码自行提升宿主 Capability；Capability 由启动配置注入；
7. human 模式的 prompt/结果/诊断和 JSON 模式事件遵循决议，脚本模式不得依赖 TTY；
8. 文件执行和 REPL 对同一表达式构造相同的 HIR/Typed Core/Semantic ID（排除决议声明的 session identity 差异）；
9. 添加进程级脚本 fixture：成功绑定、跨 submission 使用、多行函数、失败回滚、重定义、中文名称、运行 Fault 与 EOF。

出口：`ling repl` 不再返回 `L-IMPL-0001`；脚本化会话可重复、失败可回滚，且没有复制 Parser/Resolver/Type/Eval 规则。

### P12：硬化、追踪与发布候选

目标：关闭 [IMPLEMENTATION §10](IMPLEMENTATION.md#10-验收清单映射-rfc-18) 的证据缺口，形成可审查的 `v0.0.1` 候选 commit。

任务：

1. 建立规范条款 → conformance fixture → 实现路径的追踪矩阵；
2. 每条 Seed 规范至少具备正例、反例和适用的 Diagnostic/Graph/Audit 断言；
3. 增加性质测试：alpha-equivalence、Effect Row 交换/幂等、Audit round-trip、换行映射、type variable rename；
4. 若引入 `proptest` 等依赖，先更新 [DEPENDENCIES](DEPENDENCIES.md)，记录版本、许可证、MSRV、`unsafe` 与传递依赖；
5. 扩充 fuzz corpus，Linux CI 执行固定预算 smoke；Windows 本地缺少 ASan runtime 时保持明确记录，不降低 CI gate；
6. 在 Windows、Linux、macOS 执行 locked test、Clippy、Rustdoc 和 release build；
7. 完成直接/关键传递依赖许可证与 `unsafe` 审计；
8. 核对 README、错误码、CLI help、示例输出和未实现能力；
9. 生成发布候选报告，记录 commit SHA、CI run、toolchain、Schema、Unicode version 和已知限制；
10. 仅在所有门禁通过且得到单独明确授权后执行 commit、tag 或 push。

出口：所有阻断决议已接受，所有 Seed conformance 通过，工作区干净，远程三平台 CI 对同一 commit 全绿。

## 7. 依赖与执行顺序

```text
规范澄清 ──▶ P8 nominal/pattern ──▶ P9 examples/eval ──▶ P12 release
     │                │                    │                 ▲
     ├── G-12 ────────┴──────────────▶ P10 Audit ───────────┤
     └── G-14 ───────────────▶ P11 REPL ────────────────────┘
```

- P8 是所有后续阶段的共同基础；
- G-12 决议起草可与 P8 并行，P10 实现不能提前冻结 grammar；
- G-14 决议起草可与 P9/P10 并行，P11 必须复用已经稳定的静态语义；
- P12 的追踪矩阵可增量维护，但发布结论必须在 P8–P11 全部完成后生成。

建议按以下可独立评审的边界提交；本文不授权自动执行 Git 操作：

1. `docs(decision): define Seed Option and Result delivery`
2. `feat(types): complete nominal types and pattern exhaustiveness`
3. `test(conformance): add runnable Seed domain examples`
4. `feat(effects): propagate callback effects through map`
5. `docs(decision): define Audit Source grammar`
6. `feat(audit): add semantic reader and Audit round-trip`
7. `docs(decision): define transactional REPL sessions`
8. `feat(repl): reuse checked compiler pipeline in sessions`
9. `test(release): close Seed traceability and platform gates`

## 8. 验收矩阵

| 场景 | 命令/fixture | 预期 |
| --- | --- | --- |
| 人物检查 | `ling check examples/人物.ling` | 无 error，exit `0` |
| 人物运行 | `ling run examples/人物.ling` | stdout `存活\n`，exit `0` |
| ADT 运行 | `ling run examples/adt-match.ling` | stdout `受伤 30\n`，exit `0` |
| Pipeline 运行 | `ling run examples/pipeline.ling` | stdout `9\n`，exit `0` |
| record 缺字段 | conformance | 稳定 `L-TYPE-*`，不进入 evaluator |
| immutable field | RFC §13 fixture | `L-MUT-0001`，包含 field/mutability Facts |
| 非穷尽 ADT | conformance | 稳定 error 与最小缺失 witness |
| 不可达分支 | conformance | 稳定 warning，不改变执行语义 |
| guarded match | conformance | guard 为 Bool；guarded case 不证明覆盖 |
| callback Effect | `map Console.write ...` fixture | 推导 `Console.Write`；缺 Capability 时编译失败 |
| Semantic reader | schema fixtures | 允许未知扩展字段；拒绝坏版本、坏 ID、悬空引用 |
| Audit 确定性 | 两个独立进程 | stdout bytes 完全一致 |
| Audit round-trip | property/conformance | parse(render(model)) 与 model 等价 |
| REPL 回滚 | script fixture | 失败 submission 后旧状态不变、新绑定不可见 |
| REPL/Core 一致 | file + session pair | Type/Effect/结果一致，使用同一 core pipeline |
| 宿主写失败 | injected Console fixture | `L-RUNTIME-*`，exit `4`，不 panic |

## 9. 质量门禁

每个里程碑至少执行：

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
cargo doc --workspace --all-features --no-deps --locked --offline
cargo build --workspace --all-features --release --locked --offline
cargo run -p unicode-gen --locked --offline
git diff --exit-code -- crates/ling-unicode/src/generated.rs
cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline
```

附加门禁：

- 普通 build/test 在依赖已缓存后不得访问网络；
- JSON/Audit 测试断言协议字段和结构，不依赖 Rust `Debug`；
- 诊断测试断言 code、Span、severity 和稳定 Facts，不冻结可改善的全文；
- 公开输出中不得出现绝对路径、host 原始错误、内存地址或随机迭代顺序；
- 所有新依赖先审查后引入；
- 三平台 CI 和 fuzz smoke 必须对应同一候选 commit；
- 未实现功能继续显式失败，不得为通过示例添加特例。

## 10. 风险与缓解

| 风险 | 缓解 |
| --- | --- |
| nominal 泛型替换与 HM 变量混淆 | 使用不同 newtype/作用域表示 declaration parameter 与 inference variable；以 alpha-equivalence 测试 |
| exhaustiveness 范围膨胀 | 只实现 Bool 与 nominal variant；Int、List、guard completeness 明确后置 |
| constructor 名称空间与 import 冲突 | Resolver 单点定义规则；不在 Type Checker 进行字符串回退查找 |
| record literal 依赖全局字段猜测 | 优先使用期望 nominal type；无上下文时要求唯一候选，否则稳定报 ambiguous |
| `map` callback Effect 漏传 | 调用图记录高阶参数边；用 Console callback 和纯 callback 成对验证 |
| Audit 变成第二份 Semantic Graph | `AuditModel` 由 `ling-semantic` 生成；`ling-format` 只解析/渲染 |
| Audit parser 被误用为可信 Checked Core | 类型上隔离 `AuditModel` 与 `ProgramSnapshot`，不提供直接执行转换 |
| REPL 失败后残留半状态 | copy-on-write/临时 session transaction，只有完整检查成功才提交 |
| REPL 与文件模式形成两门语言 | 抽取共享编译 API，进程级 pair tests 验证一致性 |
| Schema 过早稳定 | 保持 `0.1` 和 experimental ID；不兼容变化升级版本并写迁移说明 |
| 发布门禁只在本机通过 | 候选 SHA 必须有 Windows/Linux/macOS 远程 CI 证据 |

## 11. 完成定义

本计划完成意味着 `v0.0.1 Seed` 功能和证据闭合，但仍不自动意味着已经发布。必须同时满足：

- [x] record、泛型 ADT、pattern typing 与预定义 `Option / Result` 已通过；
- [x] Bool/nominal variant 穷尽性与不可达分支诊断通过；
- [x] `人物`、ADT/match、pipeline 三个示例按精确输出运行；
- [x] `map` callback Effect/Capability 与严格顺序通过；
- [x] Semantic JSON reader、正负兼容性和独立进程确定性通过；
- [x] RFC §6.11 全节点类别、owner/source/target resolved ID 和 Audit round-trip 通过；
- [x] internal incident、snapshot mismatch 与 host fault 分别使用 exit `5`、`6`、`4`；
- [x] G-12 Accepted，Audit renderer/parser/round-trip 与 CLI 通过；
- [x] G-14 Accepted，REPL transaction、脚本模式和 Core 复用通过；
- [x] RFC §18 与 IMPLEMENTATION §10 的追踪矩阵已建立，所有未完成项均有明确阻断原因；
- [x] fmt、Clippy、tests、Rustdoc、release build、Unicode idempotence 与 fuzz target 编译已通过；实际 fuzz smoke 仍由 Ubuntu CI 验证；
- [ ] Windows、Linux、macOS 对同一候选 commit 的 CI 全绿；
- [x] 依赖、许可证、错误码、README、示例和已知限制已同步；
- [ ] 工作区干净，未实现能力无静默执行路径；
- [x] 未执行 tag/push；后续仍只在单独确认后执行。

完成上述条件后，下一阶段再评估 `v0.1 Living` 的 VM、增量编译、Formatter、LSP 与基础 Trait，不在本计划中提前实现。
