# REL-6604 Authority Audit

- Task: `REL-6604` — Performance Baseline
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:368-387`
- Release: G6
- Status: `BlockedSpec` for the G6 release gate; the existing Seed query
  baseline is recorded as trend evidence.

## Decision

`REL-6604` is `BlockedSpec` as a release-completion task. The repository has
an opt-in, deterministic `ling-db` baseline from INC-1410, including cold/warm
queries, edits, a workspace-input revision, and a 10,000-file synthetic parse.
The broader checklist also asks for LSP, VM, Native, Actor, Replay, Kernel,
device, Zed, memory, and build measurements whose public surfaces and support
contracts are not accepted or implemented.

The audit records the actual Seed samples, host/toolchain context, minimum,
maximum, and range, but freezes no absolute threshold. Treating one Windows
run as a cross-platform release target would expose hardware, scheduler,
allocation, or host-path behavior as Ling semantics.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:368-387` is a non-normative measurement list.
  It does not define benchmark schemas, warm-up/sample policy, hardware tiers,
  memory/IO measurement, variance, thresholds, or public performance promises.
- `docs/ROADMAP-1.0.md:540-547` requires G6 performance evidence and says
  regression thresholds must come from measurement rather than invented
  numbers. G6 remains gated by G1--G5 exits.
- Accepted `DEC-0019`, `DEC-0021`, and the INC-1410 implementation report
  authorize the current internal query/performance evidence only. They do not
  authorize LSP, Native, Actor, Replay, device, Kernel, Zed, or a stable
  benchmark protocol.
- The protocol inventory and support matrix keep those unimplemented surfaces
  Future, Experimental, Unavailable, or Unsupported.
- `AGENTS.md` forbids exposing host paths, allocation, map order, or unchecked
  compiler data as semantics and requires deterministic/offline evidence.

## Evidence in this repository

`docs/testing/PERFORMANCE-BASELINE.md` records the host, revision, eight Seed
scenarios, three samples each, observed work, and dispersion ranges. The
existing `tools/xtask/src/performance.rs` command emits the versioned internal
JSON artifact with fixture setup excluded from timed regions. It makes no
absolute performance claim and has no benchmark dependency.

The command and relevant test/gate evidence are reproducible with locked
offline dependencies. They do not measure memory peak, storage/thermal state,
full package builds, LSP, Native, Actor, Replay, device/kernel, or Zed
startup/highlight.

## Required authority before G6 completion

Before promotion, an Accepted performance policy must define:

1. benchmark schema, sample/warm-up/repetition count, variance statistic,
   hardware/OS/toolchain tiers, and background-load controls;
2. cold/warm build, package graph, edit, LSP, VM, Native, Actor, Replay,
   device/kernel, Zed, memory, and IO measurement boundaries;
3. deterministic/offline fixture sizes, source spans, Unicode 17.0.0, profile,
   target, and dependency inputs;
4. threshold ownership, trend comparison, regression tolerance, and release
   gate behavior; and
5. cross-process/platform evidence that does not turn host-specific timing or
   allocation into language or protocol semantics.

## Compatibility and deferred work

This audit changes no language grammar, Typed Core, diagnostics, schemas,
Semantic IDs, cache format, CLI, editor protocol, package behavior, runtime,
dependency, or public API. It preserves `ling`/`.ling`, original UTF-8 spans,
Unicode 17.0.0, deterministic ordering, and offline builds.

No benchmark dependency, absolute threshold, LSP/Native/Actor/Replay/device/
Kernel/Zed harness, memory claim, or placeholder public protocol is added.
The Seed trend baseline remains available while G6 performance policy and
broader implementations are deferred.
