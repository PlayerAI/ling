# Codex Task Prompt

## Task

- ID: `<TASK-ID>`
- Title: `<short title>`
- Size: `XS / S / M`
- Owner role: `<compiler/test/lsp/zed/release/...>`

## Goal

<One observable result. Do not ask for an entire release block.>

## Specification authority

Read in this order:

1. `<Accepted RFC / decision>`
2. `<SEMANTICS sections>`
3. `<LANGUAGE sections>`
4. `<execution-plan task section>`

If these conflict or do not decide observable behavior, stop and create a spec-gap. Do not guess.

## Preconditions

- `<dependency task>` is merged.
- `<required tests/interfaces>` exist.

## Allowed paths

- `<path/**>`

## Forbidden paths

- `<path/**>`

Do not change source syntax, public protocol, Semantic ID, error-code meaning, ABI or profile rules unless this task explicitly includes the Accepted RFC for that change.

## Required implementation

1. `<step>`
2. `<step>`
3. `<step>`

## Required tests

- Positive conformance: `<cases>`
- Negative conformance: `<cases/error codes>`
- Property/round-trip: `<cases>`
- Differential: `<engines>`
- Fuzz/security: `<if applicable>`
- Unicode/position: `<if applicable>`

## Non-goals

- `<adjacent feature deliberately deferred>`

## Acceptance commands

```bash
<exact command>
```

## Required documentation/status updates

- `docs/traceability/<release>.md`
- `docs/status/implementation-status.toml`
- `<schema/error/feature registries if affected>`

## Required final report

Return:

- result;
- files changed;
- spec/RFC coverage;
- commands actually executed and results;
- Diagnostic/Schema/Semantic ID impact;
- Unicode/determinism/security/performance impact;
- spec gaps/conflicts;
- deferred work.
