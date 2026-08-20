# G0：治理、兼容性与工程基础详细计划

> 发布块：G0  
> 目标：在扩大语言与工具表面积之前，建立规范缺口、公开协议、支持矩阵和可追踪交付机制。  
> 语义约束：本阶段不得通过实现新增语言语义。

## 1. 输入与出口

### 输入

- 已发布的 `v0.0.1 Seed`；
- `LANGUAGE.md`、`SEMANTICS.md`、`RFC-0001.md`；
- `ROADMAP-1.0.md`；
- Seed 的 conformance、Diagnostic JSON、Semantic Graph、CLI 行为。

### 出口

- 每个已知未决项都有状态、owner、阻断版本和所需证据；
- 每个公开协议都有版本字段、稳定级别和 reader/writer 规则；
- 1.0 支持矩阵草案可机器读取；
- 规范条款→实现→测试→产物的追踪机制进入 CI；
- 后续 G1～G6 可以复用同一门禁模板。

## 2. 目标目录

```text
docs/governance/
  authority.md
  gap-register.toml
  support-matrix.toml
  protocol-inventory.toml
  error-code-policy.md
  compatibility-policy.md
  release-states.md
rfcs/
  README.md
  template.md
decisions/
  README.md
  template.md
schemas/
  manifest.toml
docs/traceability/
  TEMPLATE.md
docs/status/
  implementation-status.toml
tools/xtask/src/
  governance.rs
```

## 3. 任务分解

### GOV-0101：建立规范权威索引

**规模：S；依赖：B00 Seed 基线。**

实施：

1. 创建 `docs/governance/authority.md`；
2. 列出 Accepted RFC、Draft RFC、Accepted decision、核心规范和 non-normative 文档；
3. 明确冲突处理与勘误流程；
4. 给每份规范文档加入机器可读 front matter 或旁车 manifest：`id/status/version/authority/depends_on`；
5. 实现 `cargo xtask governance check-authority`，检查链接、重复 ID、未知状态和循环依赖。

验收：

```bash
cargo xtask governance check-authority
```

必须能对故意制造的重复 RFC ID、断链和 Draft 被标为稳定依据的 fixture 报稳定错误。

非目标：不修改任何语言行为。

### GOV-0102：规范缺口台账

**规模：M；依赖：GOV-0101。**

`gap-register.toml` 每项至少包含：

```toml
[[gap]]
id = "GAP-TRAIT-COHERENCE-001"
title = "基础 Trait coherence 与 orphan rule"
status = "Open"
blocks = ["v0.1"]
authority = ["SEMANTICS.md#...", "ROADMAP-1.0.md#..."]
options = ["..."]
irreversible_consequences = ["..."]
required_evidence = ["positive", "negative", "cross-package", "migration"]
owner_role = "language-design"
```

实施：

1. 导入 `SEMANTICS §31` 全部未决项；
2. 导入 RFC-0001 后续 RFC 清单；
3. 扫描实现中 `TODO(spec)`、`UNSPECIFIED`、`experimental`；
4. 标注 G1 阻断项优先级；
5. 提供按 release/status 输出 Markdown 的生成器；
6. CI 拒绝没有 gap ID 的 `TODO(spec)`。

验收：每个 G1 规范门禁都能映射到 gap 或 Accepted decision。

### GOV-0103：RFC 与 decision 生命周期

**规模：S；依赖：GOV-0101。**

定义：

```text
Open → Draft → Proposed → Accepted / Rejected → Superseded
```

要求：

- Draft 不能成为 Stable 实现依据；
- Accepted 文档必须列出 conformance plan、compatibility impact 和 unresolved alternatives；
- Superseded 必须指向替代文档；
- 实验代码必须记录对应 Draft/Gap ID；
- 语言语义合并 PR 必须引用 Accepted ID。

创建 RFC 和 decision 模板，并由 CI 检查必需章节。

### GOV-0104：公开接口与协议总盘点

**规模：M；依赖：GOV-0101。**

在 `protocol-inventory.toml` 登记：

- CLI command/flag/exit code；
- human output（不承诺字节稳定，但承诺错误码和含义）；
- Diagnostic JSON；
- Semantic Graph JSON；
- Canonical Bytes / Semantic ID；
- Audit Source；
- Semantic Transaction；
- manifest/lock/build metadata；
- bytecode/replay/ABI/evidence（先标 Future）。

每项字段：

```text
id
current_version
stability
producer
consumer
canonical?
reader_policy
writer_policy
unknown_field_policy
migration_tool
fixtures
```

验收：`cargo xtask governance check-protocols` 能检测无版本公开 Schema 和 Stable 协议缺 fixture。

### GOV-0105：Diagnostic 错误码注册表

**规模：M；依赖：GOV-0104。**

建立分区：

```text
L0000-L0999  lexer/unicode
P0000-P0999  parser
R0000-R0999  resolve
T0000-T0999  type/trait
E0000-E0999  effect/capability
M0000-M0999  memory/ownership
C0000-C0999  concurrency
K0000-K0999  kernel/device
X0000-X0999  critical/contracts
B0000-B0999  build/package/protocol
I0000-I0999  internal/tooling
```

要求：

- code 永不复用为不同含义；
- 中英模板参数一致；
- 删除 code 进入 retired 列表；
- LSP `Diagnostic.code` 与 CLI JSON 使用同一 code；
- 修复建议使用结构化 `FixPlan`，不是只写自然语言；
- snapshot 只固定必要字段，避免把文案标点错误当协议。

### GOV-0106：Schema 生命周期与 golden corpus

**规模：L，拆 3 个 PR；依赖：GOV-0104。**

PR A：版本策略。

- major/minor 或整数 schema version 的含义；
- writer 只输出当前版本；
- reader 支持范围；
- unknown field；
- missing field；
- canonical encoding；
- hash scheme ID。

PR B：Schema manifest 与 fixtures。

```text
schemas/<name>/<version>/schema.json
schemas/<name>/<version>/valid/*.json
schemas/<name>/<version>/invalid/*.json
schemas/<name>/<version>/canonical/*.bin
```

PR C：兼容测试工具。

```bash
cargo xtask schema validate-all
cargo xtask schema compatibility --from N-1 --to N
cargo xtask schema corrupt-inputs
```

禁止用普通 JSON 序列化顺序直接作为 Semantic ID。

### GOV-0107：统一追踪矩阵

**规模：M；依赖：GOV-0102、GOV-0104。**

建立模板：

| Requirement/Spec | RFC/Decision | Core node/Schema | Implementation | Positive | Negative | Differential | Release artifact |
| --- | --- | --- | --- | --- | --- | --- | --- |

实施：

- 每个测试 fixture 带稳定 test ID；
- 每个 stable feature 有 `feature_id`；
- CI 检查链接存在；
- 人工审查语义覆盖；
- 生成 release evidence index。

不要求把每行代码映射到规范，但要求每个公开行为有证据链。

### GOV-0108：1.0 支持矩阵草案

**规模：M；依赖：GOV-0102。**

`support-matrix.toml` 最少包含：

```text
feature/profile/stability
host platform tier
native target tier
backend/device tier
standard package stability
protocol version
explicitly unsupported
```

初始原则：宁可标 Preview/Experimental，也不以“计划实现”冒充支持。

提供：

```bash
zero version --format json
zero support --format json
```

若命令尚未实现，先由 `xtask` 生成 fixtures，G1 CLI 再接入。

### GOV-0109：发布状态机器可读化

**规模：S；依赖：GOV-0108。**

创建 `implementation-status.toml`：

- feature 当前状态；
- implemented/tested/documented；
- blockers；
- last verified commit；
- supported profiles/targets。

文档网站、CLI、release notes 从该状态生成，避免三处不一致。

### GOV-0110：G0 CI 门禁

**规模：M；依赖：前述任务。**

新增 CI jobs：

```text
governance-authority
gap-register
protocol-schema
error-code-registry
traceability-links
support-matrix
canonical-determinism
seed-reproducibility
```

PR 触发规则：

- 修改规范必须运行 governance + conformance index；
- 修改 schema 必须运行兼容 corpus；
- 修改 diagnostic 必须运行中英文模板检查；
- 修改 canonical writer 必须跨两次随机 HashMap seed 比较 bytes。

## 4. G1 前必须关闭的决策

优先 RFC/decision：

1. Project/package identity、namespace、visibility、lock；
2. bytecode observable semantics、version/verifier；
3. Trait coherence/orphan 与 lowering；
4. incremental cache key、Semantic Hash upgrade；
5. Formatter preservation boundary；
6. LSP/Semantic Transaction stable vs experimental fields；
7. Ling/零、`.ling`、`zero`、`ling` languageId 的集中命名。

每个决策先写正例、反例和迁移影响，再批准实现。

## 5. G0 集成验收

```bash
cargo xtask governance check-all
cargo xtask schema validate-all
cargo xtask traceability verify --release v0.1
cargo xtask support render
cargo xtask seed reproduce
cargo test --workspace
```

退出清单：

- [ ] 所有 G1 门禁有 Accepted 文档或明确阻断；
- [ ] 所有公开 Schema/协议登记版本和 stability；
- [ ] 错误码无重复，中英模板参数一致；
- [ ] 1.0 支持矩阵草案可生成用户文档；
- [ ] Seed 基线可重复，canonical 输出无非确定来源；
- [ ] 后续 PR 模板强制填写追踪与兼容信息。
