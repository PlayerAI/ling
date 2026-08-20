# Unified traceability registry template / 统一追踪注册表模板

`registry.toml` is the sole machine-readable source for active feature traceability. Generated release reports under this directory must not be edited by hand.

`registry.toml` 是当前 feature 追踪关系的唯一机器可读来源。本目录中的 release 报告均为生成文件，不得手工修改。

## Required chain / 必需证据链

Every public behavior must form this chain:

```text
feature_id
  → Requirement/Spec heading
  → indexed RFC/Accepted decision or specification
  → checked Core node or versioned schema
  → implementation symbol
  → positive and negative evidence
  → explicit differential state
  → release artifact
```

The matrix does not map individual source lines. It maps observable public behavior to reviewable, repeatable evidence.

矩阵不追踪每一行源码；它把可观察的公开行为映射到可审查、可重复的证据。

## Feature record / Feature 记录

```toml
[[feature]]
id = "FTR-AREA-0001"
title_zh = "简体中文标题"
title_en = "English title"
release = "v0.0.1"
scope = "Public"                 # Public | Internal
stability = "Experimental"      # Experimental | Preview | Stable
requirements = [
  { path = "docs/SPEC.md", clause = "1.2" },
]
authorities = ["DEC-0001"]       # IDs from docs/governance/authority.toml
core = [
  { path = "crates/example/src/lib.rs", symbol = "pub struct CheckedNode" },
]
implementation = [
  { path = "crates/example/src/lib.rs", symbol = "pub fn execute" },
]
release_artifacts = ["examples/example.ling"]
differential_state = "Deferred" # Covered | Deferred | NotApplicable
differential_reason = "Only one execution engine exists."
differential_tracking = ["docs/execution/task.md"]
```

All public features, including Experimental and Preview features, receive immutable `FTR-*` IDs. A Stable feature must never be represented only by prose.

所有公开 feature（包括 Experimental 与 Preview）都使用不可变的 `FTR-*` ID。Stable feature 禁止只以散文形式存在。

## Conformance fixture metadata / Conformance fixture 元数据

Every `tests/conformance/<case>/expect.toml` carries its own immutable ID and feature mapping:

```toml
test_id = "TEST-CONF-AREA-BEHAVIOR"
polarity = "Positive" # Positive | Negative
feature_ids = ["FTR-AREA-0001"]
```

Renaming or moving a fixture must not silently change `test_id`. Positive fixtures must exit successfully without diagnostics. Negative fixtures must expect either a non-zero exit or at least one registered diagnostic.

重命名或移动 fixture 时不得静默修改 `test_id`。正例必须成功退出且无诊断；反例必须期待非零退出或至少一个已注册诊断。

## Named evidence / 命名证据

Rust tests that cover behavior unavailable through the generic CLI fixture harness use an evidence record:

```toml
[[evidence]]
id = "EVD-POS-AREA-BEHAVIOR" # EVD-POS-* | EVD-NEG-* | EVD-DIFF-*
kind = "Positive"             # Positive | Negative | Differential
path = "crates/example/src/lib.rs"
symbol = "named_test_function"
feature_ids = ["FTR-AREA-0001"]
```

## Verification / 验证

```text
cargo xtask traceability verify --release v0.0.1
cargo xtask traceability render --release v0.0.1
```

`verify` rejects duplicate/unknown IDs, missing paths, missing headings or symbols, unindexed authorities, public features without both test polarities, malformed fixture metadata, unsupported differential claims, and generated-report drift. It performs no network access.

`verify` 会拒绝重复或未知 ID、缺失路径、缺失标题或 symbol、未登记的权威来源、缺少正反证据的公开 feature、错误 fixture 元数据、无依据的 differential 声明以及生成报告漂移；命令不访问网络。
