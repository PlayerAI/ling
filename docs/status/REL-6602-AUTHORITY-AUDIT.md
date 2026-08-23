# REL-6602 Authority Audit

- Task: `REL-6602` — Fault Injection
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:340-352`
- Release: G6
- Status: `BlockedSpec` for the G6 release gate; Seed fault evidence is
  recorded where an accepted implementation boundary exists.

## Decision

`REL-6602` is `BlockedSpec` as a release-completion task. Accepted DEC-0042
closes only the bounded `REL-6602-SEED` documentation drift gate. Accepted
DEC-0235 separately authorizes deterministic lock-persistence injection for
storage exhaustion and interruption before replacement. The checklist mixes
implemented Seed persistence/cache boundaries with future network, device,
Actor, replay, proof/evidence, and language-server systems. There is no
accepted fault protocol defining the injection point, retry/rollback/commit
semantics, crash artifact, resource limit, diagnostic, or cross-process
oracle for those future systems.

The repository can safely preserve and document the current evidence: cache
corruption is a safe miss, lock updates are failure-atomic at the library
boundary, and VM resource limits are tested. Inventing a network partition,
device simulator, Actor restart API, replay truncation reader, proof checker,
or LSP restart protocol would create unsupported public behavior.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:340-352` is a non-normative scenario list. It
  does not define fault injection seams or recovery semantics.
- `docs/ROADMAP-1.0.md:540-547` requires fault injection as part of the G6
  reliability baseline and gates G6 on G1--G5 exits; it does not authorize
  future protocol implementations.
- Accepted `DEC-0022` defines a disposable persistent line-index cache and
  corruption-safe misses. Accepted package/lock decisions define canonical
  lock bytes and atomic library-level replacement, not an OS crash simulator.
- Accepted bytecode and VM slices define bounded decoder/resource behavior;
  they do not define device, Actor, replay, proof/evidence, remote, or LSP
  lifecycle protocols. The protocol inventory and support matrix keep those
  surfaces Future, Experimental, or Unsupported.
- Open cache, replay, package, editor, device, and concurrency gaps require
  dedicated Accepted authority before fault behavior becomes compatibility.
- `AGENTS.md` requires deterministic/offline evidence, checked Typed Core
  inputs, original UTF-8 spans, Unicode 17.0.0, bilingual registered
  diagnostics, and no placeholder public APIs.

## Evidence in this repository

`docs/testing/FAULT-INJECTION.md` maps all eleven plan scenarios to current
tests, partial boundaries, or explicit deferred states. Existing evidence
includes:

1. `ling-cache` corruption/version safe-miss tests;
2. `ling-db` persistent line-index corruption recovery;
3. project lock canonicality, failure-atomic update, malformed-input,
   no-silent-rewrite fixtures, injected partial-write `StorageFull`, and
   injected post-sync/pre-replace `Interrupted` failures; and
4. VM frame/heap resource-limit and cancellation tests for the implemented
   runtime boundary.

The documented offline commands and full workspace tests provide executable
Seed evidence. They do not prove a crash, network, device, Actor, replay,
proof, or LSP recovery contract that does not exist.

## Required authority before G6 completion

Before promoting this task, each future surface needs:

1. an Accepted protocol and implementation owner;
2. an explicit fault-injection seam and precondition;
3. retry, rollback, commit, restart, cleanup, and stable diagnostic rules;
4. bounded timeout/memory behavior and deterministic replay inputs;
5. offline positive/negative/corrupt/interrupted/crash fixtures, including
   cross-process and cross-platform expectations; and
6. retained minimized failures with a named triage owner and generated
   status/protocol/diagnostic drift checks.

## Compatibility and deferred work

This audit changes no language grammar, Typed Core, evaluator, VM, diagnostic
allocation, schema, package/lock behavior, CLI, editor protocol, dependency,
or public API. It preserves `ling`/`.ling`, original UTF-8 spans, Unicode
17.0.0, deterministic ordering, and offline builds.

The Seed child adds `cargo xtask fault verify`. The lock-persistence child adds
a private production seam with the normal filesystem implementation plus two
deterministic unit injectors. The prior lock remains byte-exact, adjacent
temporary files are removed, and the registered `L-IO-0002` diagnostic reports
the operation and stable I/O kind. Neither child claims process-crash or
cross-platform filesystem guarantees.

No network adapter, device simulator, Actor restart API, replay/proof/evidence
decoder, LSP server, fault-injection CLI, or placeholder public surface is
added. The future scenarios remain deferred until their authorities and
executable evidence are Accepted; the parent remains blocked.
