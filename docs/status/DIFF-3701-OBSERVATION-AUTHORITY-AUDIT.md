# DIFF-3701-OBSERVATION Authority Audit

## Outcome

The bounded child `DIFF-3701-OBSERVATION` is authorized by Accepted `DEC-0142`.
It records only a test-local inventory of proposed Interpreter/VM/Native
differential boundaries. Public `DIFF-3701` remains `BlockedSpec`: no harness,
Native backend, engine adapter, trace schema, normalizer, corpus, replay tool,
allowed-difference registry, or equivalence result is added.

## Normative traceability

- The G3 execution package is non-normative; its differential checklist cannot
  define engine inputs, scheduling, observation points, value/Fault semantics,
  normalization, equivalence, or allowed differences.
- Accepted Seed interpreter/VM decisions define only Seed behavior; they do not
  authorize a Native engine or cross-engine contract.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-OWNERSHIP-MODEL-001`, and
  `GAP-SEMANTIC-HASH-LIFECYCLE-001` remain Open. Native/differential protocol
  authorities are not Accepted.
- `DEC-0142` authorizes this child only; earlier FFI/Native evidence decisions
  do not supply engine-equivalence semantics.

## Current implementation boundary

`differential_harness_evidence.rs` defines sixty test-local boundaries, sorts
them by explicit local rank, rejects duplicates, and compares forward/reverse
insertion order. Its evidence tag is opaque and test-only; it is not an
execution result, engine trace, equivalence proof, allowed-difference registry,
replay record, diagnostic, Semantic ID, or public protocol.

No differential harness, Native backend, engine adapter, trace schema,
normalizer, corpus, replay tool, allowed-difference registry, dependency,
toolchain, diagnostic, or public API was added. Existing Seed interpreter/VM
paths remain unchanged.

## Evidence and deferred work

Focused tests cover the differential vocabulary, deterministic ordering,
duplicate rejection, and explicit non-authority boundaries. The parent remains
blocked until accepted authority defines engine adapters, Native execution,
trace/normalization schemas, equivalence and allowed-difference policy, replay,
corpus, diagnostics, migration, and cross-target/compiler behavior.
