# CPU-4201 Authority Audit — Scalar Reference Backend

Status: BlockedSpec

Date: 2026-08-22

## Outcome

CPU-4201 proposes a direct scalar reference path for the future Kernel
subset, covering element-wise map, multidimensional indexing, conditionals,
bounded loops, buffer reads/writes, reductions, and explicit Faults. The plan
describes this path as an oracle for later SIMD and device backends.

The task cannot be implemented as a backend or semantic oracle today. No
Kernel execution entry point, scalar backend, Device Buffer model, reference
Fault mapping, reduction behavior, trace schema, diagnostic, target, or
command is added. A reference implementation without an Accepted Kernel
contract would silently choose the language semantics it is supposed to
measure.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:137-151 is a
  non-normative implementation plan. It lists an implementation order but
  does not define Kernel syntax, Typed Core input, shapes, effects,
  ownership, numeric modes, Faults, determinism, target capabilities, or
  the oracle equivalence relation.
- docs/ROADMAP-1.0.md:381-429 places the CPU reference after the G4 Kernel
  specification gate and requires it to cover indexing, bounds, buffer
  access, map/reduce, Faults, and differential evidence. The roadmap does not
  authorize a backend before those contracts are Accepted.
- docs/SEMANTICS.md:1429-1480 sketches future Kernel and Device Buffer
  behavior, including deterministic reductions and scalar lowering, while
  docs/SEMANTICS.md:1872-1928 excludes Kernel and Native backends from the
  v0.0.1 formal subset. Existing scalar Interpreter/VM semantics are not a
  Kernel reference contract.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml and
  explicitly requires an Accepted Kernel authority plus a CPU reference
  corpus before KCHK-4101 implementation. It leaves types, effects,
  ownership, synchronization, numeric determinism, Placement, and backend
  discovery unresolved.
- docs/governance/support-matrix.toml marks BACKEND-KERNEL-CPU as
  Unsupported and Experimental with GAP-KERNEL-DEVICE-001. No CPU Kernel
  protocol or backend target is registered as supported.
- RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md. RFC-0014/0018/0019 cover
  scalar VM/bytecode foundations and do not define a Kernel execution or
  differential oracle.

## Current implementation evidence

- The repository has no Kernel, Device Buffer, scalar Kernel evaluator,
  reference backend, reduction implementation, or Kernel Fault path under
  crates or tests. The Seed CLI exposes no Kernel target or profile.
- No accepted rule fixes the input artifact and trust boundary, work-item
  ordering, shape/index representation, read/write ownership, loop bounds,
  reduction associativity/order, floating-point mode, allocation/resource
  limits, cancellation, or host/device Fault equivalence.
- The existing Interpreter and VM cannot serve as this backend without
  inventing how future Kernel constructs lower into scalar execution. Their
  accepted evidence must not be extended into an unaccepted device contract.
- No diagnostic allocation, Graph/Audit schema, Semantic ID rule, dependency,
  target/toolchain, public protocol, or CLI command is required or changed
  by this audit. Stale plan commands are not introduced.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel RFC and CPU-reference contract defining the accepted
   Typed Core or verified Kernel artifact, supported types/control flow,
   effects/capabilities, shapes/indexes, buffers/views, ownership, and
   allowed profiles/targets.
2. A deterministic execution model for work-item ordering, loop and
   reduction semantics, atomics/barriers if present, numeric modes and
   tolerances, allocation/resource limits, cancellation, and Fault
   propagation. The oracle relation to future SIMD/device paths must be
   explicit and must not hide permitted differences.
3. A verifier boundary that consumes checked Typed Core or a versioned
   verified derivative, rejects invalid Kernel artifacts before execution,
   preserves original UTF-8 byte spans and Semantic IDs, and never
   interprets unchecked AST nodes.
4. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics and structured facts for
   unsupported Kernel constructs, shape/index/bounds failures, alias/race
   violations, numeric/reduction mismatch, capability/target mismatch,
   resource exhaustion, and explicit Faults.
5. Versioned reference-output/evidence schemas, canonical ordering,
   migration and corruption tests, source-map and Unicode fixtures, and
   exact or tolerance-based differential rules for each later backend.
6. Positive and negative Kernel corpus fixtures covering map, indexing,
   conditionals, bounded loops, buffer access, reductions, invalid bounds,
   alias conflicts, floating-point edges, Faults, cancellation, and
   deterministic offline reproduction.

## Evidence and compatibility impact

The eventual backend must be a simple, auditable consumer of verified
artifacts, not a second semantic interpreter. It must publish deterministic
reference results and optional non-stable traces without exposing host
addresses, allocation order, wall-clock timing, or debug output as Ling
semantics. CPU/reference and every supported lowering require differential
fixtures with declared exact or tolerance rules, migration evidence, and
explicit Unsupported or Experimental status where coverage is absent.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, effect or capability checker,
Device Buffer, scheduler, diagnostics, schema, Semantic IDs, source spans,
CLI, dependency lock, target/toolchain, support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

CPU-4201 implementation, Kernel scalar semantics, Device Buffer operations,
reference reductions, Fault mapping, reference output/trace schemas,
differential harnesses, SIMD/GPU/accelerator backends, editor support, and
public protocol claims remain deferred until GAP-KERNEL-DEVICE-001 and the
preceding Kernel authorities are Accepted and the required executable corpus
exists.
