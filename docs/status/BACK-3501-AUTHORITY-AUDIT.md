# BACK-3501 Authority Audit — Native Backend Selection Spike

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

BACK-3501 is an exploratory comparison, not an authorization to select or add
a Native backend. The execution plan suggests comparing Cranelift, LLVM, or a
small C/Wasm transition backend on build time, debug information, target
coverage, JIT/AOT mode, license, and reproducible-build properties. It says the
spike must produce data and a recommendation only, without a public API.

The comparison still needs a defined NIR, ABI, target contract, supported
profiles, toolchain policy, and reproducibility boundary. Those authorities are
absent. No backend dependency, build script, code generator, benchmark claim,
target support entry, public API, or placeholder crate is added.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:339-350` is non-normative;
  it explicitly defers backend support to RFC-N306 and limits the spike to
  data/recommendations. It cannot authorize a toolchain or target promise.
- RFC-N306 is not present or Accepted. RFC-0001 remains Draft under DEC-0018
  and lists LLVM/Cranelift Native Backend as a v0.0.1 non-goal.
- `GAP-NATIVE-BACKEND-ABI-001` is Open and leaves NIR validity, layout, ABI,
  Fault/unwinding, thread/reentry, typed FFI, target packages, and target tiers
  unresolved. NIR-3401 through NIR-3403 remain `BlockedSpec`.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe Native as a future v0.3
  Profile and mention Cranelift/LLVM as design possibilities, not accepted
  implementation choices. Managed/Resource/ownership and Critical Profile
  gaps also affect backend legality and evidence.
- Existing accepted Seed decisions authorize only the current checked
  Typed-Core/interpreter/VM slice. They do not authorize adding a native
  compiler, changing Cargo dependencies, or claiming a target.

## Current implementation evidence

- The workspace contains no Native backend, codegen, target package, ABI, NIR,
  or backend-selection benchmark. Cargo manifests and the locked dependency
  set do not expose a Ling native target contract.
- The current compiler/evaluator/bytecode/VM path has no Native lowering or
  target-specific debug/source-map output against which to compare.
- No reproducible benchmark corpus, pinned external toolchain/container,
  license review, offline artifact policy, or cross-target matrix is registered.
- A dependency install or backend probe would change build/toolchain state and
  could falsely imply support; it is intentionally not performed by this
  authority audit.

## Required authority before implementation

The accepted Native/NIR decisions must define:

1. The NIR and ABI versions, supported target/profile matrix, codegen and
   runtime-library boundary, and the exact forms eligible for a backend spike.
2. Comparison methodology and reproducibility: pinned compiler/toolchain
   versions, target triples, flags, standard libraries, input corpus,
   warm/cold build measures, debug/source-map criteria, JIT/AOT scope, and
   resource/time limits. Host paths, clock noise, addresses, and map order
   must not become semantic claims.
3. License, security/TCB, supply-chain, offline-lock, generated-code, and
   cross-target policies, including whether external toolchains are optional
   development tools or required build inputs.
4. Semantic-preservation, ABI/FFI, Fault/unwind, Resource/Managed, Task/Actor,
   Profile, and diagnostic requirements for any recommendation; unsupported
   forms must remain explicit and produce no public support claim.
5. The artifact/status schema for a data-only spike, its review lifecycle,
   migration/rollback policy, and the rule that a recommendation does not
   freeze a backend or expose a public protocol before RFC-N306 acceptance.

## Evidence and compatibility impact

The eventual spike needs a deterministic, versioned NIR corpus; data for each
candidate and target; build/debug/source-map and license/TCB reports; repeated
offline runs with bounded resources; and documented limitations. It must not
claim runtime, ABI, target, performance, or reproducible-build support without
the owning Accepted RFCs and executable fixtures. Existing Seed tests and
interpreter/VM behavior remain the comparison baseline.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, diagnostic registry, schema, Semantic ID,
source span, runtime, or Unicode behavior. It installs no toolchain, adds no
backend dependency, allocates no diagnostic, and introduces no public target or
build protocol.

## Intentionally deferred

Cranelift/LLVM/C/Wasm comparison runs, benchmark corpus and metrics, toolchain
pinning, license/TCB review, reproducible-build claims, target matrix, backend
recommendation, dependencies, and all Native code generation remain deferred
until NIR/ABI authority and RFC-N306 (or its accepted replacement) exist.
