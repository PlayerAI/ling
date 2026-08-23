# DOC-6703 Tutorial Coverage / 教程覆盖

Status: Seed tutorial evidence inventory (2026-08-22). This matrix records the
Chinese-first and equivalent English tutorial sources without promoting the
current Seed boundary to a 1.0 release claim.

The coverage gate is inventory-only. It does not run examples or promote Seed
evidence to Stable. No placeholder syntax, stale legacy name, or future API is
introduced. Unicode 17.0.0 and original UTF-8 byte spans remain required. The
parent DOC-6703 remains BlockedSpec until G1-G5 and Stable support evidence are
accepted.

## Source matrix

| Source | Language | Expected output | Process evidence | Boundary |
| --- | --- | --- | --- | --- |
| `examples/tutorial-en.ling` | English | alive | shared six-case process test; check/run/semantic and bilingual Semantic-shape comparison | Seed; Experimental/Preview |
| `examples/tutorial-zh.ling` | Chinese-first | 存活 | shared six-case process test; check/run/semantic and bilingual Semantic-shape comparison; separate audit test | Seed; Experimental/Preview |

## Requirement matrix

| Requirement | Tutorial evidence | Authority/evidence | State |
| --- | --- | --- | --- |
| Chinese-first runnable source | `examples/tutorial-zh.ling`; Chinese domain identifiers | `DOC-6703-AUTHORITY-AUDIT`; process-level conformance | Seed evidence |
| Idiomatic English equivalent | `examples/tutorial-en.ling`; domain names are not mechanically translated | `DOC-6703-AUTHORITY-AUDIT`; process-level conformance | Seed evidence |
| Checked offline commands | locked `check` and `run` commands for both sources | `docs/TUTORIAL.md` Verification | Seed evidence |
| Semantic and Audit output | `semantic` for both; `audit` for the Chinese source | `ling.semantic/0.1`; `ling.audit/0.1` | Experimental/Preview |
| Correct missing-Capability error | `p7-missing-capability` registered negative fixture | `ERROR-CODES.md`; conformance expectation | Seed evidence |
| Bilingual terminology | Chinese-first and English explanations use natural domain vocabulary | `docs/TUTORIAL.md` sections 2–4 | Seed evidence |
| Unicode 17 and original UTF-8 spans | Chinese identifiers and explicit span guidance | `LANGUAGE.md`; `SEMANTICS.md`; `AGENTS.md` | Seed evidence |
| Unsupported 1.0 boundaries | Profile, ownership, Native/FFI, runtime, package, LSP, and Zed limits | support matrix and authority audit | Explicitly deferred |

## Verification

Run from the repository root:

```text
cargo xtask tutorial verify
```

The command checks this exact source/requirement inventory, the bilingual
tutorial markers, and the source markers. It does not execute programs,
generate syntax, allocate public diagnostics, define protocols, or change
support state. The shared process-level test executes both sources and compares
their checked Semantic shapes after excluding localized names/text and
experimental identities. Conformance tests remain the authoritative evidence
for observed output and registered diagnostics.

The parent DOC-6703 remains BlockedSpec; a future 1.0 tutorial gate requires
Accepted localization/alias policy, Stable support entries, cross-platform
reproduction, migration guidance, and release evidence.
