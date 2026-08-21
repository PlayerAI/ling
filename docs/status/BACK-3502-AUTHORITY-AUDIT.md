# BACK-3502 Authority Audit — Baseline Native Codegen

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

BACK-3502 is the first code-emission task in the non-normative G3 plan. It
proposes one backend covering target-machine selection, function/data emission,
relocations, runtime linking, object/executable output, debug/source maps,
deterministic metadata, and explicit unsupported diagnostics. Emitting any of
these artifacts would bind a target, ABI, runtime, diagnostics, and build
contract that are not Accepted.

No Native codegen, object/executable writer, linker integration, debug map,
target manifest, diagnostic, build command, or placeholder backend crate is
added. BACK-3502 remains `BlockedSpec` until NIR/ABI/backend and memory/profile
authority exists.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:352-363` is non-normative and
  follows the unaccepted BACK-3501/NIR work. It cannot authorize a machine
  target, object format, linker, executable, or public build protocol.
- BACK-3501 and NIR-3401 through NIR-3403 are `BlockedSpec`. RFC-N304/RFC-N306
  and the candidate RFC-0011 Native contract are absent or not Accepted.
- `GAP-NATIVE-BACKEND-ABI-001` leaves layout, calling convention,
  Fault/unwinding, thread/reentry, typed FFI, target packages, and target tiers
  unresolved. `GAP-OWNERSHIP-MODEL-001` leaves Value/Resource/Managed and
  cleanup representations unresolved; Profile and Critical gaps affect target
  legality and evidence.
- RFC-0001 remains Draft and explicitly excludes an LLVM/Cranelift Native
  Backend from v0.0.1. `docs/SEMANTICS.md` and `docs/LANGUAGE.md` reserve
  Native codegen for later releases.
- Accepted Seed Place/runtime decisions authorize only the current checked
  Typed Core/interpreter/VM boundary and do not define machine code or object
  artifacts.

## Current implementation evidence

- The workspace has no Native codegen, object/executable writer, linker
  integration, target manifest, debug/source-map generator, or unsupported
  Native diagnostic. The current bytecode/VM path is not machine codegen.
- No NIR verifier, ABI, runtime library, Managed handle, Resource cleanup,
  Task/Actor call convention, or target package exists for a code generator to
  consume.
- Existing CLI/project/build behavior is file-oriented and Seed/VM-scoped. No
  native output path or artifact schema is registered.
- Host compiler/linker output, paths, timestamps, addresses, section order,
  debug text, and platform defaults are not Ling semantics; emitting them now
  would create unreviewed compatibility claims.

## Required authority before implementation

The accepted NIR, Native ABI, and backend decisions must define:

1. The target/profile matrix, machine/endianness/data-layout rules, object and
   executable formats, relocation/linking/runtime-library boundaries, and
   reproducible build inputs/outputs.
2. Function/data/closure/ADT/string/Value/Resource/Managed representation,
   calling convention, Fault/unwind/cancellation/thread/reentry behavior,
   Task/Actor calls, FFI and target primitive packages, and ownership/cleanup
   obligations.
3. Lowering and codegen semantic-preservation rules, unsupported-form
   rejection, allocation/GC/handle integration, profile capability checks,
   and stable bilingual diagnostics with source-byte spans.
4. Debug/source-map identity and privacy, deterministic metadata/section and
   symbol ordering, artifact schema/versioning/migration, security/TCB, and
   host-toolchain/license/offline policies.
5. Acceptance evidence for executable equivalence against interpreter/VM,
   cross-target ABI, fault/resource cleanup, malformed/unsupported input, and
   reproducible output; no claim may exceed the selected target tier.

## Evidence and compatibility impact

The eventual implementation needs a pinned, deterministic NIR corpus, positive
and negative codegen fixtures, object/relocation/link/runtime cases, debug map
round trips, deterministic metadata checks, unsupported diagnostic fixtures,
cross-target and ABI/sanitizer evidence, and interpreter/VM/Native differential
traces. It must preserve UTF-8 source spans and exclude host paths, timestamps,
addresses, allocation/section order, map iteration, and linker debug noise from
semantic identity.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, diagnostic registry, schema, Semantic ID,
source span, runtime, or Unicode behavior. It emits no native artifact,
installs no linker/toolchain, allocates no diagnostic, and introduces no
public build or target protocol.

## Intentionally deferred

Machine/object/executable emission, relocations/linking, runtime libraries,
target and profile manifests, debug/source maps, deterministic metadata,
unsupported diagnostics, cross-target/sanitizer testing, and all Native codegen
remain deferred until BACK-3501, NIR/ABI authority, RFC-N306 (or its accepted
replacement), and the dependent memory/ownership/Profile decisions are
Accepted.
