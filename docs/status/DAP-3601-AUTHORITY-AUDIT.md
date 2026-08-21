# DAP-3601 Authority Audit — Debugger stdio Adapter

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

DAP-3601 proposes an independent Debug Adapter Protocol (DAP) process on
stdio, initially for the Explore VM and later for Native, with the editor
extension only launching the adapter. The execution plan requires stable
source maps, breakpoint/step/stack/variables behavior, ProgramSnapshot and
binary identity, Fault/exception categories, and an Accepted debugger RFC
before this work begins.

No DAP adapter, stdio process, command, wire schema, launch/attach handler,
source-map bridge, breakpoint/step/stack/locals model, Fault mapper,
capability gate, cancellation protocol, debugger manifest, dependency, or
public debugger API is added. The stale proposal spelling `zero dap
--stdio` is not carried into implementation; current authoritative CLI naming
is `ling`, and no `ling dap` command is claimed.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:516-528` is non-normative and
  explicitly makes DAP contingent on VM/Native source maps, runtime debugging
  behavior, ProgramSnapshot/binary identity, Fault categories, and an Accepted
  debugger RFC. `:547` says DAP must not block v0.1 and forbids a placeholder
  adapter.
- The backlog row `DAP-3601` points to this proposal and uses the stale
  `zero` spelling. Under repository authority, `docs/SEMANTICS.md` and
  `docs/LANGUAGE.md` fix the CLI as `ling`; lower-authority `zero` text is
  historical planning input and cannot enter commands, manifests, schemas, or
  editor integration.
- Accepted RFC-0014/RFC-0018 define experimental bytecode/VM source-map and
  Runtime Fault foundations; RFC-0019 defines the checked Interpreter–VM
  differential projection. They do not define DAP messages, debugger state,
  breakpoint semantics, Native debug metadata, or an editor transport.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and `GAP-LSP-TRANSACTION-PROTOCOL-001`
  leave public semantic/editor protocol lifecycle and transaction boundaries
  open. No debugger-specific gap or Accepted debugger RFC is registered.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open for Native execution, ABI,
  unwind/Fault, thread/reentry, FFI, targets, and debug-capable backend
  behavior. `PROTO-ABI` and `PROTO-EVIDENCE` are Planned public without
  schemas, readers, migration rules, or fixtures, and no DAP protocol entry
  exists in `docs/governance/protocol-inventory.toml`.
- RFC-N304, RFC-N305, RFC-N306, and any debugger RFC are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no DAP adapter, stdio framing/reader, debugger command,
  editor extension, Native debug metadata, launch/attach model, breakpoint
  index, stack/locals projection, or debugger protocol inventory entry.
  Existing VM source maps and Faults are internal/experimental foundations,
  not an editor-compatible DAP contract.
- No accepted rule fixes stop/continue/step granularity, breakpoint matching,
  source versus bytecode positions, variable lifetime/ownership display,
  exception/Fault mapping, actor/task views, cancellation, security or
  capability isolation, multi-session behavior, or target/engine support.
- No debugger dependency, transport, toolchain, diagnostic allocation,
  extension manifest, public protocol implementation, or `zero` command is
  required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned DAP-facing protocol and stdio framing/lifecycle: initialize,
   capability negotiation, launch/attach/disconnect, cancellation, errors,
   session isolation, message limits, transport security, and compatibility
   policy. Register it in the protocol inventory with readers, writers,
   migration, fixtures, and stability.
2. Engine-neutral debug semantics: source/bytecode/Native location mapping,
   ProgramSnapshot/binary identity, breakpoint conditions/logpoints, stop and
   step boundaries, stack/frames/scopes/variables, mutation visibility,
   Resource/Managed/ownership handling, Fault/exception categories, actor/task
   views, and capability/target/profile restrictions.
3. Accepted VM/Native debug metadata and runtime hooks that preserve UTF-8
   spans, Semantic IDs, source maps, and deterministic identity without
   exposing host paths, addresses, Rust layout, allocation order, or debug
   strings as Ling semantics. Native behavior must consume verified Typed
   Core/IR only.
4. Security and resource limits for debuggee processes, foreign/Native code,
   file and network access, evaluation/inspect expressions, timeouts,
   cancellation, concurrent clients, and sensitive source/value redaction;
   debugger tooling must not silently enter the TCB.
5. Bilingual stable diagnostics, CLI naming (`ling`), editor configuration,
   conformance and negative fixtures, and explicit Preview/Experimental
   support claims for Explore VM, Native, and future Actor/Task views.

## Evidence and compatibility impact

The eventual implementation needs DAP framing/capability/launch/attach and
cancel fixtures; source-map and UTF-8/LSP-position conversions; breakpoint,
step, stack, scope, variable, Fault/exception, Resource/Managed, and
snapshot/binary-identity cases; malformed/unknown/oversized messages;
multi-session/security/timeout tests; VM/Native differential debug evidence;
deterministic error ordering; schema migration and independent reader tests;
and offline reproducibility. It must preserve stable `L-<DOMAIN>-<NUMBER>`
diagnostics, original spans, Semantic IDs, and Unicode 17.0.0 without
promising debugger support or carrying the stale `zero` name into any public
surface.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode behavior. It adds no DAP
adapter, command, wire schema, debugger dependency, diagnostic, public
protocol implementation, extension manifest, or placeholder API.

## Intentionally deferred

DAP stdio adapter and schema, debugger lifecycle/capabilities, launch/attach/
cancel/session security, VM/Native debug metadata and source maps, breakpoint/
step/stack/variables/Fault/ownership projections, actor/task views, protocol
inventory and migrations, extension integration, fixtures, and all debugger
support claims remain deferred until an Accepted debugger RFC and the
dependent VM/Native, ABI, identity, semantic-protocol, and evidence contracts
exist.
