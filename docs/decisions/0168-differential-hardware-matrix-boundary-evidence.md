# DEC-0168: Internal differential and hardware-matrix boundary evidence / 内部 differential 与 hardware matrix 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: differential-quality  
> 相关规范/缺口：`DEC-0167` | `DEC-0166` | `DEC-0165` | `DEC-0164` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`GPU-4604-OBSERVATION` differential and hardware-matrix boundary. It records
provisional CPU/GPU references, checked artifacts, numeric comparison,
combination identity, evidence provenance, stability states, fixtures,
diagnostics, and local-machine redaction vocabulary while Kernel, GPU runtime,
numeric, and support-matrix authorities remain unresolved.

本决定只授权 `GPU-4604-OBSERVATION` 使用 test-local 的拟议 differential 与 hardware matrix 边界清单，
在 Kernel、GPU runtime、numeric 与 support-matrix 权威尚未解决时，只记录临时 CPU/GPU reference、checked artifact、numeric comparison、combination identity、evidence provenance、stability state、fixture、diagnostic 与 local-machine redaction 词汇。

## Question

GPU-4604 proposes stable combination records for OS, GPU, architecture,
runtime/driver, backend compiler, numeric mode, and known limitations, with CPU
reference comparison and explicit Experimental status for uncovered
combinations. Which planning vocabulary can be retained as bounded evidence
without defining numeric equivalence, a matrix schema, or a stable support
claim?

## Decision

1. `crates/ling-types/tests/differential_hardware_matrix_evidence.rs` keeps a
   test-local inventory of sixty provisional reference, artifact, corpus,
   numeric, combination, evidence, support-state, fixture, diagnostic,
   redaction, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.differential-hardware-matrix-observation/0`.
   They are not CPU/GPU oracles, numeric comparators, hardware identities,
   matrix records, support claims, diagnostics, public protocols, or target
   guarantees.
3. No differential harness, matrix schema, comparator, tolerance registry,
   hardware claim, dependency, target package, toolchain probe, diagnostic,
   protocol, or placeholder API is added. Public `GPU-4604` remains
   `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:378-392` is
  non-normative; matrix fields and CPU/GPU comparison do not define identity,
  numeric equivalence, tolerance, driver/toolchain policy, or stability gates.
- `docs/ROADMAP-1.0.md:381-431` requires CPU-reference/device differential
  tests and a support matrix of verified combinations and Experimental
  backends; it does not authorize a matrix schema or stable GPU combination.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4603 remain
  `BlockedSpec`; RFC-0013 and RFC-H404 are not Accepted. The Kernel/device and
  Native/backend gaps remain open, and CPU/GPU backend entries are not
  implemented support.

## Conformance plan

- Assert all sixty differential/matrix boundaries and local order; compare
  forward/reverse opaque bytes; reject duplicates.
- Defer CPU/GPU oracle relations, numeric comparator/tolerance, combination
  identity, evidence expiry, support-state transitions, diagnostics, and
  protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no differential harness, hardware matrix, stable combination, or
public support claim exists.

## Unresolved alternatives

CPU reference and GPU oracle relation; checked artifact and Kernel/Device IR
boundary; input corpus, seed, work-item and reduction ordering; numeric mode,
precision, rounding, NaN/signed-zero, overflow, tolerance, exact equality and
Fault equivalence; canonical combination identity for OS/device/vendor/
architecture/runtime/driver/backend compiler/toolchain/features/layout/limits;
known limitations, provenance, expiry, evidence bundle and reproducibility;
Unsupported/Experimental/Preview/Stable lifecycle, fallback/rejection;
positive/negative/determinism/malformed/source-map/Unicode/differential/
cross-target/resource/Fault/migration fixtures; diagnostics, local-machine,
path/address/timestamp/driver-text redaction, protocol inventory, and public
matrix status remain open under GPU-4604, GPU-4601 through GPU-4603,
DIR-4501 through DIR-4503, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
