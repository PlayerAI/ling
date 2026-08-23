# LSP code-action protocol fixtures

Authority: Accepted `docs/RFC-0044.md`.

Current marker: `ling.lsp.code-action/0.1` (Public Preview).

Executable fixtures live in `crates/ling-lsp/tests/code_action.rs`. They verify:

- exact capability negotiation and discovery for
  `source.fixAll.ling.format` plus transactional versioned Workspace Edits;
- exact one-action/one-document/one-edit output and absence of commands,
  diagnostic association, internal identity, paths, or server mutation;
- UTF-8, UTF-16, and UTF-32 projection, Unicode, BOM, CRLF, document versions,
  repeated determinism, kind filtering, and fixed failures;
- malformed and incapable clients, notifications, missing/read-only documents,
  and unchanged or invalid sources; and
- opacity of client diagnostic messages, data, Facts, repairs, and ranges.

Version 0.1 deliberately has no quick-fix, import, rename, mutability,
match-synthesis, stale-syntax, multi-file, resolve, command, annotation,
generated/dependency mutation, or server-side apply behavior.
