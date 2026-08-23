# DEC-0247: Current RC3 upstream evidence / 当前 RC3 上游证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：release engineering
> 相关 RFC/缺口：DEC-0054 | DEC-0246 | RC-6903
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes composing the current RC1→RC0 bounded inventory chain
from the RC3 readiness gate. It does not authorize or perform independent
verification.

本决定授权 RC3 就绪门禁组合当前 RC1→RC0 有限盘点链；它不授权也不执行独立验证。

## Question

How should RC3 prove that its upstream readiness inventories are current while
continuing to distinguish implementation-agent self-validation from an
independent reviewer sign-off?

## Decision

1. Amend the bounded DEC-0054 gate by calling
   `rc1_validation::check_repository` from `cargo xtask rc3 verify`; the RC1
   gate already composes the current RC0 gate.
2. Require the RC3 readiness document to state that the bounded upstream gates
   pass, both parent release gates remain `BlockedSpec`, and this does not
   constitute independent verification.
3. Fail closed with internal `GOV-RC3-VERIFICATION-0011` errors when those
   current-boundary markers drift.
4. Keep all seven RC3 check states unchanged and preserve the reviewer,
   candidate, artifact, reproduction, retention, and Go/No-Go requirements.
5. The verifier remains deterministic, read-only, and offline. It neither
   creates nor contacts a reviewer, tag, artifact, signature, service, network
   resource, or system configuration.

## Conformance plan

- Run `cargo xtask rc3 verify` and require seven checks, seven audit files, and
  one composed upstream gate.
- Run the RC1 and RC0 verifiers independently.
- Remove the upstream-pass/parent-blocked/non-independent markers in a focused
  test and require a fail-closed internal governance error.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Internal evidence composition and documentation clarification only. Ling
syntax, semantics, diagnostics, schemas, Semantic IDs, packages, dependencies,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0, protocol states, support states,
and public APIs are unchanged. No migration is required.

## Unresolved alternatives

Immutable candidate identity; independent reviewer selection/conflict policy;
clean environment/toolchain capture; artifact/signature/provenance checks;
candidate-wide conformance/corruption scope; TCB/security review; evidence
retention; rerun rules; signed comparison; and Go/No-Go remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
