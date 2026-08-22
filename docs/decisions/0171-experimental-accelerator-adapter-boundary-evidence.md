# DEC-0171: Internal experimental accelerator-adapter boundary evidence / 内部 Experimental accelerator adapter 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: accelerator-quality  
> 相关规范/缺口：`DEC-0170` | `DEC-0169` | `DEC-0168` | `DEC-0167` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`ACC-4702-OBSERVATION` Experimental accelerator-adapter boundary. It records
provisional Experimental status, verified input, adapter, capability, numeric,
resource, Fault, limitation, lifecycle, trust, supply-chain, fixture,
diagnostic, and support-exclusion vocabulary while accelerator and backend
authorities remain unresolved.

本决定只授权 `ACC-4702-OBSERVATION` 使用 test-local 的拟议 Experimental accelerator adapter 边界清单，
在 accelerator 与 backend 权威尚未解决时，只记录临时 Experimental status、verified input、adapter、capability、numeric、resource、Fault、limitation、lifecycle、trust、supply-chain、fixture、diagnostic 与 support-exclusion 词汇。

## Question

ACC-4702 allows a first TPU/NPU adapter to remain Experimental and not block
v0.4 Stable unless the support matrix includes it, while forbidding vendor
graph semantics in the core compiler. Which planning vocabulary can be retained
as bounded evidence without implementing an adapter or treating Experimental as
semantic authorization?

## Decision

1. `crates/ling-types/tests/experimental_accelerator_adapter_evidence.rs`
   keeps a test-local inventory of sixty provisional Experimental status,
   verified-input, adapter, capability, numeric, resource, Fault, limitation,
   lifecycle, trust, supply-chain, fixture, diagnostic, and support-exclusion
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.experimental-accelerator-adapter-observation/0`.
   They are not Experimental adapters, plugin packages, graph bridges, target
   entries, support claims, diagnostics, public protocols, or dependencies.
3. No TPU/NPU adapter, plugin package, graph bridge, dependency, target or
   support entry, cache/runtime API, diagnostic, protocol, or placeholder API
   is added. Public `ACC-4702` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:425-427` is
  non-normative; Experimental wording and the vendor-graph exclusion do not
  define input, ABI, capability, isolation, numeric, cache, Fault, trust, or
  evidence contracts.
- `docs/ROADMAP-1.0.md:417-431` says accelerator extensions should reuse
  Kernel verification, shape/layout, and Placement through a narrow
  interface; only support-matrix backends are 1.0 gates. It does not authorize
  an implementation or make Experimental behavior semantically unspecified.
- ACC-4701 remains `BlockedSpec`; DIR-4501 through DIR-4503 and GPU-4601
  through GPU-4605 remain blocked. RFC-H404/H405 and RFC-0013 are not
  Accepted, and accelerator support entries remain unimplemented.

## Conformance plan

- Assert all sixty Experimental-adapter boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer adapter implementation, package/target/cache/runtime/Fault paths,
  Experimental lifecycle and limitations, trust/supply-chain controls,
  diagnostics, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; Experimental wording does not create an adapter, dependency,
support claim, or public protocol.

## Unresolved alternatives

Experimental status and verified Device IR/Kernel input; plugin ABI and
capability/target identity; shape/layout/numeric/determinism/resource/
synchronization/Fault/fallback/cache contracts; limitation set, evidence,
reproducibility, promotion/deprecation/revocation/removal; frontend/vendor
graph/isolation; trust, dependencies, signatures, provenance, licenses,
offline builds, sandbox; positive/negative/malformed/migration/capability/
cache/security/source-map/Unicode/determinism/differential/lifecycle fixtures;
diagnostics, host/path/address/timestamp/driver/debug exclusion, public
support exclusion, protocol inventory, and adapter status remain open under
ACC-4702, ACC-4701, GPU-4601 through GPU-4605, DIR-4501 through DIR-4503,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
