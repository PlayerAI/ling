# Seed Corpus Freeze

This generated report freezes only the accepted v0.0.1 conformance corpus. It is not a v0.1-v0.5 history or a compatibility promise.

- Authority: `DEC-0230`
- Release: `v0.0.1`
- Unicode: `17.0.0`
- Cases: `42`
- Files: `84`
- Canonical SHA-256: `caafb863d530bddd7b22a2de4aa0a8db374bd46f407e602fa78276a126442cd8`

| Requested surface | State | Evidence |
| --- | --- | --- |
| source programs | `SeedFrozen` | `tests/conformance` |
| parser trees | `NotFrozen` | `crates/ling-syntax/tests/conformance_syntax_differential.rs`<br>`docs/status/COMPAT-6501-AUTHORITY-AUDIT.md` |
| diagnostics | `SeedFrozen` | `tests/conformance`<br>`docs/ERROR-CODES.md` |
| Semantic Graph | `SeparateProtocol` | `crates/ling-semantic/tests/project_snapshot.rs`<br>`docs/governance/protocol-inventory.toml` |
| Audit | `SeparateProtocol` | `crates/ling-format/src/lib.rs`<br>`docs/governance/protocol-inventory.toml` |
| bytecode | `SeparateProtocol` | `crates/ling-bytecode/tests/decode_verify.rs`<br>`docs/governance/protocol-inventory.toml` |
| package/lock | `SeparateProtocol` | `crates/ling-project/tests/lockfile_fixtures.rs`<br>`docs/governance/protocol-inventory.toml` |
| replay | `Unavailable` | `docs/status/COMPAT-6501-AUTHORITY-AUDIT.md`<br>`docs/governance/support-matrix.toml` |
| evidence | `Unavailable` | `docs/status/COMPAT-6501-AUTHORITY-AUDIT.md`<br>`docs/governance/support-matrix.toml` |
| Zed/LSP fixtures | `Unavailable` | `docs/status/COMPAT-6501-AUTHORITY-AUDIT.md`<br>`docs/governance/support-matrix.toml` |

`SeedFrozen` covers exact checked-in v0.0.1 bytes only. `SeparateProtocol` retains its own authority and versioning. `NotFrozen` and `Unavailable` are explicit non-claims.
