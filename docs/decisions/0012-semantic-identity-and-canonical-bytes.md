# DEC-0012：Semantic Identity 与 Canonical Bytes

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-11

## 建议决议

Seed 分离逻辑位置身份与内容身份：

- `DefinitionId`：definition kind + NFC module qualified name + NFC definition name；
- `BodyId`：alpha-normalized Checked Typed Core 的内容 hash；
- `ProgramId`：语言/Schema/Unicode 版本及排序后的 module/DefinitionId/BodyId 集合 hash。

所有公开 ID 使用：

```text
experimental:blake3:<64 lowercase hex digits>
```

不同 ID 类使用不同 Rust newtype，canonical input 具有不同 domain separator。不得把 ID 当无类型 String 互换。

### Canonical bytes

Hash 输入是自定义的版本化 length-prefixed binary encoding，不是 JSON、Rust `Debug`、HashMap iteration 或 source bytes。每个输入以 ASCII domain、语言版本和 Semantic Schema 版本开始。整数使用规定宽度的 big-endian length/value encoding；集合先按各元素 canonical bytes 排序。

BodyId：

- 包含已解析 operator、literal canonical value、type、Effect、Capability 和被引用的 DefinitionId；
- 对参数/local 名称使用首次绑定顺序的 De Bruijn-like index；
- 排除 Source Span、文件路径、注释、空白、原始 identifier spelling 和 arena index；
- 被调用 definition 的 BodyId 改变时，caller BodyId 不级联改变；依赖 edge 和 ProgramId 负责增量失效。

Definition 重命名或移动 module 会改变 DefinitionId。Seed 不承诺跨重命名持久身份；未来若引入显式 durable ID，必须升级 Schema。

互递归 CycleId 不进入 Seed 实现。G-06 已禁止 module cycle；value-level `let rec ... and ...` 在 CycleId RFC 前明确拒绝多定义互递归。

### 迁移

算法、encoding 或归一化规则变化必须升级 Semantic Schema 或 ID 前缀版本，并提供迁移说明。`experimental` 不代表可以静默改变同一版本的 hash。

## 验收证据

- 空白、注释、CRLF/LF、参数/local alpha rename 不改变 BodyId；
- literal、operator、type、Effect 或 Capability 改变会改变 BodyId；
- dependency body change 不改变 caller BodyId，但改变 ProgramId；
- 两个独立进程产生逐字节相同的 Graph JSON 和 ID；
- 路径和 HashMap random seed 不影响结果。
