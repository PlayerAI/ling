# DEC-0174: Internal cost-model boundary evidence / 内部 Cost Model 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: compiler-quality  
> 相关规范/缺口：`DEC-0173` | `DEC-0172` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory for
`PLC-4803-OBSERVATION`. It records provisional cost-factor, unit,
calibration, uncertainty, policy, profile, replay/cache, privacy, fixture,
diagnostic, and support-exclusion vocabulary while Placement and Device
authorities remain unresolved.

本决定只授权 `PLC-4803-OBSERVATION` 使用 test-local 的 Cost Model 边界清单；
在 Placement 与 Device 权威尚未解决时，只记录临时 cost factor、unit、calibration、
uncertainty、policy、profile、replay/cache、privacy、fixture、diagnostic 与 support-exclusion 词汇。

## Question

PLC-4803 proposes a conservative, explainable model using input bytes,
transfer bytes, operation count, memory footprint, launch overhead, occupancy
hints, and deadline/energy metadata, while forbidding uncalibrated estimates
from being advertised as guarantees. Which vocabulary can be retained as
bounded evidence without creating an estimator, benchmark promise, or policy
API?

## Decision

1. `crates/ling-types/tests/cost_model_evidence.rs` keeps a test-local
   inventory of sixty provisional cost factors, units, static/dynamic inputs,
   calibration/provenance, uncertainty, policy/selection, profile/replay/cache,
   privacy, fixture, diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.cost-model-observation/0`. These bytes are
   evidence only; they are not estimates, units, measurements, guarantees,
   policy inputs, cache keys, diagnostics, or support.
3. No cost model, estimator, calibration API, benchmark claim, dependency,
   target package, cache/replay field, diagnostic, or placeholder API is added.
   Public `PLC-4803` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:466-478` is
  non-normative and its warning does not define units, calibration,
  uncertainty, comparability, policy, or identity.
- `docs/ROADMAP-1.0.md:381-431` requires cost information for explainable
  Placement but does not authorize a schema or performance/energy guarantee.
- `docs/status/PLC-4803-AUTHORITY-AUDIT.md` records missing Placement, Device,
  runtime, profile, replay/cache, numeric, and diagnostic authority;
  `DEC-0173` remains prerequisite test-local evidence.

## Conformance plan

- Assert all sixty cost-model boundaries and local order; compare forward and
  reverse opaque bytes; reject duplicates.
- Defer estimator behavior, units, calibration, uncertainty, policy
  integration, profile/replay/cache semantics, diagnostics, and protocol
  behavior until accepted authority exists.

## Compatibility impact

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, source spans,
CLI/LSP, runtime, bytecode, VM, dependencies, and Unicode 17.0.0 remain
unchanged. Only test-local evidence is added; no estimator or support claim is
registered.

## Unresolved alternatives

Cost factors and canonical units; static/dynamic input domains; calibration,
provenance, confidence and uncertainty; estimate-versus-guarantee boundaries;
overflow/unknown/invalid values; hardware/device/capability/placement/buffer
context; policy precedence, limits, fallback/rejection and selection use;
diagnostic-only versus Critical/Strict/Native/replay/cache inputs; explain,
versioning, migration, corruption and privacy; host/path/address/timestamp/
driver/debug exclusions; calibration/uncertainty/determinism/topology/
fallback/differential/Unicode fixtures; diagnostics, protocol inventory, and
public cost status remain open under PLC-4803, PLC-4802, PLC-4801,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
