# KCHK-4103 Authority Audit — Shape, Index, and Bounds

Status: BlockedSpec

Date: 2026-08-22

## Outcome

KCHK-4103 proposes Kernel checking for shape and type representation, static
and dynamic dimensions, index ranges, bounds checks or proof-based
elimination, overflow, and a common host/device Fault model. It also requires
source-level diagnostics that do not depend on backend logs.

The task is not implementable as a checker or public language surface yet. No
shape type, dimension representation, index verifier, bounds pass, overflow
policy, host/device Fault mapper, diagnostic, Device Buffer API, backend hook,
or command is added. The Kernel/device and numeric Fault authorities remain
open.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:101-110 is a
  non-normative plan sketch. It lists desired checks but does not define shape
  identity, dimension arithmetic, index semantics, proof obligations,
  overflow behavior, Fault compatibility, or schema migration.
- docs/ROADMAP-1.0.md:381-429 places Kernel bounds, Device Buffer, and Fault
  behavior behind the v0.4/G4 specification gate. It requires rejection
  before backend lowering and CPU-reference/differential evidence, but is not
  an Accepted shape or bounds authority.
- docs/SEMANTICS.md:820-860 defines general Fault categories including
  BoundsViolation, IntegerOverflow, and DeviceFault. docs/SEMANTICS.md:909-947
  describes mathematical and fixed-width numeric behavior, but the
  fixed-width Fault visibility is explicitly unresolved. The Kernel section
  at docs/SEMANTICS.md:1429-1480 only sketches Device Buffers and lowering.
- docs/SEMANTICS.md:1872-1928 fixes the v0.0.1 formal subset and reserves
  Kernel, Resource/Borrow, and Native behavior; it does not authorize shape
  syntax or a bounds checker.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml and
  blocks KCHK-4103 through the missing Kernel types, buffer/address-space,
  synchronization, determinism, and backend contracts.
- GAP-NUMERIC-CHECKED-FAULT-001 is Open and leaves fixed-width overflow
  visibility in signatures and effect/failure analysis undecided. No
  accepted RFC-0013 or numeric replacement resolves these dependencies.
- RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md. The plan's RFC-H401 dependency
  and any shape/bounds RFC are not accepted authorities, and no Kernel
  protocol is registered.

## Current implementation evidence

- The repository has no Kernel, shape, dimension, Device Buffer, index, bounds,
  or capability verifier implementation under crates or tests. The Seed
  pipeline has no Kernel entry point or host/device lowering.
- No accepted rule fixes whether shapes are nominal or structural, whether
  dimensions are values or types, how unknown dimensions are represented, how
  linearization and integer conversion are defined, or how slices/views carry
  shape and address-space identity.
- No accepted proof judgment fixes static versus dynamic bounds, loop/index
  coupling, overflow in shape products or offsets, integer-width conversion,
  empty dimensions, negative indices, broadcasting, or out-of-range Fault
  provenance. No rule maps host and device failures to the same stable
  language categories.
- The support matrix marks Kernel CPU, GPU, and accelerator backends
  Unsupported and Experimental behind GAP-KERNEL-DEVICE-001. No diagnostic
  allocation, schema, Semantic ID rule, dependency, target/toolchain, or CLI
  command is required or changed by this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel RFC (RFC-0013 or an accepted replacement) defining shape,
   dimension, index, layout, address-space, view, and buffer identities and
   their Typed Core representations.
2. A precise bounds judgment for static and dynamic dimensions, loop/index
   relationships, slicing, empty shapes, negative and converted indices,
   shape products and offset arithmetic, overflow, and permitted proof
   erasure. The judgment must state its resource and recursion limits.
3. A Fault contract distinguishing compile-time rejection, checked runtime
   BoundsViolation, IntegerOverflow, DeviceFault, cancellation, and resource
   failure. Host/device equivalence, provenance, recovery, and effect/
   capability visibility must be explicit.
4. A verifier boundary that consumes checked Typed Core or a versioned verified
   derivative, rejects unsupported access before backend compilation,
   preserves original UTF-8 byte spans and Semantic IDs, and never evaluates
   unchecked AST nodes.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics with structured facts for
   shape mismatch, unknown dimensions, invalid index, failed proof, overflow,
   unsupported target, and backend-independent Fault mapping.
6. Positive and negative fixtures for rank/shape/index/layout, static and
   dynamic checks, slices and views, empty and overflow cases, proof
   elimination, host/device Faults, source-map positions, Unicode identifiers,
   CPU-reference equivalence, migration, resource limits, and deterministic
   canonical evidence.

## Evidence and compatibility impact

The eventual implementation must publish a versioned shape/index/bounds
schema, canonical ordering and migration tests, and Graph/Audit projections
that retain shape and Fault provenance without exposing host addresses,
backend logs, allocation layout, or timing as Ling semantics. It must compare
CPU reference and device behavior under the accepted exact or tolerance rules,
show unsupported targets explicitly, and classify all failures with stable
diagnostics. Any public shape or bounds protocol needs an inventory entry,
Accepted authority, reader/writer fixtures, and an explicit Preview or Stable
status.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, Device Buffer, effect or
capability checker, diagnostics, schema, Semantic IDs, source spans, CLI,
dependency lock, target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

KCHK-4103 implementation, shape and dimension types, index lowering, bounds
proofs, overflow policy, host/device Fault mapping, diagnostics, Graph/Audit
fields, CPU reference, Device IR, device backends, editor support, and public
protocol claims remain deferred until GAP-KERNEL-DEVICE-001 and
GAP-NUMERIC-CHECKED-FAULT-001 are resolved by Accepted authorities and the
required executable evidence exists.
