# Ling 规范缺口台账 / Specification Gap Register

> 状态：由 `gap-register.toml` 确定性生成
> 更新日期：2026-08-21
> 本台账记录未决问题及阻断关系，不替任何候选方案作出语义决议。

## Summary

- Total gaps: 26
- Open: 21
- Proposed: 0
- Accepted: 5
- Rejected: 0
- Superseded: 0

## Specification-gate coverage

| Gate | Release | Authority | Open gaps | Accepted decisions |
| --- | --- | --- | --- | --- |
| `G1-BYTECODE` — Bytecode version, verifier, evaluation, Fault, and compatibility | `v0.1` | `ROADMAP-1.0` | `GAP-BYTECODE-SEMANTICS-001` | `RFC-0014`, `RFC-0015`, `RFC-0016`, `RFC-0017`, `RFC-0018` |
| `G1-FORMATTER` — Author Source preservation, normalization, and localization | `v0.1` | `ROADMAP-1.0` | `GAP-FORMATTER-AUTHOR-SOURCE-001`, `GAP-AUTHOR-SOURCE-LOCALIZATION-001` | `DEC-0015` |
| `G1-INCREMENTAL` — Incremental cache keys, Semantic Hash upgrades, and invalidation | `v0.1` | `ROADMAP-1.0` | `GAP-INCREMENTAL-CACHE-001`, `GAP-SEMANTIC-HASH-LIFECYCLE-001` | `DEC-0012` |
| `G1-LSP-TRANSACTION` — LSP and Semantic Transaction Stable versus Experimental fields | `v0.1` | `ROADMAP-1.0` | `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001` | `DEC-0002`, `DEC-0012` |
| `G1-PACKAGE` — Package identity, namespace, visibility, dependency, and lock rules | `v0.1` | `ROADMAP-1.0` | `GAP-PACKAGE-NAMESPACE-001`, `GAP-PACKAGE-PROTOCOL-001` | `DEC-0007`, `RFC-0002` |
| `G1-TRAIT` — Trait declarations, solving, coherence, orphan rules, and lowering | `v0.1` | `ROADMAP-1.0` | `GAP-TRAIT-COHERENCE-001` | — |

## Gaps by earliest blocked release

### v0.1

| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |
| --- | --- | --- | --- | --- | --- |
| `GAP-AUTHOR-SOURCE-LOCALIZATION-001` | `P0` | `Open` | Equivalent localized Author Source keyword views | `FMT-1501`, `FMT-1508`, `IDE-2306` | `RFC-0003` |
| `GAP-FORMATTER-AUTHOR-SOURCE-001` | `P0` | `Open` | Author Source formatter preservation and normalization boundary | `FMT-1501`, `FMT-1502`, `FMT-1503`, `FMT-1504`, `FMT-1505`, `FMT-1506`, `FMT-1508` | — |
| `GAP-INCREMENTAL-CACHE-001` | `P0` | `Open` | Incremental query keys, invalidation, persistence, and corruption recovery | `INC-1401`, `INC-1406`, `INC-1407`, `INC-1408`, `INC-1409` | — |
| `GAP-LSP-TRANSACTION-PROTOCOL-001` | `P0` | `Open` | LSP position, snapshot, Workspace Edit, and Semantic Transaction boundary | `LSP-2102`, `LSP-2104`, `IDE-2305`, `IDE-2306`, `IDE-2309`, `LSP-2501` | `RFC-0004` |
| `GAP-SEMANTIC-HASH-LIFECYCLE-001` | `P0` | `Open` | Semantic Hash algorithm, identity upgrade, and invalidation lifecycle | `INC-1406`, `INC-1409`, `GOV-0106` | `RFC-0004` |
| `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | `P0` | `Open` | Semantic Graph and Semantic Transaction protocol lifecycle | `GOV-0106`, `INC-1406`, `IDE-2306`, `IDE-2309` | `RFC-0004` |
| `GAP-TRAIT-COHERENCE-001` | `P0` | `Open` | Base Trait coherence, orphan rules, solving, and lowering | `TRAIT-1301`, `TRAIT-1302`, `TRAIT-1303`, `TRAIT-1304`, `TRAIT-1305`, `TRAIT-1306`, `TRAIT-1307` | `RFC-0005` |
| `GAP-BYTECODE-SEMANTICS-001` | `P0` | `Accepted` | Versioned bytecode and verifier observable semantics | `VM-1209`, `VM-1210` | `RFC-0014`, `RFC-0015`, `RFC-0016`, `RFC-0017`, `RFC-0018` |
| `GAP-GOV-RFC-STATUS-001` | `P0` | `Accepted` | RFC-0001 lifecycle status mismatch | `GOV-0103`, `PRJ-1101`, `VM-1201`, `TRAIT-1301`, `INC-1401`, `FMT-1501`, `LSP-2101` | `RFC-0001` |
| `GAP-PACKAGE-NAMESPACE-001` | `P0` | `Accepted` | Package namespace and domain ownership | `PRJ-1101`, `PRJ-1104`, `PRJ-1105` | `RFC-0002` |
| `GAP-PACKAGE-PROTOCOL-001` | `P0` | `Accepted` | Project manifest, dependency graph, visibility, and lock protocol | `PRJ-1101`, `PRJ-1102`, `PRJ-1103`, `PRJ-1104`, `PRJ-1105` | `RFC-0002` |
| `GAP-SEED-BOOLEAN-OPERATORS-001` | `P0` | `Accepted` | Seed boolean operator syntax, precedence, and short-circuit boundary | `TS-3105` | `RFC-0001` |
| `GAP-UNICODE-ALIAS-SYNTAX-001` | `P1` | `Open` | Unicode Alias syntax and localized display rules | `TS-3104`, `IDE-2306`, `FMT-1501` | `RFC-0003` |

### v0.2

| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |
| --- | --- | --- | --- | --- | --- |
| `GAP-ACTOR-AWAIT-REENTRY-001` | `P1` | `Open` | Actor turn await and reentry semantics | `ACT-2301`, `ACT-2304`, `ACT-2305`, `ACT-2306` | `RFC-0009` |
| `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `P1` | `Open` | Bounded mailbox, backpressure, ordering, and supervision | `ACT-2303`, `SUP-2401`, `SUP-2402`, `SUP-2403` | `RFC-0009` |
| `GAP-ACTOR-REMOTE-DELIVERY-001` | `P1` | `Open` | Remote Actor identity, transport, and delivery strategy | `REM-2601`, `REM-2602`, `REM-2603`, `REM-2604`, `REM-2605` | `RFC-0009` |
| `GAP-DETERMINISTIC-REPLAY-001` | `P1` | `Open` | Determinism classes and Replay protocol | `REP-2501`, `REP-2502`, `REP-2503`, `REP-2504`, `REP-2505`, `REP-2506` | `RFC-0010` |
| `GAP-EFFECT-HANDLER-001` | `P1` | `Open` | Effect Row polymorphism and Handler semantics | `EFF-2101`, `EFF-2102`, `EFF-2103`, `EFF-2104`, `EFF-2105` | `RFC-0006` |
| `GAP-EFFECT-STATE-MASKING-001` | `P1` | `Open` | State effect visibility and masking | `EFF-2101`, `EFF-2102`, `EFF-2103` | `RFC-0006` |
| `GAP-STRUCTURED-TASK-001` | `P1` | `Open` | Structured Task lifecycle, cancellation, detach, and suspension | `TASK-2201`, `TASK-2202`, `TASK-2203`, `TASK-2204`, `TASK-2205`, `TASK-2206` | `RFC-0008` |

### v0.3

| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |
| --- | --- | --- | --- | --- | --- |
| `GAP-NATIVE-BACKEND-ABI-001` | `P1` | `Open` | Native backend, ABI, FFI, and target support contract | `NIR-3401`, `NIR-3402`, `NIR-3403`, `BACK-3501`, `BACK-3503`, `FFI-3601`, `FFI-3602`, `FFI-3604` | `RFC-0011` |
| `GAP-OWNERSHIP-MODEL-001` | `P1` | `Open` | Value, Managed, Resource, Borrow, Region, and Drop model | `MEM-3101`, `MEM-3102`, `MEM-3103`, `MEM-3104`, `OWN-3201`, `OWN-3202`, `OWN-3203`, `OWN-3205` | `RFC-0007` |
| `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` | `P1` | `Open` | Public lifetime inference boundary | `OWN-3203`, `OWN-3204`, `OWN-3207` | `RFC-0007` |
| `GAP-NUMERIC-CHECKED-FAULT-001` | `P2` | `Open` | Fixed-width checked arithmetic Fault visibility | `MEM-3102`, `NIR-3401`, `BACK-3503` | — |

### v0.4

| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |
| --- | --- | --- | --- | --- | --- |
| `GAP-KERNEL-DEVICE-001` | `P1` | `Open` | Kernel subset, Device Buffer, determinism, Placement, and backend capability | `KCHK-4101`, `KCHK-4102`, `KCHK-4103`, `KCHK-4104`, `KCHK-4105`, `DBUF-4401`, `DBUF-4402`, `DIR-4501`, `PLC-4801` | `RFC-0013` |

### v0.5

| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |
| --- | --- | --- | --- | --- | --- |
| `GAP-CRITICAL-PROFILE-001` | `P1` | `Open` | Critical minimum verifiable Core, Node, Contract, and evidence boundaries | `PROF-5101`, `PROF-5102`, `NODE-5301`, `CTR-5401`, `PROOF-5501`, `MC-5601`, `EVD-5801` | `RFC-0012` |

## Workflow

1. Add an Open gap before implementation would otherwise choose unspecified observable behavior.
2. Keep candidate options neutral; prototypes remain isolated and cannot create Stable behavior.
3. Move a gap to Accepted or Rejected only with an Accepted resolution document.
4. Attach positive, negative, and migration evidence before unblocking the affected release tasks.
5. Use `TODO(spec:GAP-...)` in source; an unregistered `TODO(spec)` fails the checker.
6. Run `cargo xtask governance check-gaps` and the relevant conformance suites.

## Machine source

The machine-readable source is [`gap-register.toml`](gap-register.toml). The checker rejects duplicate IDs, invalid lifecycle transitions, dangling authority/gate/supersession relations, supersession cycles, incomplete evidence categories, unmapped implementation markers, unregistered `TODO(spec)`, and report drift.
