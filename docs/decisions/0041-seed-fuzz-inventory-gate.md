# DEC-0041: Seed fuzz inventory gate / Seed 模糊测试清单门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: reliability-engineering  
> Related authority/gap: `RFC-0020`, `RFC-0002`, `DEC-0023`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `REL-6601-SEED` child. It does not
complete the G6 fuzz-coverage release gate or authorize fuzzing for future
replay, evidence, FFI, device, LSP/DAP, archive, or editor protocols. The
parent `REL-6601` remains `BlockedSpec` for those surfaces and for the G1--G5
release exits.

## Question

The repository already contains eight accepted Seed-level fuzz targets and a
reviewed corpus inventory, but target declarations, source entry points, seed
counts, and the inventory can drift independently. The current CI smoke job
does not need a new public protocol; it needs a deterministic offline check
that the existing test-only evidence remains internally consistent.

## Decision

1. `cargo xtask fuzz verify` is an internal governance command. It validates
   the excluded `fuzz/Cargo.toml` target set, exact target paths, `test/doc/bench
   = false` declarations, `fuzz_target!` source entry points, regular corpus
   files and expected seed counts, and target names recorded in
   `docs/testing/FUZZ-COVERAGE.md`.
2. The accepted Seed target set is exactly `source_bytes`, `lexer_utf8`,
   `parser_utf8`, `formatter_utf8`, `audit_schema_bytes`, `manifest_bytes`,
   `lock_bytes`, and `bytecode_bytes`, with the corpus counts recorded by the
   verifier. Unexpected targets, missing targets, nested corpus directories,
   non-regular entries, and count drift fail closed with internal `GOV-FUZZ-*`
   messages.
3. The check validates declarations and inventory only. It does not run
   libFuzzer, claim sanitizer execution on Windows, define a fuzz result
   schema, retain crash artifacts, or promote any protocol or support state.
   The pinned CI libFuzzer smoke job remains the execution evidence.
4. Future planned entry points remain explicit inventory rows without a
   placeholder binary. Adding one requires its own accepted decoder/schema,
   corpus/resource policy, owner, and evidence before extending this gate.

## Conformance plan

- Run `cargo xtask fuzz verify` offline and assert the eight targets and
  eighteen Seed corpus files are discovered deterministically.
- Mutate an isolated manifest target set in the xtask unit fixture and verify
  target-set drift fails closed.
- Run the existing locked fuzz compilation and pinned CI smoke commands; do not
  treat the inventory check as a sanitizer or cross-platform execution result.
- Repeat the check with independent processes and verify no source, semantic,
  diagnostic, schema, protocol, or support output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` validation command and a CI preflight.
  Ling source syntax, Checked Core, runtime, bytecode, diagnostics, schemas,
  Semantic IDs, CLI protocols, and Unicode 17.0.0 behavior are unchanged.
- No public protocol, diagnostic allocation, dependency, fuzz artifact, or
  cross-platform performance claim is introduced.

## Unresolved alternatives

LibFuzzer execution policy, sanitizer availability, crash retention and
triage, archives, replay/evidence, FFI, device, LSP/DAP, Zed, and release-level
G6 coverage remain governed by the parent task and later accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
