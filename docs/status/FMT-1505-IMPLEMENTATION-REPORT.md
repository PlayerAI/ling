# FMT-1505 Implementation Report: Incomplete-Source Recovery

## Outcome

FMT-1505 establishes a conservative, deterministic recovery policy for the
Author Source formatter. A valid `FormatDocument` may publish a candidate only
after comment preservation and compiler revalidation succeed. An incomplete or
invalid document is returned byte-for-byte unchanged, including its BOM,
original line endings, localized text, comments, and error-region spelling.

The policy intentionally does not guess missing delimiters, fabricate tokens,
or format a valid-looking prefix next to an error region. Callers receive an
explicit `FormatDisposition`, so a safe unchanged result is distinguishable from
a published formatting result without introducing diagnostics or a wire
protocol.

## Normative traceability

- Accepted `DEC-0023` §1 keeps formatting over the compiler-owned CST and out of
  parser/type-checker/recovery authority; §2 preserves exact scalar and comment
  bytes; §6 requires safe candidate publication; and §7 permits formatting only
  where complete spans and attachments are unambiguous, otherwise requiring the
  original bytes and diagnostics to remain intact.
- Accepted `DEC-0002` preserves original UTF-8 byte spans and source identity;
  unchanged invalid output therefore retains the compiler's original span
  domain.
- FMT-1505 in `03-G1-V0.1-LIVING.md` §7 requires conservative error-region
  handling without user-text damage.

## Implementation

- Added `FormatDisposition` with `Formatted`, `OriginalInvalidSource`, and
  `OriginalRejectedCandidate` states.
- Added `FormatResult` and `format_core_with_disposition`; existing
  `format_core` remains the compatibility convenience API and returns the same
  text as the result object.
- The invalid-source branch returns `FormatDocument::original_text()` without
  normalization or token rendering. This is the stable no-op policy for
  ambiguous/incomplete regions.
- The valid branch renders through the compiler-owned token/attachment path,
  verifies comment sequence preservation, reparses the candidate with
  `ling-syntax`, and falls back atomically to the original snapshot if either
  gate fails.

## Tests and evidence

`crates/ling-format/src/author.rs` tests cover:

- a valid source publishing `FormatDisposition::Formatted`;
- an unterminated text source preserving its exact original bytes and reporting
  `OriginalInvalidSource`; and
- a valid prefix followed by an unterminated source region retaining CRLF and
  the entire original document, proving no partial edit is published.

Executed checks:

- `cargo fmt --all`;
- `cargo clippy -p ling-format --all-targets --locked --offline -- -D warnings`;
- `cargo test -p ling-format --all-targets --locked --offline`; and
- `git diff --check`.

## Compatibility impact

The existing `format_core(&FormatDocument) -> String` behavior is unchanged.
`FormatResult` and `FormatDisposition` are in-process library types only; no
CLI/LSP field, diagnostic code, schema, Semantic ID, Audit Source byte, ABI, or
Unicode 17.0.0 table changed. Determinism follows the immutable
compiler-token/CST projection and does not depend on host paths, allocation
identity, or map order.

## Deferred work

FMT-1506 owns property and semantic-equivalence evidence over valid corpora.
FMT-1507 owns CLI/LSP integration, and FMT-1508 owns the separation proof
between Author Source formatting and canonical Audit Source rendering. A future
accepted decision may add bounded complete-region recovery, but this slice does
not infer or rewrite ambiguous error regions.

## Next target

FMT-1507, formatter CLI/LSP integration, is the next execution-plan target. It
must retain the unchanged-invalid-source contract and property evidence while
waiting for its separate accepted command and transaction protocol decisions.
