# DEC-0240: Bilingual tutorial semantic equivalence / 双语教程语义等价

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: documentation engineering
> 相关规范/缺口：`DEC-0047` | `DEC-0239` | `DOC-6703`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes a process-level comparison of the Chinese-first and
idiomatic English tutorial Semantic Graphs after excluding localized names,
text, source evidence, and experimental identities.

本决定授权在排除本地化名称、文本、源码证据与实验性标识后，对中文优先教程和地道英文教程的
Semantic Graph 进行进程级比较。

## Question

How should the repository prove the two runnable tutorials have equivalent
checked structure without requiring mechanical identifier translation or
mistaking experimental Semantic IDs for compatibility values?

## Decision

1. The shared six-case execution test retains both tutorial sources and their
   independent `check`, exact-output `run`, and `semantic` assertions.
2. For the two `tutorial` cases, derive a private comparison projection from
   the emitted `ling.semantic/0.1` JSON.
3. The projection retains schema/language/Unicode versions, entry module,
   module Capability requirements, user definition kind/type/effect/capability
   shapes, all node kind/type/effect/capability shapes, and reference
   source/target-kind topology.
4. Replace only each tutorial's single user nominal type spelling with an
   internal domain-type marker and sort unordered comparison collections.
5. Do not compare definition/node IDs, localized identifier spelling,
   user-facing literals, spans, source paths, identifier scripts, or ordering
   that is not a language equivalence requirement.
6. Require the ASCII and Chinese projections to be exactly equal. Missing or
   duplicate tutorial classifications, missing nominal types, malformed graph
   fields, or structural divergence fail the process test.
7. Parent `DOC-6703` remains `BlockedSpec`; this is current Seed equivalence
   evidence, not a localization/alias policy or Stable 1.0 tutorial claim.

## Normative basis

- Accepted DEC-0047 requires equivalent Chinese-first and idiomatic English
  Seed tutorial evidence without mechanically translating identifiers.
- Accepted DEC-0239 supplies the strict shared execution manifest and actual
  CLI process/Semantic evidence for both tutorial cases.
- Accepted Seed semantics define the checked structures being compared;
  `ling.semantic/0.1` remains Experimental/Preview evidence.

## Conformance plan

- Check, run, and emit Semantic Graphs for all six manifest cases.
- Derive one ASCII and one Chinese tutorial projection and require exact shape
  equality after only nominal-type spelling normalization.
- Retain exact localized runtime output and named-definition witnesses.
- Retain correct-error conformance and deterministic Audit evidence.
- Run tutorial, examples, CLI, governance, status, workspace, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Adds a private test-only Semantic comparison. Ling semantics, source files,
diagnostics, schemas, Semantic IDs, packages, dependencies, CLI/runtime
behavior, Unicode 17.0.0, protocols, and support states remain unchanged.

## Unresolved alternatives

Stable localization policy; translated keyword or alias behavior; prose-level
translation validation; public equivalence schema; future feature tutorials;
profile/target and ownership guidance; cross-host release samples; and G6
sign-off remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
