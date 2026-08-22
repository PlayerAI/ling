# DEC-0170: Internal accelerator-plugin interface boundary evidence / 内部 accelerator plugin interface 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: accelerator-quality  
> 相关规范/缺口：`DEC-0169` | `DEC-0168` | `DEC-0167` | `DEC-0166` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`ACC-4701-OBSERVATION` accelerator-plugin interface boundary. It records
provisional verified-input, declaration, capability, target/cache, numeric,
fallback, Fault, trust, isolation, supply-chain, fixture, diagnostic, and
protocol vocabulary while Device IR, Kernel, plugin ABI, and support
authorities remain unresolved.

本决定只授权 `ACC-4701-OBSERVATION` 使用 test-local 的拟议 accelerator plugin interface 边界清单，
在 Device IR、Kernel、plugin ABI 与 support 权威尚未解决时，只记录临时 verified-input、declaration、capability、target/cache、numeric、fallback、Fault、trust、isolation、supply-chain、fixture、diagnostic 与 protocol 词汇。

## Question

ACC-4701 proposes that TPU, NPU, and other accelerator plugins consume only
verified Device IR or Kernel Core and declare supported operations/types,
shape/layout constraints, numeric modes, capabilities, target identity, cache
identity, fallback policy, and diagnostic mapping. Which planning vocabulary
can be retained as bounded evidence without creating a public extension point?

## Decision

1. `crates/ling-types/tests/accelerator_plugin_interface_evidence.rs` keeps a
   test-local inventory of sixty provisional verified-input, declaration,
   capability, target/cache, numeric, fallback, Fault, isolation, trust,
   supply-chain, fixture, diagnostic, redaction, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.accelerator-plugin-interface-observation/0`.
   They are not plugin traits, registries, loaders, manifests, capabilities,
   target identities, dependencies, diagnostics, public protocols, or support
   claims.
3. No plugin trait, registry, loader, manifest, dependency, target package,
   cache API, diagnostic, protocol, or placeholder API is added. Public
   `ACC-4701` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:412-423` is
  non-normative; declarations and verified-input boundaries do not define
  Device IR schema, plugin ABI, capability/target/cache identity, trust,
  loading, versioning, fallback, diagnostics, or compatibility lifecycle.
- `docs/ROADMAP-1.0.md:381-431` permits accelerator extensions through a
  narrow verified interface but does not authorize a plugin API or unreviewed
  vendor semantics.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4605 remain
  `BlockedSpec`; RFC-H404/H405 and RFC-0013 are not Accepted. Kernel/device,
  Native/backend, and accelerator support entries remain unresolved.

## Conformance plan

- Assert all sixty plugin-interface boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer plugin ABI/registry/loader, declaration semantics, capability and
  target/cache identity, trust/supply-chain policy, fallback/rejection,
  diagnostics, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no plugin extension point, dependency, target, cache, or public
support claim exists.

## Unresolved alternatives

Verified Device IR/Kernel input and validation; plugin ABI/version negotiation
and declaration schema; supported ops/types, shape/layout, numeric modes and
determinism; device capabilities, capability/target/cache identity and
invalidation; fallback/rejection, Fault propagation, diagnostics; resource
ownership, cleanup and cancellation; host/frontend/plugin isolation; loading
and trust boundary; dependencies, signatures, provenance, license, offline
build, sandbox; Experimental/Supported/Unsupported lifecycle; positive,
negative, malformed, migration, source-map/Unicode, determinism, cache,
capability, fallback, security and differential fixtures; diagnostic and
host/path/address/timestamp/driver/debug exclusion, protocol inventory, and
public plugin status remain open under ACC-4701, GPU-4601 through GPU-4605,
DIR-4501 through DIR-4503, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
