# 下一步实现计划：从 AST 到可运行 Hello World

> 状态：Hello World 纵向切片已在本地完成（实现记录 0.2）
> 日期：2026-08-18  
> 当前基线：从 Source 到 Checked Core、Semantic Snapshot 和 Interpreter 的文件执行主路径已实现
> 目标版本：`0.0.1-dev`  
> 规范依据：[RFC-0001](RFC-0001.md)、[SEMANTICS](SEMANTICS.md)、[IMPLEMENTATION](IMPLEMENTATION.md)  
> 地位：本文只定义工程顺序和验收证据，不创造语言语义；与 Accepted RFC 冲突时，以 Accepted RFC 为准。

---

## 1. 本轮目标

本轮以仓库中的 [`examples/hello.ling`](../examples/hello.ling) 为最小纵向切片，已完成以下可观察结果：

```text
cargo run --locked -- run examples/hello.ling
```

预期：

```text
你好，零
```

进程退出码必须为 `0`。实现路径必须完整经过：

```text
SourceFile
  → Lexer / Layout
  → CST
  → AST
  → Resolver
  → HIR Lowering
  → Type Checker
  → Effect / Capability Checker
  → Checked Typed Core
  → ProgramSnapshot
  → Interpreter
```

禁止为尽快打印字符串而在 CLI 中识别 `Console.write`、直接解释 AST，或绕过类型、Effect、Capability 和 Checked Core。Hello World 是第一条端到端验收用例，不是特殊执行路径。

## 2. 当前状态与已知缺口

### 2.1 实施前基线

- Rust workspace、统一 lint、锁定依赖和三平台 CI；
- 原始 UTF-8 byte span、BOM 和 LF/CRLF/CR 映射；
- Unicode 17.0.0 XID、NFC、Script Set、UTS #39 数据生成与官方数据回归；
- 双语 `ling.diagnostic/0.1`；
- Lexer、offside layout、有限恢复 Parser、lossless CST；
- 去 trivia、保留 Span 和标识符安全元数据的 AST；
- 最小 CLI 和 conformance runner；
- `examples/hello.ling` 当时仅可通过 AST Lowering，CLI 以 `L-IMPL-0001` 停在 `completed_stage = "ast"`。

### 2.2 当前实现结果

- 新增 HIR、Resolver、Type、Effect/Capability、Semantic Snapshot 和 Evaluator crate，并保持 Cargo 依赖无环；
- `check`、`run`、`semantic` 共用同一条真实语义管线，CLI 不含 Hello World 特判；
- module/import、词法作用域、HM 风格推导、保守 Value Restriction、局部可变性、`State<T>`、Capability 和 Seed 内置项按 Accepted 决议实现；
- Semantic ID 使用版本化 canonical bytes 和 BLAKE3，已由独立进程确定性测试及空白不变性测试覆盖；
- `examples/hello.ling` 的 `check` 返回 `0`，`run` 精确输出 `你好，零\n`，`semantic` 输出 `ling.semantic/0.1`；
- 后续 Seed 收口批次已完成完整 record/ADT、注入式 `Option`/`Result` Prelude、Audit round-trip 和事务式 REPL；详见 [`NEXT-STEPS-SEED.md`](NEXT-STEPS-SEED.md) 与接受的 DEC-0014～0016。

### 2.3 M2 验收债务（已关闭）

在进入语义实现前，先完成 M2 的剩余出口条件：

1. 为 CST/AST 建立稳定的测试投影和快照，快照不得包含 Rust `Debug` 格式、地址、HashMap 顺序或临时索引；
2. 建立 Source 任意 bytes、Lexer 合法 UTF-8、Parser 合法 UTF-8 三类 fuzz target；
3. 为最大 comment/layout/delimiter/parser 深度加入边界值测试；
4. 为 RFC 中已解锁示例建立 parser conformance fixture；
5. 明确 AST `TypeExpression` 和 `Pattern` 的 M2/M3 边界，拒绝结构残缺但当前可能被平面 token 表示接受的形式。

M2 债务可以与规范决议起草并行，但必须在 Resolver 接口冻结前完成。

### 2.4 阻断级规范问题（决议状态）

下表记录实施前的阻断点。G-06 至 G-12、G-14 和 G-15 均已形成 Accepted 决议并回填实现；该表保留为历史追踪，不表示仍有未决语义：

| 缺口 | 阻断阶段 | 本轮需要的最小决议 |
| --- | --- | --- |
| G-06 | Resolver | 文件到模块映射、单文件/多文件边界、重复模块、import、循环、`Main` 发现规则 |
| G-07 | Type | 可泛化表达式、`mutable`/Effect 对泛化的限制、对应诊断 |
| G-08 | HIR/Mutability | Seed 是否完全后置 Borrow；参数字段更新是否可能传播到调用方 |
| G-09 | Effect | `State<T>` 的语义可见性；Capability 的静态/运行时表示；宿主失败分类 |
| G-10 | Type/Effect/Eval | `Console.write` 等 Seed 内置项的规范名称、类型、Effect、Capability 和求值行为 |
| G-11 | Snapshot | Semantic ID、内容哈希、canonical bytes 和依赖变化传播 |
| G-12 | Audit | Audit grammar、显示元数据和 round-trip 等价关系 |
| G-15 | CLI/Eval | `main` 合法签名、返回类型、Effect 上限、运行失败到退出码/JSON 的映射 |

每个问题应形成独立 decision 文档，先为 `Proposed`，评审接受后改为 `Accepted`，并回填 [IMPLEMENTATION §6](IMPLEMENTATION.md#6-规范缺口清单)。Hello World 批次按此流程关闭 G-06 至 G-11、G-15；后续 Seed 收口批次通过 Accepted DEC-0015/0016 关闭 G-12/Audit 与 G-14/REPL。

## 3. 架构边界

### 3.1 依赖方向

新增 crate 必须保持以下单向依赖：

```text
ling-ast
    ▼
ling-hir
    ▼
ling-resolve
    ▼
ling-types
    ▼
ling-effects
    ├──▶ ling-semantic ──┐
    └──▶ ling-eval ──────┤
                        ▼
                     ling-cli
```

图表示编译数据流；Cargo 依赖必须按 [IMPLEMENTATION §3.1](IMPLEMENTATION.md#31-仓库布局) 的无环约束具体落地。基础 crate 不得依赖 CLI、文件系统编排或 evaluator。

### 3.2 核心中间表示

只建立当前验收所需的三层表示：

1. **AST**：对应 Author Source，保留语法来源 Span 和 Pipeline；
2. **Resolved HIR**：所有名称引用绑定到定义，Pipeline Lowering 为普通 application，Place 完成分类；
3. **Checked Typed Core**：只包含已完成名称、类型、可变性、Effect 和 Capability 检查的可执行节点。

不得再增加“临时 AST2”“半类型 HIR”或通用插件 IR。若某个阶段需要附加信息，优先使用该阶段的专用 side table，并由稳定 newtype ID 索引。

### 3.3 标识符身份

- 名称相等只使用 Lexer 已产生的 NFC `normalized` 名称；
- Source spelling 只用于显示和 Author Source；
- UTS #39 skeleton 只用于同一已解析作用域中的安全碰撞诊断；
- skeleton 不参与查找、哈希、Semantic ID 或自动改名；
- `ScopeId`、`DefinitionId`、`ReferenceId`、`TypeId` 使用不同 newtype，禁止裸 `usize` 跨阶段传递。

## 4. 分阶段实施

### P0：关闭 M2 验收债务

负责路径：`crates/ling-syntax`、`crates/ling-ast`、`tests/`、`.github/workflows/ci.yml`。

任务：

- 增加只表达公开语法结构的 CST/AST snapshot renderer；
- 为 Hello World、record、ADT/match、pipeline、assignment 建立快照；
- 建立 fuzz workspace 或标准 `cargo-fuzz` 目录，提交初始 corpus；
- PR CI 运行确定性的短 smoke corpus；长时间 fuzz 放在定期任务，不阻塞普通离线构建；
- 修正 Type/Pattern 的残缺语法接受问题；
- 将 `examples/hello.ling` 加入 parser conformance。

出口：M2 的所有出口条件有可执行测试证据；格式化、Clippy、tests、docs 和 Unicode generator idempotence 全部通过。

### P1：规范决议批次 A（G-06、G-07、G-08）

任务：

- 起草模块与文件边界决议；
- 起草 Seed Value Restriction 决议；
- 解决 Borrow 冲突，并明确 Seed 的参数/record 更新语义；
- 将接受后的规则更新到 RFC/SEMANTICS 示例、错误码需求和 conformance 计划。

出口：G-06 至 G-08 均有 Accepted 链接。未达到出口前，只允许做不带语义承诺的私有数据结构尖峰，不合并 Resolver/Type 行为。

### P2：Resolved HIR（`ling-hir`、`ling-resolve`）

#### `ling-hir`

最小职责：

- 定义 `HirProgram`、module、definition、expression、pattern 和 type syntax；
- 保存 Author Source Span，但不保存 trivia；
- 按 DEC-0004 将 `input |> f a` Lowering 为 `f a input`；
- 将 application 统一为函数 + 有序参数；
- 分类 `Place`：局部绑定、record 字段或非 Place；
- 不执行名称查找、类型推导或运行时求值。

#### `ling-resolve`

最小职责：

- 构造模块作用域和嵌套词法作用域；
- 使用规范化名称执行定义和引用解析；
- 实现 G-06 决议确定的 module/import 规则；
- 检测重复定义、未定义名称和非法阴影；
- 在同一作用域中检测不同规范化名称的相同 skeleton；
- 产出 `ResolvedProgram`，其中每个引用都有确定的 `DefinitionId`。

首批诊断类别：未定义名称、重复定义、非法 module/import、confusable collision。具体 code 只在实现对应错误时按 [错误码策略](decisions/0001-error-code-policy.md) 顺序分配，不预占号码。

测试：

- 中文名称、NFC 等价名称和大小写敏感；
- 局部阴影与跨作用域同名；
- Latin/Cyrillic skeleton 碰撞；
- 未定义 `Console` 与内置命名空间注入；
- Pipeline Lowering 的参数顺序和 Span 来源；
- G-06 规定的单文件、多文件、重复模块和循环用例。

出口：Hello World 中 `Main`、`main`、`Console.write` 全部解析到稳定定义；不存在字符串查找遗留到 Type Checker 或 Interpreter。

### P3：类型、Pattern 与可变性（`ling-types`）

最小类型集合：

```text
Unit | Bool | Int | f64 | Text
Tuple<T...>
Function<Params, Return, EffectRow>
NominalRecord<DefinitionId, Args...>
NominalVariant<DefinitionId, Args...>
TypeVariable
```

任务：

- 采用 arena + `TypeId` 表示类型，统一算法隐藏在 crate 内部；
- 实现 occurs check、substitution、instantiation、generalization；
- 按 G-07 实现 let 多态和 Value Restriction；
- 函数参数按源码顺序进行局部推导；
- 实现 nominal record 字段检查和 ADT constructor 类型；
- 实现 Bool/nominal variant 的基础穷尽性和不可达分支检查；
- 按 G-08 检查 `<-` 左侧 Place 和可变性；
- 引入经过依赖审查的任意精度整数实现，禁止先解析为 `i64`/`i128`；
- 产出包含类型 side table 的 `TypedProgram`，不在 AST/HIR 节点上使用可变全局状态回填。

性质测试：

- substitution 组合；
- instantiation 不共享新鲜变量；
- occurs check 拒绝无限类型；
- unification 在类型变量重命名意义下对称；
- generalization 只量化环境中不自由的变量；
- 超过 `i128` 的 `Int` 字面量不丢失。

出口：`Console.write "你好，零"` 推导为 `Unit`，`main` 的类型符合 G-15 决议；类型错误在 evaluator 前被拒绝。

### P4：Effect、Capability 与内置项（`ling-effects`）

前置：接受 G-09、G-10，并在需要运行入口检查前接受 G-15。

任务：

- Effect Row 使用排序、去重的规范表示；`Pure` 是空 Row；
- 将 `Console.write` 注册为规范内置定义，而不是 Lexer 关键字或 CLI 特例；
- 由已解析调用图推导 Effect；
- 检查 `module Main requires Console.Write`；
- 缺失 Capability 为编译错误，必须早于 evaluator；
- 按 G-09 表示局部 `State<T>`，并确保 human/JSON/Graph/Audit 使用同一数据；
- 产出不可绕过的 `CheckedProgram`/`CheckedCore` 构造边界。

建议封装：`CheckedProgram` 的字段私有，只有完成 Type、Place、Effect 和 Capability 检查的入口能构造。Interpreter API 只接受该类型。

出口：Hello World 的 `main` Effect 为 `{Console.Write}`，模块 Capability 闭包包含 `Console.Write`；删除 `requires` 后得到稳定的 Capability 诊断。

### P5：ProgramSnapshot 与最小 Semantic Graph（`ling-semantic`）

前置：接受 G-11。若本阶段同时交付 Audit，则还须接受 G-12；否则 `audit` 继续明确返回未实现，不伪造 round-trip。

任务：

- 定义 `ling.semantic/0.1` 常量和类型化 Schema；
- 记录语言版本、Unicode 版本、module、definition、resolved reference、type、Effect 和 Capability；
- 按 G-11 实现 experimental Semantic ID 和 canonical bytes；
- 所有集合使用确定性排序；
- `ProgramSnapshot` 由 Checked Core 和 Semantic Graph 共同构造；
- `run` 与 `semantic` 复用同一个 snapshot builder。

出口：同一 Hello World 在两个独立进程中产生逐字节相同的 Semantic JSON；空白和参数改名对 ID 的影响符合 G-11 决议。

### P6：Checked Core Interpreter（`ling-eval`）

任务：

- 只解释 `CheckedProgram`/`CheckedCore`；
- 实现 Unit、Bool、Int、f64、Text、tuple、closure、record、variant 和 Place Cell；
- 严格 call-by-value、左到右求值；
- 使用注入接口执行 Console Capability：

```text
trait Console {
    write(text) -> HostResult<Unit>
}
```

接口示意不冻结 Rust 签名；最终错误类型和同步模型必须符合 G-09/G-15。测试使用内存 Console，CLI 使用标准输出实现。

- 宿主 I/O 失败转换为 G-15 规定的结构化运行错误，禁止 panic；
- `main` 查找和调用完全遵守 G-15，不在 CLI 中猜测入口；
- 保证 interpreter 内部 arena、`Arc` 或容器顺序不可被 Ling 程序观察。

出口：内存 Console 测试捕获精确字符串 `你好，零\n`；实际 CLI 输出相同内容并退出 `0`。

### P7：CLI 编排与 conformance 收口（`ling-cli`）

将当前单函数 pipeline 拆分为可测试的编译会话，但不引入服务容器或插件框架：

```text
load → parse → lower → resolve → type → effect/capability
     → snapshot → evaluate
```

命令行为：

- `check`：完成 Checked Core 和 snapshot 验证，不执行 `main`，成功退出 `0`；
- `run`：复用 `check` 结果后执行入口；
- `semantic`：输出确定性 Semantic JSON；
- `audit`：只有 G-12 round-trip 完成后才启用；
- `repl`：只有 G-14 接受后才启用。

`L-IMPL-0001` 随真实阶段接管逐步后移。某条路径实现后必须删除该路径的占位错误，不能让成功命令继续返回编译错误。

出口：新增端到端 fixture 覆盖成功、语法错误、名称错误、类型错误、Capability 缺失和宿主运行错误；human 与 JSON 共享同一根因 code。

## 5. Hello World 验收矩阵

| 场景 | 命令 | 预期 |
| --- | --- | --- |
| 检查成功 | `ling check examples/hello.ling` | stdout/stderr 为空，exit `0` |
| 运行成功 | `ling run examples/hello.ling` | stdout 为 `你好，零\n`，stderr 为空，exit `0` |
| Semantic 输出 | `ling semantic examples/hello.ling` | 符合 `ling.semantic/0.1`，两进程 bytes 相同 |
| 缺少 Capability | 删除 `requires Console.Write` 后 `check` | 稳定 `L-CAP-*`，exit `1`，不执行 Console |
| 参数类型错误 | `Console.write 1` | 稳定 `L-TYPE-*`，exit `1` |
| 名称错误 | `Console.writ` | 稳定 `L-NAME-*`，exit `1` |
| 宿主写入失败 | 注入失败 Console | 稳定 `L-RUNTIME-*` 与 G-15 指定退出码 |
| 非法入口 | `main` 签名不符合 G-15 | 编译期稳定诊断，不进入 evaluator |

## 6. 测试与质量门禁

每个阶段合并前执行：

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo doc --workspace --all-features --no-deps --locked
cargo run -p unicode-gen --locked
git diff --exit-code -- crates/ling-unicode/src/generated.rs
```

附加要求：

- 普通 build/test 必须离线可运行；
- 新依赖先更新 `docs/DEPENDENCIES.md`，记录版本、许可证、MSRV、`unsafe` 和传递依赖；
- conformance fixture 必须声明规范条款，不用快照替代语义断言；
- 诊断测试断言 code、Span 和稳定 Facts，不固定可改善的自然语言全文；
- 不以 Rust `Debug` 输出作为公开 JSON、Semantic ID 或测试协议；
- Windows、Linux、macOS CI 均已配置；远程三平台运行结果必须在首次推送后确认，确认前不得宣称跨平台门禁已通过。

## 7. 建议提交顺序

每项应保持可独立评审和回退；此处只定义边界，不授权自动执行 `git commit` 或 `git push`。

1. `test(syntax): close M2 snapshots and fuzz coverage`
2. `docs(decisions): resolve G-06 through G-08`
3. `feat(resolve): add HIR lowering and lexical name resolution`
4. `feat(types): add Seed HM inference and mutability checks`
5. `docs(decisions): resolve G-09, G-10, and G-15`
6. `feat(effects): check Seed effects and capabilities`
7. `docs(decisions): resolve G-11 and snapshot identity`
8. `feat(semantic): build deterministic ProgramSnapshot`
9. `feat(eval): execute checked Seed core with injected Console`
10. `feat(cli): make hello check/run/semantic conformance pass`

G-12/Audit 和 G-14/REPL 可在 Hello World 文件执行主路径完成后进入下一批次，避免同时冻结三个面向用户的协议。

## 8. 完成定义

本计划的“完成”只表示 Hello World 纵向切片完成，不等于 `v0.0.1 Seed` 发布完成。必须同时满足：

- [x] M2 验收债务关闭；
- [x] G-06 至 G-11、G-15 有 Accepted 决议并与实现一致；
- [x] `examples/hello.ling` 不含特殊标记或测试专用语法；
- [x] Resolver、Type、Effect、Capability、Snapshot、Interpreter 均为真实执行路径；
- [x] `check` 成功返回 `0`；
- [x] `run` 输出 `你好，零` 并返回 `0`；
- [x] 缺 Capability、类型错误和名称错误在执行前被拒绝；
- [x] Semantic JSON 可复现并通过 Schema 测试；
- [ ] 全 workspace fmt、Clippy、tests、docs、Unicode idempotence 和三平台 CI 通过：本地门禁已通过，远程三平台 CI 待首次推送后确认；
- [x] README 的状态、命令示例和未实现能力与实际行为一致。

后续工作由 [`NEXT-STEPS-SEED.md`](NEXT-STEPS-SEED.md) 接续，处理完整 record/ADT 示例、Audit round-trip、REPL 会话和 `v0.0.1` 全部发布门禁。
