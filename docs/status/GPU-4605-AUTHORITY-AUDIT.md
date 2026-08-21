# GPU-4605 Authority Audit — Error Normalization

Status: BlockedSpec

Date: 2026-08-22

## Outcome

GPU-4605 proposes converting vendor logs into stable categories such as
`UnsupportedFeature`, `DeviceUnavailable`, `CompileFailure`, `LaunchFailure`,
`OutOfDeviceMemory`, `DeviceLost`, and `NumericModeUnsupported`, while keeping
vendor details only as a non-stable supplemental field.

No GPU error categories, diagnostic codes, vendor-log parser, Fault mapper,
schema, or public error payload can be added yet. The Kernel/Device runtime,
target capability, numeric/determinism, resource, device-loss, and support
contracts are not Accepted. The repository's registered `L-<DOMAIN>-<NUMBER>`
codes are the sole public allocation authority; plan labels cannot become
public codes by implementation.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:394-408` is a
  non-normative implementation plan. It lists proposed categories and says
  vendor detail is non-stable, but does not define Fault provenance, category
  boundaries, structured facts, severity, retry/cancellation behavior,
  localization, schema versioning, or code allocations.
- `docs/ROADMAP-1.0.md:381-431` requires device Fault mapping and explicit
  unsupported-target behavior, but does not authorize GPU diagnostics or
  define their public compatibility contract.
- `docs/decisions/0013-main-and-runtime-failures.md` governs current main and
  runtime failure boundaries; it does not define GPU/device categories,
  vendor-log normalization, or target-specific public facts. `docs/ERROR-CODES.md`
  and `docs/governance/error-code-lock.toml` remain the only allocation and
  compatibility sources.
- `docs/SEMANTICS.md:1429-1480, 1858-1931` and
  `docs/LANGUAGE.md:1347-1381` exclude Kernel/GPU/Native behavior from the
  v0.0.1 Seed subset. Existing Seed diagnostics cannot be relabeled as device
  errors.
- `DIR-4501` through `DIR-4503` and `GPU-4601` through `GPU-4604` are
  `BlockedSpec`; no accepted Device IR, adapter, runtime, numeric, or
  differential Fault contract exists. `GAP-KERNEL-DEVICE-001` and
  `GAP-NATIVE-BACKEND-ABI-001` remain Open, and `BACKEND-GPU` is Unsupported.
- No `RFC-H404` or Accepted GPU error-normalization authority exists. RFC-0013
  is only a candidate topic and cannot allocate public diagnostics.

## Current implementation evidence

- The repository has no GPU vendor-log parser, error-normalization layer,
  Device Fault mapper, GPU diagnostic schema, public GPU error code, or
  positive/negative normalization fixtures under `crates` or `tests`.
- No accepted rule fixes category precedence, retryability, cancellation,
  severity, source-span mapping, numeric mismatch semantics, device-loss
  recovery, resource-exhaustion facts, or distinction between compiler,
  runtime, backend, and user-program failures.
- No accepted policy defines which vendor facts may be retained, redacted, or
  exposed as structured non-stable fields; paths, addresses, timestamps,
  driver text, and debug output must not leak into Ling identity.
- No diagnostic allocation, public protocol, dependency, CLI command, target,
  or support claim is required or changed by this audit. The public CLI and
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

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

GPU-4605 implementation, category taxonomy, Fault mapping, vendor-log parsing,
public error-code allocation, structured payloads, redaction, migration and
normalization fixtures, editor support, and public protocol claims remain
deferred until the GPU/Kernel/Device runtime authorities and the registered
diagnostic lifecycle are Accepted with executable evidence.
