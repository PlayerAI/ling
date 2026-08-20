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
- `manifest_bytes`: arbitrary bytes through two bounded `ling.toml` version 1 reads with distinct diagnostic source labels; successful semantic models, failure codes/spans, and Diagnostic JSON rendering must remain deterministic.

Run a deterministic corpus smoke pass:

```text
cargo +nightly-2026-08-15 fuzz run source_bytes fuzz/corpus/source_bytes -- -runs=256
cargo +nightly-2026-08-15 fuzz run lexer_utf8 fuzz/corpus/lexer_utf8 -- -runs=256
cargo +nightly-2026-08-15 fuzz run parser_utf8 fuzz/corpus/parser_utf8 -- -runs=256
cargo +nightly-2026-08-15 fuzz run manifest_bytes fuzz/corpus/manifest_bytes -- -runs=256
```

Long-running jobs must use a separate artifact directory and retain minimized
crash inputs. Corpus changes are reviewed like conformance fixtures; CI must
never overwrite them automatically.

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
