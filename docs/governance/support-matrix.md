# Ling 1.0 support matrix draft / Ling 1.0 支持矩阵草案

> Generated deterministically from `support-matrix.toml`; do not edit this report manually.
> 本报告由 `support-matrix.toml` 确定性生成；不得手工编辑。

- Matrix target: `1.0-draft`
- Status: `Draft`
- Current compiler: `0.0.1-dev`
- Current language: `0.0.1-dev`
- Unicode: `17.0.0`
- Updated: `2026-08-22`

This draft separates current evidence from candidate 1.0 scope. Empty current-profile lists mean the Seed implementation is unprofiled; candidate profile entries are planning input, not support claims. No Native target, VM, device backend, or Critical guarantee is currently supported.

本草案严格区分当前证据与 1.0 候选范围。当前 profile 为空表示 Seed 实现尚未 profile 化；候选 profile 仅是规划输入，不是支持声明。目前不支持 Native target、VM、设备 backend 或 Critical 保证。

## Feature/profile/stability

| Feature | Current state | Stability | Current profiles | Candidate 1.0 profiles | Boundary | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| `FTR-SEED-0001` | `Implemented` | `Experimental` | — | `Explore` | The Seed interpreter is unprofiled; Explore inclusion is a draft target, not current selectable-profile support. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md) |
| `FTR-SEED-0002` | `Implemented` | `Experimental` | — | `Explore` | Diagnostics work in the unprofiled Seed toolchain; profile-specific severity and restrictions are not implemented. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md)<br>[`docs/ERROR-CODES.md`](../ERROR-CODES.md) |
| `FTR-SEED-0003` | `Implemented` | `Experimental` | — | `Explore` | Unicode 17 rules are implemented for Seed; profile-specific identifier policies remain unimplemented. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md)<br>[`crates/ling-unicode/src/generated.rs`](../../crates/ling-unicode/src/generated.rs) |
| `FTR-SEED-0004` | `Implemented` | `Experimental` | — | `Explore` | The current type/Place subset is Seed-only; Native and Critical subsets require later accepted decisions. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md) |
| `FTR-SEED-0005` | `Implemented` | `Experimental` | — | `Explore` | Seed Effect/Capability checking exists without a selectable Profile; later profile allowlists are unresolved. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md) |
| `FTR-SEED-0006` | `Implemented` | `Experimental` | — | `Explore` | Semantic Graph, canonical identity, and Audit remain Experimental/Preview protocols and are not Stable 1.x contracts. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md)<br>[`docs/governance/protocol-inventory.toml`](../governance/protocol-inventory.toml) |
| `FTR-SEED-0007` | `Implemented` | `Experimental` | — | `Explore` | The shared Seed pipeline and library-level base VM are tested on CI hosts, but no Profile selector, CLI VM backend, Native target, or Critical verifier exists. | [`docs/traceability/v0.0.1.md`](../traceability/v0.0.1.md)<br>[`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) |

## Profiles

| Profile | Current state | Stability | Selectable | 1.0 candidate | Allowed Effects | Memory models | Runtime models | Explicitly unsupported |
| --- | --- | --- | ---: | ---: | --- | --- | --- | --- |
| `Critical` | `Unavailable` | `Experimental` | no | yes | — | — | — | Critical subset validation<br>bounded-time or bounded-memory guarantee<br>Node, Contract, model checking, or evidence bundle |
| `Explore` | `Unavailable` | `Experimental` | no | yes | — | — | — | profile selection or validation<br>selectable/default VM or JIT execution<br>general managed runtime or GC commitment |
| `Native` | `Unavailable` | `Experimental` | no | yes | — | — | — | profile selection or validation<br>AOT or Native lowering<br>Managed Island<br>FFI or stable ABI |

## Host platform tiers

| Host | Platform | Runner | Architecture | Tier | Stability | Build | Tests | Release artifacts | Last verified commit |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | --- |
| `HOST-LINUX-LATEST` | Linux | `ubuntu-latest` | CI runner default; not a target commitment | `Tier2` | `Experimental` | yes | yes | no | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `HOST-MACOS-LATEST` | macOS | `macos-latest` | CI runner default; not a target commitment | `Tier2` | `Experimental` | yes | yes | no | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |
| `HOST-WINDOWS-LATEST` | Windows | `windows-latest` | CI runner default; not a target commitment | `Tier2` | `Experimental` | yes | yes | no | `652d19b9eaec2ab607edfe1a1e7ea742c861cf91` |

## Native target tiers

| Target ID | Target | Tier | Stability | Implemented | Backend | Blockers |
| --- | --- | --- | --- | ---: | --- | --- |
| `TARGET-NATIVE-AOT` | No committed Ling Native target | `Unsupported` | `Experimental` | no | — | `GAP-NATIVE-BACKEND-ABI-001`<br>`GAP-OWNERSHIP-MODEL-001` |

## Backend/device tiers

| Backend | Kind | Device | Tier | Stability | Implemented | Profiles | Blockers |
| --- | --- | --- | --- | --- | ---: | --- | --- |
| `BACKEND-ACCELERATOR` | Kernel accelerator | TPU/NPU/other accelerator | `Unsupported` | `Experimental` | no | — | `GAP-KERNEL-DEVICE-001` |
| `BACKEND-GPU` | Kernel GPU | GPU | `Unsupported` | `Experimental` | no | — | `GAP-KERNEL-DEVICE-001` |
| `BACKEND-INTERPRETER` | Reference interpreter | Host CPU | `Tier2` | `Experimental` | yes | — | — |
| `BACKEND-KERNEL-CPU` | Kernel CPU reference/SIMD | CPU | `Unsupported` | `Experimental` | no | — | `GAP-KERNEL-DEVICE-001` |
| `BACKEND-NATIVE` | Native AOT | Host CPU | `Unsupported` | `Experimental` | no | — | `GAP-NATIVE-BACKEND-ABI-001`<br>`GAP-OWNERSHIP-MODEL-001` |
| `BACKEND-VM` | Bytecode virtual machine | Host CPU | `Tier2` | `Experimental` | yes | — | — |

## Standard package stability

| ID | Package | Version | State | Stability | Implemented | Packaged | Profiles | Explicitly unsupported |
| --- | --- | --- | --- | --- | ---: | ---: | --- | --- |
| `STD-LING-PRELUDE` | `Ling.Prelude` | `0.0.1-dev` | `BuiltinOnly` | `Preview` | yes | no | — | manifest-based installation or version selection<br>package registry distribution<br>profile-specific standard package surface |

## Protocol versions

| Protocol | Visibility | Version | Stability | Implemented |
| --- | --- | --- | --- | ---: |
| `PROTO-ABI` | `Planned public` | — | `Future` | no |
| `PROTO-AUDIT-SOURCE` | `Public` | `ling.audit/0.1` | `Preview` | yes |
| `PROTO-BUILD-METADATA` | `Public` | `ling.project.artifact/0.1` | `Experimental` | yes |
| `PROTO-BYTECODE` | `Public` | `ling.bytecode/1.2` | `Experimental` | yes |
| `PROTO-CANONICAL-BYTES` | `Public` | `file-mode v1 and package-aware v2 domain encodings` | `Experimental` | yes |
| `PROTO-CLI` | `Public` | `0.0.1-dev` | `Preview` | yes |
| `PROTO-CLI-COMPLETION` | `Public` | `ling.cli-completion/0.1` | `Preview` | yes |
| `PROTO-CLI-EXIT` | `Public` | `0.0.1-dev` | `Preview` | yes |
| `PROTO-CLI-INIT` | `Public` | `ling.init/0.1` | `Preview` | yes |
| `PROTO-CLI-TEST` | `Public` | `ling.test/0.1` | `Preview` | yes |
| `PROTO-DIAGNOSTIC-JSON` | `Public` | `ling.diagnostic/0.1` | `Preview` | yes |
| `PROTO-EVIDENCE` | `Planned public` | — | `Future` | no |
| `PROTO-FORMAT-CLI` | `Public` | `ling.format/0.1` | `Preview` | yes |
| `PROTO-HUMAN-OUTPUT` | `Public` | `0.0.1-dev` | `Preview` | yes |
| `PROTO-INTERNAL-INCIDENT` | `Internal` | `ling.internal-incident/0.1` | `Internal` | yes |
| `PROTO-LOCKFILE` | `Public` | `ling.lock/1` | `Experimental` | yes |
| `PROTO-LSP-DIAGNOSTIC` | `Public` | `ling.lsp.diagnostic/0.2` | `Experimental` | yes |
| `PROTO-LSP-DIAGNOSTIC-CONTROL` | `Public` | `ling.lsp.diagnostic-control/0.1` | `Preview` | yes |
| `PROTO-LSP-DOCUMENT-SYMBOL` | `Public` | `ling.lsp.document-symbol/0.1` | `Preview` | yes |
| `PROTO-LSP-FORMATTING` | `Public` | `ling.lsp.formatting/0.1` | `Experimental` | yes |
| `PROTO-LSP-HOVER` | `Public` | `ling.lsp.hover/0.1` | `Preview` | yes |
| `PROTO-LSP-LIFECYCLE` | `Public` | `ling.lsp.lifecycle/0.1` | `Preview` | yes |
| `PROTO-LSP-NAVIGATION` | `Public` | `ling.lsp.navigation/0.1` | `Preview` | yes |
| `PROTO-LSP-OVERLAY` | `Public` | `ling.lsp.overlay/0.2` | `Experimental` | yes |
| `PROTO-LSP-PREPARE-RENAME` | `Public` | `ling.lsp.prepare-rename/0.1` | `Preview` | yes |
| `PROTO-LSP-PUBLISH-DIAGNOSTICS` | `Public` | `ling.lsp.publish-diagnostics/0.2` | `Experimental` | yes |
| `PROTO-LSP-PULL-DIAGNOSTICS` | `Public` | `ling.lsp.pull-diagnostics/0.2` | `Preview` | yes |
| `PROTO-LSP-REFERENCES` | `Public` | `ling.lsp.references/0.1` | `Preview` | yes |
| `PROTO-LSP-RENAME` | `Public` | `ling.lsp.rename/0.1` | `Preview` | yes |
| `PROTO-LSP-WORKSPACE` | `Public` | `ling.lsp.workspace/0.1` | `Experimental` | yes |
| `PROTO-PACKAGE-IDENTITY` | `Public` | `v1 domain encodings` | `Experimental` | yes |
| `PROTO-PACKAGE-MANIFEST` | `Public` | `ling.manifest/1` | `Experimental` | yes |
| `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` | `Public` | `ling.semantic/0.2` | `Experimental` | yes |
| `PROTO-PROJECT-CHECK` | `Public` | `ling.project.check/0.1` | `Experimental` | yes |
| `PROTO-REPL-JSON` | `Public` | `ling.repl/0.1` | `Preview` | yes |
| `PROTO-REPLAY` | `Planned public` | — | `Future` | no |
| `PROTO-SEMANTIC-GRAPH-JSON` | `Public` | `ling.semantic/0.1` | `Experimental` | yes |
| `PROTO-SEMANTIC-ID` | `Public` | `experimental:blake3:` | `Experimental` | yes |
| `PROTO-SEMANTIC-QUERY` | `Public` | `ling.semantic-query/0.1` | `Preview` | yes |
| `PROTO-SEMANTIC-TRANSACTION` | `Public` | `ling.semantic-transaction/0.1` | `Preview` | yes |
| `PROTO-SEMANTIC-TRANSACTION-RESULT` | `Public` | `ling.semantic-transaction-result/0.1` | `Preview` | yes |
| `PROTO-VM-CONTROL` | `Public` | `ling.vm.control/0.1` | `Experimental` | yes |

## Explicitly unsupported

| ID | Area | Capability | Reason | Blockers | Sources |
| --- | --- | --- | --- | --- | --- |
| `UNSUP-CONCURRENCY-REPLAY` | Concurrency/runtime | Task, Actor, remote delivery, supervision, or deterministic Replay | The corresponding semantics and runtime protocols remain future work. | `GAP-STRUCTURED-TASK-001`<br>`GAP-ACTOR-AWAIT-REENTRY-001`<br>`GAP-ACTOR-REMOTE-DELIVERY-001`<br>`GAP-ACTOR-MAILBOX-SUPERVISOR-001`<br>`GAP-DETERMINISTIC-REPLAY-001` | [`docs/governance/gap-register.toml`](../governance/gap-register.toml) |
| `UNSUP-CRITICAL` | Critical | Critical Core, Node timing guarantees, Contract verification, model checking, or evidence bundles | The minimum verifiable Critical boundary remains unresolved and unimplemented. | `GAP-CRITICAL-PROFILE-001` | [`docs/governance/gap-register.toml`](../governance/gap-register.toml) |
| `UNSUP-DEVICE` | Heterogeneous | Kernel, SIMD, GPU, TPU/NPU, Device Buffer, or Placement | No accepted Kernel/device subset, capability discovery, or backend exists. | `GAP-KERNEL-DEVICE-001` | [`docs/governance/gap-register.toml`](../governance/gap-register.toml) |
| `UNSUP-LSP-EDITOR` | Tooling | LSP document features, Zed extension, or semantic mutation | RFC-0004, RFC-0023, RFC-0026, and RFC-0032 implement the bounded lifecycle, document overlay, whole-document formatting response, and deterministic push diagnostics. Pull diagnostics, navigation, remaining document features, range/on-type formatting, Zed integration, semantic mutation, and general transaction boundaries remain unresolved. | `GAP-LSP-TRANSACTION-PROTOCOL-001`<br>`GAP-FORMATTER-AUTHOR-SOURCE-001`<br>`GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` | [`docs/governance/gap-register.toml`](../governance/gap-register.toml)<br>[`docs/RFC-0032.md`](../RFC-0032.md) |
| `UNSUP-NATIVE-FFI` | Native | Native code generation, stable ABI, FFI, or target artifacts | Ownership, target, ABI, and FFI contracts are unresolved and unimplemented. | `GAP-OWNERSHIP-MODEL-001`<br>`GAP-NATIVE-BACKEND-ABI-001` | [`docs/governance/gap-register.toml`](../governance/gap-register.toml) |
| `UNSUP-PACKAGES` | Project/package | Package installation, publication, or registry distribution | Accepted DEC-0228 defers registry behavior through Ling 1.0. RFC-0002 local manifest, content identity, vendored dependency, project-check, and lock workflows remain available at their recorded stability, while publication, installation, and registry distribution remain Unsupported. | — | [`docs/RFC-0002.md`](../RFC-0002.md)<br>[`docs/decisions/0228-registry-deferred-through-v1.md`](../decisions/0228-registry-deferred-through-v1.md)<br>[`docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md`](../ling_execution_plan/10-G6-V1.0-STABILIZATION.md) |
| `UNSUP-PROFILE-SELECTION` | Profile | Explore, Native, or Critical selection/enforcement | The Seed CLI has no profile option and no profile validation pass. | `GAP-CRITICAL-PROFILE-001`<br>`GAP-NATIVE-BACKEND-ABI-001` | [`docs/SEMANTICS.md`](../SEMANTICS.md)<br>[`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md) |
| `UNSUP-SUPPORT-CLI-JSON` | CLI/protocol | ling version --format json and ling support --format json | GOV-0108 generates internal governance fixtures only; no Accepted public JSON contract or CLI command exists. | — | [`docs/ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md`](../ling_execution_plan/02-G0-GOVERNANCE-AND-COMPATIBILITY.md) |
| `UNSUP-VM` | Execution | CLI bytecode emission/loading and VM backend selection | VM-1201 through VM-1210 provide Experimental versioned bytecode, verifier-gated library execution, differential/resource/cancellation evidence, and the separate ling.vm.control/0.1 host API. No Accepted CLI bytecode command, backend selector, default execution limits, or full-Seed VM lowering contract exists. | — | [`docs/RFC-0014.md`](../RFC-0014.md)<br>[`docs/RFC-0020.md`](../RFC-0020.md)<br>[`crates/ling-bytecode/src/lib.rs`](../../crates/ling-bytecode/src/lib.rs)<br>[`crates/ling-vm/src/lib.rs`](../../crates/ling-vm/src/lib.rs)<br>[`tests/bytecode/README.md`](../../tests/bytecode/README.md)<br>[`docs/ling_execution_plan/03-G1-V0.1-LIVING.md`](../ling_execution_plan/03-G1-V0.1-LIVING.md) |

## Tier policy

- `Tier1`: Release-blocking build, full conformance, published artifacts, and an explicit compatibility commitment.
- `Tier2`: Build and full workspace tests run in CI, but no downloadable-artifact or long-term compatibility commitment is made.
- `Tier3`: Best-effort build evidence only; failures do not block release.
- `Unsupported`: No implementation or compatibility claim; callers must not infer fallback.

## Future CLI fixtures

The checked-in JSON files are internal `ling.governance.*` fixtures with `implemented: false`. They do not create `ling version` or `ling support`, and they are not public compatibility contracts. A later accepted CLI/protocol task must define and migrate any public schema.

```text
cargo xtask support verify
cargo xtask support render
cargo xtask support render-version-fixture
cargo xtask support render-support-fixture
```
