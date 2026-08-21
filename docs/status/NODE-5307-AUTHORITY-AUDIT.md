# NODE-5307 authority audit — Node conformance

Status: **BlockedSpec**
Date: 2026-08-22
Owner: Codex
Release: G5

## Outcome

`NODE-5307` is a non-normative conformance checklist, not an implementation
authorization. The execution plan asks for evidence covering initialization,
multi-tick state, multi-rate execution, stale or missing inputs, deadline
hits and misses, fallback, restart or safe mode, replay, and deterministic
static scheduling. It does not define the Node language contract, a versioned
fixture or manifest format, an oracle, an evidence schema, or the diagnostic
and identity rules needed to judge those cases.

No accepted RFC currently supplies the missing contract. The planned RFC-K502
authority is absent, and the prerequisites represented by the open Critical,
replay, ownership, concurrency, Native/ABI, and evidence gaps remain
unresolved. The task therefore remains `BlockedSpec`; a runner, fixture
schema, oracle, or public command would invent semantics below the repository's
authority boundary.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:288-300` is an execution
  checklist only. It is below accepted RFCs, `SEMANTICS.md`, `LANGUAGE.md`,
  conformance tests, and `ROADMAP-1.0.md` under `AGENTS.md`.
- `docs/SEMANTICS.md` defines the v0.0.1 Seed as the first twelve Core forms
  plus `Console.Write`; `NodeStep` and the Node runtime are not Seed
  semantics. Its later Node discussion is a design sketch and its reserved
  feature boundary forbids silently treating an absent implementation as
  supported behavior.
- `docs/LANGUAGE.md` likewise excludes Node, Task, Actor, Native, Ownership,
  Kernel, and related facilities from the first version. The roadmap places
  Node conformance in G5 after the earlier replay, resource/Native, and
  restricted-lowering exits.
- `GAP-CRITICAL-PROFILE-001` is open for the minimum verifiable Core, Node,
  Contract, and evidence boundary. It does not yet authorize Node timing,
  Fault, boundedness, or evidence behavior.
- `GAP-DETERMINISTIC-REPLAY-001` is open for event ordering, effect/input
  identity, corruption, privacy, divergence, and migration. `PROTO-REPLAY` is
  a Future inventory entry without a versioned schema or executable fixtures.
- Actor/task/mailbox/re-entry, ownership, Native/ABI, and Kernel/Device gaps
  remain open. Consequently the Node/Actor boundary and target execution
  model cannot be inferred from implementation notes or from the execution
  plan.
- Accepted `RFC-0019` covers interpreter–VM logical event equivalence and
  source-span/ProgramId differential evidence. Accepted `RFC-0020` covers VM
  host cancellation and resource/fuzz evidence. Neither defines Node ticks,
  static schedules, virtual clocks, target deadlines, or replay conformance.

## Repository evidence

The repository has no accepted Node parser or Checked Core contract, Node
runtime, static scheduler, virtual clock, target adapter, Native backend,
Actor bridge, replay adapter, Node fixture manifest, or conformance oracle.
Existing interpreter/VM tests and differential evidence cover the Seed
pipeline or VM host controls only; they cannot serve as a Node conformance
oracle. There is also no registered public protocol or stable diagnostic
allocation for the checklist's cases.

The checklist leaves material observable questions unanswered, including:

- initial state and state transition identity across ticks;
- rate, clock, target, input absence, stale-input, and ordering semantics;
- deadline, overrun, Fault precedence, fallback, restart, and safe-mode rules;
- schedule graph, WCET, memory, ABI, simulation/reference/target evidence
  boundaries;
- replay event/effect identity, divergence, corruption, privacy, and migration;
- the bridge between Node, Actor, Task, ownership, and bounded mailbox rules;
- stable Semantic IDs, original UTF-8 byte spans, bilingual registered
  `L-<DOMAIN>-<NUMBER>` diagnostics, and deterministic bounded evidence.

## Required authority before implementation

An accepted replacement for RFC-K502 and the related open gaps must define,
at minimum:

1. A versioned conformance protocol, fixture manifest, oracle, expected state
   and output trace, Fault/evidence representation, and compatibility policy.
2. Exact initialization, tick, state, multi-rate, input absence/staleness,
   deadline/overrun, fallback, restart, and safe-mode semantics.
3. Static schedule, clock/rate/target/WCET/memory/ABI rules and explicit
   simulation-versus-reference-versus-hardware evidence boundaries.
4. Replay log and identity rules, including event/effect/input/output order,
   privacy, corruption, divergence, and migration behavior.
5. Node–Actor–Task bridges, ownership, mailbox/backpressure, bounded output,
   re-entry, and failure/safe-state behavior.
6. Stable Semantic ID and source-span rules, bilingual registered diagnostics,
   deterministic ordering, and resource/evidence limits.
7. Offline executable positive, negative, boundary, Unicode 17.0.0, CRLF/BOM,
   migration, replay, and interpreter/VM/Native differential fixtures.

## Compatibility and deferred work

This audit changes no parser, resolver, Checked Typed Core, evaluator,
bytecode, VM, Node, Native, Actor, CLI, LSP, diagnostic, schema, dependency,
or public protocol. It preserves Unicode 17.0.0 and original UTF-8 byte spans,
and keeps host paths, wall-clock timing, addresses, allocation details, and
debug output out of Ling identity. Conformance evidence must distinguish
reference simulation from target execution and fail closed when expected
behavior is unknown.

Implementation is deferred until `NODE-5301` through `NODE-5306`, the Node,
Critical, replay, ownership/concurrency, Native/ABI, and evidence authorities,
and their executable fixtures are accepted. Do not add a placeholder
conformance runner, fixture schema, oracle, diagnostic allocation, CLI/LSP
route, public protocol, support claim, or API while those decisions are open.
