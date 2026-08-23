# Ling performance baseline

Status: Seed measurement evidence (2026-08-22)

This is a trend baseline, not a release threshold. The existing
`cargo xtask performance baseline` command measures the implemented
`ling-db` query boundary with three samples per scenario and excludes fixture
construction from timed regions. The command emits the internal
`ling.performance-baseline/1` JSON shape; the checked-in historical artifact is
[`INC-1410-PERFORMANCE-BASELINE.json`](../status/INC-1410-PERFORMANCE-BASELINE.json).

## Measurement context

| Field | Value |
| --- | --- |
| Date | 2026-08-22 |
| Revision | `7142fad` (`rel-6603-security-audit`) |
| OS | Windows 10.0.26200.9168, `x86_64-pc-windows-msvc` |
| CPU identifier | `AMD64 Family 25 Model 97 Stepping 2, AuthenticAMD` |
| Logical processors | 32 |
| Rust / Cargo | `1.97.1` / `1.97.1` |
| Samples per scenario | 3 |
| Synthetic workspace | 10,000 logical `.ling` files |
| Timed region | Fixture setup excluded; query/parse operation only |
| Dispersion reported | Minimum, maximum, and range of the three samples; no threshold is frozen |

The host identifier is recorded exactly as exposed by the environment. Memory
capacity, storage model, thermal state, and background load were not measured;
those omissions prevent a cross-host performance claim.

## Observed Seed samples

Values are nanoseconds from the run recorded on this host. `range` is
`max - min` and is a transparent dispersion measure, not a statistical
confidence interval.

| Scenario | Samples (ns) | Min | Max | Range | Observable work |
| --- | --- | ---: | ---: | ---: | --- |
| `cold_check` | 3,503,800; 1,428,700; 1,455,400 | 1,428,700 | 3,503,800 | 2,075,100 | 24 trace events, 9 misses, 15 hits |
| `warm_check` | 19,700; 21,200; 20,200 | 19,700 | 21,200 | 1,500 | 8 trace events, 0 misses, 8 hits |
| `single_character_edit` | 1,329,900; 1,553,400; 1,286,500 | 1,286,500 | 1,553,400 | 266,900 | 20 trace events, 5 misses, 15 hits |
| `signature_edit` | 1,302,100; 1,306,200; 1,291,600 | 1,291,600 | 1,306,200 | 14,600 | 20 trace events, 5 misses, 15 hits |
| `cross_package_edit` | 1,410,900; 1,389,800; 1,401,200 | 1,389,800 | 1,410,900 | 21,100 | 24 trace events, 9 misses, 15 hits |
| `synthetic_10k_cold_parse` | 638,338,700; 625,650,600; 639,011,300 | 625,650,600 | 639,011,300 | 13,360,700 | 20,000 misses, 10,000 parsed files |
| `synthetic_10k_warm_parse` | 36,557,100; 37,077,600; 37,110,400 | 36,557,100 | 37,110,400 | 553,300 | 20,000 hits, 10,000 parsed files |
| `synthetic_10k_single_edit` | 35,492,600; 36,268,700; 35,154,200 | 35,154,200 | 36,268,700 | 1,114,500 | 2 misses, 19,998 hits, 10,000 parsed files |

These numbers are a single local observation. They are useful for comparing a
like-for-like rerun at the same revision and toolchain; they are not targets
for another operating system, CPU, profile, or future compiler version.

## Plan coverage

| Planned measurement | State | Evidence / boundary |
| --- | --- | --- |
| cold check/build | Partial | `cold_check` measures checked query construction; a full release build benchmark is not frozen. |
| warm/no-op build | Partial | `warm_check` measures query reuse; no package build command is accepted. |
| single-file edit latency | Covered for Seed query boundary | `single_character_edit`, `signature_edit`. |
| large workspace edit latency | Covered for synthetic parse boundary | `synthetic_10k_single_edit`; package-graph scale remains deferred. |
| LSP diagnostics/hover/completion | Deferred | LSP/IDE surfaces are not implemented/accepted. |
| VM startup/throughput | Deferred | VM conformance/differential tests exist, but no stable benchmark protocol or threshold. |
| Native compile/runtime | Deferred | No Native backend is implemented for Seed. |
| Actor/task overhead | Deferred | Actor/Task runtime is Future/Unsupported. |
| Replay overhead | Deferred | Replay/evidence protocol is Future/Unsupported. |
| Kernel CPU/GPU | Deferred | Device/kernel surfaces are Future/Unsupported. |
| memory peak | Deferred | Not measured; a platform/resource measurement contract is absent. |
| Zed startup/highlight | Deferred | Tree-sitter fixtures are correctness/differential tests, not a startup benchmark. |

## Reproduction

Run from a clean, dependency-locked checkout:

```text
cargo xtask performance baseline
```

The command is opt-in and does not change source semantics, caches, schemas,
diagnostics, Semantic IDs, or public protocols. Record host/toolchain/revision
and retain the complete JSON output for every comparison. Do not convert this
baseline into a hard gate until an Accepted performance policy defines sample
count, warm-up, variance, hardware tiers, memory/IO measurement, and threshold
ownership.

## Internal matrix drift check

The repository has one offline inventory check for this document:

```text
cargo xtask performance verify
```

The check validates the exact twelve planned-measurement rows and their current
Covered/Partial/Deferred states. It also strictly parses the checked-in
`ling.performance-baseline/1` artifact and verifies its eight-scenario order,
three-sample cardinality, 10,000-file fixture, timed-region exclusion, and
deterministic trace/hit/miss/completed-work observations. Nanosecond values are
required only to be non-zero; they are never compared to a threshold or host.
The check does not run the timing harness, freeze a threshold, make a
cross-host claim, or turn the Seed trend baseline into a release gate. A
missing memory measurement is recorded as Deferred until an Accepted resource
policy exists.
Fixture construction is excluded from timed regions, and the harness makes no absolute performance claim. Do not convert this baseline into a hard gate
until an Accepted performance policy defines the release comparison rules.
