# CLI-1704-FILE implementation report

## Scope

This report covers only the Accepted DEC-0039 standalone-file child of
`CLI-1704`. It does not claim the unresolved project/workspace test command.

## Implemented behavior

- `ling test [--format human|json] <file-or-directory>` requires one existing
  UTF-8 operand.
- A file runs one `.ling` `Main` program; a directory recursively discovers
  `.ling` files without following symlinks and sorts slash-normalized relative
  names by UTF-8 bytes.
- Each case uses the existing checked compiler pipeline and evaluator. Console
  output is captured; compile and runtime diagnostics are emitted in their
  existing bilingual form on stderr.
- JSON success/failure reports use `ling.test/0.1`; human output is one
  deterministic bilingual summary line. Empty selections use `L-TEST-0001`;
  discovery failures use `L-IO-0004`.

## Evidence

- Unit coverage: `crates/ling-cli/src/test_runner.rs`.
- Process coverage: `crates/ling-cli/tests/test.rs`.
- Schema corpus: `schemas/test/0.1/`.
- Protocol fixture: `tests/protocols/test/README.md`.
- Authority: `docs/decisions/0039-cli-test-file-runner.md` and the updated
  `CLI-1704` authority audit.

## Compatibility and deferrals

The child does not read or write manifests/locks, select packages or
workspaces, introduce test syntax or annotations, or add filtering, assertions,
snapshots, property tests, parallel scheduling, cancellation, or persistent
artifacts. The parent `CLI-1704` remains `BlockedSpec` for that broader
surface.
