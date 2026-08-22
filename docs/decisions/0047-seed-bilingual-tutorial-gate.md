# DEC-0047: Seed bilingual tutorial coverage gate / Seed 双语教程覆盖门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: documentation-engineering  
> Related authority/gap: `RFC-0001`, `RFC-0019`, `DEC-0018`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `DOC-6703-SEED` child. It does not
complete the G6 bilingual-tutorial release gate or authorize Stable
localization, aliases, profiles, ownership, Native/FFI, concurrency, package,
LSP, Zed, migration, or release behavior. The parent `DOC-6703` remains
`BlockedSpec` until the 1.0 support matrix and release evidence are Accepted.

## Question

The repository already contains Chinese-first and semantically equivalent
English Seed tutorial sources, bilingual instructions, registered negative
fixtures, and explicit unsupported boundaries. Without a drift check, those
sources or their boundary explanations could silently diverge from the
process-level evidence and support registries. A documentation-only verifier
can protect this evidence without executing programs or adding language
behavior.

## Decision

1. `cargo xtask tutorial verify` is an internal governance command. It reads
   `docs/testing/TUTORIAL-COVERAGE.md`, `docs/TUTORIAL.md`, and the two tutorial
   sources, validating exactly two source rows and eight requirement rows.
2. The verifier checks expected language/output labels, non-empty evidence
   cells, bilingual tutorial headings, checked Semantic/Audit and negative-
   fixture markers, Unicode 17.0.0 and original UTF-8 span guidance, and the
   explicit unsupported 1.0 boundary markers. It fails closed with internal
   `GOV-TUTORIAL-MATRIX-*` messages.
3. The command validates inventory and source/document markers only. It does
   not run examples, generate source, define syntax or localized aliases,
   allocate public diagnostics, define schemas/protocols, or promote
   Experimental/Preview Seed evidence to Stable.
4. The command is included in the governance-authority CI gate. A future
   tutorial promotion requires Accepted localization policy, stable support
   entries, positive/negative fixtures, deterministic cross-platform runs,
   migration guidance, and retained release evidence.

## Conformance plan

- Run `cargo xtask tutorial verify` offline and assert two bilingual sources
  and eight requirement rows.
- Mutate an isolated source row, requirement row, policy phrase, tutorial
  boundary marker, or source marker and verify the gate fails closed.
- Run the existing locked process-level tutorial and conformance tests without
  treating this inventory gate as execution or Stable-support evidence.
- Repeat independent processes and verify no source, semantic, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` validation command, coverage document,
  and CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- The tutorial source files and observed outputs are existing Seed evidence;
  no public API, localized keyword, profile, ownership rule, migration
  promise, security claim, or placeholder surface is added.

## Unresolved alternatives

Stable tutorial content, localization/alias policy, profile and target
guidance, future runtime/package/editor manuals, migration/deprecation text,
and release sample policy remain governed by the parent `DOC-6703` and later
Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
