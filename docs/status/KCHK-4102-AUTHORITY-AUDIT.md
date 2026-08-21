# KCHK-4102 Authority Audit — Kernel Effect and Capability Checks

Status: BlockedSpec

Date: 2026-08-22

## Outcome

KCHK-4102 proposes a restricted Effect and Capability set for Kernel code:
Pure, DeviceRead over a buffer, DeviceWrite over a buffer, and a possible
DeviceBarrier. It also proposes rejecting Clock, Network, File, Actor,
Console, and ordinary Random effects unless a deterministic RNG Kernel model
is accepted.

The proposal cannot be implemented as a checker or public semantic surface.
No Kernel effect vocabulary, capability environment, verifier pass, Graph
field, diagnostic, Device Buffer API, deterministic RNG, barrier operation,
backend hook, or command is added. KCHK-4101 and the open Kernel/device
authority remain prerequisites.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:88-99 is a
  non-normative plan sketch. It lists example labels but does not define
  Effect Row parameterization, capability issuance, effect elimination,
  synchronization, failure, or compatibility behavior. Its RFC dependency
  is unresolved.
- docs/SEMANTICS.md:1045-1101 defines the general Effect Row, Pure, and
  allocation concepts, while docs/SEMANTICS.md:1118-1170 defines the
  general Capability boundary. These sections do not accept DeviceRead,
  DeviceWrite, DeviceBarrier, deterministic RNG, or Kernel-specific
  capability issuance.
- docs/SEMANTICS.md:1429-1480 describes future Kernel purity, explicit
  device capabilities, Device Buffers, and determinism. It is a design
  description, not an accepted Kernel Effect/Capability contract.
- docs/SEMANTICS.md:1872-1928 fixes the v0.0.1 formal Effect and Capability
  subset to Pure and Console.Write and explicitly reserves Kernel in the
  non-implemented list. No Seed checker extension is authorized.
- docs/ROADMAP-1.0.md:381-429 makes Kernel Effect/Capability restrictions a
  v0.4/G4 specification gate and requires rejection before backend
  compilation. The roadmap does not supply the missing normative rules.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml and
  explicitly leaves allowed Kernel effects, device capabilities, ownership,
  synchronization, numeric determinism, and backend discovery unresolved.
  GAP-EFFECT-HANDLER-001 and GAP-EFFECT-STATE-MASKING-001 are also Open for
  general handler/elimination and effect-visibility interactions.
- RFC-0013 is only a candidate in the gap register and is absent as an
  accepted file. RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md. No Kernel Effect/Capability
  protocol is registered in docs/governance/protocol-inventory.toml.

## Current implementation evidence

- The repository has no Kernel, Device Buffer, effect-capability checker,
  barrier primitive, or deterministic RNG implementation under crates or
  tests. The Seed pipeline only supports its accepted Pure and Console.Write
  boundary.
- No accepted rule fixes whether DeviceRead and DeviceWrite are parameterized
  by buffer identity, address space, view mutability, or ownership; whether a
  barrier is an Effect, Capability, or verifier annotation; or how effects
  cross Kernel calls, transfers, reductions, and CPU fallback.
- No accepted rule defines handler/elimination interaction, capability
  delegation or revocation, effect polymorphism, deterministic RNG state,
  resource limits, cancellation, DeviceFault propagation, or target
  capability discovery. Backend logs cannot substitute for source-level
  diagnostics.
- The support matrix marks Kernel CPU, GPU, and accelerator backends
  Unsupported and Experimental behind GAP-KERNEL-DEVICE-001. No diagnostic
  allocation, schema, Semantic ID rule, dependency, toolchain, public
  protocol, or CLI command is required or changed by this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel RFC (RFC-0013 or an accepted replacement) and the
   KCHK-4101 matrix that define the complete Kernel Effect vocabulary,
   parameterization, purity, allocation, call boundaries, and profile/target
   support.
2. A capability judgment covering issuance, imports, delegation, ownership,
   address spaces, buffer views, DeviceRead/DeviceWrite exclusivity,
   DeviceBarrier ordering, transfer and synchronization capabilities, and
   rejection of forged or hidden capabilities.
3. Effect Row rules for Kernel calls, higher-order and polymorphic code,
   handlers/elimination, deterministic RNG, CPU fallback, cancellation,
   DeviceFault and resource-limit paths. The rules must state which
   operations are forbidden rather than relying on backend compilation.
4. A verifier boundary that consumes checked Typed Core or a versioned
   verified derivative, preserves original UTF-8 byte spans and Semantic IDs,
   rejects unsupported effects before lowering, and does not interpret
   unchecked AST nodes.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics with structured facts
   for forbidden effects, missing capabilities, buffer/view mismatch,
   barrier ordering, target/profile mismatch, and unsupported fallback.
6. Executable positive and negative fixtures for effect unions, nested calls,
   capability minimization and forgery, handler interaction, deterministic
   RNG, buffer read/write/transfer/barrier ordering, faults, cancellation,
   CPU-reference equivalence, migration, Unicode source mapping, and
   deterministic canonical evidence.

## Evidence and compatibility impact

The eventual checker must publish a versioned machine-readable effect and
capability schema, canonical ordering and migration tests, and Graph/Audit
projections that identify effect and capability provenance without exposing
host addresses, driver text, allocation order, or timing as language
semantics. Device and CPU paths need differential fixtures with exact or
tolerance-based numeric rules, and unsupported targets must reject or use a
specification-permitted fallback visibly. Any public effect/capability
protocol requires an inventory entry, Accepted authority, reader/writer
fixtures, and an explicit Preview or Stable status.

This audit changes no Effect Row implementation, Capability environment,
Typed Core, evaluator, bytecode, VM, Native backend, scheduler, memory
category, ownership behavior, Device Buffer, diagnostics, schema, Semantic
IDs, source spans, CLI, dependency lock, target/toolchain, or Unicode 17.0.0
behavior.

## Intentionally deferred

KCHK-4102 implementation, Kernel Effect and Capability definitions, verifier
rules, deterministic RNG, barrier semantics, diagnostics, Graph/Audit fields,
CPU reference, Device IR, device backends, editor support, and public
protocol claims remain deferred until the Kernel/device and effect-system gaps
are resolved by Accepted authorities and executable evidence.
