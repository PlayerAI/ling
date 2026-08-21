# PLC-4805 Authority Audit — Device Binary Cache

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PLC-4805 proposes a cache for device binaries. The plan lists Program/Semantic
ID, Device IR version, backend/version, target architecture, runtime/driver
compatibility, numeric mode, profile, and compiler options as cache-key inputs,
and requires corruption to fall back to recompilation without changing program
semantics.

That proposal is not an implementation authority. The accepted cache decision
DEC-0022 authorizes only an explicitly opt-in, disposable internal cache for a
validated `ling-db` line-index payload. It expressly excludes persistent
dependent-query graphs, compiler IR serialization, migrations, shared roots,
locking, and a public cache protocol. A device-binary cache would require a
separate protocol and a checked Device IR/backend contract; neither exists.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:495-510` is a
  non-normative plan fragment. It does not define a Device IR schema, binary
  format, backend ABI, signing/trust model, driver compatibility identity,
  cache lifecycle, permissions, migration, eviction, or a public/private
  protocol boundary.
- `docs/ROADMAP-1.0.md:421-431` makes Placement and device-binary cache
  correctness a G4.6 goal and an exit criterion, but it does not authorize a
  cache artifact or define its observable behavior.
- Accepted DEC-0019 and DEC-0022 cover deterministic internal query/cache
  boundaries only. DEC-0022 permits a bounded disposable line-index payload;
  it forbids unchecked compiler IR/bytecode deserialization and leaves
  migrations and broader persistent caches open under
  `GAP-INCREMENTAL-CACHE-001`.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` do not accept Device IR,
  Placement, backend, driver, or device-binary behavior for v0.0.1. The
  prerequisite tasks PLC-4801 through PLC-4804 are BlockedSpec, and
  `GAP-KERNEL-DEVICE-001` plus `GAP-NATIVE-BACKEND-ABI-001` remain Open.
- No Accepted RFC-H405 (or replacement) defines placement/cache semantics,
  and the protocol inventory contains no device-binary cache contract.

## Current implementation evidence

- `crates/ling-cache` implements the DEC-0022 envelope and
  `crates/ling-db` persists only a derived line-index payload. That existing
  internal cache is not a Device binary cache and cannot be widened into one
  without accepted authority.
- The repository has no checked Device IR, backend artifact schema, compiler
  option identity, runtime/driver compatibility record, numeric/profile
  integration, signed artifact verification, cache namespace, or device-binary
  fixtures under `crates` or `tests`.
- No accepted rule fixes canonical Device IR/binary bytes, Program versus
  Semantic ID selection, backend/target/driver version ranges, numeric and
  Profile compatibility, corruption or ABI mismatch handling, recompilation
  guarantees, cache locking/eviction, path/permission policy, or cross-process
  sharing.
- A cache hit must never bypass Typed-Core checking, Device IR validation,
  backend capability checks, or required Fault/effect semantics. None of those
  device boundaries currently has an accepted implementation contract.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A checked, versioned Device IR and backend artifact format with canonical
   bytes, target/profile/numeric semantics, source/semantic identity, and
   explicit validation boundaries.
2. A cache-key and namespace protocol covering compiler/language/Unicode and
   schema versions, Program/Semantic ID, Device IR, backend/toolchain, target
   architecture, runtime/driver compatibility, numeric mode, Profile, and
   compiler options, with deterministic ordering and migration rules.
3. Safe miss/recompile behavior for corruption, unknown versions, ABI or
   capability mismatch, environment changes, stale options, and invalid
   signatures; no cache result may alter language semantics or Fault behavior.
4. Artifact trust, signing/verification, permissions, path isolation,
   atomic publication, concurrent writers, eviction, disk limits, and whether
   caches are disposable, shared, encrypted, or portable.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   cache hit/miss, rejection, corruption, incompatibility, unavailable
   backend, recompilation, and resource/security failures.
6. Offline positive/negative, corruption, migration, cross-toolchain,
   cross-target, numeric/Profile, privacy/security, determinism, replay, and
   CPU/device differential fixtures, plus protocol-inventory lifecycle
   evidence if any artifact becomes public.

## Evidence and compatibility impact

The eventual cache must be an optimization over a verified Device IR/backend
pipeline, never a second compiler or an authority for program identity. It must
keep cache metadata distinct from Ling semantics, preserve original UTF-8
spans and Semantic IDs, and exclude host paths, addresses, timestamps,
allocation order, and unstable driver/debug text from identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

PLC-4805 implementation, device-binary artifact serialization, cache key and
namespace, backend/driver compatibility, trust and permissions, migration,
locking, eviction, corruption handling, diagnostics, editor integration, and
public protocol claims remain deferred until RFC-H405 (or an Accepted
replacement), PLC-4801/4802/4803/4804, Device IR/backend ABI, and executable
offline fixtures are Accepted. DEC-0022's line-index cache remains the only
applicable cache authority.
