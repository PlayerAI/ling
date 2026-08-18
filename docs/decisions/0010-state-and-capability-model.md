# DEC-0010：`State<T>` 与 Capability 模型

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-09

## 建议决议

### State Effect

每个合法 `<-` 表达式产生 `State<T>` Effect，其中 `T` 是被写 Place 的已解析 nominal/root 类型。Effect Row 中 label 排序、去重；`Pure` 仍严格等于空 Row。

`State<T>`：

- 出现在 Typed Core、function type、Semantic Graph 和 Audit Source；
- human 输出可折叠显示，但不得从 JSON/Graph/Audit 隐藏；
- 不要求 module Capability，因为它只操作当前 Checked Core 明确拥有的 local Place；
- 不包含宿主内存地址、arena index 或 Rust 类型名。

### Capability

Capability 同时具有两个一致视图：

1. 编译期：module `requires` 声明的静态授权集合；
2. 运行期：CLI 注入 evaluator 的不可伪造 host handle。

Effect 描述行为，Capability 描述授权。解析后的调用图计算最小 Capability 闭包；声明缺失是进入 evaluator 前的 `L-CAP-*` error，多余声明是 warning。Checked Core 保存已验证的 requirement，Evaluator 不能自行补授权。

### 宿主失败

宿主 Capability 操作失败是 Runtime Fault，不是编译错误、普通 Ling value 或 panic。它产生 `L-RUNTIME-*`，按 DEC-0013/G-15 使用 exit code `4`。诊断可以包含稳定的 host error category，但不得暴露依赖操作系统的原始错误文本为机器协议。

## 验收证据

- Effect Row 顺序不影响相等、Graph 或 hash input；
- local assignment 显示 `State<T>` 但不要求 module capability；
- `Console.Write` 缺失声明在执行前失败；
- unused Capability warning 基于同一调用图；
- 注入失败产生结构化 Fault 且不 panic。
