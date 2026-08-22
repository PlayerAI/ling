# Ling fuzz targets

This directory is excluded from the main workspace so normal locked builds do
not compile or download fuzz-only dependencies.

Pinned tooling:

- `cargo-fuzz` 0.13.2;
- `libfuzzer-sys` 0.4.13;
- Rust `nightly-2026-08-15` (the same toolchain used by CI).

Targets:

- `source_bytes`: arbitrary bytes through UTF-8/BOM/newline source decoding;
- `lexer_utf8`: valid UTF-8 through Source and Lexer/Layout;
- `parser_utf8`: valid UTF-8 through Source, Parser/CST, and valid-CST AST lowering.
- `formatter_utf8`: valid and invalid UTF-8 through the compiler CST-backed Format IR and formatter disposition.
- `audit_schema_bytes`: UTF-8 bytes through the canonical Audit schema decoder and bilingual diagnostic JSON renderer.
- `manifest_bytes`: arbitrary bytes through two bounded `ling.toml` version 1 reads with distinct diagnostic source labels; successful semantic models, failure codes/spans, and Diagnostic JSON rendering must remain deterministic.
- `lock_bytes`: arbitrary bytes through two bounded `ling.lock/1` reads; validated models, canonical bytes, failure codes/spans, and Diagnostic JSON rendering must remain deterministic.
- `bytecode_bytes`: arbitrary bytes through the bounded `ling.bytecode/1.0` decoder and independent verifier twice; verified models or structured bilingual diagnostics must be deterministic and diagnostics remain bounded. Checked-in `hex:` seeds preserve exact valid/corrupt binary fixtures in reviewable text.

The inventory in [`docs/testing/FUZZ-COVERAGE.md`](../docs/testing/FUZZ-COVERAGE.md)
records every planned G6 entry point, its current harness/corpus, and whether
the repository has an implementation to exercise. A missing future protocol is
recorded as an explicit gap; it is not represented by a placeholder fuzz
binary.

Before running the fuzzers, verify that the checked-in target declarations,
source entry points, corpus counts, and inventory names remain synchronized:

```text
cargo xtask fuzz verify
```

Run a deterministic corpus smoke pass:

```text
cargo +nightly-2026-08-15 fuzz run source_bytes fuzz/corpus/source_bytes -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run lexer_utf8 fuzz/corpus/lexer_utf8 -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run parser_utf8 fuzz/corpus/parser_utf8 -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run formatter_utf8 fuzz/corpus/formatter_utf8 -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run audit_schema_bytes fuzz/corpus/audit_schema_bytes -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run manifest_bytes fuzz/corpus/manifest_bytes -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run lock_bytes fuzz/corpus/lock_bytes -- -runs=256 -timeout=120 -rss_limit_mb=2048
cargo +nightly-2026-08-15 fuzz run bytecode_bytes fuzz/corpus/bytecode_bytes -- -runs=256 -timeout=120 -rss_limit_mb=2048
```

Long-running jobs must use a separate artifact directory and retain minimized
crash inputs. Corpus changes are reviewed like conformance fixtures; CI must
never overwrite them automatically. A crash is triaged by the Ling maintainers
with the owning crate listed in the inventory; minimized input, toolchain,
target, and reproduction command are retained with the issue.

## Windows host note

The MSVC `cargo-fuzz` binary requires the Visual Studio LLVM AddressSanitizer
runtime (`clang_rt.asan_dynamic-x86_64.dll`). A machine without that optional
component can still build every fuzz target with:

```text
cargo check --manifest-path fuzz/Cargo.toml --bins --locked
```

Running with `--sanitizer none` is not an equivalent workaround on MSVC because
libFuzzer coverage symbols still require the sanitizer runtime. The pinned
Ubuntu CI smoke pass is therefore the portable execution gate until the Windows
host has the Visual Studio AddressSanitizer component installed.
