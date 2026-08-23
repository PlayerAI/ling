# DEC-0216: Internal Support-Matrix Item Audit boundary evidence / 内部逐项支持矩阵审计边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role: stabilization
> 相关规范/缺口：`DEC-0215` | `ROADMAP-1.0` | `GAP-REGISTER` | `SUPPORT-MATRIX` | `PROTOCOL-INVENTORY`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`STAB-6101-OBSERVATION`. It records provisional per-item audit, traceability,
support-state, fail-closed, compatibility, release-evidence, and fixture
vocabulary while the candidate Stable set and 1.0 promotion rules remain
unresolved.

本决定只授权 `STAB-6101-OBSERVATION` 使用 test-local 的逐项审计、
traceability、support state、fail-closed、compatibility、release evidence 与
fixture 边界清单；在候选 Stable 集合和 1.0 晋级规则尚未解决时，只记录临时
词汇，不把任何能力晋级为 Stable。

## Question

STAB-6101 proposes auditing every candidate Stable Feature/Profile/Target
against Accepted authority, compiler and execution support, conformance,
diagnostics, editors, compatibility, limitations, and release evidence. Which
vocabulary can be retained as bounded evidence without defining the candidate
set or promoting the current `1.0-draft` support matrix?

## Decision

1. `crates/ling-types/tests/support_matrix_item_audit_evidence.rs` keeps a
   test-local inventory of sixty provisional item identity, authority,
   compiler/execution, conformance/editor, compatibility/evidence,
   traceability, support-state, failure, and fixture boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.support-matrix-item-audit-observation/0`. These
   bytes are observation evidence only; they are not matrix rows, Stable
   identities, diagnostics, protocols, compatibility promises, or support
   claims.
3. Experimental, Preview, Unavailable, Unsupported, and Stable remain distinct
   local categories. This decision preserves every current matrix state and
   grants no promotion or demotion authority.
4. `MissingField`, `ConflictingField`, `StaleField`, `UnverifiableField`, and
   `FailClosed` record the intended audit posture only. They do not define a
   public audit result or error code.
5. Existing `cargo xtask support verify` continues to validate the draft
   matrix and truthful unsupported claims. It is not reinterpreted as proof of
   1.0 Stable support.
6. No Stable row, candidate registry, compatibility promise, release artifact,
   CLI/LSP/Zed route, diagnostic allocation, public protocol, support claim,
   or placeholder API is added. Public `STAB-6101` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:32-49` is a
  non-normative audit template. It defines no candidate set, Stable identity,
  promotion/demotion rule, evidence polarity, or compatibility version.
- `docs/status/STAB-6101-AUTHORITY-AUDIT.md` records the missing G1-G5 exits,
  candidate inventory, Stable criteria, compatibility policy, release
  evidence, diagnostics, and fixtures.
- `docs/governance/support-matrix.toml` declares `matrix_target =
  "1.0-draft"`; current states are Experimental, Preview, Unavailable, or
  Unsupported rather than Stable commitments.
- `docs/ROADMAP-1.0.md` requires Accepted authority and executable evidence
  before promotion. Planning text cannot create semantics or Stable support.
- `DEC-0215` authorizes only test-local Critical-runtime/target vocabulary and
  leaves the final G5 dependency unavailable.

## Conformance plan

- Assert all sixty support-matrix-item-audit categories and local order;
  compare forward/reverse opaque bytes; reject duplicates; retain candidate,
  authority, all current support states, Stable, missing-field, fail-closed,
  and protocol boundaries together.
- Run the existing draft support-matrix verifier without changing its matrix
  target or claims.
- Defer candidate inventory, Stable promotion/demotion, compatibility,
  diagnostics, release binding, public protocols, and support claims until
  Accepted authority and per-row executable evidence exist.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP/Zed, runtime, bytecode, VM, dependencies, support-matrix states, and
Unicode 17.0.0 remain unchanged. The current draft matrix and internal support
fixtures are not reinterpreted as 1.0 Stable contracts; only test-local
boundary evidence is added.

## Unresolved alternatives

Candidate Stable Feature/Profile/Target inventory; stable identity/version;
inclusion, exclusion, promotion, demotion and removal rules; Accepted clauses;
parser/checker/Typed-Core and Interpreter/VM/Native/Device evidence;
positive/negative/differential conformance; diagnostic, LSP and Zed coverage;
bidirectional clause/symbol/test traceability; evidence polarity; known
limitations; source/Semantic/canonical/schema/CLI/package/bytecode/replay/ABI/
profile/target compatibility and migration; protocol readers/writers and
golden corpora; deterministic offline builds; Tier scope, target/toolchain and
artifact identity; independent review, security, fuzz/fault/TCB/licensing and
release-candidate binding; fail-closed missing/conflicting/stale/unverifiable
rows; bilingual stable diagnostics and machine schema; per-row positive,
negative, migration, corruption, Unicode 17.0.0, BOM/CRLF, source-span and
determinism fixtures; protocol inventory and public support remain open under
STAB-6101, STAB-6102, STAB-6103, ROADMAP-1.0, all incomplete G1-G5 exits,
open gaps, the draft SUPPORT-MATRIX, and missing G6 stabilization authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
