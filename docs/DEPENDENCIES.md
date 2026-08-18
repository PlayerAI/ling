# Rust 依赖记录 / Rust Dependency Record

> 状态：`0.0.1-dev` 初始审查  
> 锁文件：仓库根目录 `Cargo.lock`  
> 最低 Rust 版本（MSRV）：1.85  
> 审查日期：2026-08-18

普通构建不得访问网络；`Cargo.lock` 是当前可复现依赖集合。升级直接依赖必须更新本记录并重新运行 workspace、Unicode 和 Schema 测试。

Normal builds must not access the network. `Cargo.lock` is the currently reproducible dependency set. Direct dependency upgrades must update this record and rerun workspace, Unicode, and schema tests.

## 直接依赖 / Direct dependencies

| Package | Locked | Purpose | License | Upstream MSRV | `unsafe` in direct source | Maintenance evidence |
| --- | --- | --- | --- | --- | --- | --- |
| `serde` | 1.0.229 | Diagnostic schema serialization and test deserialization | MIT OR Apache-2.0 | 1.56 | Yes, in `serde_core` formatting/serialization internals | Current crates.io release inspected 2026-08-18 |
| `serde_json` | 1.0.151 | Deterministic diagnostic JSON writer and conformance parsing | MIT OR Apache-2.0 | 1.71 | Yes, including validated UTF-8 fast paths | Current crates.io release inspected 2026-08-18 |
| `unicode-ident` | 1.0.24 | Unicode 17 `XID_Start` / `XID_Continue` | (MIT OR Apache-2.0) AND Unicode-3.0 | 1.71 | Yes, bounds-check-elided table lookup | Reports `UNICODE_VERSION = (17, 0, 0)` and has a compile-time assertion |
| `unicode-normalization` | 0.1.25 | Unicode 17 NFC normalization | MIT OR Apache-2.0 | 1.36 | Yes, checked scalar construction and Hangul normalization | Reports `UNICODE_VERSION = (17, 0, 0)` and has a compile-time assertion |
| `blake3` | 1.8.6 | DEC-0012 DefinitionId, BodyId, and ProgramId hashing over custom canonical bytes | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | Not declared upstream | Yes, architecture-specific SIMD and C/assembly FFI paths | Current crates.io release inspected 2026-08-18; default `std` feature only |
| `num-bigint` | 0.5.1 | Mathematical `Int` parsing, typed literals, canonical bytes, and interpreter arithmetic | MIT OR Apache-2.0 | 1.60 | Yes, validated UTF-8 construction, division internals, and target intrinsics | Current crates.io release inspected 2026-08-18; default `std` feature only |
| `sha2` | 0.11.0 | Maintainer-only SHA-256 verification for pinned Unicode inputs | MIT OR Apache-2.0 | 1.85 | Yes, architecture-specific accelerated compression paths | Current crates.io release inspected 2026-08-18; not linked into the `ling` binary |
| `toml` | 1.1.4 | Test-only `expect.toml` conformance reader | MIT OR Apache-2.0 | 1.85 | No in the direct crate (`forbid(unsafe_code)`) | Current crates.io release inspected 2026-08-18 |
| `libfuzzer-sys` | 0.4.13 | Fuzz-only LLVM libFuzzer runtime under the excluded `fuzz/` workspace | (MIT OR Apache-2.0) AND NCSA | Not declared upstream; not part of the main workspace MSRV | Yes, native runtime and FFI boundary | Current crates.io release inspected 2026-08-18; never linked into `ling` |

Workspace code uses `unsafe_code = "deny"`. Dependency `unsafe` is not inherited by that lint, so the entries above are reviewed explicitly and must be rechecked on upgrades.

The `fuzz/` directory has its own lockfile and is excluded from the root workspace. Normal `cargo build/test --workspace --locked` therefore does not resolve fuzz-only dependencies. Fuzz jobs pin `cargo-fuzz` 0.13.2 (MIT OR Apache-2.0; upstream does not declare an MSRV).

## 关键传递依赖 / Key transitive dependencies

- Serialization derive path: `serde_derive`, `proc-macro2`, `quote`, `syn`, and `unicode-ident`.
- JSON path: `serde_core`, `itoa`, `memchr`, and `zmij`.
- Normalization path: `tinyvec` and `tinyvec_macros`.
- BLAKE3 path: `arrayref`, `arrayvec`, `constant_time_eq`, `cc`, `find-msvc-tools`, and `shlex`; target-selected SIMD/assembly implementations remain encapsulated by `blake3`.
- Arbitrary-precision integer path: `num-integer`, `num-traits`, and build-time `autocfg`.
- Unicode checksum tool path: `digest`, `block-buffer`, `crypto-common`, `hybrid-array`, `typenum`, `cfg-if`, `cpufeatures`, and target-specific `libc`.
- Test-only TOML path: `serde_spanned`, `toml_datetime`, `toml_parser`, `toml_writer`, and `winnow`.
- Fuzz-only path: `arbitrary`, `cc`, and the native LLVM libFuzzer runtime selected by `libfuzzer-sys`.

The exact versions are authoritative in `Cargo.lock`. A full per-transitive unsafe and license inventory remains an M0 release-gate task; until it is complete, this record must not be presented as a supply-chain approval for `v0.0.1`.
