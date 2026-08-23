# Ling Tutorial / Ling 教程

Status: Seed bilingual tutorial evidence (2026-08-22). This tutorial is a
copyable introduction to the implemented v0.0.1 Seed boundary. It does not
define new syntax or promise a 1.0 Stable feature.

## Authority and scope / 权威与范围

The language and semantics references, accepted Seed decisions, conformance
fixtures, and the checked implementation are authoritative. This tutorial is
explanatory material below those sources. The current support matrix marks the
Seed features as `Experimental`; Semantic Graph and Audit Source are
Experimental/Preview protocols, and no Profile selector or ownership checker
is available.

语言与语义参考、已接受的 Seed 决议、一致性测试以及经过检查的实现具有更高
权威。本教程只是面向使用者的说明，不新增语法，也不把当前 Seed 宣称为 1.0
Stable。当前支持矩阵将 Seed 能力标记为 `Experimental`；Semantic Graph 与
Audit Source 属于 Experimental/Preview 协议，Profile 选择器和 ownership
checker 尚未提供。

## 1. Install-free first run / 无安装首个运行

From the repository root, use the locked offline commands:

```text
cargo run --locked --offline -- check examples/hello.ling
cargo run --locked --offline -- run examples/hello.ling
cargo run --locked --offline -- semantic examples/hello.ling
```

在仓库根目录执行锁定且离线的命令：

```text
cargo run --locked --offline -- check examples/hello.ling
cargo run --locked --offline -- run examples/hello.ling
cargo run --locked --offline -- semantic examples/hello.ling
```

`check` succeeds without stdout/stderr. `run` prints `你好，零` followed by a
line feed. `semantic` emits the versioned `ling.semantic/0.1` JSON protocol;
its Experimental Semantic IDs are not compatibility values.

`check` 成功时不输出 stdout/stderr；`run` 输出 `你好，零` 和换行；`semantic`
输出带版本的 `ling.semantic/0.1` JSON 协议。其中 Experimental Semantic ID
不是兼容性承诺的一部分。

## 2. Chinese-first tutorial / 中文优先教程

The complete runnable source is [`examples/tutorial-zh.ling`](../examples/tutorial-zh.ling):

```ling
type 人物 =
    { 姓名: Text
      mutable 血量: Int
      最大血量: Int }

let 受到伤害 伤害 人物 =
    { 人物 with
        血量 = max 0 (人物.血量 - 伤害) }

let 状态文字 人物 =
    if 人物.血量 == 0 then
        "死亡"
    else
        "存活"
```

This example uses domain terms naturally: `人物` is the record, `血量` is a
mutable field, and `受到伤害` describes the operation. The update expression
creates a record value with the changed field; the `mutable` declaration makes
the later place assignment legal. The module declares `Console.Write`, which
is both the static Capability requirement and the observed `Console.Write`
Effect.

完整的可运行源码见
[`examples/tutorial-zh.ling`](../examples/tutorial-zh.ling)。示例自然使用
`人物`、`血量`、`受到伤害` 等领域术语：record 更新产生新的 record 值，字段的
`mutable` 声明使后续 place assignment 合法；module 的 `requires Console.Write`
同时表达静态 Capability 要求和实际的 `Console.Write` Effect。

Run it with:

```text
cargo run --locked --offline -- check examples/tutorial-zh.ling
cargo run --locked --offline -- run examples/tutorial-zh.ling
cargo run --locked --offline -- semantic examples/tutorial-zh.ling
cargo run --locked --offline -- audit examples/tutorial-zh.ling
```

Expected output is `存活` plus a line feed. Audit output starts with
`audit ling.audit/0.1` for ordinary Seed input (`ling.audit/0.2` when checked
Handler evidence is present) and reports Unicode `17.0.0`.

运行命令如上；预期输出为 `存活` 和换行。Audit 输出以
普通 Seed 输入以 `audit ling.audit/0.1` 开始（含 checked Handler 证据时使用
`ling.audit/0.2`），并报告 Unicode `17.0.0`。

## 3. Equivalent English tutorial / 等价英文教程

The English source is [`examples/tutorial-en.ling`](../examples/tutorial-en.ling).
The identifiers are intentionally idiomatic English rather than a mechanical
translation of each spelling:

```ling
type Person =
    { name: Text
      mutable health: Int
      maxHealth: Int }

let takeDamage damage person =
    { person with
        health = max 0 (person.health - damage) }

let statusText person =
    if person.health == 0 then
        "dead"
    else
        "alive"
```

The declaration structure and checked semantics are equivalent to the Chinese
tutorial, while names and user-facing text fit an English domain vocabulary.
The process test proves this by comparing their version/entry facts, module
requirements, definition and node kind/type/effect/capability shapes, and
reference topology after normalizing only the user nominal type spelling. It
does not compare localized names, text, spans, source evidence, or Experimental
IDs. The English source prints `alive` followed by a line feed:

```text
cargo run --locked --offline -- check examples/tutorial-en.ling
cargo run --locked --offline -- run examples/tutorial-en.ling
cargo run --locked --offline -- semantic examples/tutorial-en.ling
```

中文教程和英文教程在声明结构与检查后语义上等价，但标识符和文本分别符合各自
的领域表达，而不是逐字翻译。英文版本输出 `alive` 和换行。

## 4. Correct errors / 正确错误

Do not remove `requires Console.Write` to “make the example simpler”. The
compiler must reject a program that calls `Console.write` without that
Capability. The registered fixture
[`p7-missing-capability`](../tests/conformance/p7-missing-capability/expect.toml)
is the copyable negative example; it asserts a bilingual registered
`L-<DOMAIN>-<NUMBER>` diagnostic and the documented exit behavior.

不要为了“简化”示例而删除 `requires Console.Write`。调用 `Console.write` 却没有
Capability 时，编译器必须拒绝程序。可复制的反例是
[`p7-missing-capability`](../tests/conformance/p7-missing-capability/expect.toml)，
它检查双语注册错误码和文档规定的退出行为。

Other negative fixtures cover invalid entry points, type errors, immutable
fields, non-exhaustive matches, mixed scripts, malformed literals, and runtime
format faults. They are the authoritative error examples; tutorial prose does
not invent new diagnostic codes.

其他反例覆盖入口、类型、不可变字段、非穷尽 match、混合脚本、非法字面量和运行
时格式 Fault。它们才是权威错误示例，教程不会自行发明诊断码。

## 5. Boundaries / 边界

- `Console.Write` is the only Seed host capability used here; networking,
  files, time, and randomness are outside the Seed tutorial.
- There is no selectable Profile, Native/FFI ABI, ownership/borrow checker,
  Task/Actor runtime, package registry, LSP server, or Zed language server in
  this tutorial.
- Semantic/Audit output is useful for inspection and AI tooling but remains
  Experimental/Preview; do not persist its IDs as a Stable interface.
- Preserve original UTF-8 byte spans and Unicode 17.0.0 behavior when adapting
  examples. Keep normal builds and tests `--locked --offline`.

- 本教程只使用 Seed 的 `Console.Write` host capability；网络、文件、时间和随机数
  不在 Seed 范围内。
- 本教程不提供可选 Profile、Native/FFI ABI、ownership/borrow checker、
  Task/Actor runtime、包注册表、LSP 或 Zed language server。
- Semantic/Audit 输出便于检查和 AI 工具使用，但仍是 Experimental/Preview；
  不要把其中 ID 当作 Stable 接口持久化。
- 改写示例时保留原始 UTF-8 byte span 与 Unicode 17.0.0 行为；普通构建和测试
  使用 `--locked --offline`。

## Verification / 验证

```text
cargo test -p ling-cli --test conformance seed_examples_check_run_and_emit_semantic_graphs --locked --offline
cargo test -p ling-cli --test conformance audit_output_is_deterministic_and_round_trips --locked --offline
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
cargo xtask tutorial verify
```

These commands validate the runnable example matrix, bilingual tutorial
Semantic-shape equivalence, canonical Audit output, the complete Seed
traceability registry, and tutorial inventory drift. They do not promote any
feature to Stable or authorize future syntax.

这些命令验证可运行示例矩阵、规范 Audit 输出和完整 Seed 追踪注册表，但不会把
教程清单漂移误报为实现完成；它们不会把任何能力提升为 Stable，也不会授权未来
语法。
