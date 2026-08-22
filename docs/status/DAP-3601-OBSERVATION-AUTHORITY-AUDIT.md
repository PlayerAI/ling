# DAP-3601-OBSERVATION Authority Audit — Debugger Boundary Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DAP-3601-OBSERVATION is limited to test-local vocabulary for a future DAP
debugger. It does not define a transport, debugger command, runtime hook,
source-map contract, or editor protocol. Public DAP-3601 remains
`BlockedSpec`, and the stale `zero dap --stdio` spelling is not implemented.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:516-528` is non-normative and
  makes DAP contingent on VM/Native source maps, runtime debugging behavior,
  ProgramSnapshot/binary identity, Fault categories, and an Accepted debugger
  RFC. It cannot authorize a wire schema or debugger semantics.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public CLI as `ling`,
  require verified Typed Core input, and preserve UTF-8 spans and Semantic IDs;
  they do not authorize DAP support for the Seed subset.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental bytecode/VM,
  source-map, Fault, and differential foundations only. They do not define
  DAP messages, stop/step behavior, Native debug metadata, or editor support.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
  and `GAP-NATIVE-BACKEND-ABI-001` remain Open. No debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Current implementation evidence

- The workspace has no DAP adapter, stdio framing/reader, debugger command,
  extension, Native debug metadata, launch/attach model, breakpoint index,
  stack/locals projection, or debugger protocol inventory entry.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule fixes stop/continue/step granularity, source versus
  bytecode positions, variable ownership display, Fault mapping, actor/task
  views, capability isolation, sessions, or target/engine support.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned DAP-facing protocol and stdio framing/lifecycle with
   initialization, capability negotiation, launch/attach/disconnect,
   cancellation, errors, session isolation, message limits, security, readers,
   writers, migrations, fixtures, and protocol-inventory stability.
2. Engine-neutral debug semantics for source/bytecode/Native locations,
   ProgramSnapshot/binary identity, breakpoints, stops/steps, frames/scopes/
   variables, mutation, Resource/Managed/ownership, Faults, actor/task views,
   and capability/target/profile restrictions.
3. Accepted VM/Native metadata and runtime hooks that preserve UTF-8 spans,
   Semantic IDs, deterministic identity, and host-output exclusions while
   consuming verified Typed Core/derived artifacts only.
4. Security and resource limits for debuggee execution, foreign code,
   evaluation/inspection, timeouts, cancellation, concurrent clients, and
   sensitive source/value redaction.
5. Stable bilingual diagnostics, authoritative `ling` CLI naming, editor
   configuration, conformance/negative fixtures, and explicit support claims.

## Compatibility and intentionally deferred work

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode 17.0.0 behavior. DAP
parsing/framing, lifecycle, debugger semantics, security, metadata, extension
integration, protocol inventory/migration, fixtures, and support claims remain
deferred until an Accepted debugger RFC and dependent VM/Native and protocol
contracts exist.
