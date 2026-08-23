# CBK-5901-OBSERVATION Authority Audit — Trusted Compiler Route Evidence

Status: BlockedSpec
Date: 2026-08-23

## Outcome

Accepted `DEC-0213` permits only test-local trusted-compiler-route vocabulary.
It does not select a route or authorize Native IR, ABI/FFI, target packages,
translation validation, proof-producing lowering, machine-code verification,
diagnostics, protocols, or Native/Critical support.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:585-597` is a
  non-normative alternatives list dependent on absent RFC-K508.
- `docs/status/CBK-5901-AUTHORITY-AUDIT.md` records missing route, target,
  ABI/ownership/Fault, proof/equivalence/TCB, migration, and fixture authority.
- `docs/IMPLEMENTATION.md` excludes Native/Critical work from Seed;
  Native/ownership/kernel/Critical gaps remain Open and `PROTO-ABI` is Future.
- The support matrix records Native/Critical as Unsupported/Unavailable;
  accepted bytecode/VM verification has a distinct portable scope.

## Current implementation evidence

The observation adds one isolated test with sixty explicit route, Core/target,
IR/ABI, identity/build, equivalence/proof/trust, failure, fixture, protocol, and
support boundaries. It sorts by explicit local rank, rejects duplicates,
compares opaque bytes for forward/reverse input order, retains all five route
alternatives, and selects none. No backend, lowering, validator, proof, ABI,
target, diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must select and scope a route; define Native IR, ABI,
ownership/FFI/Fault/target behavior; establish equivalence, proof, checker,
trust, TCB, and fail-closed rules; identify build/toolchain/artifacts; define
bilingual diagnostics; preserve checked Typed Core, Semantic IDs, and original
UTF-8 spans; and provide offline cross-target fixtures. Seed behavior,
dependencies, support state, and Unicode 17.0.0 remain unchanged.

## Deferred work

CBK-5901 route selection, Native/Critical backend work, ABI/FFI/target
packages, validators and proof tooling, diagnostics, protocols, and support
remain deferred until Accepted authority and executable evidence exist. No
placeholder compiler-route API is created.
