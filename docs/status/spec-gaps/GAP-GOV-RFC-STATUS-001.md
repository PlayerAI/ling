# GAP-GOV-RFC-STATUS-001：RFC-0001 生命周期状态不一致

> 发现时状态：Open（当前状态以机器台账为准）
> 发现任务：GOV-0101
> 发现日期：2026-08-20
> 类型：Governance / specification lifecycle
> 语言行为影响：无直接变更
> 机器台账：[`docs/governance/gap-register.toml`](../../governance/gap-register.toml) 中同名 gap；本文件保留发现证据，不是第二份状态权威

## 触发条件

[`RFC-0001.md`](../../RFC-0001.md) 的源文件元数据明确写为 `Draft`，但根仓库指导的旧扩展段和 BASE-0001 盘点曾将它描述为 `Accepted`。已发布的 `v0.0.1` tag 只能证明发布动作，不能自动改变 RFC 生命周期。

## 缺失或冲突

仓库缺少一份可验证的 RFC-0001 接受记录，或者缺少将其保持 Draft 的明确治理结论。因此：

- RFC-0001 不得在权威索引中标为 Accepted 或 Stable basis；
- 不得通过修改实现、快照或任务状态来隐式决定 RFC 生命周期；
- `SEMANTICS.md`、`LANGUAGE.md`、Accepted decisions 和 conformance corpus 仍按根 `AGENTS.md` 的既有顺序适用。

## 可观察影响

本缺口不改变现有 Ling 程序的语法、类型、求值、诊断或 CLI 行为。它影响的是实现依据的可审计性：若后续任务只能引用 RFC-0001 才能冻结公开行为，该任务必须等待 RFC 生命周期明确；若更高优先级的现有规范或 Accepted decision 已完整决定行为，可继续按那些依据执行。

## 受影响工作

- 所有把 RFC-0001 直接称为 Accepted 的治理或发布文档；
- 需要以 Accepted RFC 扩展 Seed 范围的 G1 及后续任务；
- RFC/decision 生命周期检查与后续规范追踪。

## 候选处理（不构成决议）

1. 完成正式评审并将 RFC-0001 明确接受，同时记录接受日期和接受依据；
2. 明确保留 Draft 状态，修正所有把它称为 Accepted 的引用；
3. 在接受前修订或拆分 RFC-0001，并为兼容性影响提供迁移说明。

## 需要的 RFC/decision

需要项目维护者通过受审查的 RFC 生命周期变更或独立治理 decision 明确 RFC-0001 的状态。GOV-0101 只登记事实，不选择候选方案。

## 暂停边界

暂停任何“仅因 RFC-0001 已 Accepted”才被视为获得授权的语义或公开协议工作。治理索引、缺口登记、只读审计和不改变语言行为的工具工作可以继续。
