# DEC-0231: Current compiler compatibility-boundary evidence / 当前编译器兼容边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: compatibility governance
> 相关规范/缺口：`CONFORMANCE` | `DEC-0230` | `COMPAT-6502`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a truthful compatibility boundary for the current
`0.0.1-dev` compiler. It proves unchanged acceptance of the frozen v0.0.1 Seed
corpus and records that v0.1-v0.5 have no released historical inputs. It is not
a Ling 1.0 compatibility promise.

本决定授权为当前 `0.0.1-dev` 编译器建立真实的兼容边界。它证明冻结的 v0.0.1
Seed 语料可原样接受，并记录 v0.1-v0.5 尚无已发布历史输入。它不是 Ling 1.0
兼容性承诺。

## Question

Which compiler compatibility outcomes can be stated before a Ling 1.0
compiler and accepted v0.1-v0.5 releases exist?

## Decision

1. The current compiler is identified exactly as `0.0.1-dev` in
   `Development` state. It must not be described as a 1.0 compiler.
2. v0.0.1 has outcome `AcceptUnchanged`, governed by `CONFORMANCE` and bound to
   DEC-0230's exact frozen Seed corpus digest.
3. v0.1 through v0.5 have outcome `NoReleasedVersion`, governed by this
   decision. That state is neither accept-with-warning, auto-migration, nor
   actionable rejection: there is no historical input to classify.
4. The matrix records zero verified general N-1 compiler edges. Independently
   versioned schemas and bytecode readers retain their own authorities and do
   not imply source/compiler release compatibility.
5. `cargo xtask compatibility verify` validates exact release order, outcome/
   authority pairs, evidence paths, compiler/Unicode markers, Seed digest,
   zero N-1 claim, and generated report drift. It is an internal CI gate, not
   a public CLI or matrix protocol.
6. Parent `COMPAT-6502` remains `BlockedSpec`. Future outcome rows require an
   actual released version, Accepted per-surface semantics, original corpus,
   and executable warning/migration/rejection evidence with stable bilingual
   diagnostics where applicable.

## Normative basis

- `CONFORMANCE` and DEC-0230 provide executable v0.0.1 unchanged-input
  evidence and a deterministic frozen corpus identity.
- Accepted diagnostic, Unicode 17.0.0, UTF-8 span, and canonical-byte decisions
  continue to govern the Seed evidence.
- Existing schema and protocol registries truthfully report current-only or
  surface-specific readers and zero general N-1 compatibility edges.
- `ROADMAP-1.0` requires a future compiler matrix but cannot create nonexistent
  releases or compatibility semantics.

## Conformance plan

- Verify the current compiler/version state and Unicode version exactly.
- Bind v0.0.1 `AcceptUnchanged` to the frozen Seed digest and `CONFORMANCE`.
- Verify v0.1-v0.5 remain `NoReleasedVersion` in canonical order and cannot be
  relabeled as warning, migration, or rejection outcomes.
- Require `cargo xtask compatibility verify` in the always-on CI contract.
- Run corpus, traceability, schema compatibility, Seed reproduction,
  governance, status, workspace, lint, formatting, and offline gates.

## Compatibility impact

This decision adds internal evidence and an explicit non-claim only. It
changes no source, compiler, parser, resolver, evaluator, diagnostic, Semantic
ID, schema, bytecode, package/lock, editor, CLI, dependency, Unicode version,
or runtime behavior. It adds no warning, migration, rejection, reader, or
public compatibility protocol.

## Unresolved alternatives

An actual 1.0 compiler subject; released v0.1-v0.5 inputs; per-surface
compatibility units; warning and suppression policy; semantic migration;
actionable rejection diagnostics; rollback; N-1 readers; cross-platform
evidence; and 1.x compatibility remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
