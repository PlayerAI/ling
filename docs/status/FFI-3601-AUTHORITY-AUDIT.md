# FFI-3601 Authority Audit — FFI Declaration Model

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

FFI-3601 proposes a typed declaration model whose fields include the ABI,
foreign symbol, argument/result layout, ownership transfer, borrow duration,
threading, reentrancy, Error/Fault mapping, Capability, profile availability,
and target constraints. The proposal is an execution-plan item, not an
accepted language or binary-interface specification.

No FFI syntax, declaration AST/HIR, Checked Core node, ABI schema, foreign
symbol resolver, target package, raw-pointer boundary, or public FFI API is
added. The v0.0.1 Seed compiler and its interpreter/VM behavior remain
unchanged until the required ownership, Native ABI, target, and protocol
decisions are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:411-429` is non-normative
  and explicitly depends on the absent RFC-N305. It does not define grammar,
  type-checking, lowering, ABI layout, or foreign-call observability.
- `docs/SEMANTICS.md:1831-1868` lists the information a future FFI boundary
  must expose and limits `primitive` definitions to trusted Target Packages;
  it does not accept a declaration syntax or an executable FFI contract.
- `docs/LANGUAGE.md:1249-1287` describes Hermetic Build Graph and Typed FFI
  requirements, while `docs/SEMANTICS.md:1872-1931` keeps Native backend and
  related features outside the v0.0.1 formal subset. These sections do not
  authorize implementation of the future boundary.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open. It blocks FFI-3601 and records
  Native IR validity, layout, ABI, Fault/unwinding, thread/reentry, FFI,
  Target Primitive, and target-tier behavior as unaccepted. Its next action
  is RFC-0011 after RFC-0007 defines the memory categories exposed to Native
  and FFI.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` remain
  Open. Copy/move, borrow, region, drop, Managed/Resource boundaries, and
  public lifetime representation therefore cannot be selected by an FFI
  implementation.
- `docs/governance/protocol-inventory.toml:495-515` registers `PROTO-ABI` as
  Planned public with no schema, version, reader/writer policy, migration
  tool, or fixtures. It explicitly says that layouts, calling convention,
  ownership transfer, exceptions/Faults, target identity, and symbol
  versioning require accepted RFCs.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no accepted FFI declaration node, ABI schema, foreign
  symbol resolver, target-primitive package, linker adapter, or executable
  FFI call boundary. Existing `ling-syntax`, `ling-ast`, `ling-hir`, type
  checking, bytecode, interpreter, and VM code implements the Seed boundary,
  not an FFI protocol.
- `primitive`, `target`, and related Native/FFI terms are reserved or
  descriptive in the language documents; reservation does not make a public
  declaration legal. v0.0.1 must reject unimplemented features with a stable
  diagnostic rather than silently treating them as foreign calls.
- No FFI diagnostic allocation, schema version, target matrix, dependency,
  toolchain, linker, generated binding, or unsafe/raw-pointer surface is
  present or required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Declaration grammar, name/symbol identity, module/package visibility,
   source-span and Semantic-ID projection, and the boundary between Author,
   Audit, and Typed FFI views.
2. A versioned platform-neutral ABI contract: scalar and aggregate layout,
   alignment, endianness, calling convention, symbol/version rules,
   fixed-layout records, spans, callbacks, opaque handles, allocator pairs,
   unsupported constructs, and target-specific extensions.
3. Ownership, borrow duration, mutability, aliasing, Resource/Managed
   lifetime, drop/callback behavior, thread safety, blocking, cancellation,
   reentrancy, and cross-boundary exception/Fault rules, as authorized by the
   accepted ownership RFC.
4. Error/Fault normalization, capability requirements, profile availability,
   target constraints, foreign-code trust/TCB and license/provenance policy,
   and deterministic compile-time rejection behavior.
5. Checked Core lowering and runtime/linker interfaces that consume verified
   declarations without re-running or inventing semantics, plus the
   versioned `PROTO-ABI` schema, compatibility/migration rules, and executable
   fixtures.

## Evidence and compatibility impact

The eventual implementation needs positive and negative declaration/parser and
type-check fixtures; layout and calling-convention conformance; ownership,
borrow, lifetime, allocator, callback, thread/reentry, blocking, and
Error/Fault cases; capability/profile/target rejection; symbol/version and
schema migration tests; sanitizer/fuzz coverage; cross-target and foreign
compiler differential evidence; and deterministic diagnostics. It must retain
original UTF-8 byte spans, stable Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>`
diagnostics, and Unicode 17.0.0 behavior without exposing Rust layout,
addresses, paths, hash-map order, or host ABI as Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime, or
Unicode behavior. It adds no FFI syntax, ABI schema, diagnostic, dependency,
toolchain, target package, public protocol implementation, or placeholder API.

## Intentionally deferred

FFI declaration grammar and AST/HIR/Checked Core nodes, ABI and layout
verification, symbol resolution, ownership/borrow/lifetime lowering, foreign
call runtime/linker support, Target Primitive Packages, capability/profile and
target checks, Error/Fault mapping, protocol schema/fixtures, diagnostics,
sanitizer/fuzz/differential tests, and all Native/FFI claims remain deferred
until RFC-N305 and the dependent ownership, Native ABI, target, runtime, and
`PROTO-ABI` authorities are Accepted.
