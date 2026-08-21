# DAP-3602 Authority Audit — Zed Debugger Registration

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

DAP-3602 proposes registering a debugger in a Zed language configuration,
having an extension launch the DAP adapter, mapping launch/attach settings,
running a build task, and converting a Ling run task into a debug scenario.
The proposal depends on DAP-3601 and the VM/Native debugger contracts; it is
not an accepted editor, extension, CLI, or public protocol specification.

No Zed extension directory, manifest, language configuration, debugger
registration, launch/attach mapping, build task, executable locator,
environment contract, or public editor integration is added. The plan's
`zero build` spelling is stale and is not copied into implementation; the
authoritative CLI name is `ling`, but no `ling build` contract is claimed by
this task.

## Normative traceability

- `docs/ling_execution_plan/05-ZED-EXTENSION.md:516-536` is non-normative and
  makes registration contingent on DAP-3601, stable VM/Native source maps,
  runtime breakpoint/step/stack/variables behavior, ProgramSnapshot/binary
  identity, Fault categories, and an Accepted debugger RFC. `:547` forbids
  false buttons or placeholder adapters before G3.
- The backlog row `DAP-3602` points to this proposal and inherits stale
  `zero` command examples. `docs/SEMANTICS.md` and `docs/LANGUAGE.md` fix
  the public CLI as `ling`; historical planning spellings cannot enter Zed
  manifests, build tasks, locators, or command schemas.
- Accepted RFC-0014/RFC-0018/RFC-0019 provide experimental bytecode/VM
  source-map, Fault, and Interpreter–VM differential foundations only. They
  do not define a Zed extension manifest, debugger registration, DAP launch
  mapping, or Native debug metadata.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` and `GAP-LSP-TRANSACTION-PROTOCOL-001`
  leave public editor/semantic protocol lifecycle and transaction boundaries
  open. No Zed or debugger protocol entry is registered in
  `docs/governance/protocol-inventory.toml`.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open for Native execution, ABI,
  unwind/Fault, threads/reentry, FFI, targets, and debug metadata. `PROTO-ABI`
  and `PROTO-EVIDENCE` are Planned public without schemas, readers, migration
  rules, or fixtures.
- RFC-N304, RFC-N305, RFC-N306, and any debugger RFC are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The repository contains the Zed planning document but no Zed extension
  package, manifest, language config, debugger registration, DAP adapter,
  launch task, or executable locator. Existing VM source maps/Faults are
  internal/experimental foundations, not an editor integration contract.
- No accepted rule defines extension version/installation, executable
  discovery and trust, project-root/environment propagation, `ling` build/run
  arguments, launch versus attach ownership, session restart, capability
  negotiation, target/profile selection, or failure reporting.
- No editor dependency, network/package installation, toolchain, diagnostic
  allocation, public protocol implementation, or stale `zero` command is
  required for this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. DAP-3601's versioned adapter and wire/lifecycle contract, including
   capabilities, launch/attach/disconnect/cancel, message limits, errors,
   session isolation, and security; the adapter must be inventoried with
   reader/writer, migration, fixtures, and stability metadata.
2. A Zed extension package contract: manifest identity/version, language
   configuration fields, debugger registration shape, adapter discovery and
   installation, supported Zed/DAP versions, update/rollback behavior,
   permissions, platform targets, and offline/locked development workflow.
3. Deterministic mapping from Ling project/run/build scenarios to DAP launch
   and attach arguments: explicit root and `.ling` sources, `ling` command
   spelling, profile/target/capabilities, environment, working directory,
   source/binary identity, diagnostics, and exit/cancellation behavior.
4. Source-map, ProgramSnapshot/binary identity, breakpoint/step/stack/scope/
   variable/Fault/ownership semantics and Native/VM metadata contracts that
   the extension must consume without reinterpreting programs or exposing
   host paths, addresses, Rust layout, allocation order, or debug text.
5. Stable bilingual diagnostics, conformance and negative fixtures, Zed smoke
   tests, security/resource limits, and explicit Preview/Experimental support
   claims; missing adapter/build/runtime support must be visible rather than a
   nonfunctional button.

## Evidence and compatibility impact

The eventual implementation needs manifest/config/registration fixtures;
adapter discovery and version/capability negotiation; launch/attach/build/run
argument and project-root/environment cases; `.ling`/UTF-8 source mapping;
breakpoint/step/stack/scope/variable/Fault/ownership projections; malformed or
unknown configuration, missing executable, cancellation, timeout, permission,
and multi-session failures; VM/Native smoke and differential evidence; schema
migration and offline reproducibility. It must preserve stable
`L-<DOMAIN>-<NUMBER>` diagnostics, original spans, Semantic IDs, Unicode
17.0.0, and the authoritative `ling` CLI without promising Zed support before
the required contracts exist.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, CLI, extension, or Unicode behavior. It adds no Zed
package, manifest, debugger registration, DAP adapter, build task, dependency,
diagnostic, public protocol implementation, or placeholder API.

## Intentionally deferred

Zed extension package and language configuration, debugger registration,
adapter discovery/versioning, launch/attach/build/debug locator mapping,
project-root/environment/permission rules, VM/Native debug metadata and source
maps, DAP/session security, fixtures/smoke tests, protocol inventory and
migrations, and all Zed debugger support claims remain deferred until DAP-3601
and an Accepted debugger/Native/editor protocol authority exist.
