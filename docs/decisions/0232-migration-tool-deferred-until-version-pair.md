# DEC-0232: Migration tool deferred until an accepted version pair / 迁移工具推迟至版本对获接受

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: compatibility governance
> 相关规范/缺口：`DEC-0230` | `DEC-0231` | `COMPAT-6503`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision keeps a public Ling migration tool unavailable while Ling has
only one released source version and no accepted source-version pair. It
records the exact readiness blockers without reserving a command or inventing
transformation semantics.

本决定规定：当 Ling 只有一个已发布源码版本且没有获接受的源码版本对时，公开迁移
工具保持不可用。它精确记录就绪阻断项，但不预留命令，也不虚构转换语义。

## Question

Should Ling expose a migration command before two governed source versions and
an accepted semantic transformation contract exist?

## Decision

1. No. The public migration command is `Absent`; `migrate` remains a rejected
   plan-only root command and no spelling is reserved for future use.
2. Migration implementation requires at least two real released source
   versions, an Accepted ordered version pair, and explicit input/output
   syntax, semantic equivalence, diagnostics, compatibility, and rollback
   authority.
3. The nine `COMPAT-6503` requirements—parser/semantic transaction, dry run,
   semantic diff, stale-edit check, backup/transaction, formatter, post-check/
   test, machine-readable report, and human-choice stop—remain exactly
   `Unavailable` with explicit blockers and evidence.
4. Existing parser, Semantic ID, formatter, source-span, LSP, and project APIs
   are not combined into a migration path. Their individual authority does not
   define cross-version transformation, atomic writes, or equivalence.
5. `cargo xtask migration verify` is an internal CI drift gate. Its manifest,
   report, states, and `GOV-MIGRATE-*` labels are governance evidence, not a
   public protocol, diagnostic domain, or Ling CLI.
6. Parent `COMPAT-6503` remains `BlockedSpec`. A future implementation must be
   authorized by an Accepted RFC/decision for a concrete version pair and must
   deliver all nine capabilities as one failure-atomic vertical slice.

## Normative basis

- DEC-0230 proves only one released Seed corpus; DEC-0231 records v0.1-v0.5 as
  `NoReleasedVersion`, not migration inputs.
- Accepted DEC-0002, DEC-0012, DEC-0015, and DEC-0023 govern spans, identities,
  Audit Source, and Author Source preservation independently; none defines a
  source-version migration transaction.
- The protocol inventory contains no migration report or Semantic Transaction
  protocol authorized for this task, and the public command catalog must not
  imply unimplemented behavior.

## Conformance plan

- Assert `migrate` remains rejected by the implemented command catalog and no
  `Migrate` command variant or parser route exists.
- Verify one released source version, no accepted pair, an absent public
  command, and all nine requirements in canonical order as `Unavailable`.
- Validate every blocker/evidence path and generated report drift.
- Require `cargo xtask migration verify` in the always-on CI contract.
- Run CLI, corpus, compatibility, governance, status, workspace, lint,
  formatting, deterministic, and offline gates.

## Compatibility impact

This decision adds an explicit non-claim and internal drift evidence only. It
changes no command, source, parser, resolver, formatter, LSP, diagnostic,
Semantic ID, schema, package, dependency, Unicode version, or runtime behavior.
No command spelling is reserved and no user input changes meaning.

## Unresolved alternatives

Concrete source-version pairs; parser/checked-semantic transformations;
equivalence and semantic diff; stale edit and atomic write policy; backup and
rollback; formatter/post-check/test orchestration; public CLI/report schema;
diagnostics; ambiguity handling; and resumable human decisions remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
