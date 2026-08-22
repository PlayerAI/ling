# DAP-3602-OBSERVATION Authority Audit — Zed Debugger Registration Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DAP-3602-OBSERVATION is limited to test-local vocabulary for a future Zed
debugger integration. It does not define an extension package, manifest,
language configuration, debugger registration, adapter locator, launch task,
or editor protocol. Public DAP-3602 remains `BlockedSpec`; stale `zero build`
text is not implemented.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:516-536` is non-normative and
  makes registration contingent on DAP-3601, stable VM/Native source maps,
  runtime breakpoint/step/stack/variables behavior, ProgramSnapshot/binary
  identity, Fault categories, and an Accepted debugger RFC.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix the public CLI as `ling` and
  source extension as `.ling`; historical `zero` spellings cannot enter Zed
  manifests, tasks, locators, or commands.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental VM/source-map,
  Fault, and differential foundations only. They do not define Zed manifests,
  adapter discovery, launch mappings, or Native/editor metadata.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
  and `GAP-NATIVE-BACKEND-ABI-001` remain Open. No Zed or debugger protocol is
  registered in `docs/governance/protocol-inventory.toml`.

## Current implementation evidence

- The repository has no Zed extension package, manifest, language config,
  debugger registration, DAP adapter, launch task, executable locator, or
  editor protocol entry.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule fixes extension installation/trust, executable discovery,
  project-root/environment propagation, `ling` build/run arguments,
  launch/attach ownership, target/profile selection, or failure reporting.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. DAP-3601's versioned adapter and wire/lifecycle contract, inventoried with
   capabilities, readers/writers, migration, fixtures, and stability metadata.
2. A Zed extension contract for manifest identity/version, language fields,
   debugger registration, adapter discovery/installation, Zed/DAP versions,
   update/rollback, permissions, platform targets, and offline/locked workflow.
3. Deterministic mapping from Ling project/run/build scenarios to DAP launch or
   attach arguments, explicit roots and `.ling` sources, authoritative `ling`
   command spelling, profile/target/capabilities, environment, working
   directory, source/binary identity, diagnostics, and cancellation behavior.
4. Source-map, ProgramSnapshot/binary identity, breakpoint/step/stack/scope/
   variable/Fault/ownership and VM/Native metadata contracts consumed as
   verified artifacts without exposing host paths, addresses, Rust layout,
   allocation order, or debug strings as Ling semantics.
5. Stable bilingual diagnostics, conformance/negative fixtures, Zed smoke
   tests, security/resource limits, and explicit Preview/Experimental support
   claims.

## Compatibility and intentionally deferred work

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode 17.0.0 behavior. Extension
packaging/configuration, registration/discovery, launch mapping, permissions,
VM/Native metadata, DAP/session integration, fixtures, protocol inventory,
migrations, and Zed support claims remain deferred.
