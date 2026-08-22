# DEC-0051: Seed DAP status inventory gate / Seed DAP 状态盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: editor-integration  
> Related authority/gap: `RFC-0004`, `RFC-0014`, `DEC-0050`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `ZED-6804-SEED` child. It does not
implement or register DAP, authorize debugger semantics, expose controls,
define launch/attach behavior, or promote DAP to Preview or Stable. The parent
`ZED-6804` remains `BlockedSpec`; its non-blocking release policy remains in
force.

## Question

The repository records that incomplete debugger support must not block the
language and basic editor surface, while DAP-3601 through DAP-3603 remain
blocked by absent protocol, adapter, registration, runtime metadata, and
fixtures. How can the project protect the exact DAP status matrix and its
authority-audit evidence without creating a debugger process or public
protocol?

## Decision

1. `cargo xtask dap verify` is an internal governance command. It reads
   `docs/testing/DAP-STATUS.md` and validates exactly nine DAP surfaces with
   their `Unavailable`, `Future`, `Partial foundation only`, or `Unsupported`
   states and non-empty evidence/authority cells.
2. The verifier checks the non-blocking `Unavailable / Future` policy, the
   distinction between VM/Fault foundations and DAP, the no-debugger-control
   boundary, and three DAP-3601/3602/3603 authority-audit marker files. It
   fails closed with internal `GOV-DAP-STATUS-*` messages.
3. The command validates inventory and historical-audit markers only. It does
   not run a debugger, register DAP, read settings, contact a registry,
   allocate diagnostics, define wire fields, or change runtime behavior.
4. The command is included in the governance-authority CI gate. DAP Preview
   promotion requires an Accepted wire/lifecycle contract, executable adapter,
   Zed registration, debug metadata, deterministic fixtures, security/resource
   limits, and cross-platform/offline evidence; Stable requires release and
   independent verification evidence.

## Conformance plan

- Run `cargo xtask dap verify` offline and assert nine surfaces: four
  unavailable, three future, one partial foundation, and one unsupported.
- Mutate a matrix row/state, policy phrase, audit marker, or stale-name
  boundary and verify the gate fails closed.
- Run `cargo xtask ci verify` and the existing locked governance, status,
  support, and traceability checks without treating the inventory as DAP
  execution or editor-debugger evidence.
- Repeat independent processes and verify that no debugger process, network
  request, extension registration, source, diagnostic, schema, protocol,
  cache, or system configuration is changed.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- Runtime Fault, VM control, and source-map evidence remain experimental
  foundations, not DAP semantics. No adapter, debugger command, extension
  registration, launch/attach API, migration promise, or placeholder public
  API is added.

## Unresolved alternatives

DAP wire/framing and lifecycle, capabilities, launch/attach/security,
breakpoints/step, stack/scopes/variables, Fault mapping, Task/Actor views,
source-map and identity rules, installation/provenance, platform support,
offline behavior, cancellation/resource limits, and migration remain governed
by DAP-3601 through DAP-3603 and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
