# DEC-0239: Seed example execution manifest / Seed 示例执行清单

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: documentation engineering
> 相关规范/缺口：`DEC-0046` | `DOC-6702`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes one internal manifest as the shared source of truth
for the checked-in Seed examples exercised by governance and CLI process tests.
It proves current examples execute as recorded; it does not promote any feature
or output protocol to Stable.

本决定授权使用一个内部清单，作为治理门禁和 CLI 进程测试共同使用的 Seed 示例事实来源。它证明当前示例
按记录执行，但不将任何功能或输出协议提升为 Stable。

## Question

How should the repository prevent the documented example inventory and the
hard-coded process-test list from drifting independently?

## Decision

1. `tests/examples/seed-cases.toml` is the internal executable-example
   manifest with schema `ling.seed-example-cases/0`.
2. The exact set contains six checked-in files: one core-minimal example,
   three core-realistic examples, and the Chinese/English tutorial pair.
3. Every case records a stable internal ID, exact `examples/*.ling` path,
   role, exact LF-terminated stdout, Semantic definition-name witness, and
   ASCII or Chinese identifier classification.
4. `cargo xtask examples verify` parses the manifest strictly, rejects unknown
   fields, duplicate IDs/paths, set or classification drift, unsafe/missing
   paths, malformed stdout, and missing Semantic witnesses.
5. The CLI process test reads that same manifest and runs `ling check`,
   `ling run`, and `ling semantic` for every case. It asserts successful empty
   check output, exact UTF-8 runtime stdout, empty stderr, the experimental
   Semantic schema, and the named definition witness.
6. Negative/error examples remain governed by the registered conformance
   corpus. Audit determinism remains covered by its existing process test.
7. Parent `DOC-6702` remains `BlockedSpec` until the future Accepted 1.0
   support matrix and all Stable-capability example dimensions exist.

## Normative basis

- Accepted DEC-0046 authorizes the bounded Seed example inventory and its
  anti-promotion guardrails.
- Accepted Seed semantics and conformance evidence authorize the behavior of
  the six existing sources; this manifest neither adds nor changes semantics.
- Repository governance requires evidence paths, commands, outputs, and
  support claims to match verifiable current artifacts.

## Conformance plan

- Verify the strict six-case manifest and exact role/path classifications.
- Reject traversal/missing source paths and malformed expected stdout in a
  focused isolated test.
- Execute all six cases through `check`, `run`, and `semantic` from the shared
  manifest and validate exact observable results.
- Retain negative conformance and deterministic Audit tests.
- Run examples, CLI, xtask, governance, status, workspace, Clippy, formatting,
  deterministic, and offline gates.

## Compatibility impact

This consolidates existing internal test metadata and adds the previously
separately tested hello example to the shared process loop. Ling semantics,
diagnostics, schemas, Semantic IDs, packages, dependencies, CLI behavior,
runtime output, Unicode 17.0.0, protocols, and support states remain unchanged.

## Unresolved alternatives

Stable 1.0 example policy; future feature examples; public example-manifest
schema; profile/target matrices; ownership, package, backend, concurrency, and
editor examples; cross-host release fixtures; and G6 sign-off remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
