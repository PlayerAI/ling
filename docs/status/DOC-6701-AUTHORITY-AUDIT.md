# DOC-6701 Authority Audit

- Task: `DOC-6701` — Formal Documentation Set
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:389-404`
- Release: G6
- Status: `BlockedSpec` for the G6 release gate; the current formal-document
  inventory is recorded as preparatory evidence.

## Decision

`DOC-6701` is `BlockedSpec`. The checklist asks for manuals covering the
implemented Seed language as well as Task/Actor/Replay, Native/Ownership/FFI,
Kernel/Device, Critical/Node/Contract/Evidence, LSP/Zed,
Migration/Compatibility, and Security/Disclosure. The repository has a
substantial Seed reference set and governance/status reports, but the future
capabilities have no accepted semantics, protocols, implementations, or
release support to document as complete.

The inventory therefore records each manual's current source, state, and
boundary. It does not fill future chapters with invented syntax, APIs,
protocol schemas, examples, support claims, or stale command names.

Accepted `DEC-0045` closes only the bounded `DOC-6701-SEED` child: the
internal `cargo xtask docs verify` command prevents drift in the twelve-row
formal-set inventory and its anti-promotion policy text. It does not generate
manuals or promote any future state to a G6 documentation sign-off.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:389-404` is a non-normative documentation
  checklist. It does not authorize any future language feature, protocol,
  backend, editor integration, migration promise, or security policy.
- `docs/ROADMAP-1.0.md:548-554` requires reference, profile, package, FFI,
  kernel, Critical, LSP, migration, and security documentation for a release;
  the roadmap remains planning authority with `stable_basis = false`.
- Accepted RFCs/decisions and `docs/SEMANTICS.md` authorize the Seed language,
  source spans, diagnostics, effects/capabilities, package/lock library slice,
  Audit Source, formatter preservation, query/cache boundaries, and VM
  evidence only. Their reports explicitly defer future manuals.
- Governance authority, protocol, support, lifecycle, gap, traceability, and
  status registries preserve explicit Experimental/Preview/Future/Unsupported
  states. Documentation cannot promote a state.
- `AGENTS.md` requires bilingual registered diagnostics, Unicode 17.0.0,
  deterministic/offline behavior, checked Typed Core execution, original
  UTF-8 spans, and no placeholder public APIs or stale legacy names.

## Evidence in this repository

`docs/testing/DOCUMENTATION-INVENTORY.md` maps all twelve planned manuals to
current documents, conformance/implementation evidence, and missing authority.
The Seed set includes `LANGUAGE.md`, `SEMANTICS.md`, `ERROR-CODES.md`, RFCs and
decisions, package/lock/effect/VM/formatter reports, governance registries,
Tree-sitter grammar documentation, and the new reliability/security/performance
audits.
`cargo xtask docs verify` checks only the deterministic manual/state inventory
and the policy phrases that keep future chapters and stale names explicit.

The future manuals remain planning/status evidence only. No stable Task,
Actor, Replay, Native, FFI, Kernel, Device, Critical, proof/evidence, LSP,
package publication, migration, deprecation, SBOM, or disclosure contract is
claimed.

## Required authority before G6 completion

Before promoting this task, each manual needs:

1. an Accepted specification and lifecycle record with normative clauses;
2. implementation/conformance/diagnostic/schema/protocol links and explicit
   profile/target support;
3. positive, negative, malformed, migration, compatibility, Unicode 17.0.0,
   original-span, deterministic/offline, and cross-platform evidence;
4. bilingual examples and operational instructions that do not imply
   unsupported features or host behavior; and
5. generated documentation, registry, status, traceability, and release-note
   drift checks.

## Compatibility and deferred work

This audit changes no language grammar, semantics, Typed Core, diagnostics,
schemas, Semantic IDs, CLI, package behavior, runtime, editor protocol,
dependency, or public API. It preserves `ling`/`.ling`, original UTF-8 spans,
Unicode 17.0.0, deterministic ordering, and offline builds.

No future manual is presented as implemented, and no placeholder examples,
command, protocol, backend, migration promise, or security claim is added.
The complete G6 documentation set remains deferred until its authorities,
implementations, and release evidence exist.
