# KCHK-4101 Authority Audit — Kernel Allowed Capability Matrix

Status: BlockedSpec

Date: 2026-08-22

## Outcome

KCHK-4101 proposes a machine-readable matrix for the language capabilities
accepted by a Kernel subset, with entries for values and records, restricted
ADTs, Managed and Resource values, allocation, recursion, Task/Actor and
network effects, loops, calls, and trait dispatch. The matrix is intended to
feed documentation, Semantic Graph schemas, and compiler tests.

The task cannot be implemented as a language or compiler feature today. No
Kernel matrix schema, Kernel checker, Graph field, compiler pass, diagnostic,
backend capability API, Device Buffer model, or public command is added. The
plan's proposed RFC-H401 dependency and the RFC-0013 Kernel authority are not
present as accepted documents.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:69-83 is a
  non-normative implementation plan. It supplies a table shape and examples,
  but does not define the language meaning of Kernel, the checker boundary,
  versioned schema, diagnostics, or compatibility policy. Its RFC-H401
  dependency is unresolved.
- docs/ROADMAP-1.0.md:381-429 places Kernel, Device Buffer, determinism,
  backend capability, and Placement work in the v0.4/G4 gate. The roadmap
  requires a separate Kernel verification pass and a CPU reference before
  device lowering; it is not an Accepted semantic authority.
- docs/SEMANTICS.md:1429-1480 describes a future Kernel model (parallel
  purity, Device Buffer forms, reduction determinism, and lowering targets).
  docs/SEMANTICS.md:1872-1928 fixes the v0.0.1 formal subset and explicitly
  reserves Kernel in the non-implemented schema list. The descriptive model
  therefore cannot authorize a Seed implementation or a stable matrix.
- docs/LANGUAGE.md presents Kernel and Placement as future language models,
  while the authoritative Seed boundary excludes Kernel. It does not supply
  an accepted capability matrix or device protocol.
- docs/RFC-0001.md:1406 lists RFC-0013 as future Kernel and heterogeneous
  device work. RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md; no RFC-0013 file or accepted
  replacement is registered.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml. It
  blocks KCHK-4101 and related device tasks, identifies the missing allowed
  types/control flow/effects, ownership/address spaces, synchronization,
  numeric/reduction determinism, Placement, and backend discovery, and names
  drafting RFC-0013 plus a CPU reference corpus as the next action.
- docs/governance/support-matrix.toml records Kernel CPU, GPU, and accelerator
  backends as Unsupported and Experimental, all blocked by
  GAP-KERNEL-DEVICE-001. No Kernel protocol is registered in the protocol
  inventory.

## Current implementation evidence

- The repository has no Kernel, Device Buffer, Placement, or capability
  checker implementation under crates or tests. The current Seed compiler
  consequently has no matrix consumer or stable rejection path.
- The existing language and semantic documents mention Kernel only as future
  or reserved surface. v0.0.1 does not implement Kernel, Native backends,
  Resource/Borrow, Task, Actor, or profile enforcement, so implementing a
  matrix would imply unsupported syntax and runtime behavior.
- No accepted rule defines which Typed Core nodes may enter a Kernel, the
  legal value/layout and ADT subset, loop or recursion bounds, allocation
  policy, call/dispatch rule, Effect and Capability rows, buffer ownership or
  address spaces, alias/race proof, numeric mode, reduction order, target
  feature discovery, or fallback behavior.
- No stable diagnostic allocation, Graph/Audit schema version, Semantic ID
  identity, CLI command, backend ABI, dependency, or device toolchain is
  required or changed by this audit. Stale plan names are not introduced as
  implementation commands.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned RFC-0013 (or an explicitly accepted replacement) that resolves
   the RFC-H401 name and defines the Kernel subset, its profiles, and its
   relationship to Typed Core, Native, Device IR, and CPU reference execution.
2. A canonical matrix schema with stable capability identifiers, conditions,
   rejection categories, target/profile scope, source and Semantic ID
   provenance, Graph/Audit projection, canonical bytes, migrations, and
   deterministic ordering. The schema must not expose host paths, addresses,
   driver logs, allocation order, or hash-map order as language semantics.
3. Normative rules for values, fixed layouts, ADTs, Resource and Managed
   boundaries, allocation, recursion and loop bounds, calls and static
   dispatch, forbidden Task/Actor/Network/IO effects, explicit
   Device/Buffer capabilities, alias and race proofs, bounds and overflow,
   synchronization, and declared floating-point/reduction determinism.
4. A compiler/verifier contract that consumes checked Typed Core or a
   versioned verified derivative, rejects unsupported constructs before
   backend compilation, preserves original UTF-8 byte spans and Semantic IDs,
   and never executes or interprets unchecked AST nodes.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics and structured facts for
   every rejection, including profile/target/capability mismatch, with
   registry, schema, and compatibility evidence.
6. CPU scalar reference semantics and positive, negative, bounds, alias/race,
   numeric, migration, Unicode/source-map, determinism, and device-differential
   fixtures before any backend or editor integration can claim support.

## Evidence and compatibility impact

The eventual implementation must provide machine-readable matrix golden files,
round-trip and migration tests, deterministic canonical bytes, source-map
checks for UTF-8 positions, and compiler tests for every accepted and rejected
capability. It must compare CPU reference behavior with each supported
lowering, record exact versus tolerance-based numeric rules, test unsupported
targets and explicit fallback, and keep device faults and vendor details out
of stable language semantics. Any public matrix or capability protocol needs a
protocol-inventory entry, an Accepted authority, reader/writer fixtures, and
an explicit Preview or Stable claim.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, Device Buffer, scheduler,
diagnostic registry, schema, Semantic ID, source span, CLI, support matrix
claim, dependency lock, target/toolchain, or Unicode 17.0.0 behavior.

## Intentionally deferred

KCHK-4101 implementation, the matrix schema, Kernel checker, Graph/Audit
projection, diagnostics, CPU reference, Device IR, SIMD/GPU/accelerator
backends, Placement, editor support, and all device capability claims remain
deferred until GAP-KERNEL-DEVICE-001 is resolved by an Accepted authority and
the required executable evidence exists.
