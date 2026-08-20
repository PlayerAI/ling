# Rust 依赖记录 / Rust Dependency Record

> 状态：`0.0.1-dev` 初始审查  
> 锁文件：仓库根目录 `Cargo.lock`  
> 最低 Rust 版本（MSRV）：1.85  
> 审查日期：2026-08-19

普通构建不得访问网络；`Cargo.lock` 是当前可复现依赖集合。升级直接依赖必须更新本记录并重新运行 workspace、Unicode 和 Schema 测试。

Normal builds must not access the network. `Cargo.lock` is the currently reproducible dependency set. Direct dependency upgrades must update this record and rerun workspace, Unicode, and schema tests.

## 直接依赖 / Direct dependencies

| Package | Locked | Purpose | License | Upstream MSRV | `unsafe` in direct source | Maintenance evidence |
| --- | --- | --- | --- | --- | --- | --- |
| `serde` | 1.0.229 | Diagnostic schema serialization and test deserialization | MIT OR Apache-2.0 | 1.56 | Yes, in `serde_core` formatting/serialization internals | Current crates.io release inspected 2026-08-18 |
| `serde_json` | 1.0.151 | Deterministic Diagnostic/Semantic/REPL JSON plus Audit string escaping and conformance parsing | MIT OR Apache-2.0 | 1.71 | Yes, including validated UTF-8 fast paths | Current crates.io release inspected 2026-08-18; usage scope rechecked 2026-08-19 |
| `unicode-ident` | 1.0.24 | Unicode 17 `XID_Start` / `XID_Continue` | (MIT OR Apache-2.0) AND Unicode-3.0 | 1.71 | Yes, bounds-check-elided table lookup | Reports `UNICODE_VERSION = (17, 0, 0)` and has a compile-time assertion |
| `unicode-normalization` | 0.1.25 | Unicode 17 NFC normalization | MIT OR Apache-2.0 | 1.36 | Yes, checked scalar construction and Hangul normalization | Reports `UNICODE_VERSION = (17, 0, 0)` and has a compile-time assertion |
| `blake3` | 1.8.6 | DEC-0012 DefinitionId, BodyId, and ProgramId hashing over custom canonical bytes; stable internal-incident fingerprints | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | Not declared upstream | Yes, architecture-specific SIMD and C/assembly FFI paths | Current crates.io release inspected 2026-08-18; default `std` feature only |
| `num-bigint` | 0.5.1 | Mathematical `Int` parsing, typed literals, canonical bytes, and interpreter arithmetic | MIT OR Apache-2.0 | 1.60 | Yes, validated UTF-8 construction, division internals, and target intrinsics | Current crates.io release inspected 2026-08-18; default `std` feature only |
| `rustyline` | 15.0.0 | Interactive TTY line editing and explicit Ctrl-C/EOF events for DEC-0016; non-TTY scripts retain the standard-input path | MIT | Not declared as a fixed version; full workspace verified with Rust 1.85 | No in the direct crate | Pinned below current 18.x to preserve the workspace MSRV; all default features disabled; Windows/Unix support and interrupt API reviewed 2026-08-19 |
| `sha2` | 0.11.0 | Maintainer-only SHA-256 verification for pinned Unicode inputs and generated diagnostic compatibility fingerprints | MIT OR Apache-2.0 | 1.85 | Yes, architecture-specific accelerated compression paths | Current crates.io release inspected 2026-08-18; not linked into the `ling` binary |
| `toml` | 1.1.4 | RFC-0002 `ling.toml` reader plus `expect.toml` conformance decoding; `ling-project` rejects TOML 1.1-only syntax before decoding to preserve the Accepted TOML 1.0 boundary | MIT OR Apache-2.0 | 1.85 | No in the direct crate (`forbid(unsafe_code)`) | Current crates.io release inspected 2026-08-18; production scope and explicit TOML 1.0 compatibility guard reviewed 2026-08-21 |
| `libfuzzer-sys` | 0.4.13 | Fuzz-only LLVM libFuzzer runtime under the excluded `fuzz/` workspace | (MIT OR Apache-2.0) AND NCSA | Not declared upstream; not part of the main workspace MSRV | Yes, native runtime and FFI boundary | Current crates.io release inspected 2026-08-18; never linked into `ling` |

Workspace code uses `unsafe_code = "deny"`. Dependency `unsafe` is not inherited by that lint, so the entries above are reviewed explicitly and must be rechecked on upgrades.

The `fuzz/` directory has its own lockfile and is excluded from the root workspace. Normal `cargo build/test --workspace --locked` therefore does not resolve fuzz-only dependencies. Fuzz jobs pin `cargo-fuzz` 0.13.2 (MIT OR Apache-2.0; upstream does not declare an MSRV).

## 非 Rust 开发工具 / Non-Rust development tooling

这些工具不进入 `ling` 二进制，也不参与普通 Cargo 构建。各自目录中的锁文件是版本与完整性权威；首次安装可能访问网络，锁定安装完成后的生成和测试必须离线可运行。

These tools are not linked into the `ling` binary and do not participate in normal Cargo builds. Their directory-local lockfiles are authoritative for versions and integrity. Initial installation may access the network; generation and tests must run offline after the locked installation.

| Package | Locked | Scope and purpose | License | Install/native boundary | Alternatives and maintenance evidence |
| --- | --- | --- | --- | --- | --- |
| `tree-sitter-cli` | 0.26.12 | Dev-only parser generation, C compilation, corpus tests, and example parsing for `editors/tree-sitter-ling`; exact version and SHA-512 integrity are in its `package-lock.json` | MIT | npm package has an explicitly allowlisted install script that downloads the matching upstream CLI executable; generated C is committed and the CLI is never shipped in `ling` | A global CLI, `cargo install`, or an unpinned release binary would weaken repository-local reproducibility. Current npm release, official documentation, Windows support, and package metadata inspected 2026-08-20. |

## 锁定的传递依赖 / Locked transitive dependencies

下表覆盖根 workspace 锁文件中的全部非直接第三方 package。`Unsafe source` 是保守的源码存在性检查：`Yes` 表示缓存的 crate 源码中至少一个 Rust 文件包含 `unsafe` token，可能位于目标条件、测试或未启用 feature 中；它不表示该路径一定进入 `ling` 二进制。`Not declared` 表示 package manifest 未声明 `rust-version`，不是兼容性保证。

The table covers every indirect third-party package in the root workspace lockfile. `Unsafe source` is a conservative source-presence check: `Yes` means at least one Rust file in the cached crate source contains an `unsafe` token, possibly in target-gated, test-only, or disabled-feature code; it does not claim that the path is linked into `ling`. `Not declared` means the package manifest has no `rust-version`, not that compatibility is guaranteed.

| Package | Locked | License expression | Upstream MSRV | Unsafe source |
| --- | --- | --- | --- | --- |
| `arrayref` | 0.3.9 | BSD-2-Clause | Not declared | Yes |
| `arrayvec` | 0.7.8 | MIT OR Apache-2.0 | 1.51 | Yes |
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | 1.0 | No |
| `bitflags` | 2.13.1 | MIT OR Apache-2.0 | 1.56.0 | Yes |
| `block-buffer` | 0.12.1 | MIT OR Apache-2.0 | 1.85 | Yes |
| `cc` | 1.4.3 | MIT OR Apache-2.0 | 1.65.0 | Yes |
| `cfg_aliases` | 0.2.2 | MIT | Not declared | No |
| `cfg-if` | 1.0.4 | MIT OR Apache-2.0 | 1.32 | No |
| `clipboard-win` | 5.4.1 | BSL-1.0 | Not declared | No |
| `const-oid` | 0.10.2 | Apache-2.0 OR MIT | 1.85 | Yes |
| `constant_time_eq` | 0.4.2 | CC0-1.0 OR MIT-0 OR Apache-2.0 | 1.85.0 | Yes |
| `cpufeatures` | 0.3.0 | MIT OR Apache-2.0 | 1.85 | Yes |
| `crypto-common` | 0.2.2 | MIT OR Apache-2.0 | 1.85 | No |
| `digest` | 0.11.3 | MIT OR Apache-2.0 | 1.85 | No |
| `error-code` | 3.4.0 | BSL-1.0 | Not declared | No |
| `equivalent` | 1.0.2 | Apache-2.0 OR MIT | 1.6 | No |
| `find-msvc-tools` | 0.1.11 | MIT OR Apache-2.0 | 1.65.0 | Yes |
| `hashbrown` | 0.17.1 | MIT OR Apache-2.0 | 1.85.0 | Yes |
| `hybrid-array` | 0.4.14 | MIT OR Apache-2.0 | 1.85 | Yes |
| `indexmap` | 2.14.0 | Apache-2.0 OR MIT | 1.85 | Yes |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | 1.68 | Yes |
| `libc` | 0.2.189 | MIT OR Apache-2.0 | 1.65 | Yes |
| `log` | 0.4.33 | MIT OR Apache-2.0 | 1.71.0 | No |
| `memchr` | 2.8.3 | Unlicense OR MIT | 1.61 | Yes |
| `nix` | 0.29.0 | MIT | 1.69 | No |
| `num-integer` | 0.1.47 | MIT OR Apache-2.0 | 1.31 | No |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | 1.60 | Yes |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | 1.71 | Yes |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | 1.71 | No |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | 1.56 | Yes |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | 1.71 | No |
| `serde_spanned` | 1.1.1 | MIT OR Apache-2.0 | 1.85 | Yes |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | 1.46.0 | Yes |
| `syn` | 3.0.3 | MIT OR Apache-2.0 | 1.71 | Yes |
| `tinyvec` | 1.12.0 | Zlib OR Apache-2.0 OR MIT | Not declared | Yes |
| `tinyvec_macros` | 0.1.1 | MIT OR Apache-2.0 OR Zlib | Not declared | No |
| `toml_datetime` | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 | Yes |
| `toml_parser` | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 | Yes |
| `toml_writer` | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 | No |
| `typenum` | 1.20.1 | MIT OR Apache-2.0 | 1.41.0 | No |
| `unicode-segmentation` | 1.13.3 | MIT OR Apache-2.0 | 1.85.0 | No |
| `unicode-width` | 0.2.2 | MIT OR Apache-2.0 | 1.66 | No |
| `utf8parse` | 0.2.2 | Apache-2.0 OR MIT | Not declared | No |
| `windows-sys` | 0.59.0 | MIT OR Apache-2.0 | 1.60 | No |
| `windows-targets` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_aarch64_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_aarch64_msvc` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_i686_gnu` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_i686_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_i686_msvc` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_x86_64_gnu` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_x86_64_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `windows_x86_64_msvc` | 0.52.6 | MIT OR Apache-2.0 | 1.56 | No |
| `winnow` | 1.0.4 | MIT | 1.65.0 | Yes |
| `zmij` | 1.0.23 | MIT | 1.71 | Yes |

The excluded fuzz lockfile adds four indirect packages not already listed above:

| Package | Locked | License expression | Upstream MSRV | Unsafe source |
| --- | --- | --- | --- | --- |
| `arbitrary` | 1.4.2 | MIT OR Apache-2.0 | 1.63.0 | Yes |
| `getrandom` | 0.4.3 | MIT OR Apache-2.0 | 1.85 | Yes |
| `jobserver` | 0.1.35 | MIT OR Apache-2.0 | 1.85 | Yes |
| `r-efi` | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | 1.68 | Yes |

## 审查结论与边界 / Review conclusion and boundaries

- 所有 manifest license expression 至少提供一个与项目 Apache-2.0 分发兼容的宽松许可分支；Rustyline 使用 MIT，`clipboard-win`/`error-code` 使用 BSL-1.0，`libfuzzer-sys` 额外要求 NCSA，`unicode-ident` 额外要求 Unicode-3.0。发布材料仍须保留适用的版权和许可文本。
- main binary 的关键 `unsafe` 边界位于 BLAKE3/SHA-2 硬件加速、Unicode table lookup、JSON/serialization fast path 和任意精度整数依赖内；workspace 自有代码继续 `deny(unsafe_code)`。
- build/test/fuzz-only native boundaries are isolated in `cc`, `libfuzzer-sys`, platform support crates, and their target-selected dependencies; `libfuzzer-sys` is never linked into `ling`.
- This is an engineering inventory based on locked manifests and cached source for the stated versions, not legal advice or a proof of transitive code safety. Any lockfile change invalidates the inventory and requires review.

The exact Rust package versions remain authoritative in `Cargo.lock` and `fuzz/Cargo.lock`; `editors/tree-sitter-ling/package-lock.json` is authoritative for its npm development tool. The locked per-package license/MSRV/unsafe-presence inventory is complete for the candidate Rust graphs; the clean candidate and same-SHA platform/fuzz/MSRV CI evidence are recorded in `SEED-RELEASE-REPORT.md`.
