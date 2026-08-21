# DOC-6702 Example Coverage / 双层示例覆盖

Status: Seed evidence inventory (2026-08-22). This document is a
reproducibility and authority index for `DOC-6702`; it is not a claim that the
G6 documentation gate or any future capability is complete.

## Scope and authority

The execution plan asks every future `Stable` capability to provide a minimal
copyable example, a realistic project example, a correct-error example,
Audit/Semantic output, a Chinese-identifier example, and profile/Effect/
ownership notes. The current support matrix records all seven Seed features as
`Implemented` but `Experimental`; the Prelude is `Preview`, and no feature is
currently `Stable`. Consequently this inventory closes the evidence gap for
the implemented Seed boundary while keeping the G6 requirement blocked until a
1.0 support matrix exists.

The authority order remains accepted RFCs and decisions, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, conformance fixtures, and implementation evidence.
`docs/ROADMAP-1.0.md` and the execution plan define release work but cannot
authorize syntax, protocols, profiles, ownership rules, or runtime behavior.

## Seed two-layer matrix

| Requirement | Layer 1: minimal/reproducible | Layer 2: realistic or negative evidence | Audit/Semantic and status |
| --- | --- | --- | --- |
| Checked execution and entry | `examples/hello.ling`; `p7-hello-run` | `examples/人物.ling`; `p9-person-run`; invalid-main and missing-main fixtures | `hello` has deterministic Semantic/Audit tests; Seed evidence covered |
| Diagnostics and exits | `p7-type-error`, `p7-missing-capability` | record, match, builtin, mixed-script, and runtime-fault fixtures | Stable diagnostic codes and bilingual payloads are checked; Seed evidence covered |
| Unicode 17 and Chinese names | `examples/hello.ling` and `examples/人物.ling` | ADT/person fixtures with Chinese definitions and fields; mixed-script negative fixture | Original UTF-8 spans and Unicode 17.0.0 are tested; Seed evidence covered |
| Types, patterns, and Place | `examples/adt-match.ling` | `examples/人物.ling`, record/pattern and exhaustiveness fixtures | Semantic graphs are emitted for process examples; Seed evidence covered |
| Effect and Capability | `requires Console.Write` in the executable examples | missing-capability and higher-order-capability negatives | Audit records `Console.Write`; there is no selectable Profile; ownership is outside Seed |
| Semantic Graph and Audit Source | `examples/hello.ling` | `examples/人物.ling`, `examples/adt-match.ling`, and `examples/pipeline.ling` | `ling.semantic/0.1` and `ling.audit/0.1` remain Experimental/Preview protocols |
| Deterministic tooling | README command matrix and `seed reproduce` | conformance runner, independent-process Semantic/Audit tests, and CI | Offline/locked commands are evidence only; no Stable 1.x promise is made |

The existing process-level test
`seed_examples_check_run_and_emit_semantic_graphs` covers the three realistic
Seed examples (`人物`, ADT/match, and pipeline) for check, run, and Semantic
schema/name evidence. The independent `hello` Semantic and Audit tests cover
the minimal example and canonical output. The conformance runner executes both
positive and negative fixtures, including the correct-error layer.

## Capability-to-example traceability

| Feature ID | Accepted/registered authority | Positive examples | Negative/error evidence | Deferred boundary |
| --- | --- | --- | --- | --- |
| `FTR-SEED-0001` | `RFC-0001` §18.1, `DEC-0013` | hello, person, ADT, pipeline | invalid/missing/non-main entry | VM/default backend and future concurrency remain outside Seed |
| `FTR-SEED-0002` | `SEMANTICS` §26, `DEC-0001`, `DEC-0002` | hello and all process examples | registered diagnostic conformance corpus | Future CLI commands and migration formats need new authority |
| `FTR-SEED-0003` | `SEMANTICS` §§3.3, 3.6 | Chinese person/ADT names | mixed-script and hidden-control tests | Profile-specific identifier policy is future |
| `FTR-SEED-0004` | `DEC-0005`, `DEC-0008`, `DEC-0009`, `DEC-0014`, `DEC-0017` | ADT/match, person mutation, pipeline | arity, record, exhaustiveness, immutable-place errors | Ownership/Borrow beyond the Seed boundary is future |
| `FTR-SEED-0005` | `DEC-0010`, `DEC-0011`, `RFC-0019` | `Console.Write` examples | missing and higher-order capability fixtures | Effect handlers, selectable profiles, and new capabilities are future |
| `FTR-SEED-0006` | `DEC-0012`, `DEC-0015`, `RFC-0019` | Semantic/Audit output | reader, verifier, and malformed-schema negatives | IDs and schemas are not Stable 1.x contracts |
| `FTR-SEED-0007` | `docs/IMPLEMENTATION.md`, CI contract, `RFC-0019` | locked CLI and conformance commands | deterministic and malformed-input checks | package build/registry and editor tooling remain future |

## Reproduction commands

Run from the repository root with the locked dependency set:

```text
cargo run --locked --offline -- check examples/hello.ling
cargo run --locked --offline -- run examples/hello.ling
cargo run --locked --offline -- semantic examples/hello.ling
cargo run --locked --offline -- audit examples/人物.ling
cargo run --locked --offline -- run examples/人物.ling
cargo run --locked --offline -- run examples/adt-match.ling
cargo run --locked --offline -- run examples/pipeline.ling
cargo test -p ling-cli --test conformance seed_examples_check_run_and_emit_semantic_graphs --locked --offline
cargo test -p ling-cli --test conformance audit_output_is_deterministic_and_round_trips --locked --offline
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

Expected observable boundaries are: successful checks have empty stdout and
stderr; the runnable examples produce `你好，零`, `存活`, `受伤 30`, and `9`
respectively; Semantic output identifies `ling.semantic/0.1`; Audit output
identifies `ling.audit/0.1` and Unicode `17.0.0`; and negative fixtures return
their registered `L-<DOMAIN>-<NUMBER>` diagnostics. Exact semantic IDs are
experimental and must not be copied into prose as stable values.

## G6 completion requirements

Before `DOC-6702` can become complete, every capability admitted to the 1.0
support matrix must have both layers, at least one correct-error case, the
applicable Audit/Semantic or other public output, bilingual instructions where
the output is public, and explicit profile/Effect/ownership limitations. The
release evidence must link each example to an Accepted clause, implementation
symbol, conformance fixture, diagnostic/schema/protocol ID, deterministic and
offline check, and supported host/profile.

No example in this inventory introduces placeholder syntax, a future API,
unaccepted ownership semantics, an unsupported backend, a migration promise,
or a stale legacy command/source name.
