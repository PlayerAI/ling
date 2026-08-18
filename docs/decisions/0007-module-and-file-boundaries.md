# DEC-0007：Seed Module 与多文件边界

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-06

## 建议决议

### 编译单元

`ling check/run/semantic/audit <entry.ling>` 接受一个入口文件。入口文件所在目录是本次编译的 module root。v0.0.1 不搜索父目录、环境变量、包缓存或网络。

每个文件最多声明一次 `module`，且它必须是第一个非注释声明。入口文件可省略 module declaration，此时仅在该次编译中视为 `Main`；被 import 的文件必须显式声明 module。

### Import

Seed 只接受：

```ebnf
import_decl = "import", qualified_name, [ "as", identifier ];
```

Import 位于 module declaration 之后、普通 declaration 之前。`import Game.Math` 解析到 `<module-root>/Game/Math.ling`，默认别名是最后一段 `Math`；`as` 可指定别名。Import 只引入 module alias，不提供 glob、open、selective import 或隐式未限定名称。

源文件中的 module declaration 必须与 import 的 qualified name 完全一致。路径只用于加载和诊断，不进入名称相等、DefinitionId 或 BodyId。

### 确定性与错误

- 名称段按 NFC、大小写敏感比较；
- 即使宿主文件系统不区分大小写，也必须检查源码 module 名和路径段的精确大小写；
- 同一 compilation 中重复 module、重复 alias、找不到文件、声明不匹配和 import cycle 均为名称阶段错误；
- cycle 在 Seed 中一律拒绝，不区分 type/value cycle；
- module graph 按规范化 qualified name 排序后遍历，禁止依赖目录枚举顺序。

`check` 可以检查任意入口 module；只有 `run` 要求 G-15 规定的 `Main` 入口。

## 未选择方案

- **按工作目录隐式发现全部 `.ling` 文件**：结果依赖宿主目录内容，破坏可复现性；
- **文件名自动成为所有 module 的身份**：重命名路径会无意改变语义身份；
- **允许 cycle 并延迟到类型阶段**：Seed 没有递归 module 初始化规则；
- **将 import 成员注入当前作用域**：增加碰撞和来源不透明性。

## 验收证据

- 单文件显式/隐式 `Main`；
- alias import、缺失文件、声明不匹配、大小写不匹配；
- duplicate module/alias；
- 两节点和三节点 cycle；
- 不同目录枚举顺序产生相同 module graph。
