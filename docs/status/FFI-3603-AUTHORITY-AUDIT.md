# FFI-3603 Authority Audit — FFI Shim Generator

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

FFI-3603 proposes generated shims for layout assertions, bounds/null checks,
ownership conversion, string encoding, callback trampolines, Fault mapping,
Capability checks, and audit metadata. It also requires generated provenance
to participate in the build hash. These are execution-plan requirements, not
an accepted code-generation, ABI, safety, or evidence protocol.

No shim generator, template, generated C/Rust source, layout assertion,
pointer check, ownership adapter, encoding policy, callback trampoline,
Fault/capability bridge, provenance record, build-hash input, or public
generator API is added. Seed compilation and execution remain unchanged until
the declaration, C ABI, ownership, Native backend, target, and evidence
authorities are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:445-458` is non-normative.
  It lists generated responsibilities but does not define template inputs,
  generated-language ABI, trust boundaries, failure behavior, ownership
  conversions, string encodings, or canonical provenance/build-hash bytes.
- `docs/SEMANTICS.md:1831-1868` requires future FFI ABI, ownership, lifetime,
  mutability, thread/reentry, Error/Fault, Capability, Target, and validation
  contracts and limits primitives to trusted Target Packages. It does not
  authorize generated shims or make generator output language semantics.
- `docs/LANGUAGE.md:1179-1191` describes provenance for AI/tool changes and
  `:1276-1287` describes Typed FFI requirements; neither fixes a generated
  artifact schema, trust model, or build-hash projection.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open and blocks the FFI declaration,
  C ABI, and target boundary needed by a shim. `GAP-OWNERSHIP-MODEL-001` and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` leave ownership, borrow, drop, Managed,
  Resource, and public lifetime conversion unresolved.
- `docs/governance/protocol-inventory.toml:495-515` registers `PROTO-ABI`
  without a schema or compatibility policy, while `:517-537` registers
  `PROTO-EVIDENCE` without identity, provenance, checksum, signature,
  proof/test-linkage, redaction, reader, writer, migration, or fixture rules.
  Neither protocol authorizes generated output or hash inputs.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no accepted FFI shim input schema, generator, generated
  artifact directory, target package, layout/ownership verifier, callback or
  Fault adapter, provenance schema, or public build-hash protocol. Existing
  code-generation and bytecode tools do not define a Native/FFI shim boundary.
- No accepted rule says whether generated C, Rust, or another target language
  is trusted, independently verified, checked into source, rebuilt during
  compilation, or represented in Semantic IDs. Host templates and generator
  implementation details cannot answer those questions.
- No generated artifact, native compiler, linker, dependency, unsafe surface,
  diagnostic allocation, or public protocol implementation is required for
  this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned, canonical shim input and output schema: declaration/ABI
   version, target identity, layout facts, ownership and capability facts,
   encoding, callback/error rules, source spans/Semantic IDs, generator and
   template identity, and explicit unknown/unsupported behavior.
2. Sound generated checks for layout, null/bounds/overflow, mutability and
   encoding; ownership conversion and allocator/drop behavior; callback
   lifetime, thread/reentry, cancellation and panic/unwind isolation; and
   Error/Fault/Capability normalization. The generated code must consume
   verified declarations and cannot invent facts.
3. Trust and verification boundaries for templates, generated source, C
   headers, compilers/linkers, target packages, and runtime support; offline
   input closure; deterministic ordering; path/timestamp/build-ID exclusion;
   provenance, license/TCB, tamper, and reproducibility rules.
4. The relationship between generated artifacts, Semantic IDs, canonical
   bytes, cache/release identities, and `PROTO-ABI`/`PROTO-EVIDENCE`, including
   versioning, compatibility, migration, redaction, and independent readers.
5. Stable bilingual diagnostics and fixture obligations for malformed metadata,
   unsupported targets/types, failed checks, generator failures, and any
   runtime Fault, without exposing host paths, addresses, allocation order, or
   debug text as Ling semantics.

## Evidence and compatibility impact

The eventual implementation needs golden input/output shims; independent
layout/header/compiler/linker checks; positive and negative null/bounds,
overflow, encoding, ownership/drop, allocator, callback, thread/reentry,
blocking, Fault, capability, and target fixtures; malformed/tampered metadata
tests; generator determinism and clean repeated-build checks; provenance,
license/TCB, checksum and schema migration evidence; sanitizer/fuzz coverage;
cross-target and C-compiler differential tests; and exact generated-source
source-map/UTF-8-span checks. It must preserve stable Semantic IDs, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and Unicode 17.0.0 behavior while keeping
generator output and host details separate from language semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime, or
Unicode behavior. It adds no generator, template, generated artifact,
dependency, toolchain, build-hash input, diagnostic, public protocol
implementation, or placeholder API.

## Intentionally deferred

Shim input/output schemas, layout and safety checks, ownership and allocator
conversion, string encoding, callbacks, Fault/Capability mapping, target and
linker integration, generator trust/TCB, provenance and build-hash projection,
protocol readers/migrations, fixtures, sanitizer/fuzz/differential evidence,
and all generated Native/FFI claims remain deferred until RFC-N305 and the
dependent ownership, Native ABI, target, runtime, `PROTO-ABI`, and
`PROTO-EVIDENCE` authorities are Accepted.
