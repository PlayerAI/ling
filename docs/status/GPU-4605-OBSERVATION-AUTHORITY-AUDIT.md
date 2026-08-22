# GPU-4605-OBSERVATION Authority Audit — Error-Normalization Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0169` authorizes test-local vocabulary only. No GPU error
category, public code, vendor-log parser, Fault mapper, diagnostic schema,
dependency, diagnostic allocation, public protocol, or support claim is
added. `docs/ERROR-CODES.md` and `error-code-lock.toml` remain the only public
allocation authorities; GPU-4605 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:394-408` lists
  proposed categories and non-stable vendor detail but is non-normative and
  does not define Fault provenance, category precedence, structured facts,
  severity, retry/cancellation, localization, schema versioning, or code
  allocation.
- `docs/ROADMAP-1.0.md:381-431` requires Device Fault mapping and explicit
  unsupported-target behavior, but does not authorize GPU diagnostics or
  define their public compatibility contract.
- `docs/decisions/0013-main-and-runtime-failures.md` governs current Seed
  failure boundaries; `docs/ERROR-CODES.md` and
  `docs/governance/error-code-lock.toml` remain the sole public allocation and
  compatibility sources and do not define GPU categories.
- DIR-4501 through DIR-4503 and GPU-4601 through GPU-4604 remain
  `BlockedSpec`; RFC-0013 and RFC-H404 are not Accepted, and the
  Kernel/device and Native/backend gaps remain open.

## Current implementation evidence

- No GPU vendor-log parser, error-normalization layer, Device Fault mapper,
  GPU diagnostic schema, public GPU error code, or normalization fixture
  exists under `crates` or `tests`.
- No accepted rule fixes category precedence, retryability, cancellation,
  severity, source-span mapping, numeric mismatch semantics, device-loss
  recovery, resource-exhaustion facts, or the distinction between compiler,
  runtime, backend, and user-program failures.
- No accepted policy defines which vendor facts may be retained, redacted, or
  exposed as structured non-stable fields; paths, addresses, timestamps,
  driver text, and debug output must not leak into Ling identity.
- No diagnostic allocation, public protocol, dependency, CLI command, target,
  or support claim is required or changed by this evidence. The public CLI and
  source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel/Device Fault taxonomy with provenance separating source,
   verifier, compiler, runtime, backend, device, resource, cancellation, and
   host failures, plus category precedence and recovery/retry behavior.
2. Stable bilingual `L-<DOMAIN>-<NUMBER>` allocations, structured fact
   schemas, redaction/privacy policy, severity, source-map/Semantic-ID rules,
   and compatibility/migration requirements. Plan labels must remain internal
   until registered.
3. Target/capability and numeric/determinism contracts defining unsupported
   features, unavailable devices, compile/launch failures, memory exhaustion,
   device loss, and numeric-mode mismatch without changing program semantics.
4. A verified Device IR/runtime boundary and explicit Fault mapping for
   malformed modules, queue/synchronization failure, cleanup, cancellation,
   and backend-specific details; unchecked AST nodes must never reach it.
5. Offline positive/negative fixtures for each category, malformed and corrupt
   vendor input, bilingual rendering, source spans/Unicode, redaction,
   determinism, migration, and cross-backend equivalence.

## Evidence and compatibility impact

The eventual normalizer must be a deterministic adapter from accepted backend
events to the registered diagnostic/Fault schema. Vendor detail may be retained
only under an explicitly non-stable, redacted field and must not alter Semantic
IDs, cache identity, or source semantics. Unknown vendor events must have a
declared safe classification rather than silently becoming a new public code.

This evidence changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

Category taxonomy, Fault mapping, vendor-log parsing, public error-code
allocation, structured payloads, redaction, migration and normalization
fixtures, editor support, and public protocol claims remain deferred until the
GPU/Kernel/Device runtime authorities and the registered diagnostic lifecycle
are Accepted with executable evidence.
