# FMT-1506 Implementation Report: Formatter Property Evidence

## Outcome

FMT-1506 adds a deterministic formatter property corpus over the compiler-owned
Author Source path. Every valid fixture is checked for idempotence, compiler
token/signature equivalence, exact comment preservation, and equality of the
full checked semantic snapshot produced by the existing AST → HIR → resolve →
type → effect → Semantic Graph pipeline.

The corpus includes compact core syntax, CRLF with Unicode/documentation and
line-end comments, and nested multiline block comments. The tests are ordinary
offline Rust tests with a fixed corpus rather than an unbounded random generator,
so failures are reproducible and do not introduce a new property-testing
dependency or a second language authority.

## Normative traceability

- Accepted `DEC-0023` §1 keeps the formatter over the compiler CST; §2 requires
  exact scalar and comment bytes; §4 fixes attachment/order preservation; §6
  requires idempotence and compiler revalidation; and §7 fixes conservative
  invalid-source behavior.
- Accepted `DEC-0002` preserves original UTF-8 byte spans used by the compiler
  pipeline and the formatter's lossless projection.
- FMT-1506 in `03-G1-V0.1-LIVING.md` §7 requires
  `fmt(fmt(x)) == fmt(x)`, parse equivalence, checked-core equivalence for valid
  programs, and comment preservation.

## Implementation

- Added formatter dev-dependencies for the existing AST, HIR, resolver, type,
  and effect pipeline; no runtime dependency or public wire protocol changed.
- Added a fixed property corpus in `crates/ling-format/src/author.rs`.
- `syntax_signature` compares non-trivia compiler token kinds and exact token
  spelling before and after formatting.
- `comment_signature` compares every line/documentation/block comment in source
  order and exact original spelling.
- `semantic_snapshot` runs both source forms through `ling-semantic::build` and
  compares canonical `ling.semantic/0.1` JSON, covering checked semantics rather
  than merely formatter output text.
- The existing FMT-1505 disposition and FMT-1504 attachment guards remain in
  the same path, so property failures cannot publish partial output.

## Tests and evidence

The corpus covers:

- generic identity/application and a pure `Main` entry;
- CRLF, Unicode text, documentation comments, and a line-end comment with
  `Console.Write` capability; and
- nested multiline block comments plus conditional expressions.

Executed checks:

- `cargo fmt --all -- --check`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo test -p ling-format --all-targets --locked --offline` (19 tests);
- `cargo test --workspace --all-targets --locked --offline`;
- `cargo xtask governance check-all`; and
- `git diff --check`.

## Compatibility impact

Only formatter test-time dependencies and in-process test helpers changed. No
language grammar, parser semantics, diagnostics, Semantic IDs, Audit Source
bytes, CLI/LSP fields, ABI, or Unicode 17.0.0 tables changed. Corpus execution
is offline and deterministic; it compares canonical semantic JSON and excludes
host paths, allocation identity, hash-map order, and debug output.

## Deferred work

FMT-1507 owns CLI/LSP formatting integration and range/edit protocol decisions.
FMT-1508 owns the explicit proof that Author Source formatting does not replace
or mutate canonical Audit Source rendering. Broader generated/fuzzed corpora can
be added later without changing the accepted formatter boundary.

## Next target

FMT-1507, formatter CLI/LSP integration, is the next execution-plan target, but
its public command and transaction fields remain governed by their separate
accepted protocol decisions.
