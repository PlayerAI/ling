# DEC-0284: Private cross-process Task trace reconstruction evidence / 私有跨进程 Task trace 重建证据

> 状态：Accepted<br>
> 提出日期：2026-09-04<br>
> 决定日期：2026-09-04<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：DEC-0109 | DEC-0267 | DEC-0282 | DEC-0283 | GAP-DETERMINISTIC-REPLAY-001 | REP-2506<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest executable REP-2506 package that can
compare reconstruction of the existing private DEC-0267 Task trace across
independent operating-system processes. It deliberately uses fresh copies of
the same unit-test binary and fixed in-memory inputs. It is not log playback,
cross-toolchain certification, a public acceptance harness, or a portable
Replay contract.

本决定定义 REP-2506 可执行证据的最小边界：在独立操作系统进程中重建现有私有
DEC-0267 Task trace，并比较其结果。证据明确使用同一个 unit-test binary 的全新进程
与固定的内存输入；它不是日志播放、跨 toolchain 认证、公开验收工具或可移植 Replay
契约。

## Question

What exact crate-private evidence may demonstrate repeatable reconstruction of
the current checked Task trace in independent same-binary processes, including
source-normalization equivalence and changed-recipe distinction, without
inventing unresolved log, player, schema, Program binding, provenance,
cross-platform, cache, toolchain, diagnostic, or public acceptance contracts?

## Decision

1. **Scoped authority.** This decision authorizes only one
   crate-private, `cfg(test)` REP-2506 evidence matrix in `ling-eval`. It adds
   no production process runner, Replay reader/player, persisted log,
   acceptance artifact, cache manager, toolchain lock, public API, command,
   diagnostic, schema, or protocol and does not close
   `GAP-DETERMINISTIC-REPLAY-001`.

2. **Independent same-binary processes.** Each probe must start a fresh copy
   of the current `ling-eval` unit-test executable with `std::process::Command`.
   Parent and child therefore share one compiled binary, target, profile, and
   test implementation, but not process-local runtime state. This is evidence
   about same-binary reconstruction only; it does not establish compiler,
   dependency, profile, target, host, or release identity across builds.

3. **Bounded hermetic input.** A child must clear its inherited environment
   before startup and report the observed environment-entry count. It must
   construct the checked program, fixed scheduler configuration, Task
   argument, and empty host script entirely from constants compiled into the
   test binary. The evidence path must not read source, trace, cache, schema,
   configuration, or dependency files and must not use the network. This
   demonstrates absence of such inputs from this path, not clean-cache tooling
   or general build reproducibility.

4. **Checked-only origin.** Every child must execute the existing
   `SourceFile → parse → AST → HIR → resolve → typecheck → Effect check`
   pipeline and schedule only checked Task Core through
   `run_task_schedule`. No unresolved AST, untyped HIR, decoded external
   document, or hand-built public trace may enter evaluation.

5. **Existing private trace only.** Children may emit a lowercase hexadecimal
   transport encoding of exact `TaskExecutionTrace::canonical_bytes()` solely
   to the parent test. The bytes remain crate-private test evidence: they are
   not an Effect Log, checkpoint, stable serialization, IPC protocol, trusted
   input, public schema, compatibility promise, or privacy-safe artifact.

6. **Fixed repeatability evidence.** The
   `independent-process-repeatability` case must start exactly three fresh LF
   probes and require identical environment counts and complete private trace
   bytes. Three is a bounded regression-test cardinality, not a statistical
   confidence threshold or public determinism-class definition.

7. **Source-independent equivalence.** The
   `source-independent-process-equivalence` case must compare two fresh
   processes whose checked programs differ only by source ID/name and
   LF versus UTF-8 BOM plus CRLF spelling. Their complete private canonical
   trace bytes and environment counts must be equal. The compiler pipeline
   continues to preserve original UTF-8 byte spans; this trace comparison does
   not redefine source positions or canonical source text.

8. **Changed-recipe distinction.** The
   `changed-recipe-process-distinction` case must separately change the checked
   child Task body and the root Task argument. Baseline, changed-body, and
   changed-argument private trace bytes must be pairwise distinct. This proves
   only that these concrete checked recipes do not collapse to the same
   private evidence. It does not validate a serialized mutation, reject a
   Program/Schema mismatch, or allocate a public divergence diagnostic.

9. **Environment and limit evidence.** The
   `empty-environment-bounded-process` case must require an observed child
   environment-entry count of zero, the fixed repeat count, and nonzero
   explicit scheduler bounds. It does not define a public environment
   fingerprint, wall-clock timeout, operating-system sandbox, resource
   profile, or denial-of-service guarantee.

10. **Exact boundary inventory.** The evidence must retain all eighteen
    DEC-0109 concerns exactly once and classify them as follows:

    - same-binary child evidence: process isolation, toolchain identity, and
      target identity;
    - private trace-comparison evidence: cache isolation, input snapshot, log
      generation, Program binding, observable equivalence, repeatability,
      divergence, resource limits, and offline mode;
    - deferred public contract: profile identity, Replay playback, Schema
      binding, mutation rejection, provenance, and platform boundary.

    Here `toolchain identity` and `target identity` mean only that one already
    compiled executable starts its copies. `cache isolation` means the probe
    path accepts no cache input. `log generation` means only construction of
    the existing in-memory private Task trace. `Program binding` means only
    the private checked-recipe preflight inherited from DEC-0282. None of
    these labels promotes a public Replay guarantee.

11. **Negative public-surface gate.** The
    `deferred-cross-process-public-surface-absence` case must prove that
    production evaluator/project/bytecode/VM sources expose none of the named
    cross-process acceptance APIs, the CLI exposes no Replay command, no
    `L-REPLAY-*` diagnostic or Replay schema exists, and `PROTO-REPLAY`
    remains unimplemented `Future` inventory with no version, fixtures, or
    canonical/public-schema claim.

12. **Completion boundary.** Acceptance plus passing evidence may mark
    REP-2506 `Done` only as an internal Experimental same-binary reconstruction
    baseline. Public cross-process Replay acceptance, generator/player
    interoperation, persisted logs, mutated Program/Schema refusal,
    provenance, cache/toolchain reproducibility, cross-backend equivalence,
    cross-platform support, and Stable compatibility remain blocked pending
    RFC-0010 or replacement Accepted authority.

## Conformance plan

The implementation must provide exactly five named parent evidence cases:

1. `independent-process-repeatability`;
2. `source-independent-process-equivalence`;
3. `changed-recipe-process-distinction`;
4. `empty-environment-bounded-process`; and
5. `deferred-cross-process-public-surface-absence`.

The first four cases must invoke ignored child probes only through fresh
copies of the current test executable. The fifth must enforce the exact
eighteen-concern disposition inventory and negative public-surface boundary.
Focused `ling-eval` tests, lint, workspace tests, governance/status checks,
formatting, and the retained interpreter/VM Task suites must pass offline.

No acceptance claim may rely on Cargo build-cache cleanliness, a different
binary or toolchain, external files, network state, process timing, hash-map
order, Rust debug formatting, host paths, or an unregistered protocol.

## Compatibility impact

- **Source and semantics:** none. The matrix executes existing accepted Task
  syntax and checked behavior; it adds no Ling construct or observable source
  semantics.
- **CLI/LSP/editor and diagnostics:** none. No command, option, language-server
  behavior, error code, output format, or process-facing diagnostic is added.
- **Schema/protocol/data:** none. Child stdout is private test plumbing, not a
  public or stored format. No schema, protocol version, reader, writer,
  migration, retention, integrity, or compatibility contract is created.
- **Semantic IDs and runtime:** none. Existing private checked-recipe identity
  and Task trace bytes are observed without changing compiler, evaluator,
  scheduler, bytecode, VM, ABI, package, dependency, or backend behavior.
- **Determinism and Unicode:** the fixed test asserts same-binary exact-byte
  repeatability and LF/BOM+CRLF equivalence only. Unicode remains 17.0.0 and
  original UTF-8 spans remain preserved; no cross-platform determinism class
  is claimed.

## Unresolved alternatives

- RFC-0010 or replacement authority must define the public Replay
  generator/player, versioned event/checkpoint format, Program/Semantic ID and
  Schema bindings, mutation/corruption refusal, divergence diagnostics,
  privacy, integrity, migration, unknown-field, and retention behavior.
- Build and execution provenance must define compiler/toolchain/profile/target
  identity, dependency and input snapshots, cache and network policy,
  environment fingerprinting, resource limits, signed artifacts, and the
  supported host/platform matrix before reproducible cross-build claims.
- Observable-equivalence and differential authority must define which values,
  Effects, Faults, ordering, diagnostics, Actor/Task traces, interpreter/VM
  backends, and allowed nondeterminism participate in public acceptance.
- Persisted cross-process IPC artifacts, CI result schemas, public offline
  tools, cross-backend runs, cross-platform runs, and Stable support remain
  future work. The private stdout marker is not a candidate wire format.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
