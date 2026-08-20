# Project fixture matrix

`tests/projects/` contains both focused fixtures owned by PRJ-1101 through PRJ-1105 and the seven end-to-end project families required by PRJ-1106:

- `single-package`
- `multi-module`
- `path-dependency`
- `cycle`
- `visibility`
- `offline-lock`
- `unicode-names`

Each PRJ-1106 directory contains an `expect.toml` contract. `expected-diagnostics.json` freezes the complete ordered public Diagnostic JSON array. Successful cases also name an `expected-graph.json` test snapshot and an exact lock byte artifact; failing cases declare both graph and lock as `absent`.

`expected-graph.json` is test-only evidence over the public `ling-project` model. It is not a public Ling schema or a replacement for `ling.semantic/0.2`. Lock artifacts are real canonical `ling.lock/1` bytes, and the path-dependency artifact is cross-checked against the independent schema golden.

Normal tests copy each project to a temporary physical root before resolving it. This proves that expected outputs contain only logical coordinates and prevents tests from modifying checked-in project sources or locks.

Run the contract with:

```text
cargo test -p ling-project --test project_fixtures --locked --offline
```

The ignored snapshot writer is intentionally explicit and must be followed by review plus the normal test:

```text
cargo test -p ling-project --test project_fixtures --locked --offline bless_named_project_fixture_expectations -- --ignored --exact
```

CLI project selection and command behavior are not exercised here; they remain owned by PRJ-1107.
