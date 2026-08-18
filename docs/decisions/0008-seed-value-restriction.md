# DEC-0008：Seed Value Restriction

> 状态：Accepted
> 日期：2026-08-18
> 关闭缺口：G-07

## 建议决议

Seed 使用保守的句法 Value Restriction。只有不可变 binding 且 RHS 是下列 non-expansive value 时，才泛化不在环境中自由的类型变量：

- 带至少一个参数的函数 binding；
- Unit、Bool、Int、f64、Text literal；
- 已解析 immutable name；
- 元素均为 non-expansive value 的 tuple/list；
- 所有字段均为 non-expansive value，且 nominal record 类型不含 `mutable` 字段的 record；
- payload 均为 non-expansive value 的 variant constructor application。

以下 binding 不泛化：

- `let mutable`；
- application、pipeline、if、match、运算、assignment；
- 含 mutable field 的 record value；
- Effect Row 非空的表达式；
- 未来可能创建 Managed/Resource identity 的表达式。

不泛化本身不是错误，binding 获得包含单态变量的 scheme。若后续使用因该限制产生类型冲突，主错误仍是类型不统一，并附加稳定 Facts：

```text
generalization = "restricted"
restriction_reason = "mutable_binding" | "expansive_rhs" |
                     "mutable_field" | "effectful_rhs"
```

Seed 不提供显式 `forall` 或绕过 restriction 的类型注解。

## 理由

SEMANTICS §6.4 允许“句法值”或“已证明 Pure 且不创建 identity”。在 effect/type pipeline 完全建立前，尝试证明任意计算表达式 Pure 会形成泛化与 Effect 推导的循环依赖。保守规则保持类型安全，后续 RFC 可以扩大 non-expansive 集合而不让旧程序改变含义。

## 验收证据

- polymorphic identity 在 Int/Text 两处实例化成功；
- mutable binding 不泛化；
- effectful application 不泛化；
- immutable tuple/list/variant value 正确泛化；
- occurs check 与 restriction Facts 稳定。
