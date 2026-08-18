# DEC-0001：错误码分配策略

> 状态：Accepted  
> 日期：2026-08-18  
> 关闭缺口：G-01

## 决议

错误码采用 `L-<DOMAIN>-<NUMBER>`，domain 表示最早能够确定根因的编译阶段，而不是最终输出命令。初始 domain 为：

```text
IO        宿主输入输出
LEX       解码、词法与字符级合法性
SYNTAX    layout、parser 与 CST
NAME      名称解析与作用域安全
TYPE      类型推导与检查
MUT       Place 与可变性
EFFECT    Effect 推导与检查
CAP       Capability 要求
SEMANTIC  Semantic Graph、ID 与 Audit
RUNTIME   已检查 Core 的运行错误
IMPL      预发布阶段明确拒绝尚未实现的路径
INTERNAL  编译器缺陷；同时生成 incident ID
```

每个 domain 独立从 `0001` 单调分配。号码不编码严重程度、文件、命令或语言。`docs/ERROR-CODES.md` 是唯一注册表；删除功能时保留 code 并标记 deprecated，不回收号码。

自然语言消息可以改善，但 code 的根因含义保持稳定。Facts 字段名称与类型属于机器协议；新增可选字段兼容，删除或改型不兼容。Repair 必须声明是否可能改变语义。

## 理由

按根因阶段分配使同一错误在 `check`、`run`、REPL 和 JSON 输出中共享身份，也避免把本地化文本或内部 Rust 类型变成协议。

`IMPL` 仅用于尚未发布的显式拒绝路径。它不能让占位实现通过验收；真正实现阶段接管该路径后必须删除对应 `L-IMPL-0001` 输出。

