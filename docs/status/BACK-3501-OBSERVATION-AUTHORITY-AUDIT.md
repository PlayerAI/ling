# BACK-3501-OBSERVATION Authority Audit

## Outcome

The bounded child `BACK-3501-OBSERVATION` is authorized by Accepted
`DEC-0132`. It records only a test-local inventory of proposed Native
backend-selection comparison boundaries. Public `BACK-3501` remains
`BlockedSpec`: no backend is selected, no toolchain is installed, no benchmark
is run, and no target/support claim is made.

## Normative traceability

- The G3 execution package is non-normative; its candidate list and comparison
  dimensions cannot define an eligible NIR, ABI, target, profile, or toolchain
  contract.
- RFC-N306 is absent or not Accepted. RFC-0001 remains Draft under DEC-0018
  and treats LLVM/Cranelift Native Backend as a non-goal for the Seed release.
- Accepted `DEC-0131`, `DEC-0130`, and `DEC-0129` define only test-local
  verifier/lowering/design vocabulary. They do not authorize Native codegen,
  backend selection, or support claims.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and Profile/Task/
  Actor gaps remain Open. The accepted Seed Typed Core/interpreter/VM boundary
  remains the only executable authority.
- `DEC-0132` authorizes this child only.

## Current implementation boundary

`native_backend_selection_evidence.rs` defines fifty-four test-local
boundaries, sorts them by explicit local rank, rejects duplicates, and
compares forward/reverse insertion order. Its evidence tag is opaque and
test-only; it is not a backend choice, dependency declaration, benchmark,
target claim, build recipe, license approval, public protocol, or
performance/reproducibility result.

No backend dependency, toolchain, build script, code generator, benchmark
corpus, target support entry, public API, or placeholder crate was added. The
existing accepted Seed Typed Core and VM bytecode pipeline remains the only
executable path.

## Evidence and deferred work

Focused tests cover the complete comparison vocabulary, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent
remains blocked until accepted authority defines NIR/ABI/profile/target
eligibility, comparison corpus and metrics, toolchain/offline/license/TCB
policy, reproducibility and host-noise boundaries, semantic/ABI/FFI/Fault/
Resource/Managed/Task/Actor preservation, data-only artifact/review schema,
recommendation/migration rules, and public support criteria.
