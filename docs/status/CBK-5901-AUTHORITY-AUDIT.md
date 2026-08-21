# CBK-5901 Authority Audit

- Task: `CBK-5901` — Trusted Compiler Route Decision
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:585-597`
- Release: G5
- Status: `BlockedSpec`

## Decision

CBK-5901 is `BlockedSpec`. The execution plan asks the project to choose among
a verified restricted backend, translation validation, proof-producing
lowering, a controlled C subset bridge, and target-machine-code verification,
with an initial small Critical Core and limited target. It identifies
RFC-K508 as a dependency but does not provide a decision, selection criteria,
target contract, trust model, or compatibility consequences.

No accepted RFC-K508 or replacement authorizes a Native/Critical compiler route.
The Seed implementation boundary explicitly excludes Native Backend, and the
support matrix marks Native and Critical profiles/targets unavailable. Choosing
a route now would invent ABI, ownership, layout, FFI, Fault, target, proof,
and TCB semantics and would create an unsupported public capability.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:585-597` is a non-normative proposal. It does not
  define the meaning of “verified”, the trusted computing base, accepted
  machine-code equivalence, external verifier trust, target scope, or the
  semantics-preserving boundary for C/Native bridges.
- `docs/IMPLEMENTATION.md:17` explicitly excludes Native Backend, Resource/
  Borrow Checker, Task/Actor/Node/Kernel, proof tooling, and related future
  capabilities from the Seed engineering target. No route decision may enlarge
  v0.0.1 without an Accepted authority.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open. It leaves Native IR validity,
  layout, ABI, unwinding/Fault, threads/reentry, typed FFI, Target Primitive
  Packages, and target tiers unresolved; its candidate RFC-0011 is not
  present or Accepted. `GAP-OWNERSHIP-MODEL-001`, `GAP-KERNEL-DEVICE-001`,
  and `GAP-CRITICAL-PROFILE-001` leave dependent memory, device, profile, and
  evidence boundaries open.
- `PROTO-ABI` is Planned public/Future with no version, schema, canonical
  encoding, reader/writer, migration policy, or fixtures. It explicitly says
  that layout, calling convention, ownership transfer, exceptions/Faults,
  target identity, and symbol versioning require accepted RFCs.
- The support matrix records Native and Critical profiles as unavailable and
  `TARGET-NATIVE-AOT`/`BACKEND-NATIVE` as Unsupported, with no committed Ling
  Native target. These are current support constraints, not an implementation
  route decision.
- Accepted RFC-0014 through RFC-0020 authorize the portable bytecode/VM path,
  checked-snapshot boundary, VM verification, differential evidence, and host
  cancellation/resource evidence. RFC-0014 explicitly excludes native ABI,
  linker, build artifact identity, and source-level Native features; none
  authorizes a Native or proof-producing backend.
- `ROADMAP-1.0.md:324-379` makes Native/FFI a future G3 gate and requires
  ownership, ABI, target, TCB, and cross-engine differential evidence before a
  Native backend can be supported. The roadmap is Planning authority.

## Evidence in this repository

There is no Native IR, backend selector, compiler-route decision, target package,
ABI/FFI contract, translation validator, proof-producing lowering, machine-code
verifier, controlled-C bridge, or Critical backend fixture under `crates/`,
`tests/`, or `schemas/`. Existing bytecode lowering/verifier code is the
accepted VM path and cannot be relabeled as Native or Critical. No `ling` CLI,
LSP request, diagnostic, or public protocol claims CBK-5901 support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. The selected compiler route and initial scope, including supported Core
   constructs, targets/profiles, route alternatives rejected, compatibility
   and migration, and the criteria for expanding beyond the first target.
2. Native/backend-neutral IR, layout, calling convention, ownership/resource,
   effects/capabilities, FFI, unwinding/Fault, threading/reentry, startup and
   shutdown semantics, and Target Primitive Package boundaries.
3. A sound trust/equivalence model for verified lowering, translation
   validation, proof-producing passes, controlled C bridges, or machine-code
   verification: proof obligations, certificates, independent checker, TCB,
   assumptions, optimization boundaries, and fail-closed behavior.
4. Target/toolchain/profile identity and reproducible/offline build rules,
   artifact/digest domains, external tool versions, source/binary mapping,
   accepted nondeterminism, and cross-target differential evidence. Host paths,
   addresses, Rust layout, timestamps, and debug output must not become Ling
   identity.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and schemas for
   unsupported constructs, invalid lowering, ABI/target mismatch, proof or
   certificate failure, unavailable verifier, unsafe bridge, and resource or
   TCB violations.
6. Offline positive, negative, malformed, target/ABI, ownership/FFI,
   differential, proof/translation, reproducibility, Unicode 17.0.0,
   BOM/CRLF, source-span, and repeated-build fixtures. No route may claim
   Native or Critical support before its support-matrix evidence is complete.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no Native backend,
IR, ABI/FFI dependency, target package, proof checker, compiler route,
diagnostic, CLI command, or placeholder API, and it introduces no stale `zero`
names.

CBK-5901 remains deferred until the Native/ABI, ownership, Critical Profile,
kernel/device, Contract/Proof, evidence, and reproducible-build authorities are
Accepted with executable fixtures and a truthful support-matrix update.
