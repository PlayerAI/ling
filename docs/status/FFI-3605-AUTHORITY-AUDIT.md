# FFI-3605 Authority Audit — FFI Fuzz and Sanitizer Suite

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

FFI-3605 proposes fuzzing and sanitizer coverage for malformed ABI metadata,
wrong layouts, callbacks after owner drop, double free, pinning, concurrent
callbacks, exception/unwind crossing, hostile C-library returns, and
Capability denial. These are valuable safety-test objectives, but they are
not an accepted ABI, ownership, runtime, target, or test-result contract.

No FFI fuzz target, sanitizer configuration, C harness, unsafe fixture,
toolchain dependency, failure taxonomy, corpus format, suppression policy,
security boundary, or public test-report protocol is added. The Seed
compiler, interpreter, bytecode, VM, diagnostics, and offline dependency
lock remain unchanged until the underlying FFI and Native contracts are
Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:476-488` is non-normative.
  It names adversarial cases but does not define the ABI metadata schema,
  ownership/lifetime model, expected rejection or Fault result, sanitizer
  configuration, C target, corpus, timeout, resource limit, or report
  compatibility.
- `docs/ROADMAP-1.0.md:373-379` makes sanitizer, fuzz, and cross-target
  evidence a future Native/FFI release gate; it does not authorize a harness
  or make an unclassified finding a language semantic.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open and records ABI/layout,
  unwinding/Fault, thread/reentry, Typed FFI, Target Primitive, and target
  tiers as unaccepted. `GAP-OWNERSHIP-MODEL-001` and
  `GAP-OWNERSHIP-PUBLIC-LIFETIME-001` leave ownership, borrow, drop, resource,
  Managed, and exported lifetime behavior unresolved.
- `docs/SEMANTICS.md:1831-1868` and `docs/LANGUAGE.md:1276-1287` require a
  future FFI boundary to define ABI, ownership, lifetime, thread/reentry,
  Error/Fault, Capability, Target, verification, and trusted-package rules;
  they do not define fuzz expectations or sanitizer findings.
- `docs/governance/protocol-inventory.toml:495-537` leaves `PROTO-ABI` and
  `PROTO-EVIDENCE` Planned public without schema, version, reader/writer,
  migration, provenance, checksum, finding-severity, or fixtures. A local
  fuzz report cannot be claimed as a public evidence bundle.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no accepted FFI declaration/ABI reader, Native backend,
  target package, C harness, sanitizer configuration, fuzz corpus, hostile
  foreign-library adapter, callback/allocator runtime, or cross-target test
  matrix. Existing Rust tests do not establish C boundary safety or Ling
  runtime semantics.
- No accepted rule distinguishes a memory-safety violation from an expected
  compile-time rejection, Error/Fault, test-harness failure, host crash,
  timeout, or unsupported-target result. Treating sanitizer output as a
  language verdict now would freeze unspecified behavior.
- No C compiler, sanitizer, fuzzer, generated bindings, unsafe code, test
  dependency, diagnostic allocation, or public report implementation is
  required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The ABI/metadata and ownership/lifetime contracts under test, including
   layouts, spans, callbacks, allocators, pinning, aliasing, threads/reentry,
   unwind/Fault, Capability, target/profile, and all expected reject/error
   outcomes for malformed or hostile foreign inputs.
2. A bounded fuzz and sanitizer test protocol: target functions, input
   grammar/seed corpus, mutation and generation rules, compiler/toolchain and
   sanitizer versions, time/memory/stack limits, parallelism, crash/timeout
   handling, reproducibility, minimization, suppression/allowlist policy, and
   severity/classification of findings.
3. Isolation and security rules for foreign libraries, callbacks, allocator
   pairs, subprocesses, files/network, hardware, and test-only capabilities;
   no host crash, address, path, timing, allocator, or sanitizer text may
   become Ling semantics or stable diagnostic facts.
4. Canonical, versioned corpus and result/evidence schemas linked to program,
   ABI, target, dependency-lock, toolchain, generated-shim, Semantic-ID, and
   provenance identities, with independent readers, migration, redaction,
   license/TCB, and offline/reproducibility rules.
5. Stable bilingual diagnostics and release gates that define when a finding
   blocks a task/profile/target, how known limitations are recorded, and how
   positive/negative, cross-target, differential, and regression evidence is
   independently verified.

## Evidence and compatibility impact

The eventual implementation needs deterministic malformed metadata and wrong
layout corpus; callback-after-drop, double-free, pinning, concurrent callback,
unwind crossing, hostile-return, and Capability-denial fixtures; independent
sanitizer and fuzz runs with minimized reproducers; cross-target and foreign
compiler differential tests; resource-limit, cancellation, crash/timeout,
suppression, and regression evidence; provenance/license/TCB and schema
migration checks; and offline reproducibility. It must preserve original
UTF-8 byte spans, stable Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>`
diagnostics, and Unicode 17.0.0 behavior while keeping host sanitizer/fuzzer
details outside Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, or Unicode behavior. It adds no fuzz target,
sanitizer configuration, C harness, test dependency, toolchain, corpus,
diagnostic, public protocol implementation, or placeholder API.

## Intentionally deferred

FFI/ABI fuzz targets, sanitizer and foreign-toolchain setup, adversarial C
libraries, corpus and result/evidence schemas, finding classification and
release gates, crash/timeout/resource policies, cross-target/differential
evidence, provenance/license/TCB checks, and all Native/FFI safety claims
remain deferred until RFC-N305 and the dependent ownership, Native ABI,
target, runtime, `PROTO-ABI`, and `PROTO-EVIDENCE` authorities are Accepted.
