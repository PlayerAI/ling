# CLI-1701-CATALOG implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0036. The
parent `CLI-1701` task remains `BlockedSpec`: no new command, alias, shared
option group, project service, query/patch surface, or shell-completion API is
implemented.

## Normative scope

- DEC-0003 authorizes a small hand-written parser for the current CLI surface.
- DEC-0013 governs Seed run/check behavior; current accepted Preview slices
  govern formatter, project-check, and LSP launch behavior.
- DEC-0036 §§1–4 authorize only internal command identity centralization.

## Implementation

- `crates/ling-cli/src/command_catalog.rs` owns the existing private command
  enum, root parser, and canonical display names.
- `crates/ling-cli/src/main.rs` reuses that catalog; option validation,
  execution, diagnostics, usage, and exit behavior remain in their existing
  paths.
- Tests cover all current roots, canonical names, `project check` identity,
  and rejection of unimplemented `build`/`test` roots.

## Evidence

```text
cargo fmt --all -- --check
cargo test -p ling-cli --all-features --locked --offline
cargo clippy -p ling-cli --all-targets --all-features --locked --offline -- -D warnings
```

## Compatibility and determinism

No command spelling, alias, option, output, exit code, diagnostic, schema,
protocol, Semantic ID, source-span, runtime, bytecode, VM, or Unicode
17.0.0 behavior changed. No future execution-plan command is advertised.

## Deferred work

Unified public command registry, project/build/test selection, shared options,
formatter/query/patch transactions, shell completion, help policy, service
ownership, and migration remain deferred to parent `CLI-1701` and dependent
CLI tasks.
