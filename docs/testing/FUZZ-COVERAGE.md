# Ling fuzz coverage inventory

Status: implemented-reader inventory and smoke coverage (2026-08-23)

This document is the evidence record for `REL-6601`. It distinguishes a
checked-in fuzz harness from a planned entry point that has no implementation.
The latter is an explicit gap, not an implied protocol or a placeholder
binary. The inventory does not claim that G6 or `v1.0` is complete; the G6
release gate still depends on the G1--G5 exits.

## Harness policy

The CI smoke job uses the same bounded baseline for every available harness:

| Field | Baseline |
| --- | --- |
| Toolchain | `nightly-2026-08-15` |
| `cargo-fuzz` | `0.13.2` |
| Corpus replay | `-runs=256` |
| Per-input timeout | `-timeout=120` seconds |
| RSS limit | `-rss_limit_mb=2048` |
| Dictionary | None checked in; corpus inputs are the seed set |
| Crash triage | Ling maintainers; the owning compiler/package/runtime area records the issue and retains the minimized input, toolchain, target, and reproduction command |
| Artifact rule | Long runs use a separate artifact directory; CI never rewrites checked-in corpora |

The Windows host can perform the locked compilation check. Portable execution
of the libFuzzer smoke job is provided by the pinned Ubuntu CI job because the
MSVC AddressSanitizer runtime is an optional host component.

## Current harnesses

| Planned entry point | Harness | Corpus | Dictionary | Timeout / RSS | Triage owner | Evidence |
| --- | --- | ---: | --- | --- | --- | --- |
| lexer / parser / Unicode | `source_bytes`, `lexer_utf8`, `parser_utf8` | 2 / 2 / 5 | none | 120 s / 2048 MB | compiler maintainers | `fuzz/fuzz_targets/*.rs`, `fuzz/corpus/{source_bytes,lexer_utf8,parser_utf8}` |
| formatter | `formatter_utf8` | 1 | none | 120 s / 2048 MB | formatter/compiler maintainers | `fuzz/fuzz_targets/formatter_utf8.rs` |
| diagnostic / schema decoder | `audit_schema_bytes`, `semantic_schema_bytes` | 1 / 2 | none | 120 s / 2048 MB | diagnostics/format/semantic maintainers | `fuzz/fuzz_targets/{audit_schema_bytes,semantic_schema_bytes}.rs` |
| bytecode verifier | `bytecode_bytes` | 2 | none | 120 s / 2048 MB | bytecode/VM maintainers | `fuzz/fuzz_targets/bytecode_bytes.rs` |
| package / lock | `manifest_bytes`, `lock_bytes` | 4 / 1 | none | 120 s / 2048 MB | project/package maintainers | `fuzz/fuzz_targets/{manifest_bytes,lock_bytes}.rs` |

The available targets compare repeated decoding or projection, preserve the
original source labels and byte spans where the API exposes them, and bound
diagnostic rendering. The formatter target exercises the compiler-owned Format
IR and does not introduce a formatter CLI or an edit protocol. The semantic
schema target exercises both isolated exact-version readers (`ling.semantic/0.1`
and `ling.semantic/0.2`) without treating either as executable checked input.

## Planned entry points without a current harness

| Planned entry point | Current state | Why no harness is added | Owner when authorized |
| --- | --- | --- | --- |
| archive | Not implemented | No accepted archive decoder or package archive protocol exists in the current Seed workspace. | package maintainers |
| replay / evidence | Future / unsupported | No accepted replay or evidence reader/writer implementation is present. | runtime/evidence maintainers |
| FFI metadata / shims | Future / unsupported | No accepted FFI metadata or shim contract is implemented. | FFI/target maintainers |
| Device IR / binary metadata | Future / unsupported | No device IR or binary metadata implementation is present. | device/backend maintainers |
| LSP / DAP protocol | Future / unsupported | LSP and DAP public protocol behavior is not accepted for the Seed surface. | editor/protocol maintainers |
| Zed Tree-sitter corpus | Differential corpus only | `editors/tree-sitter-ling/test/corpus` and its scripts are deterministic parser/editor fixtures, not a libFuzzer harness. | editor maintainers |

These rows remain visible so future work cannot silently claim coverage. They
must acquire an accepted protocol or implementation contract, a bounded input
decoder, a corpus and dictionary decision, timeout/RSS policy, a named triage
owner, and positive/negative deterministic evidence before being promoted.

## Reproduction commands

The canonical commands and pinned tool versions are maintained in
[`fuzz/README.md`](../../fuzz/README.md) and `.github/workflows/ci.yml`. The
locked local compile gate is:

```text
cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline
```

The local compile gate was run on 2026-08-22. The actual libFuzzer smoke run is
the Ubuntu CI job described above; no Windows sanitizer result is implied by
the local compile check.

## Scope and compatibility

This inventory and the additional Seed harnesses change no Ling grammar,
Typed Core semantics, diagnostic allocation, schema version, CLI, editor
protocol, package publication behavior, or runtime authority. They preserve
the `ling` CLI and `.ling` extension, original UTF-8 byte spans, Unicode
17.0.0 requirements, deterministic ordering, and offline locked builds.
