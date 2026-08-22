# CLI-1702-EXIT implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0037. The
parent `CLI-1702` task remains `BlockedSpec`: no new output schema, project
failure policy, retry behavior, or future command exit contract is implemented.

## Normative scope

- DEC-0013 authorizes the implemented Seed exit categories.
- DEC-0037 §§1–4 authorize only centralization of the current numeric values.

## Implementation

- `crates/ling-cli/src/exit_catalog.rs` owns the six existing internal exit
  constants.
- `crates/ling-cli/src/main.rs` imports the catalog; all existing handlers,
  output paths, diagnostics, and `ExitCode` selection remain unchanged.
- A unit test asserts every value and the required category distinctions.

## Evidence

```text
cargo fmt --all -- --check
cargo test -p ling-cli --all-features --locked --offline
cargo clippy -p ling-cli --all-targets --all-features --locked --offline -- -D warnings
```

## Compatibility and determinism

No command, option, output, exit behavior, diagnostic, schema, protocol,
Semantic ID, runtime, bytecode, VM, or Unicode 17.0.0 behavior changed. No
future exit category or public error mapping is advertised.

## Deferred work

Unified public output contracts, project/build/test/formatter/query/patch
failure policies, structured responses, retry rules, and future exit mappings
remain deferred to parent CLI-1702 and its dependent tasks.
