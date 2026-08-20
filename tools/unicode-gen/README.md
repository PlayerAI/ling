# Unicode 17 table generator

`unicode-gen` is a maintainer-only tool. It reads the committed Unicode 17.0.0 inputs under `data/`, verifies every file against the built-in SHA-256 manifest, generates `crates/ling-unicode/src/generated.rs` plus `editors/tree-sitter-ling/src/unicode-identifiers.generated.js`, and formats the Rust result with the pinned `rustfmt`.

```bash
cargo run -p unicode-gen --locked
```

Generation is byte-idempotent. CI reruns the command and rejects any diff in either generated file. Normal `cargo build` and `cargo test` do not execute this tool and do not access the network.

## Inputs

- UCD 17.0.0: <https://www.unicode.org/Public/17.0.0/ucd/>
- UTS #39 data 17.0.0: <https://www.unicode.org/Public/17.0.0/security/>
- Checksums: [`data/SHA256SUMS`](data/SHA256SUMS)
- Unicode data license: [`data/LICENSE-UNICODE.txt`](data/LICENSE-UNICODE.txt)

The Rust tables cover forbidden identifier properties, Script and Script_Extensions, Identifier_Status, Identifier_Type, and confusable mappings. The JavaScript data contains the exact `XID_Start` and `XID_Continue` ranges from the same pinned `DerivedCoreProperties.txt`; the Tree-sitter adapter adds Ling's `_` start rule. XID and NFC remain independently checked against the same Unicode version by conformance tests in `crates/ling-unicode/tests/`.

The optional positional form is:

```text
unicode-gen [data-directory] [rust-output-file] [tree-sitter-output-file]
```
