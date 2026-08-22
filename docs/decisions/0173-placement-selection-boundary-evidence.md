# DEC-0173: Internal placement-selection boundary evidence / 内部 Placement 选择边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: compiler-quality  
> 相关规范/缺口：`DEC-0172` | `DEC-0171` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PLC-4802-OBSERVATION`. It records provisional static-filter, artifact,
runtime-availability, policy/cost, profile, decision, replay, privacy, and
fixture vocabulary while RFC-H405 and the Device/Native authorities remain
unresolved.

本决定只授权 `PLC-4802-OBSERVATION` 使用 test-local 的 Placement 选择边界清单；
在 RFC-H405 以及 Device/Native 权威尚未解决时，只记录临时 static filter、artifact、
runtime availability、policy/cost、profile、decision、replay、privacy 与 fixture 词汇。

## Question

PLC-4802 sketches compile-time legality and capability filtering, artifact
preparation, runtime available-device matching, policy/cost choice, recording,
and profile-specific replay. Which planning vocabulary can be retained as
bounded evidence without creating a selector, runtime protocol, or replay
semantics?

## Decision

1. `crates/ling-types/tests/placement_selection_evidence.rs` keeps a
   test-local inventory of sixty provisional pipeline, static/runtime,
   target/capability, policy/cost, fallback/rejection, profile/replay,
   schema/cache, privacy, fixture, diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.placement-selection-observation/0`. These bytes
   are evidence only; they are not candidate artifacts, runtime facts,
   selector decisions, replay records, cache keys, diagnostics, or support.
3. No candidate filter, runtime matcher, policy/cost API, decision recorder,
   replay protocol, dependency, target package, diagnostic, or placeholder API
   is added. Public `PLC-4802` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:448-464` is
  non-normative; its phase sketch does not define legality, runtime facts,
  policy precedence, deterministic choice, or replay compatibility.
- `docs/ROADMAP-1.0.md:381-431` requires explicit, explainable and replayable
  Placement decisions but does not authorize a selector before G4 authority.
- `docs/status/PLC-4802-AUTHORITY-AUDIT.md` records the missing RFC-H405,
  Device IR, target/capability, cost, fallback, profile, replay, and diagnostic
  contracts; `DEC-0172` remains prerequisite test-local evidence.

## Conformance plan

- Assert all sixty selection boundaries and local order; compare forward and
  reverse opaque bytes; reject duplicates.
- Defer candidate filtering, artifact preparation, runtime matching,
  policy/cost selection, decision/replay/cache behavior, diagnostics, and
  protocol behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Only test-local evidence is added; no selector or replay support is
registered.

## Unresolved alternatives

Static legality and verified artifact shape; capability, target, toolchain,
feature-version, topology and buffer identity; runtime availability and
remote facts; policy/cost inputs and precedence; deterministic choice;
fallback/rejection/conflict/missing-device/cancellation/resource/Fault behavior;
Critical/Strict/Native profiles and fixed placement; decision, replay, stale
and migration handling; explain/cache/provenance/privacy; host/driver
exclusions; fixtures, diagnostics, protocol inventory, and public selection
status remain open under PLC-4802, PLC-4801, ACC-4702, ACC-4701,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing RFC-H405.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
