# Ling schema corpus / Ling Schema Corpus

This directory is the machine corpus for GOV-0106. [`registry.toml`](registry.toml) is the inventory; [`SCHEMA-LIFECYCLE.md`](../docs/governance/SCHEMA-LIFECYCLE.md) defines the Draft engineering policy.

本目录是 GOV-0106 的机器 corpus。`registry.toml` 是清单；`SCHEMA-LIFECYCLE.md` 定义 Draft 工程策略。

Current JSON packages:

```text
schemas/<name>/<version>/schema.json
schemas/<name>/<version>/valid/*.json
schemas/<name>/<version>/invalid/*.json
schemas/<name>/<version>/invalid/*.expect.toml
schemas/<name>/<version>/canonical/*.bin
```

- `valid` files must satisfy the declared schema and the real implementation reader when one exists.
- Every `invalid` JSON file has an expectation sidecar naming `InvalidJson`, `SchemaViolation`, or `ReaderViolation`.
- Only protocols already declared canonical have byte-golden files.
- The registry explicitly uses `NoPreviousVersion`; this corpus does not claim an N-1 reader.
- No file in this directory is evaluator input. Semantic Graph readers return isolated data only.

Run after locked dependencies have been fetched:

```text
cargo xtask schema validate-all
cargo xtask schema compatibility --from N-1 --to N
cargo xtask schema corrupt-inputs
```
