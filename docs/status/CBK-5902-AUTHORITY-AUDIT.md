# CBK-5902 Authority Audit

- Task: `CBK-5902` — Lowering Validator
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:599-609`
- Release: G5
- Status: `BlockedSpec`

## Decision

CBK-5902 is `BlockedSpec`. The execution plan asks every lowering to check
type/layout, control flow, value mapping, contract-preservation facts,
memory/alias behavior, source/binary correspondence, and rejection of
unsupported constructs. It does not define which compiler route is being
validated, the representation at either side of the boundary, the meaning of
equivalence, or the trust and failure model for the validator. The task also
depends on CBK-5901, whose required RFC-K508 compiler-route decision is absent.

No accepted authority permits a Native/Critical lowering validator. Implementing
one now would invent Native IR, ABI, ownership, layout, memory/alias, Contract,
proof, target, source-map, and artifact-identity semantics, and could turn a
checker for an unsupported backend into an unsupported public capability.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:599-609` is a non-normative checklist. It does not
  specify the source and target languages, a validator soundness theorem,
  proof obligations, accepted backend transformations, or an independent
  checker/TCB boundary.
- `docs/IMPLEMENTATION.md:17` excludes Native Backend, Resource/Borrow
  Checker, Task/Actor/Node/Kernel, proof tooling, and related future
  capabilities from the v0.0.1 Seed target. A validator cannot enlarge that
  target without an Accepted authority.
- Accepted RFC-0014 defines the checked Typed Core to `ling.bytecode/1.x`
  lowering and an independent bytecode verifier. Its artifact is an
  interpreter/VM protocol and explicitly has no native calling convention,
  memory layout, or FFI ABI. The existing bytecode verifier therefore cannot
  be relabeled as a Native or Critical lowering validator.
- Accepted RFC-0019 defines Interpreter–VM differential evidence and states
  that a three-way Interpreter–VM–Native oracle requires a future accepted
  Native execution contract. It does not authorize source-to-binary
  correspondence or Native equivalence checking.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open: Native IR validity, layout, ABI,
  unwinding/Fault, thread/reentry, typed FFI, target primitive packages, and
  target tiers are unresolved. `GAP-OWNERSHIP-MODEL-001`,
  `GAP-KERNEL-DEVICE-001`, and `GAP-CRITICAL-PROFILE-001` leave dependent
  memory, device, profile, and evidence semantics open.
- `PROTO-ABI` and `PROTO-EVIDENCE` are Future planned public protocols with no
  version, canonical schema, reader/writer, identity, or verification fixtures.
  The support matrix keeps Native and Critical capabilities unavailable or
  unsupported, which is a support constraint rather than a validator contract.
- `ROADMAP-1.0.md:324-379` plans Native lowering and differential conformance
  only after accepted ownership, ABI, target, FFI, and TCB boundaries. The
  roadmap is planning authority and cannot authorize this implementation.

## Evidence in this repository

The workspace contains no Native IR, Native lowering validator, translation
validator, source/binary correspondence schema, contract-preservation checker,
memory/alias proof checker, target artifact verifier, or Critical validator
fixture under `crates/`, `tests/`, or `schemas/`. `crates/ling-bytecode` has an
accepted VM lowering and verifier with a narrower bytecode scope; it does not
validate Native or machine-code equivalence. No CLI, LSP request, diagnostic,
public protocol, or support entry claims CBK-5902.

## Required authority before implementation

An accepted RFC-K508 or replacement must define, at minimum:

1. The exact validation boundary and supported route: Typed Core to a
   backend-neutral IR, IR to target code, source-to-binary correspondence, or
   a specified composition; supported Core constructs, targets, profiles, and
   rejected transformations must be explicit.
2. Versioned IR, type/layout, control-flow, value-representation,
   evaluation-order, Effect/Capability, Contract, memory/alias, ownership,
   Resource/Drop, Fault, threading, FFI, and target/ABI semantics, including
   what is observable and what is host-only.
3. A soundness/equivalence model for validation or proof-producing lowering:
   obligations, certificates, independent checker, trusted computing base,
   assumptions, optimization limits, error classification, resource bounds,
   and fail-closed behavior for malformed or unsupported input.
4. Canonical source/target identity and correspondence rules: original UTF-8
   byte spans, Unicode 17.0.0, source and artifact digests, target/toolchain
   identity, reproducible/offline builds, and exclusions for host paths,
   addresses, timestamps, Rust layout, and debug output.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and schemas for type or
   layout mismatch, invalid control/value mapping, missing preservation facts,
   alias/ownership violations, source-map mismatch, unsupported constructs,
   unavailable target tools, and validator failure.
6. Offline positive, negative, malformed, unsupported, control-flow,
   contract, memory/alias, source-span, target/ABI, differential,
   cross-target, repeated-build, fuzz/property, and Unicode/BOM/CRLF fixtures.
   Native or Critical support must remain unclaimed until those fixtures and
   the support-matrix evidence are complete.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, or Semantic ID rule. It preserves the accepted `ling` CLI and `.ling`
source extension, checked Typed Core boundary, original UTF-8 spans, Unicode
17.0.0, deterministic identity rules, and the existing bytecode/VM route.

It deliberately adds no Native IR, validator, proof checker, backend or target
dependency, source/binary protocol, diagnostic, CLI command, public API, or
placeholder crate, and introduces no stale `zero` names. CBK-5902 remains
deferred until CBK-5901 and the Native/ABI, ownership, Contract/Proof,
Critical-profile, evidence, and reproducible-build authorities are Accepted
with executable fixtures and a truthful support-matrix update.
