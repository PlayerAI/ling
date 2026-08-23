# `ling.project.command/0.1` and project artifact fixture boundary

Accepted RFC-0025 defines manifest-selected semantic project commands:

```text
ling check|run|test --manifest-path <path>/ling.toml --locked --offline
ling build --manifest-path <path>/ling.toml --locked --offline \
  --profile explore --target semantic --output <new-path>
```

`crates/ling-cli/tests/project_commands.rs` is the executable fixture suite.
It verifies deterministic path-free `ling.project.command/0.1` reports,
dependency-using checked execution, one isolated entry smoke test, registered
semantic failures with logical source spans, and canonical create-new
`ling.project.artifact/0.1` bytes whose reported identity is SHA-256 of the
complete artifact.

The suite also preserves the distinct RFC-0024 graph-only command and existing
positional file modes. It rejects mixed selection, missing locked/offline
promises, unsupported build profiles/targets, and output replacement without
mutating source or lock inputs. The artifact is checked semantic JSON, not
executable bytecode, native/Wasm output, publication, or Stable 1.0 evidence.
