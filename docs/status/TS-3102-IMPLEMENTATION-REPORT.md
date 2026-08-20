# TS-3102 Tree-sitter grammar skeleton implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `14fb7986501abda6eed178b5b7af405fcb0313e9`
> Verified baseline: `main@07398a90e2caa3155af13716d075bd30590ce0b0`

## Outcome

TS-3102 adds [`editors/tree-sitter-ling`](../../editors/tree-sitter-ling/), a standalone-ready in-tree development mirror for Ling's editor-oriented concrete-syntax parser. The generated parser and its locked local toolchain are committed, and the grammar covers the breadth-first categories required by the execution plan: source file, declarations, types, expressions, patterns, identifiers, literals, and comments.

The package contains 60 generated named CST node types and 23 corpus cases across declarations, expressions, patterns, Unicode input, malformed input, and future syntax. Every generated named node type appears in a corpus expectation. The grammar-local Hello World example parses without `ERROR` or `MISSING` nodes.

This parser is tooling evidence, not a language authority. A tolerant Tree-sitter parse does not make source valid Ling; accepted decisions, compiler specifications, conformance tests, and the compiler parser retain their established precedence.

## Normative clauses and decisions covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3102: breadth-first grammar skeleton, intuitive CST, explicit precedence/associativity, and a corpus case for every top-level category.
- [`14-FIRST-SPRINT-CODEX-TASKS.md`](../ling_execution_plan/14-FIRST-SPRINT-CODEX-TASKS.md) Task G: runnable generation/test/example workflow, Unicode-capable identifiers, rule coverage, bounded incomplete-edit recovery, no external scanner without an ADR, and explicit compiler differences.
- Accepted DEC-0002: the compiler remains authoritative for original UTF-8 byte spans and source-position behavior.
- Accepted DEC-0004: pipeline spelling, left associativity, and its precedence below the current arithmetic/comparison/equality layers.
- Accepted DEC-0005: current literal/delimiter forms and explicit exclusion of character, raw, interpolated, and multiline literals.
- Accepted DEC-0006: current offside and nested-comment semantics are recorded as authoritative constraints; TS-3102 implements only a documented scanner-free approximation pending TS-3103.
- Accepted DEC-0007: module, capability, import, alias, qualified-name, and file-boundary forms.
- Accepted DEC-0009 and DEC-0010: record updates, mutable places, assignment spelling, and `requires` capability syntax.
- Accepted DEC-0013 and DEC-0014: Unit entry patterns and ordinary resolved Prelude constructor names.

Draft RFC-0001, SEMANTICS, and LANGUAGE remain lower-authority Seed baseline inputs. No Draft clause is promoted by this grammar.

## Implementation

- `grammar.js` defines one shallow visible `binary_expression` node with explicit precedence values instead of exposing a mechanical precedence-wrapper tower.
- The current Seed precedence order is assignment, pipeline, equality, comparison, additive, multiplicative, application, projection, and unary. Assignment is restricted to syntactic place forms, and its right-hand side excludes assignment chaining.
- Zero-parameter bindings use `let_declaration`; parameterized bindings use the CST-only `function_definition` view over the compiler's single `LetDeclaration` concept.
- Bare pattern identifiers use the syntax-neutral `identifier_pattern`. Only qualified or payload-bearing constructor forms use `constructor_pattern`; resolution remains responsible for distinguishing bindings from zero-payload constructors.
- The identifier token uses Tree-sitter's `XID_Start`/`XID_Continue` property support plus `_`. Compiler Unicode 17.0.0 normalization and security checks remain authoritative.
- TS-3102 intentionally uses no external scanner. A private atomic newline-plus-ASCII-space `_indent` token supports single-level skeleton blocks without claiming exact offside behavior.
- Ordinary non-nested block comments are recognized. Nested block comments, exact indent/dedent state, delimiter-local soft newlines, and serialized incremental scanner state remain TS-3103 work.
- Built-in Tree-sitter `ERROR`/`MISSING` recovery represents editor state only and is never a successful Ling AST or Typed Core path.
- `tree-sitter-cli` is locked to 0.26.12 in the directory-local npm lockfile. Its install script is explicitly allowlisted; generated C sources are committed so normal Rust builds remain independent and offline.
- All language bindings and publication settings remain disabled until a real consumer requires them. No placeholder public integration API was added.
- [`KNOWN-DIFFERENCES.md`](../../editors/tree-sitter-ling/KNOWN-DIFFERENCES.md) records every deliberately incomplete compiler/grammar boundary and names its owning follow-up task.

The implementation follows KISS by building one breadth-first grammar without an early scanner, DRY by keeping the compiler and grammar map authoritative rather than duplicating semantic checks, YAGNI by excluding future language forms and unused bindings, and SRP by isolating the editor parser under one standalone-ready package.

## Specification gaps or conflicts

- [`GAP-GOV-RFC-STATUS-001`](spec-gaps/GAP-GOV-RFC-STATUS-001.md) remains open. Foundational RFC-0001 syntax is still Draft, so the grammar does not make it Stable or Accepted.
- [`GAP-SEED-BOOLEAN-OPERATORS-001`](spec-gaps/GAP-SEED-BOOLEAN-OPERATORS-001.md) remains open. `&&` and `||` terminate as finite error input; TS-3105 remains `BlockedSpec` and this task does not choose their precedence or semantics.
- Tree-sitter's Unicode property tables do not provide a repository-controlled Unicode 17.0.0 version contract. TS-3104 must generate or otherwise pin identifier ranges and establish compiler-lexer differential evidence.
- The scanner-free layout token cannot compare indentation depths or emit dedents, and the regex block-comment token cannot nest. These are explicit TS-3103 obligations, not silently accepted differences.
- A parser package now exists, so the support registry no longer claims that a Tree-sitter grammar is unsupported. LSP, a Zed integration package, formatting, and semantic mutation remain explicitly unsupported.

No new specification gap was required: each incomplete behavior already has an execution-plan owner, and the unresolved boolean behavior already has a registered gap.

## Tests and verification

Executed locally on 2026-08-20 against the tree committed as `14fb7986501abda6eed178b5b7af405fcb0313e9`:

- `npm run verify` with npm offline mode — `tree-sitter generate`, 23/23 corpus parses, and the grammar-local Hello World parse passed.
- Generated-parser idempotence audit — six generated files checked, zero SHA-256 changes after regeneration.
- Named-node coverage audit — 60 generated named node types, zero absent from corpus expectations.
- Locked dependency audit — one npm dependency: `tree-sitter-cli@0.26.12`.
- Shared-example diagnostic parse — one of four current root examples is error-free; the three layout-sensitive differences are explicitly listed for TS-3103 and are not claimed as parity.
- `cargo xtask governance check-all` — five checks, 42 documents, 26 gaps, 17 lifecycle records, 18 protocols, and 56 diagnostic codes.
- `cargo xtask schema validate-all` — three schemas, four valid fixtures, six invalid fixtures, and one canonical byte fixture.
- `cargo xtask schema compatibility --from N-1 --to N` — zero verified N-1 edges and three explicit `NoPreviousVersion` records.
- `cargo xtask schema corrupt-inputs` — 23 deterministic mutations passed.
- `cargo xtask traceability verify --release v0.0.1` — seven features, 32 conformance fixtures, 51 evidence records, and seven deferred differential paths.
- `cargo xtask support verify` — seven features, three profiles, three hosts, one native target, six backends, one standard package, 18 protocols, and nine explicitly unsupported records.
- `cargo xtask status verify` — 13 tasks, all Done; seven features retain explicit stabilization blockers.
- `cargo xtask ci verify` — eight named gates, 19 commands, and three workspace-test hosts.
- `cargo xtask seed reproduce` — four surfaces, eight independent processes, and 41,866 compared output bytes.
- Unicode 17.0.0 regeneration — passed with no generated-source diff.
- Explicit Semantic Graph, CLI output, and Seed example determinism tests — passed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo test --workspace --all-features --locked --offline` — 234 tests passed, plus doc-test harnesses.
- `cargo +1.85 check --workspace --all-features --locked --offline` — passed with the declared MSRV.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Execution-plan `SHA256SUMS.txt` — all 27 entries passed after the backlog transition.
- Active local Markdown target audit — 761 targets across 82 files resolved, zero missing; the frozen execution-plan baseline and installed npm dependencies were excluded.
- `git diff --check` — passed after enforcing the package's LF policy.

## Compatibility impact

- Language semantics and compiler parser: unchanged.
- Public diagnostics: unchanged; no code, severity, bilingual template, Fact, Repair, or original UTF-8 byte span changed.
- Public schemas/protocols and CLI: unchanged.
- Semantic IDs and canonical bytes: unchanged; CST names and Tree-sitter recovery nodes are excluded from identity inputs.
- Support registry: the stale unsupported Tree-sitter clause was removed; remaining editor-tooling limitations are unchanged.
- Dependencies: no Rust dependency changed. One repository-local npm development dependency and its integrity lock were added; it is neither linked into nor shipped with `ling`.

## Determinism and Unicode

All six generated parser artifacts are byte-idempotent under the exact locked CLI. Corpus order and expected trees are checked in, and no host path, timestamp, allocation identity, hash-map order, or debug output enters a language or public protocol artifact.

Ling remains pinned to Unicode 17.0.0. The Tree-sitter skeleton accepts Unicode XID-shaped identifiers for editing, but it does not claim version parity, NFC validation, confusable detection, mixed-script policy, or identifier-security enforcement. Those remain compiler-owned until TS-3104 adds generated-range and differential evidence.

## Intentionally deferred

- TS-3103 exact offside scanner/equivalent layout implementation, serialized scanner state, full newline/comment cases, and nested block comments. This is the next Ready task.
- TS-3104 generated Unicode 17.0.0 identifier parity and compiler differential corpus.
- TS-3105 exhaustive precedence-pair evidence; boolean operators remain blocked by the accepted gap process.
- TS-3106 complete pattern/type edge coverage, TS-3107 systematic edit recovery, and TS-3108 shared compiler differential testing.
- Tree-sitter queries, Zed extension packaging, LSP integration, formatter integration, and language-specific published bindings.
- Any Stable Tree-sitter node compatibility guarantee.
