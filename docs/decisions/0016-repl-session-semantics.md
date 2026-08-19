# DEC-0016：Seed REPL 会话与事务语义

> 状态：Accepted
> 提出日期：2026-08-18
> 接受日期：2026-08-19
> 关闭缺口：G-14

## 背景

REPL 必须复用文件模式的 Parser、Resolver、Type、Effect/Capability、Semantic 和 Eval Core，同时需要定义 submission 边界、失败回滚、重定义和可脚本测试的输出协议。

## 建议决议

### 输入与完成判定

- REPL 的逻辑 module 固定为 `Main`；启动时由宿主配置 Capability，submission 不能声明或提升 Capability；
- 空行在 delimiter/layout 均闭合时提交当前 buffer；EOF 在空 buffer 时正常退出，在非空且完整时提交后退出，在不完整时产生语法诊断；
- human TTY 模式可显示 prompt，非 TTY 脚本模式不输出 prompt；
- `Ctrl-C` 清空未提交 buffer，不回滚已提交状态；连续 EOF 终止进程。

### 会话状态与重定义

- 每个 submission 在上一已提交状态的不可变快照上编译和求值；只有完整 check 与求值成功后才原子提交；
- parse/name/type/effect/capability/runtime 任一失败均回滚该 submission 的全部 definition 和 value；Console 等已经发生的外部输出不能伪装回滚，诊断必须标记此次 submission 未提交；
- 顶层 `let` 可在后续 submission 中重定义同名 value；旧 closure 仍引用旧 DefinitionId/value，新引用解析到最新 generation；
- type definition 和 module/import 在 Seed REPL 中不允许重定义；需要重启 session 才能改变；
- confusable collision 对当前可见 generation 和同一 submission 生效；被明确重定义替代的旧 generation 不参与后续可见作用域碰撞；
- generation 是 session identity 的一部分，不改变文件模式 DEC-0012 DefinitionId 规则。REPL ID 使用独立 `ling.repl-definition-id/v1` domain。

### 输出协议

- expression submission 成功时 human 模式输出规范 value 与完整类型；Unit 不输出 value 行；
- declaration submission 成功时 human 模式输出 `name : type`；
- JSON 模式每个 submission 输出一个 `ling.repl/0.1` JSON object，包含 `status`、`committed`、`submission`、可选 `name/type/value/effects` 或 `diagnostics`；
- Console output 仍写 stdout。为避免与 JSON event 混流，`--format json` 的 Console output 表示为 `console` event，不直接写裸文本；
- 正常 EOF 返回 `0`；invalid usage 返回 `2`；编译失败不会终止交互式会话，脚本模式处理全部输入后若存在失败返回 `1`；未捕获 runtime fault 返回 `4`。

## 实现约束

- 文件与 REPL 必须调用同一编译 orchestration 和 Checked Core evaluator；
- session transaction 不复制名称、类型或 Effect 规则；
- 不把字符串拼接成特殊 AST 节点来绕过 parser；允许用规范 module wrapper 表示完整 submission，但 wrapper 映射必须保留 Source Span；
- session host handles 不进入 Semantic ID 或序列化状态。

## 验收证据

- 成功 binding 可在下一 submission 使用；
- 编译失败和 runtime fault 后新 binding 不可见，旧 binding/value 保持；
- 重定义后新引用得到新 generation，旧 closure 行为不变；
- 多行函数、中文名称、confusable、EOF 和中断有进程级 fixture；
- file/session pair 产生等价 Type/Effect/Value，排除已声明的 session identity；
- human 与 JSON 脚本模式输出确定且不依赖 TTY。
