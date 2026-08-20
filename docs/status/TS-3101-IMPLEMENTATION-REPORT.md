# TS-3101 Seed grammar map implementation report

> Status: **Done**
> Completed: 2026-08-20
> Final verified implementation commit: `4d1b643bd1a971bcd01d101cd81411557d3c3074`
> Verified baseline: `main@b18501d2669048e685e91f7e1f5ce80528676604`

## Outcome

TS-3101 creates [`docs/grammar-map.md`](../grammar-map.md), the auditable input contract for the future `tree-sitter-ling` grammar. It maps 52 current Seed syntax rows across lexical/layout rules, declarations/types, patterns, and expressions to exact compiler CST/AST concepts, proposed Tree-sitter nodes, existing evidence, and planned corpus cases. It separately inventories eight private parsing/recovery helpers and eight groups of deliberately deferred or rejected syntax.

The work was recorded in two implementation slices:

- `c09f98631fba32a8102d1cd8717a8d757ecaab28` adds the grammar map, bilingual README discovery links, authority registration, and traceability evidence.
- `4d1b643bd1a971bcd01d101cd81411557d3c3074` records the boolean-operator mismatch discovered by the audit as a formal specification gap and TS-3105 blocker.

No Tree-sitter repository, generated parser, Zed extension, query, external scanner, new Ling parser rule, or public syntax was added.

## Normative clauses and decisions covered

- [`05-ZED-EXTENSION.md`](../ling_execution_plan/05-ZED-EXTENSION.md) TS-3101: every proposed grammar rule maps to existing syntax authority/corpus or is explicitly private recovery structure.
- Accepted DEC-0002: original UTF-8 byte spans and normalized line-ending boundaries remain compiler-owned.
- Accepted DEC-0004: pipeline precedence, left associativity, continuation form, and final-argument lowering boundary.
- Accepted DEC-0005: numeric/Text/Bool/Unit literals, tuple disambiguation, record/list separators, and unsupported character/raw/interpolated/multiline literals.
- Accepted DEC-0006: offside layout tokens, continuations, comments, delimiter behavior, recovery bounds, and EOF handling.
- Accepted DEC-0007: module/import forms, aliases, ordering, and qualified names.
- Accepted DEC-0009 and DEC-0010: record update, assignable Place spelling, `<-`, and module `requires` boundaries.
- Accepted DEC-0013 and DEC-0014: Unit `main` pattern and ordinary resolved Prelude constructor names.
- Draft RFC-0001, SEMANTICS, and LANGUAGE remain lower-authority Seed baseline inputs and are labeled Draft rather than promoted.

The map is registered as Active implementation evidence with `stable_basis = false`. Proposed Tree-sitter node names are not public Ling protocols, semantic nodes, or Semantic ID inputs.

## Implementation

- The map defines shallow, query-oriented node naming, private-helper conventions, built-in `ERROR`/`MISSING` recovery boundaries, and the compiler's continued role as the validity oracle.
- Lexical/layout rows cover source decoding, Unicode 17.0.0 identifiers, qualified names, literals, comments, offside tokens, continuations, and member separators.
- Declaration/type rows cover module/capability/import ordering, `let`, record/variant/alias declarations, type parameters, named/applied/product/tuple/function types, and the current declaration-level annotation boundary.
- Pattern rows cover bindings, wildcard, literals, qualified/unqualified constructors, Unit, tuple, and record patterns without inventing semantic roles in the lexer.
- Expression rows cover blocks, `if`, `match`/guards, assignment, pipeline, equality/comparison/arithmetic, unary, application, projection, names/literals, Unit/group/tuple, record/update, and list forms.
- Arithmetic/comparison/equality map to one proposed named `binary_expression` node. Precedence belongs in grammar declarations and corpus cases rather than a deep visible wrapper tower.
- Eight private helpers factor declaration/expression/pattern/type selection, member separation, layout blocks, continuation handling, and bounded editing recovery. They cannot be cited as language features.
- Eight deferred groups prevent placeholder support for recursive `and` groups, unresolved boolean operators, parameter annotations, future literals, rejected delimiter shapes, shebangs, localized/raw identifiers, and post-Seed features.
- Existing compiler fixtures are linked directly; every row also names the required future Tree-sitter corpus file/case family.
- The bilingual README now indexes the map. Authority and traceability generated views include it, and the aggregate governance count assertion advances to the current registry size.

The design follows KISS by mapping one current Seed surface rather than building a second parser. It follows DRY by pointing to the compiler's existing nodes and fixtures. It follows YAGNI by excluding future syntax and by keeping helpers private. The map has one responsibility: constrain subsequent grammar work.

## Specification gaps or conflicts

- `RFC-0001` remains Draft even though the repository has an implemented/released Seed. [`GAP-GOV-RFC-STATUS-001`](spec-gaps/GAP-GOV-RFC-STATUS-001.md) remains open, so foundational rows are labeled **Seed baseline**, not Accepted or Stable.
- Draft SEMANTICS §8.3 specifies short-circuit `&&`/`||`, while Draft RFC-0001's Seed EBNF omits them and the compiler parser rejects the lexer tokens. [`GAP-SEED-BOOLEAN-OPERATORS-001`](spec-gaps/GAP-SEED-BOOLEAN-OPERATORS-001.md) now blocks TS-3105 from choosing acceptance, precedence, associativity, or lowering.
- `and` is a current keyword token, but no current production/corpus defines recursive binding groups; Accepted DEC-0012 explicitly defers multi-definition recursion. The map requires finite error recovery and no declaration node.
- Draft design examples show per-parameter annotations, but the current Seed parser only accepts the declaration-level annotation. The map does not promote the example syntax.
- Most future keywords are still lexed as identifiers by the compiler. Tree-sitter must mirror current lexical behavior without highlighting them as active feature syntax.

TS-3102 remains executable because it can build the bounded compiler-mirroring skeleton and treat unresolved operators as error input. TS-3105 must wait for its newly registered decision.

## Tests and verification

Executed locally on 2026-08-20 against the final verified implementation commit:

- Grammar-map inventory audit — 52 mapped syntax rows, eight private helper rows, and eight deferred/rejected groups.
- Grammar-map local links — 144 targets resolved, zero missing.
- `cargo xtask governance check-authority` — passed: 42 documents, 16 Accepted; `GRAMMAR-MAP` is Active evidence and not a Stable basis.
- `cargo xtask governance check-gaps` — passed: 26 Open gaps and six G1 gates; the new gap blocks TS-3105.
- `cargo xtask governance check-all` — passed: five checks, 42 documents, 26 gaps, 17 lifecycle records, 18 protocols, and 56 diagnostic codes.
- `cargo xtask traceability verify --release v0.0.1` — passed: seven features, 32 conformance fixtures, 51 evidence records, and seven deferred differential paths.
- `cargo xtask status verify` — passed: 12 tasks, all Done; seven features retain explicit stabilization blockers.
- `cargo xtask ci verify` — passed: the eight-job, 19-command, three-host CI contract is unchanged.
- `cargo test --package xtask --locked --offline` — 91 tests passed.
- `cargo test --workspace --all-features --locked --offline` — 234 tests passed, including doc-test harnesses.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` — passed.
- `cargo +1.85 check --workspace --all-features --locked --offline` — passed with the declared MSRV.
- `cargo doc --workspace --all-features --no-deps --locked --offline` — passed.
- `cargo build --workspace --all-features --release --locked --offline` — passed.
- Execution-plan `SHA256SUMS.txt` — all 27 entries passed after the backlog transition.
- Active local Markdown target audit — 754 targets resolved, zero missing; the frozen execution-plan baseline was excluded.
- `git diff --check` — passed.

No Tree-sitter test is claimed because TS-3101 intentionally creates no grammar. `tree-sitter generate`, `tree-sitter test`, and shared-corpus differential testing begin with TS-3102/TS-3108.

## Compatibility impact

- Language semantics and compiler parser: unchanged.
- Public diagnostics: unchanged; no code, severity, bilingual template, Fact, Repair, or original UTF-8 byte span changed.
- Public schemas/protocols and CLI: unchanged.
- Semantic IDs and canonical bytes: unchanged; Tree-sitter names are explicitly excluded from identity inputs.
- Governance: authority registry version remains 1 with one new Active evidence record; the gap registry remains version 1 with one new Open P0 gap.
- Dependencies: unchanged.

## Determinism and Unicode

The map points to deterministic existing source fixtures and fixes proposed rule spellings textually. Generated authority, gap, and traceability views remain drift-checked. No timestamp, host path, hash-map order, allocation identity, or Rust debug representation enters a public artifact.

Unicode remains pinned to 17.0.0. Tree-sitter is explicitly prevented from replacing compiler XID, NFC, confusable/security, or original-byte-span validation. TS-3104 will address editor-token coverage through generated/range or scanner evidence without changing language authority.

## Intentionally deferred

- TS-3102 grammar repository/skeleton, generated parser, and initial corpus files.
- TS-3103 external offside scanner and serialized incremental state.
- TS-3104 Unicode lexer differential implementation.
- TS-3105 boolean operators until `GAP-SEED-BOOLEAN-OPERATORS-001` has an Accepted resolution.
- TS-3106 detailed pattern/type grammar, TS-3107 recovery, and TS-3108 differential harness.
- Zed queries, extension packaging, LSP, and all post-Seed language syntax.
- Any Stable Tree-sitter node compatibility promise.
