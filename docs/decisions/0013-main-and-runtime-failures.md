# DEC-0013：`main` 与运行错误

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-15

## 建议决议

### 入口

`ling run <entry.ling>` 按 DEC-0007 编译入口 module，并要求其规范 module name 为 `Main`。显式 `module Main` 与入口文件省略 module declaration 的 implicit `Main` 等价。

`Main` 必须有且只有一个 module-scope `main`，其签名为：

```text
Unit -> Unit ! ε
```

Author Source 必须表现为一个 Unit pattern 参数，例如 `let main () = ...`；普通值 binding `let main = ...` 不作为入口。`ε` 在 Seed 只可包含已实现的 `State<T>` 和 `Console.Write`，且所有外部 Effect 必须由 module Capability 声明覆盖。

Seed 暂不接受 `Result` 返回入口；exit code `3` 保留给未来接受的 Result-main RFC，不伪造支持。

### 执行顺序

`run` 固定执行：load → parse → resolve → type/place → effect/capability → snapshot → locate main → evaluate。`check` 执行同一路径但不 locate/evaluate main，因此 library module 可被独立检查。

### 退出码

接受 RFC §15 的编号：

```text
0 success
1 compile/check error
2 invalid CLI usage
3 runtime Result error（Seed 保留，当前不可达）
4 runtime Fault / host Capability failure
5 internal compiler error
6 semantic snapshot mismatch
```

Runtime Fault 产生稳定 `L-RUNTIME-*` code 和结构化 category。Internal error 产生 `L-INTERNAL-*` 及 incident ID；两者不得伪装成用户 Type/Capability 错误。Human/JSON 只改变渲染，不改变 exit code。

### Console 输出

Evaluator 的 Console 是注入接口。CLI 实现写 stdout，diagnostic 写 stderr。成功 Hello World 的 stdout 精确为 UTF-8 `你好，零\n`，stderr 为空。

## 验收证据

- explicit/implicit Main 成功；
- missing、duplicate、值 binding main 和错误签名均在执行前失败；
- `check` 不执行 Console；
- compile error、host failure、internal error 和 snapshot mismatch 使用不同退出码；
- memory Console 与 CLI stdout 使用相同 Checked Core 路径。
