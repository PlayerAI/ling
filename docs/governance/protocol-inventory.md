# Ling 公开接口与协议清单 / Public Protocol Inventory

> 状态：由 `protocol-inventory.toml` 确定性生成
> 更新日期：2026-08-24
> 本清单记录当前兼容边界，不新增语言语义或协议承诺。

## Summary

- 49 records: 45 current public, 1 internal, 3 Future.
- Current public stability: 16 Experimental, 29 Preview, 0 Stable.
- `Stable` means the ROADMAP-1.0 1.x commitment. No current Seed protocol has passed that gate; stable diagnostic codes remain a documented compatibility subset inside the Preview Diagnostic protocol.

## Inventory

| ID | Visibility | Category | Current version | Stability | Public schema | Canonical | Fixtures |
| --- | --- | --- | --- | --- | --- | --- | ---: |
| `PROTO-CLI` | Public | CLI | `0.0.1-dev` | `Preview` | no | no | 10 |
| `PROTO-CLI-EXIT` | Public | CLI | `0.0.1-dev` | `Preview` | no | yes | 4 |
| `PROTO-PROJECT-CHECK` | Public | CLI | `ling.project.check/0.1` | `Experimental` | no | no | 2 |
| `PROTO-LSP-CODE-ACTION` | Public | LSP | `ling.lsp.code-action/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-COMPLETION` | Public | LSP | `ling.lsp.completion/0.2` | `Preview` | no | no | 8 |
| `PROTO-LSP-DIAGNOSTIC` | Public | LSP | `ling.lsp.diagnostic/0.2` | `Experimental` | no | no | 3 |
| `PROTO-LSP-DIAGNOSTIC-CONTROL` | Public | LSP | `ling.lsp.diagnostic-control/0.1` | `Preview` | no | no | 5 |
| `PROTO-LSP-DOCUMENT-SYMBOL` | Public | LSP | `ling.lsp.document-symbol/0.1` | `Preview` | no | no | 3 |
| `PROTO-LSP-FORMATTING` | Public | LSP | `ling.lsp.formatting/0.1` | `Experimental` | no | no | 2 |
| `PROTO-LSP-HOVER` | Public | LSP | `ling.lsp.hover/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-LIFECYCLE` | Public | LSP | `ling.lsp.lifecycle/0.1` | `Preview` | no | no | 7 |
| `PROTO-LSP-NAVIGATION` | Public | LSP | `ling.lsp.navigation/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-OVERLAY` | Public | LSP | `ling.lsp.overlay/0.2` | `Experimental` | no | no | 5 |
| `PROTO-LSP-PREPARE-RENAME` | Public | LSP | `ling.lsp.prepare-rename/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-PUBLISH-DIAGNOSTICS` | Public | LSP | `ling.lsp.publish-diagnostics/0.2` | `Experimental` | no | no | 4 |
| `PROTO-LSP-PULL-DIAGNOSTICS` | Public | LSP | `ling.lsp.pull-diagnostics/0.2` | `Preview` | no | no | 5 |
| `PROTO-LSP-REFERENCES` | Public | LSP | `ling.lsp.references/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-RENAME` | Public | LSP | `ling.lsp.rename/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-REQUEST-CANCELLATION` | Public | LSP | `ling.lsp.request-cancellation/0.1` | `Preview` | no | no | 4 |
| `PROTO-LSP-RESOURCE-LIMITS` | Public | LSP | `ling.lsp.resource-limits/0.1` | `Preview` | no | no | 5 |
| `PROTO-LSP-SCHEDULING` | Public | LSP | `ling.lsp.scheduling/0.1` | `Preview` | no | no | 5 |
| `PROTO-LSP-SEMANTIC-TOKENS` | Public | LSP | `ling.lsp.semantic-tokens/0.1` | `Preview` | no | no | 6 |
| `PROTO-LSP-WORKSPACE` | Public | LSP | `ling.lsp.workspace/0.1` | `Experimental` | no | no | 3 |
| `PROTO-LSP-WORKSPACE-SYMBOL` | Public | LSP | `ling.lsp.workspace-symbol/0.1` | `Preview` | no | no | 3 |
| `PROTO-HUMAN-OUTPUT` | Public | Human output | `0.0.1-dev` | `Preview` | no | no | 4 |
| `PROTO-CLI-INIT` | Public | JSON | `ling.init/0.1` | `Preview` | yes | no | 5 |
| `PROTO-CLI-TEST` | Public | JSON | `ling.test/0.1` | `Preview` | yes | no | 5 |
| `PROTO-DIAGNOSTIC-JSON` | Public | JSON | `ling.diagnostic/0.1` | `Preview` | yes | no | 8 |
| `PROTO-FORMAT-CLI` | Public | JSON | `ling.format/0.1` | `Preview` | yes | no | 5 |
| `PROTO-LOCKFILE` | Public | JSON | `ling.lock/1` | `Experimental` | yes | yes | 8 |
| `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.2` | `Experimental` | yes | yes | 6 |
| `PROTO-REPL-JSON` | Public | JSON | `ling.repl/0.1` | `Preview` | yes | no | 5 |
| `PROTO-SEMANTIC-GRAPH-JSON` | Public | JSON | `ling.semantic/0.1` | `Experimental` | yes | yes | 6 |
| `PROTO-SEMANTIC-QUERY` | Public | JSON | `ling.semantic-query/0.1` | `Preview` | yes | no | 6 |
| `PROTO-SEMANTIC-TRANSACTION` | Public | JSON | `ling.semantic-transaction/0.1` | `Preview` | yes | no | 6 |
| `PROTO-SEMANTIC-TRANSACTION-RESULT` | Public | JSON | `ling.semantic-transaction-result/0.1` | `Preview` | yes | no | 6 |
| `PROTO-CANONICAL-BYTES` | Public | Canonical identity | `file-mode v1 and package-aware v2 domain encodings` | `Experimental` | no | yes | 2 |
| `PROTO-PACKAGE-IDENTITY` | Public | Canonical identity | `v1 domain encodings` | `Experimental` | no | yes | 9 |
| `PROTO-SEMANTIC-ID` | Public | Canonical identity | `experimental:blake3:` | `Experimental` | no | yes | 4 |
| `PROTO-AUDIT-SOURCE` | Public | Text protocol | `ling.audit/0.2` | `Preview` | yes | yes | 3 |
| `PROTO-CLI-COMPLETION` | Public | Text protocol | `ling.cli-completion/0.1` | `Preview` | yes | yes | 5 |
| `PROTO-BUILD-METADATA` | Public | Package metadata | `ling.project.artifact/0.1` | `Experimental` | no | yes | 3 |
| `PROTO-PACKAGE-MANIFEST` | Public | Package metadata | `ling.manifest/1` | `Experimental` | no | no | 26 |
| `PROTO-BYTECODE` | Public | Bytecode | `ling.bytecode/1.2` | `Experimental` | no | no | 7 |
| `PROTO-VM-CONTROL` | Public | Runtime control | `ling.vm.control/0.1` | `Experimental` | no | no | 4 |
| `PROTO-INTERNAL-INCIDENT` | Internal | Incident | `ling.internal-incident/0.1` | `Internal` | no | no | 1 |
| `PROTO-REPLAY` | Planned public | Replay | — | `Future` | no | no | 0 |
| `PROTO-ABI` | Planned public | ABI | — | `Future` | no | no | 0 |
| `PROTO-EVIDENCE` | Planned public | Evidence | — | `Future` | no | no | 0 |

## Reader, writer, and migration policies

### `PROTO-CLI` — Ling command and option surface

- Producer: ling executable
- Consumer: humans; shell scripts; editor and build integrations
- Reader policy: The hand-written parser accepts --help/-h, --version/-V, file-oriented run/check/semantic/audit/test, manifest-selected locked/offline project run/check/test/build, repl, fmt, init, query, patch, completion, the distinct Experimental project graph check, and the Preview lsp --stdio launcher; current non-LSP non-completion commands accept the DEC-0254 format/language/color/verbosity policy, while unknown, duplicate, mixed, or incompatible forms are rejected with exit 2.
- Writer policy: Help and version output describe only implemented commands; file and project compiler commands route through their shared checked pipelines; non-LSP output follows DEC-0254 without changing machine schemas or exit classes; completion emits only its versioned protocol script; and lsp --stdio rejects output-policy flags and retains protocol-only framed output.
- Unknown-field policy: Not field-based: unknown commands, options, formats, and capabilities are rejected.
- Migration tool: None; incompatible command or option changes require an accepted specification and release migration notes.
- Authority: `DEC-0003`, `DEC-0013`, `DEC-0015`, `DEC-0016`, `DEC-0028`, `DEC-0037`, `DEC-0038`, `DEC-0039`, `DEC-0253`, `DEC-0254`, `DEC-0255`, `DEC-0256`, `RFC-0004`, `RFC-0024`, `RFC-0025`, `RFC-0027`, `RFC-0028`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`docs/decisions/0253-current-cli-command-model.md`](../decisions/0253-current-cli-command-model.md), [`docs/decisions/0254-cli-output-policy.md`](../decisions/0254-cli-output-policy.md), [`docs/RFC-0025.md`](../RFC-0025.md), [`docs/RFC-0027.md`](../RFC-0027.md), [`docs/RFC-0028.md`](../RFC-0028.md), [`crates/ling-cli/src/command_catalog.rs`](../../crates/ling-cli/src/command_catalog.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/src/output_policy.rs`](../../crates/ling-cli/src/output_policy.rs), [`crates/ling-cli/src/project.rs`](../../crates/ling-cli/src/project.rs), [`crates/ling-cli/src/completion.rs`](../../crates/ling-cli/src/completion.rs)
- Fixtures: [`crates/ling-cli/src/command_catalog.rs`](../../crates/ling-cli/src/command_catalog.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/tests/help.rs`](../../crates/ling-cli/tests/help.rs), [`crates/ling-cli/tests/output_policy.rs`](../../crates/ling-cli/tests/output_policy.rs), [`crates/ling-cli/tests/project_commands.rs`](../../crates/ling-cli/tests/project_commands.rs), [`crates/ling-cli/tests/completion.rs`](../../crates/ling-cli/tests/completion.rs), [`tests/protocols/cli-output-policy/README.md`](../../tests/protocols/cli-output-policy/README.md), [`tests/protocols/project-command/README.md`](../../tests/protocols/project-command/README.md), [`tests/protocols/cli-completion/README.md`](../../tests/protocols/cli-completion/README.md)
- Notes: The compiler package version remains the broad CLI version. DEC-0253 accepts the command model, DEC-0254 the output policy, DEC-0255 the initializer, DEC-0256 the standalone/project test-mode composition, RFC-0027 query/patch, and RFC-0028 completion while preserving command-specific schemas and exits.

### `PROTO-CLI-EXIT` — Ling process exit-code mapping

- Producer: ling process
- Consumer: shells; CI jobs; editor and build integrations
- Reader policy: Interpret 0 as success, 1 as compile/check failure or an early LSP exit, 2 as invalid usage, 4 as runtime, host, or LSP transport fault, 5 as internal compiler error, and 6 as semantic snapshot mismatch; 3 is reserved and unreachable in Seed.
- Writer policy: Human versus JSON rendering never changes the exit class; run and scripted REPL preserve the accepted compile/runtime distinction, while the LSP lifecycle preserves shutdown-before-exit status.
- Unknown-field policy: Not field-based: unassigned exit values have no compatibility meaning.
- Migration tool: None; changing an assigned meaning requires an accepted decision and explicit compatibility guidance.
- Authority: `DEC-0013`, `DEC-0016`, `DEC-0037`, `DEC-0254`, `RFC-0004`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`docs/decisions/0254-cli-output-policy.md`](../decisions/0254-cli-output-policy.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/src/output_policy.rs`](../../crates/ling-cli/src/output_policy.rs)
- Fixtures: [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/tests/output_policy.rs`](../../crates/ling-cli/tests/output_policy.rs), [`tests/conformance/p7-hello-run/expect.toml`](../../tests/conformance/p7-hello-run/expect.toml), [`tests/conformance/p12-text-format-fault/expect.toml`](../../tests/conformance/p12-text-format-fault/expect.toml)
- Notes: Exit 3 remains reserved for a future accepted Result-returning main and is not current behavior; DEC-0254 requires rendering policy to leave every assigned exit unchanged.

### `PROTO-PROJECT-CHECK` — Ling project graph check

- Producer: ling project check
- Consumer: shell scripts; CI jobs; local project tooling
- Reader policy: The command requires exactly one --manifest-path ending in ling.toml and exactly one --locked option; it validates only the explicit local RFC-0002 project root and rejects unknown options or unsupported project subcommands.
- Writer policy: Emit one deterministic path-free human line or one ling.project.check/0.1 JSON object; validation diagnostics use existing Diagnostic JSON and no command writes locks, sources, caches, or build artifacts.
- Unknown-field policy: JSON report fields are current-writer-only; incompatible report changes require a new protocol version.
- Migration tool: None; ling.project.check/0.1 is Experimental and current-writer-only.
- Authority: `RFC-0024`, `RFC-0002`, `DEC-0003`, `DEC-0013`
- Sources: [`docs/RFC-0024.md`](../RFC-0024.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs)
- Fixtures: [`crates/ling-cli/tests/project_check.rs`](../../crates/ling-cli/tests/project_check.rs), [`tests/protocols/project-check/README.md`](../../tests/protocols/project-check/README.md)
- Notes: Graph validation only: semantic compilation, run/test/build, workspace search, registry/network behavior, and lock update mode remain deferred.

### `PROTO-LSP-CODE-ACTION` — Ling bounded transactional LSP code action

- Producer: ling lsp --stdio; ling-lsp compiler-CST formatter plan
- Consumer: LSP clients with exact code-action literal and transactional versioned Workspace Edit support; editor and integration test harnesses
- Reader policy: Enable textDocument/codeAction only when codeActionLiteralSupport contains source.fixAll.ling.format and workspace.workspaceEdit promises documentChanges=true plus failureHandling=transactional; validate the standard request range and context while treating client diagnostics as opaque filter context.
- Writer policy: From one complete immutable snapshot, emit zero actions or exactly one preferred source.fixAll.ling.format action containing one versioned documentChanges TextEdit derived solely from the accepted compiler-CST FormatEdit; recheck complete freshness, enforce the 1 MiB response bound, and never apply the edit.
- Unknown-field policy: Ignore ordinary unknown request and capability members while rejecting malformed known members; capability, discovery, params, kind filtering, diagnostic opacity, plan, projection, title, version, result, empty-result, freshness, bound, or failure changes require a new marker and migration evidence.
- Migration tool: None; ling.lsp.code-action/0.1 has no predecessor, incapable clients retain the prior initialize response without a provider, and clients gate on the exact discovery marker and transactional capabilities.
- Authority: `RFC-0044`, `RFC-0041`, `RFC-0039`, `RFC-0038`, `RFC-0031`, `RFC-0030`, `RFC-0029`, `RFC-0026`, `RFC-0023`, `RFC-0005`, `RFC-0004`, `DEC-0001`, `DEC-0002`, `DEC-0012`, `DEC-0015`, `DEC-0019`, `DEC-0023`, `DEC-0029`, `DEC-0034`, `DEC-0057`, `DEC-0071`, `DEC-0072`, `DEC-0081`
- Sources: [`docs/RFC-0044.md`](../RFC-0044.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0026.md`](../RFC-0026.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0031.md`](../RFC-0031.md), [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/RFC-0039.md`](../RFC-0039.md), [`docs/RFC-0041.md`](../RFC-0041.md), [`docs/decisions/0001-error-code-policy.md`](../decisions/0001-error-code-policy.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0015-audit-source-format.md`](../decisions/0015-audit-source-format.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0023-author-source-formatter-preservation.md`](../decisions/0023-author-source-formatter-preservation.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0034-lsp-internal-diagnostic-ordering-boundary.md`](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md), [`docs/decisions/0057-formatter-in-process-edit-projection.md`](../decisions/0057-formatter-in-process-edit-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0072-lsp-diagnostic-span-projection.md`](../decisions/0072-lsp-diagnostic-span-projection.md), [`docs/decisions/0081-ide-code-action-repair-index.md`](../decisions/0081-ide-code-action-repair-index.md), [`crates/ling-format/src/edit.rs`](../../crates/ling-format/src/edit.rs), [`crates/ling-lsp/src/code_action.rs`](../../crates/ling-lsp/src/code_action.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/src/code_action.rs`](../../crates/ling-lsp/src/code_action.rs), [`crates/ling-lsp/tests/code_action.rs`](../../crates/ling-lsp/tests/code_action.rs), [`tests/protocols/lsp-code-action/README.md`](../../tests/protocols/lsp-code-action/README.md), [`docs/status/IDE-2309-IMPLEMENTATION-REPORT.md`](../status/IDE-2309-IMPLEMENTATION-REPORT.md)
- Notes: Version 0.1 closes the Seed code-action surface with one actual checked formatter plan. Missing-import, confusable-rename, mutability, match-case, stale-syntax, diagnostic quick-fix, multi-document, resolve, command, generated/dependency mutation, cancellation, general Semantic Transaction, and Stable behavior remain out of scope.

### `PROTO-LSP-COMPLETION` — Ling checked deterministic LSP completion and resolve

- Producer: ling-lsp textDocument/completion; ling-lsp completionItem/resolve
- Consumer: LSP clients with standard completion support and optional detail/documentation resolveSupport
- Reader policy: Clients without detail and documentation resolveSupport receive exact ling.lsp.completion/0.1 discovery and items; capable clients gate on ling.lsp.completion/0.2 plus ling.lsp.completion-resolve/0.1, preserve the opaque data object, and send the exact item to completionItem/resolve.
- Writer policy: Emit a bounded deterministic Completion List from one completely checked immutable snapshot; validate each replacement through a fresh compiler; for negotiated 0.2 retain at most 1024 opaque snapshot-bound handles and resolve only DEC-0080 checked signature, Effect, and Capability facts plus RFC-0043-attached `///` text while preserving exact PlainText edits.
- Unknown-field policy: Ignore ordinary unknown request fields while rejecting malformed known capability, completion, data, or lazy members; negotiation, request, context, candidate, validation, ranking, item, handle, expiry, snapshot, metadata, rendering, edit-preservation, bound, result, or failure changes require a new marker and migration evidence.
- Migration tool: Automatic capability negotiation: clients lacking both detail and documentation resolveSupport retain exact ling.lsp.completion/0.1 behavior; capable clients receive ling.lsp.completion/0.2 items and gate resolve on ling.lsp.completion-resolve/0.1.
- Authority: `RFC-0043`, `RFC-0042`, `RFC-0039`, `RFC-0038`, `RFC-0037`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0010`, `DEC-0012`, `DEC-0019`, `DEC-0023`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0079`, `DEC-0080`
- Sources: [`docs/RFC-0043.md`](../RFC-0043.md), [`docs/RFC-0042.md`](../RFC-0042.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0037.md`](../RFC-0037.md), [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/RFC-0039.md`](../RFC-0039.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0010-state-and-capability-model.md`](../decisions/0010-state-and-capability-model.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0023-author-source-formatter-preservation.md`](../decisions/0023-author-source-formatter-preservation.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`docs/decisions/0079-ide-completion-source-inventory.md`](../decisions/0079-ide-completion-source-inventory.md), [`docs/decisions/0080-ide-completion-checked-metadata.md`](../decisions/0080-ide-completion-checked-metadata.md), [`crates/ling-db/src/checked_completion_catalog.rs`](../../crates/ling-db/src/checked_completion_catalog.rs), [`crates/ling-db/src/completion_metadata_index.rs`](../../crates/ling-db/src/completion_metadata_index.rs), [`crates/ling-format/src/comments.rs`](../../crates/ling-format/src/comments.rs), [`crates/ling-format/src/format_ir.rs`](../../crates/ling-format/src/format_ir.rs), [`crates/ling-lsp/src/completion.rs`](../../crates/ling-lsp/src/completion.rs), [`crates/ling-lsp/src/completion_resolve.rs`](../../crates/ling-lsp/src/completion_resolve.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/checked_completion_catalog.rs`](../../crates/ling-db/src/checked_completion_catalog.rs), [`crates/ling-db/src/completion_metadata_index.rs`](../../crates/ling-db/src/completion_metadata_index.rs), [`crates/ling-lsp/tests/completion.rs`](../../crates/ling-lsp/tests/completion.rs), [`crates/ling-lsp/tests/completion_resolve.rs`](../../crates/ling-lsp/tests/completion_resolve.rs), [`tests/protocols/lsp-completion/README.md`](../../tests/protocols/lsp-completion/README.md), [`tests/protocols/lsp-completion-resolve/README.md`](../../tests/protocols/lsp-completion-resolve/README.md), [`docs/status/IDE-2307-IMPLEMENTATION-REPORT.md`](../status/IDE-2307-IMPLEMENTATION-REPORT.md), [`docs/status/IDE-2308-IMPLEMENTATION-REPORT.md`](../status/IDE-2308-IMPLEMENTATION-REPORT.md)
- Notes: The negotiated resolve Preview presents existing DEC-0080 facts and only directly attached `///` Author Source text; it never fabricates documentation or changes an RFC-0042 edit. Incomplete recovery, zero-width insertion, auto-imports, snippets, generated/dependency/builtin metadata, persistent handles, cancellation, AI ranking, and Stable lifecycle remain out of scope.

### `PROTO-LSP-DIAGNOSTIC` — Ling LSP compiler diagnostic adapter

- Producer: ling-lsp diagnostic adapter
- Consumer: future LSP diagnostic publishers; editor and integration test harnesses
- Reader policy: Accept a non-empty set of unique exact Ling URI/source identities, including RFC-0032 temporary untitled identities, and existing registered compiler diagnostics with required primary spans; project every primary and related span strictly through the explicit negotiated encoding, rejecting the complete call on any identity, range, or boundary failure.
- Writer policy: Emit exactly one path-free URI plus LSP Diagnostic JSON value per input in DEC-0034 order; preserve registered code, bilingual message, severity, Facts, Semantic ID, repairs, explicit related-label order, and original-byte position truth without publication or mutation.
- Unknown-field policy: The 0.2 adapter has no JSON reader; its exact output keys are current-writer-only, and incompatible field, severity, message, data, URI, ordering, or projection evolution requires a new protocol marker and migration evidence.
- Migration tool: No tool; RFC-0032 0.2 accepts every valid 0.1 non-temporary input with byte-identical output and only extends the source set to validated temporary untitled identities.
- Authority: `RFC-0031`, `RFC-0032`, `DEC-0001`, `DEC-0002`, `DEC-0029`, `DEC-0034`, `DEC-0072`
- Sources: [`docs/RFC-0031.md`](../RFC-0031.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/SEMANTICS.md`](../SEMANTICS.md), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/decisions/0001-error-code-policy.md`](../decisions/0001-error-code-policy.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0034-lsp-internal-diagnostic-ordering-boundary.md`](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md), [`docs/decisions/0072-lsp-diagnostic-span-projection.md`](../decisions/0072-lsp-diagnostic-span-projection.md), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-lsp/src/diagnostics.rs`](../../crates/ling-lsp/src/diagnostics.rs)
- Fixtures: [`crates/ling-lsp/tests/diagnostic_adapter.rs`](../../crates/ling-lsp/tests/diagnostic_adapter.rs), [`tests/protocols/lsp-diagnostic/README.md`](../../tests/protocols/lsp-diagnostic/README.md), [`docs/status/LSP-2201-IMPLEMENTATION-REPORT.md`](../status/LSP-2201-IMPLEMENTATION-REPORT.md)
- Notes: RFC-0031 defined the pure 0.1 non-temporary adapter. RFC-0032 advances it compatibly to 0.2 solely for validated temporary untitled identities and uses it from the separate push publisher. Deduplication, root-cause caps, suppression, tags, code-description URLs, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-DIAGNOSTIC-CONTROL` — Ling LSP diagnostic storm control

- Producer: ling lsp --stdio; ling-lsp diagnostic control
- Consumer: LSP clients; editor hosts; integration test harnesses
- Reader policy: Read optional initializationOptions.lingDiagnosticControl object limits, applying defaults for absent known members; accept maxPerDocument 1..4096 and maxPerWorkspace 1..65536, reject malformed known members before initialization, and ignore unknown ordinary fields.
- Writer policy: Preserve the first exact code/range/Semantic-ID/Facts root, apply document then URI-ordered workspace caps without mutating the compiler set, append registered L-LSP-0001 summaries with exact counts/ranges, and feed the same complete controlled map to push and pull.
- Unknown-field policy: Unknown ordinary initialization fields are ignored; malformed known limits and malformed internal adapter values fail atomically. Incompatible root identity, limit, summary, ordering, configuration, or discovery behavior requires a new marker and migration evidence.
- Migration tool: No standalone tool; clients discover the exact active immutable limits and 0.1 marker during initialize, while push and pull advertise their corresponding 0.2 versions.
- Authority: `RFC-0034`, `RFC-0033`, `RFC-0032`, `RFC-0031`, `DEC-0001`, `DEC-0034`
- Sources: [`docs/RFC-0034.md`](../RFC-0034.md), [`docs/SEMANTICS.md`](../SEMANTICS.md), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/RFC-0033.md`](../RFC-0033.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/RFC-0031.md`](../RFC-0031.md), [`docs/decisions/0001-error-code-policy.md`](../decisions/0001-error-code-policy.md), [`docs/decisions/0034-lsp-internal-diagnostic-ordering-boundary.md`](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-lsp/src/diagnostic_control.rs`](../../crates/ling-lsp/src/diagnostic_control.rs), [`crates/ling-lsp/src/publication.rs`](../../crates/ling-lsp/src/publication.rs), [`crates/ling-lsp/src/pull_diagnostics.rs`](../../crates/ling-lsp/src/pull_diagnostics.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/diagnostic_control.rs`](../../crates/ling-lsp/tests/diagnostic_control.rs), [`crates/ling-lsp/tests/push_diagnostics.rs`](../../crates/ling-lsp/tests/push_diagnostics.rs), [`crates/ling-lsp/tests/pull_diagnostics.rs`](../../crates/ling-lsp/tests/pull_diagnostics.rs), [`tests/protocols/lsp-diagnostic-control/README.md`](../../tests/protocols/lsp-diagnostic-control/README.md), [`docs/status/LSP-2204-IMPLEMENTATION-REPORT.md`](../status/LSP-2204-IMPLEMENTATION-REPORT.md)
- Notes: Control is a stateless post-adapter LSP projection. It does not change parser recovery or compiler diagnostics and does not expose the crate-private Trait solver; a future production solver resource diagnostic remains subject to the same exact-root rule once separately authorized.

### `PROTO-LSP-DOCUMENT-SYMBOL` — Ling LSP Document Symbols

- Producer: ling lsp --stdio; ling-lsp document-symbol provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Negotiate an optional object-valued textDocument.documentSymbol capability and optional boolean hierarchicalDocumentSymbolSupport; accept only Ready-state textDocument/documentSymbol requests naming an exact current Ling URI, while notifications perform no work.
- Writer policy: Capture one immutable snapshot, resolve exact visible bytes, build at most 4096 compiler structural nodes, and return either one module-rooted DocumentSymbol tree with separate original-byte-projected full/selection ranges or the same tree's URI-bound SymbolInformation pre-order fallback.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; incompatible kind mapping, hierarchy, field, order, limit, snapshot, temporary-isolation, projection, or failure behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.document-symbol/0.1 is Preview with no predecessor and clients gate on documentSymbolProvider plus the exact lingDocumentSymbols discovery object.
- Authority: `RFC-0036`, `RFC-0004`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0073`
- Sources: [`docs/RFC-0036.md`](../RFC-0036.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0073-ide-resolved-definition-index.md`](../decisions/0073-ide-resolved-definition-index.md), [`crates/ling-db/src/resolved_outline.rs`](../../crates/ling-db/src/resolved_outline.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/document_symbols.rs`](../../crates/ling-lsp/src/document_symbols.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/document_symbols.rs`](../../crates/ling-lsp/tests/document_symbols.rs), [`tests/protocols/lsp-document-symbol/README.md`](../../tests/protocols/lsp-document-symbol/README.md), [`docs/status/IDE-2301-IMPLEMENTATION-REPORT.md`](../status/IDE-2301-IMPLEMENTATION-REPORT.md)
- Notes: Document symbols are presentation derived from resolved source structure, not Semantic ID publication. Local bindings, inferred details, dynamic registration, progress, partial results, asynchronous cancellation, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-FORMATTING` — Ling LSP bounded document formatting

- Producer: ling lsp --stdio; ling-lsp formatting adapter
- Consumer: Preview LSP clients; editor and integration test harnesses
- Reader policy: Accept only textDocument/formatting requests for a current open writable RFC-0023 overlay with exact textDocument.uri and fixed tabSize=4, insertSpaces=true options; reject notifications without work and reject malformed, closed, missing, or read-only requests without mutation.
- Writer policy: Return zero edits for unchanged or invalid source and exactly one whole-document TextEdit for a safely published formatter candidate; project the original end in the negotiated encoding, preserve an existing BOM outside the edit, and never apply or persist the result.
- Unknown-field policy: Reject unknown request, textDocument, and options fields in 0.1; emit no result extension fields, annotation, version, URI, WorkspaceEdit, or Semantic Transaction value.
- Migration tool: None; ling.lsp.formatting/0.1 is Experimental with no predecessor, and incompatible option, range, cardinality, or snapshot behavior requires a new marker and migration evidence.
- Authority: `RFC-0026`, `RFC-0023`, `DEC-0029`, `DEC-0023`, `DEC-0057`
- Sources: [`docs/RFC-0026.md`](../RFC-0026.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-format/src/edit.rs`](../../crates/ling-format/src/edit.rs)
- Fixtures: [`crates/ling-lsp/tests/formatting.rs`](../../crates/ling-lsp/tests/formatting.rs), [`tests/protocols/lsp-formatting/README.md`](../../tests/protocols/lsp-formatting/README.md)
- Notes: Whole-document formatting only. Range/on-type formatting, format-on-save, minimal diffs, filesystem reads/writes, Workspace Edits, cancellation, and Semantic Transactions remain deferred.

### `PROTO-LSP-HOVER` — Ling checked LSP hover

- Producer: ling lsp --stdio; ling-lsp hover provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Negotiate optional object-valued textDocument.hover capabilities and the first supported plaintext or markdown contentFormat in client order; accept only Ready-state textDocument/hover requests naming one exact current Ling URI and exact negotiated position, while notifications perform no work.
- Writer policy: Capture and revalidate one immutable snapshot, consume a complete bounded compiler checked-hover index, select the smallest exact original-byte identifier span, and return deterministic bilingual checked type/kind/mutability/Effect/Capability/Trait-selection facts plus that exact range, or null when no target exists.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; incompatible target taxonomy, selection, type normalization, fact/markup order, bound, snapshot, temporary-isolation, projection, or failure behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.hover/0.1 is Preview with no predecessor and clients gate on hoverProvider plus the exact lingHover discovery object.
- Authority: `RFC-0037`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `RFC-0036`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0027`, `DEC-0029`, `DEC-0060`, `DEC-0071`, `DEC-0074`, `DEC-0075`, `DEC-0078`, `DEC-0080`
- Sources: [`docs/RFC-0037.md`](../RFC-0037.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0036.md`](../RFC-0036.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0027-trait-checked-core-dictionary-witness.md`](../decisions/0027-trait-checked-core-dictionary-witness.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0060-seed-effect-row-snapshot.md`](../decisions/0060-seed-effect-row-snapshot.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0074-ide-typed-definition-observation.md`](../decisions/0074-ide-typed-definition-observation.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`docs/decisions/0078-ide-rename-reference-span-observation.md`](../decisions/0078-ide-rename-reference-span-observation.md), [`docs/decisions/0080-ide-completion-checked-metadata.md`](../decisions/0080-ide-completion-checked-metadata.md), [`crates/ling-db/src/checked_hover_index.rs`](../../crates/ling-db/src/checked_hover_index.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/hover.rs`](../../crates/ling-lsp/src/hover.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/checked_hover_index.rs`](../../crates/ling-db/src/checked_hover_index.rs), [`crates/ling-lsp/tests/hover.rs`](../../crates/ling-lsp/tests/hover.rs), [`tests/protocols/lsp-hover/README.md`](../../tests/protocols/lsp-hover/README.md), [`docs/status/IDE-2302-IMPLEMENTATION-REPORT.md`](../status/IDE-2302-IMPLEMENTATION-REPORT.md)
- Notes: Hover is a presentation projection of complete checked compiler facts. It deliberately omits documentation and profile/resource claims for which no accepted compiler observation exists, and never publishes resolver/Semantic IDs, implementation ordinals, paths, or debug output.

### `PROTO-LSP-LIFECYCLE` — Ling LSP lifecycle and stdio transport

- Producer: ling lsp --stdio; ling-lsp lifecycle server
- Consumer: LSP clients; editor and integration test harnesses
- Reader policy: Accept one JSON-RPC 2.0 object per CRLF Content-Length frame; reject malformed framing, invalid IDs, batches, unsupported lifecycle state, and invalid initialize metadata without guessing or converting URIs to host paths.
- Writer policy: Emit only framed compact UTF-8 JSON-RPC responses for initialize and shutdown; preserve request IDs, flush each response, and write no unframed protocol bytes or human text to stdout.
- Unknown-field policy: Unknown JSON-RPC object fields and ASCII transport headers are ignored for this Preview; unknown methods are rejected only when they are requests, and future incompatible fields require a new protocol version.
- Migration tool: None; `ling.lsp.lifecycle/0.1` is current-writer-only and a future Stable/editor contract requires an accepted migration specification.
- Authority: `RFC-0004`, `DEC-0029`, `DEC-0257`, `DEC-0258`, `RFC-0029`
- Sources: [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0257-current-lsp-lifecycle-skeleton.md`](../decisions/0257-current-lsp-lifecycle-skeleton.md), [`docs/decisions/0258-current-lsp-position-encoding-boundary.md`](../decisions/0258-current-lsp-position-encoding-boundary.md), [`crates/ling-source/src/position.rs`](../../crates/ling-source/src/position.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-source/src/position.rs`](../../crates/ling-source/src/position.rs), [`crates/ling-lsp/tests/lifecycle.rs`](../../crates/ling-lsp/tests/lifecycle.rs), [`crates/ling-lsp/tests/position_encoding.rs`](../../crates/ling-lsp/tests/position_encoding.rs), [`crates/ling-cli/tests/lsp.rs`](../../crates/ling-cli/tests/lsp.rs), [`tests/protocols/lsp-lifecycle/README.md`](../../tests/protocols/lsp-lifecycle/README.md), [`docs/status/LSP-2101-IMPLEMENTATION-REPORT.md`](../status/LSP-2101-IMPLEMENTATION-REPORT.md), [`docs/status/LSP-2102-IMPLEMENTATION-REPORT.md`](../status/LSP-2102-IMPLEMENTATION-REPORT.md)
- Notes: DEC-0257 accepts the RFC-0004 lifecycle implementation as bounded LSP-2101; DEC-0258 accepts DEC-0029 source projection plus RFC-0004 initialize negotiation as bounded LSP-2102. RFC-0029 extends the initialize capability object with explicit textDocumentSync incremental support and the current overlay marker without changing lifecycle transitions. Snapshots, cancellation, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-NAVIGATION` — Ling resolver-backed LSP navigation

- Producer: ling lsp --stdio; ling-lsp navigation provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Validate optional object-valued definition, declaration, and typeDefinition capabilities plus optional boolean dynamicRegistration; accept only Ready-state requests naming an exact current Ling URI and exact negotiated position, while notifications perform no work.
- Writer policy: Capture and revalidate one immutable snapshot, select one exact resolver reference, and return either one URI/range Location for the user definition/binding or direct checked nominal type definition, or null for absent/unsupported targets; never synthesize paths, URIs, virtual documents, arrays, or LocationLinks.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; incompatible method mapping, target policy, span, type peeling, field/cardinality, bound, snapshot, URI, failure, or null behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.navigation/0.1 is Preview with no predecessor and clients gate on the three standard provider booleans plus the exact lingNavigation discovery object.
- Authority: `RFC-0038`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `RFC-0037`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`
- Sources: [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0037.md`](../RFC-0037.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`crates/ling-db/src/definition_projection.rs`](../../crates/ling-db/src/definition_projection.rs), [`crates/ling-db/src/navigation_index.rs`](../../crates/ling-db/src/navigation_index.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/navigation.rs`](../../crates/ling-lsp/src/navigation.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/navigation_index.rs`](../../crates/ling-db/src/navigation_index.rs), [`crates/ling-lsp/tests/navigation.rs`](../../crates/ling-lsp/tests/navigation.rs), [`tests/protocols/lsp-navigation/README.md`](../../tests/protocols/lsp-navigation/README.md), [`docs/status/IDE-2303-IMPLEMENTATION-REPORT.md`](../status/IDE-2303-IMPLEMENTATION-REPORT.md)
- Notes: Navigation is a source presentation of unique resolver identity. Builtins/Prelude without source, generated/primitive virtual documents, multiple targets, and public identity/provenance remain explicit null or deferred behavior rather than fabricated locations.

### `PROTO-LSP-OVERLAY` — Ling LSP Preview document overlay

- Producer: ling lsp --stdio; ling-lsp overlay adapter
- Consumer: Preview LSP clients; editor and integration test harnesses
- Reader policy: Accept only restricted ling://workspace, ling://dependency, and untitled://ling URI forms; require an open writable document, a strictly newer non-negative version, and 1-64 ordered full or negotiated-position range changes; reject invalid, stale, closed, malformed, boundary-invalid, oversized, or read-only batches without VFS or version mutation.
- Writer policy: Apply every change in protocol order to immutable local SourceFile snapshots, rebuild SourceMap/line indexes after each entry, publish the final exact UTF-8 bytes once, preserve overlay precedence, reveal disk or remove temporary files on close, and expose no SourceId, revision, or host path on the wire.
- Unknown-field policy: Unknown ordinary JSON-RPC fields are ignored by the Experimental parser; rangeLength is rejected, range/start/end/line/character shapes are validated exactly by type and bounds, and incompatible URI/version/edit evolution requires a new protocol version.
- Migration tool: None; the 0.2 server accepts every valid 0.1 single-full-change message, and clients discover the new marker and incremental capability during initialize.
- Authority: `RFC-0023`, `RFC-0029`, `RFC-0004`, `DEC-0019`, `DEC-0029`, `DEC-0069`, `DEC-0070`, `DEC-0259`
- Sources: [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/decisions/0069-lsp-utf8-edit-primitive.md`](../decisions/0069-lsp-utf8-edit-primitive.md), [`docs/decisions/0070-lsp-position-edit-projection.md`](../decisions/0070-lsp-position-edit-projection.md), [`docs/decisions/0259-current-lsp-open-document-overlay.md`](../decisions/0259-current-lsp-open-document-overlay.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-source/src/lib.rs`](../../crates/ling-source/src/lib.rs), [`crates/ling-source/src/position.rs`](../../crates/ling-source/src/position.rs), [`crates/ling-source/src/vfs.rs`](../../crates/ling-source/src/vfs.rs)
- Fixtures: [`crates/ling-lsp/tests/overlay.rs`](../../crates/ling-lsp/tests/overlay.rs), [`crates/ling-lsp/tests/incremental_changes.rs`](../../crates/ling-lsp/tests/incremental_changes.rs), [`tests/protocols/lsp-overlay/README.md`](../../tests/protocols/lsp-overlay/README.md), [`docs/status/LSP-2103-IMPLEMENTATION-REPORT.md`](../status/LSP-2103-IMPLEMENTATION-REPORT.md), [`docs/status/LSP-2104-IMPLEMENTATION-REPORT.md`](../status/LSP-2104-IMPLEMENTATION-REPORT.md)
- Notes: DEC-0259 accepts RFC-0023 full-text overlays as bounded LSP-2103. RFC-0029 implements LSP-2104 by advancing the Experimental marker to 0.2, advertising incremental sync, and adding bounded ordered UTF-8/16/32 range batches with failure-atomic publication and full-replacement equivalence. Compiler queries, snapshots, stale analysis, diagnostics, Workspace Edits, cancellation, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-PREPARE-RENAME` — Ling checked LSP prepare rename

- Producer: ling lsp --stdio; ling-lsp prepare-rename provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Validate optional object-valued textDocument.rename capability, boolean dynamicRegistration/prepareSupport, and prepareSupportDefaultBehavior=1; accept only Ready-state textDocument/prepareRename requests with exact URI and unsigned-u32 position while notifications perform no work.
- Writer policy: Capture and revalidate one immutable snapshot, require complete resolution/type/Effect checking, select one exact declaration or resolver expression reference, and return null or exactly one standard range-with-placeholder for a writable source-backed target.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; selection, writability, range, placeholder, null, bound, snapshot, or failure changes require a new marker and migration evidence.
- Migration tool: None; ling.lsp.prepare-rename/0.1 is Preview with no predecessor and clients gate on renameProvider.prepareProvider plus the exact lingPrepareRename discovery object.
- Authority: `RFC-0040`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `RFC-0038`, `RFC-0039`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0077`, `DEC-0078`
- Sources: [`docs/RFC-0040.md`](../RFC-0040.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/RFC-0039.md`](../RFC-0039.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`docs/decisions/0077-ide-rename-identifier-observation.md`](../decisions/0077-ide-rename-identifier-observation.md), [`docs/decisions/0078-ide-rename-reference-span-observation.md`](../decisions/0078-ide-rename-reference-span-observation.md), [`crates/ling-db/src/reference_search_index.rs`](../../crates/ling-db/src/reference_search_index.rs), [`crates/ling-db/src/rename_identifier.rs`](../../crates/ling-db/src/rename_identifier.rs), [`crates/ling-lsp/src/prepare_rename.rs`](../../crates/ling-lsp/src/prepare_rename.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/reference_search_index.rs`](../../crates/ling-db/src/reference_search_index.rs), [`crates/ling-lsp/tests/prepare_rename.rs`](../../crates/ling-lsp/tests/prepare_rename.rs), [`tests/protocols/lsp-prepare-rename/README.md`](../../tests/protocols/lsp-prepare-rename/README.md), [`docs/status/IDE-2305-IMPLEMENTATION-REPORT.md`](../status/IDE-2305-IMPLEMENTATION-REPORT.md)
- Notes: Prepare Rename is read-only and receives no new name. New-name legality, confusable/collision policy, visibility/coherence simulation, edits, and DefinitionId migration remain IDE-2306 work rather than fabricated prepare-rename checks.

### `PROTO-LSP-PUBLISH-DIAGNOSTICS` — Ling LSP deterministic push diagnostics

- Producer: ling lsp --stdio; ling-lsp compiler diagnostic publisher
- Consumer: Preview LSP clients; editor hosts; integration test harnesses
- Reader policy: Accept no client request; successful source mutations schedule one immutable complete-state analysis ticket, and completion is accepted only while every lifecycle, encoding, revision, document, version, origin, byte, and project-input identity remains current.
- Writer policy: After each stdio message boundary, publish URI-sorted changed textDocument/publishDiagnostics params from a complete current result, include versions only for open documents, clear removed entries, reject oversized or failed output atomically, and replace the committed ledger only after all notifications are valid.
- Unknown-field policy: The 0.2 capability and notification writer uses standard publishDiagnostics fields plus exact Experimental discovery markers; incompatible trigger, ticket, controlled diagnostic-set, version, ordering, ledger, or clearance evolution requires a new protocol marker and migration evidence.
- Migration tool: Gate on the exact advertised 0.2 marker and ling.lsp.diagnostic-control/0.1 discovery values; 0.2 replaces unbounded 0.1 diagnostic sets with deterministic explicit bounded sets.
- Authority: `RFC-0034`, `RFC-0032`, `RFC-0031`, `RFC-0030`, `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0034`, `DEC-0035`, `DEC-0071`, `DEC-0072`
- Sources: [`docs/RFC-0034.md`](../RFC-0034.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/RFC-0031.md`](../RFC-0031.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0034-lsp-internal-diagnostic-ordering-boundary.md`](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md), [`docs/decisions/0035-lsp-internal-diagnostic-batch-boundary.md`](../decisions/0035-lsp-internal-diagnostic-batch-boundary.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0072-lsp-diagnostic-span-projection.md`](../decisions/0072-lsp-diagnostic-span-projection.md), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-lsp/src/publication.rs`](../../crates/ling-lsp/src/publication.rs), [`crates/ling-lsp/src/diagnostic_control.rs`](../../crates/ling-lsp/src/diagnostic_control.rs)
- Fixtures: [`crates/ling-lsp/tests/push_diagnostics.rs`](../../crates/ling-lsp/tests/push_diagnostics.rs), [`crates/ling-db/tests/workspace_diagnostics.rs`](../../crates/ling-db/tests/workspace_diagnostics.rs), [`tests/protocols/lsp-publish-diagnostics/README.md`](../../tests/protocols/lsp-publish-diagnostics/README.md), [`docs/status/LSP-2202-IMPLEMENTATION-REPORT.md`](../status/LSP-2202-IMPLEMENTATION-REPORT.md)
- Notes: The 0.2 writer preserves 0.1 scheduling, freshness, ledger, version, and clearance behavior while applying RFC-0034 shared deterministic root control before publication. Cancellation, progress, fixes, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-PULL-DIAGNOSTICS` — Ling LSP deterministic pull diagnostics

- Producer: ling lsp --stdio; ling-lsp pull diagnostic provider
- Consumer: LSP 3.17 pull-diagnostic clients; editor hosts; integration test harnesses
- Reader policy: Negotiate only an object-valued capabilities.textDocument.diagnostic member; accept current exact Ling document requests or a bounded unique workspace previous-result list, the exact optional provider identifier, and ordinary forward-compatible fields without progress or partial-result work.
- Writer policy: Return URI-sorted full or unchanged document/workspace reports from one current immutable RFC-0032 ticket, preserve exact adapter arrays and open-document versions, clear previous-only removed URIs, derive stateless domain-separated BLAKE3 result IDs, and replace oversized successes with bounded RequestFailed errors.
- Unknown-field policy: Known members with wrong types, invalid identities, duplicates, unsupported identifiers, unknown current documents, and over-limit previous-result arrays are rejected before analysis; unknown ordinary fields and progress tokens are accepted and ignored. Incompatible provider, request, report, result-ID, bound, or failure behavior requires a new protocol marker and migration evidence.
- Migration tool: Gate on the exact 0.2 provider identifier and ling.lsp.diagnostic-control/0.1 discovery values; 0.2 preserves 0.1 request/report shapes but returns the shared controlled diagnostic set.
- Authority: `RFC-0034`, `RFC-0033`, `RFC-0032`, `RFC-0031`, `RFC-0030`, `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0034`, `DEC-0071`, `DEC-0072`
- Sources: [`docs/RFC-0034.md`](../RFC-0034.md), [`docs/RFC-0033.md`](../RFC-0033.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/RFC-0031.md`](../RFC-0031.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0034-lsp-internal-diagnostic-ordering-boundary.md`](../decisions/0034-lsp-internal-diagnostic-ordering-boundary.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0072-lsp-diagnostic-span-projection.md`](../decisions/0072-lsp-diagnostic-span-projection.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-lsp/src/publication.rs`](../../crates/ling-lsp/src/publication.rs), [`crates/ling-lsp/src/pull_diagnostics.rs`](../../crates/ling-lsp/src/pull_diagnostics.rs), [`crates/ling-lsp/src/diagnostic_control.rs`](../../crates/ling-lsp/src/diagnostic_control.rs)
- Fixtures: [`crates/ling-lsp/tests/pull_diagnostics.rs`](../../crates/ling-lsp/tests/pull_diagnostics.rs), [`crates/ling-lsp/tests/push_diagnostics.rs`](../../crates/ling-lsp/tests/push_diagnostics.rs), [`crates/ling-lsp/tests/diagnostic_adapter.rs`](../../crates/ling-lsp/tests/diagnostic_adapter.rs), [`tests/protocols/lsp-pull-diagnostics/README.md`](../../tests/protocols/lsp-pull-diagnostics/README.md), [`docs/status/LSP-2203-IMPLEMENTATION-REPORT.md`](../status/LSP-2203-IMPLEMENTATION-REPORT.md)
- Notes: The 0.2 provider preserves 0.1 stateless request/report/result-ID behavior and consumes the same RFC-0034 controlled arrays as push without changing pending work or the ledger. Dynamic registration, cancellation, progress, partial results, refresh, related-document maps, notebooks, fixes, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-REFERENCES` — Ling checked LSP references

- Producer: ling lsp --stdio; ling-lsp references provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Validate optional object-valued textDocument.references capability and optional boolean dynamicRegistration; accept only Ready-state requests with exact URI/u32 position and required boolean context.includeDeclaration, while notifications perform no work.
- Writer policy: Capture and revalidate one immutable snapshot, require complete resolution/type/Effect checking, select an exact declaration or resolver expression reference, and return canonical URI/range Location arrays for every same-target expression reference plus the optional declaration, bounded at 16384.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; relation vocabulary/emission, precedence, selection, declaration policy, order, cardinality, bound, snapshot, URI, range, empty-result, or failure changes require a new marker and migration evidence.
- Migration tool: None; ling.lsp.references/0.1 is Preview with no predecessor and clients gate on referencesProvider plus the exact lingReferences discovery object.
- Authority: `RFC-0039`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `RFC-0037`, `RFC-0038`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0076`, `DEC-0078`
- Sources: [`docs/RFC-0039.md`](../RFC-0039.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0037.md`](../RFC-0037.md), [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`docs/decisions/0076-ide-resolved-reference-reverse-index.md`](../decisions/0076-ide-resolved-reference-reverse-index.md), [`docs/decisions/0078-ide-rename-reference-span-observation.md`](../decisions/0078-ide-rename-reference-span-observation.md), [`crates/ling-db/src/reference_search_index.rs`](../../crates/ling-db/src/reference_search_index.rs), [`crates/ling-db/src/reference_span_index.rs`](../../crates/ling-db/src/reference_span_index.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/references.rs`](../../crates/ling-lsp/src/references.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/reference_search_index.rs`](../../crates/ling-db/src/reference_search_index.rs), [`crates/ling-lsp/tests/references.rs`](../../crates/ling-lsp/tests/references.rs), [`tests/protocols/lsp-references/README.md`](../../tests/protocols/lsp-references/README.md), [`docs/status/IDE-2304-IMPLEMENTATION-REPORT.md`](../status/IDE-2304-IMPLEMENTATION-REPORT.md)
- Notes: The full relation vocabulary is read/write/call/type/implementation; version 0.1 emits only read/write/call because type and implementation surfaces lack resolver-owned occurrence identities. No relation or identity metadata is added to standard Location results.

### `PROTO-LSP-RENAME` — Ling checked transactional LSP rename

- Producer: ling lsp --stdio; ling-lsp rename provider
- Consumer: LSP 3.18 clients with transactional versioned Workspace Edit support; editor hosts; integration test harnesses
- Reader policy: Validate object-valued workspace.workspaceEdit capability, boolean documentChanges, and the standard failureHandling vocabulary; enable Ready-state textDocument/rename only for documentChanges=true plus failureHandling=transactional, with exact URI, unsigned-u32 position, and string newName.
- Writer policy: Capture and revalidate one immutable complete checked snapshot, select only resolver-owned writable definitions, bindings, or explicit import aliases, validate Unicode 17.0.0 policy, simulate all replacements through resolution/type/Effect checking and identity/topology checks, then emit deterministic URI-ordered versioned documentChanges or no result.
- Unknown-field policy: Ignore ordinary unknown request and capability fields while rejecting malformed known members; capability, target, name, occurrence, simulation, identity, ordering, version, result, null, bound, snapshot, or failure changes require a new marker and migration evidence.
- Migration tool: None; ling.lsp.rename/0.1 is Preview with no predecessor, no unversioned changes fallback, and clients gate on the exact transactional workspace capability plus lingRename discovery object.
- Authority: `RFC-0041`, `RFC-0040`, `RFC-0039`, `RFC-0038`, `RFC-0004`, `RFC-0005`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0077`, `DEC-0078`
- Sources: [`docs/RFC-0041.md`](../RFC-0041.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0038.md`](../RFC-0038.md), [`docs/RFC-0039.md`](../RFC-0039.md), [`docs/RFC-0040.md`](../RFC-0040.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0075-ide-resolved-reference-index.md`](../decisions/0075-ide-resolved-reference-index.md), [`docs/decisions/0077-ide-rename-identifier-observation.md`](../decisions/0077-ide-rename-identifier-observation.md), [`docs/decisions/0078-ide-rename-reference-span-observation.md`](../decisions/0078-ide-rename-reference-span-observation.md), [`crates/ling-db/src/rename_alias_index.rs`](../../crates/ling-db/src/rename_alias_index.rs), [`crates/ling-db/src/reference_search_index.rs`](../../crates/ling-db/src/reference_search_index.rs), [`crates/ling-lsp/src/rename.rs`](../../crates/ling-lsp/src/rename.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/src/rename_alias_index.rs`](../../crates/ling-db/src/rename_alias_index.rs), [`crates/ling-lsp/tests/rename.rs`](../../crates/ling-lsp/tests/rename.rs), [`tests/protocols/lsp-rename/README.md`](../../tests/protocols/lsp-rename/README.md), [`docs/status/IDE-2306-IMPLEMENTATION-REPORT.md`](../status/IDE-2306-IMPLEMENTATION-REPORT.md)
- Notes: The bounded standard Workspace Edit is proposed to the client and is never applied by the server. General Semantic Transactions, language Alias syntax, localized Author Source, generated or dependency mutation, module/file rename, type-only identities, cancellation, annotations, and Stable lifecycle remain out of scope.

### `PROTO-LSP-REQUEST-CANCELLATION` — Ling Preview LSP request cancellation

- Producer: ling lsp --stdio; ling-lsp cancellation dispatcher
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Associate exact string and number request IDs before execution, accept only notification-form $/cancelRequest with one valid params.id, cancel the currently live exact association, ignore unknown, duplicate, and late cancellation, and reject duplicate live request IDs without executing them.
- Writer policy: Propagate one monotonic cooperative token through compiler-backed analysis, check at bounded stages and immediately before publication, return standard -32800 when observed, and publish no partial response, Workspace Edit, completion-resolve batch, workspace index, semantic-token history, diagnostic, or compiler cache entry.
- Unknown-field policy: Ignore ordinary unknown cancellation parameter members; malformed notifications have no effect and no response, while incompatible method, ID, precedence, discovery, checkpoint, cleanup, publication, or error behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.request-cancellation/0.1 is Preview with no predecessor, and clients discover its exact experimental marker while standard non-cancelling clients remain unchanged.
- Authority: `RFC-0049`, `RFC-0048`, `RFC-0045`, `RFC-0044`, `RFC-0043`, `RFC-0041`, `RFC-0030`, `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0021`, `DEC-0030`, `DEC-0031`, `DEC-0032`
- Sources: [`docs/RFC-0049.md`](../RFC-0049.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0041.md`](../RFC-0041.md), [`docs/RFC-0043.md`](../RFC-0043.md), [`docs/RFC-0044.md`](../RFC-0044.md), [`docs/RFC-0045.md`](../RFC-0045.md), [`docs/RFC-0048.md`](../RFC-0048.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0021-deterministic-parallel-scheduling.md`](../decisions/0021-deterministic-parallel-scheduling.md), [`docs/decisions/0030-lsp-request-snapshot-boundary.md`](../decisions/0030-lsp-request-snapshot-boundary.md), [`docs/decisions/0031-lsp-internal-cancellation-boundary.md`](../decisions/0031-lsp-internal-cancellation-boundary.md), [`docs/decisions/0032-lsp-internal-scheduling-boundary.md`](../decisions/0032-lsp-internal-scheduling-boundary.md), [`crates/ling-types/src/solver.rs`](../../crates/ling-types/src/solver.rs), [`crates/ling-types/src/lib.rs`](../../crates/ling-types/src/lib.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/request_cancellation.rs`](../../crates/ling-lsp/src/request_cancellation.rs), [`crates/ling-lsp/src/rename.rs`](../../crates/ling-lsp/src/rename.rs), [`crates/ling-lsp/src/completion.rs`](../../crates/ling-lsp/src/completion.rs), [`crates/ling-lsp/src/completion_resolve.rs`](../../crates/ling-lsp/src/completion_resolve.rs), [`crates/ling-lsp/src/workspace_symbols.rs`](../../crates/ling-lsp/src/workspace_symbols.rs), [`crates/ling-lsp/src/semantic_tokens.rs`](../../crates/ling-lsp/src/semantic_tokens.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/cancellation.rs`](../../crates/ling-lsp/tests/cancellation.rs), [`tests/protocols/lsp-request-cancellation/fixtures/v1.json`](../../tests/protocols/lsp-request-cancellation/fixtures/v1.json), [`tests/protocols/lsp-request-cancellation/README.md`](../../tests/protocols/lsp-request-cancellation/README.md), [`docs/status/LSP-2502-CANCELLATION-IMPLEMENTATION-REPORT.md`](../status/LSP-2502-CANCELLATION-IMPLEMENTATION-REPORT.md)
- Notes: The Preview adds no deadline, timeout, progress, quota, server-initiated cancellation, parallel compiler mutation, persistent state, VM/runtime cancellation, Stable editor compatibility, or general Semantic Transaction claim.

### `PROTO-LSP-RESOURCE-LIMITS` — Ling bounded Preview LSP resources

- Producer: ling lsp --stdio; ling-lsp resource accounting and request admission
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Count exact decoded UTF-8 overlay bytes and live string/number request-ID associations using the fixed session and per-operation scopes; retain existing transport, completion, diagnostic, and Trait solver bounds.
- Writer policy: Advertise the exact Preview marker and return -32803 with registered bilingual L-LSP-0002 data for request-form LSP-owned hard-limit failures, while notifications remain response-free and every failure is publication-atomic.
- Unknown-field policy: The marker and resource error data have exact fixed shapes; clients may ignore Experimental discovery, while incompatible units, scopes, limits, precedence, facts, cleanup, retry, or privacy behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.resource-limits/0.1 is Preview with no predecessor and adds no JSON-RPC method or client-selected non-diagnostic configuration.
- Authority: `RFC-0051`, `RFC-0050`, `RFC-0049`, `RFC-0042`, `RFC-0032`, `RFC-0029`, `RFC-0023`, `RFC-0005`, `RFC-0004`, `DEC-0019`, `DEC-0033`
- Sources: [`docs/RFC-0051.md`](../RFC-0051.md), [`docs/RFC-0050.md`](../RFC-0050.md), [`docs/RFC-0049.md`](../RFC-0049.md), [`docs/RFC-0042.md`](../RFC-0042.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0005.md`](../RFC-0005.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0033-lsp-internal-byte-accounting-boundary.md`](../decisions/0033-lsp-internal-byte-accounting-boundary.md), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`crates/ling-lsp/src/resource.rs`](../../crates/ling-lsp/src/resource.rs), [`crates/ling-lsp/src/request_cancellation.rs`](../../crates/ling-lsp/src/request_cancellation.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/src/resource.rs`](../../crates/ling-lsp/src/resource.rs), [`crates/ling-lsp/tests/resource_limits.rs`](../../crates/ling-lsp/tests/resource_limits.rs), [`tests/protocols/lsp-resource-limits/fixtures/v1.json`](../../tests/protocols/lsp-resource-limits/fixtures/v1.json), [`tests/protocols/lsp-resource-limits/README.md`](../../tests/protocols/lsp-resource-limits/README.md), [`docs/status/LSP-2504-IMPLEMENTATION-REPORT.md`](../status/LSP-2504-IMPLEMENTATION-REPORT.md)
- Notes: The Preview counts explicit UTF-8 bytes and logical associations only; it adds no allocator/RSS promise, OOM recovery, adaptive quota, eviction, partial result, deadline, total compiler fuel, Stable lifecycle, or Semantic Transaction claim.

### `PROTO-LSP-SCHEDULING` — Ling deterministic Preview LSP scheduling

- Producer: ling lsp --stdio; ling-lsp logical scheduler
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Preserve JSON-RPC request and state wire order, classify current compiler work as Interactive, Analysis, or Background, service ready diagnostics before Background work, and apply fixed logical fairness bursts without observing host timing.
- Writer policy: Advertise the exact Preview marker, retain message-boundary diagnostic debounce, cancel superseded diagnostic tickets through compiler stage checkpoints, reject stale completion, and publish only complete current-snapshot results atomically.
- Unknown-field policy: The marker has an exact fixed shape; clients may ignore Experimental discovery, while incompatible class, bound, ordering, debounce, supersession, publication, or privacy behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.scheduling/0.1 is Preview with no predecessor and adds no JSON-RPC method or client configuration.
- Authority: `RFC-0050`, `RFC-0049`, `RFC-0045`, `RFC-0034`, `RFC-0033`, `RFC-0032`, `RFC-0030`, `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0021`, `DEC-0030`, `DEC-0031`, `DEC-0032`
- Sources: [`docs/RFC-0050.md`](../RFC-0050.md), [`docs/RFC-0049.md`](../RFC-0049.md), [`docs/RFC-0045.md`](../RFC-0045.md), [`docs/RFC-0034.md`](../RFC-0034.md), [`docs/RFC-0033.md`](../RFC-0033.md), [`docs/RFC-0032.md`](../RFC-0032.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0021-deterministic-parallel-scheduling.md`](../decisions/0021-deterministic-parallel-scheduling.md), [`docs/decisions/0030-lsp-request-snapshot-boundary.md`](../decisions/0030-lsp-request-snapshot-boundary.md), [`docs/decisions/0031-lsp-internal-cancellation-boundary.md`](../decisions/0031-lsp-internal-cancellation-boundary.md), [`docs/decisions/0032-lsp-internal-scheduling-boundary.md`](../decisions/0032-lsp-internal-scheduling-boundary.md), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs), [`crates/ling-lsp/src/scheduler.rs`](../../crates/ling-lsp/src/scheduler.rs), [`crates/ling-lsp/src/publication.rs`](../../crates/ling-lsp/src/publication.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/scheduling.rs`](../../crates/ling-lsp/tests/scheduling.rs), [`crates/ling-lsp/tests/push_diagnostics.rs`](../../crates/ling-lsp/tests/push_diagnostics.rs), [`tests/protocols/lsp-scheduling/fixtures/v1.json`](../../tests/protocols/lsp-scheduling/fixtures/v1.json), [`tests/protocols/lsp-scheduling/README.md`](../../tests/protocols/lsp-scheduling/README.md), [`docs/status/LSP-2503-SCHEDULER-IMPLEMENTATION-REPORT.md`](../status/LSP-2503-SCHEDULER-IMPLEMENTATION-REPORT.md)
- Notes: The Preview adds no wall-clock duration, latency SLA, deadline, dynamic priority, host-load adaptation, worker pool, parallel mutable request, response reordering, progress, partial result, quota, persistent queue, Stable lifecycle, or Semantic Transaction claim.

### `PROTO-LSP-SEMANTIC-TOKENS` — Ling bounded LSP semantic tokens

- Producer: ling lsp --stdio; ling-lsp semantic-token provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Validate the standard textDocument.semanticTokens capability, require full plus relative encoding and a nonempty RFC-0046 selected type legend, advertise delta only when requested, accept Ready-state full and negotiated delta requests for one exact tracked URI, and recover an invalid or expired delta base to full.
- Writer policy: Capture and revalidate one immutable snapshot, consume only RFC-0047 abstract tokens, project through the selected legend and negotiated UTF-8, UTF-16 or UTF-32 encoding, emit standard relative groups, retain at most 32 opaque results, and publish deterministic full or canonical one-edit delta responses atomically within fixed limits.
- Unknown-field policy: Ignore ordinary unknown capability and request members while rejecting malformed known members; incompatible taxonomy, provider, legend, position, result-ID, retention, delta, limit, privacy, freshness, cancellation, or failure behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.semantic-tokens/0.1 is Preview with no predecessor and clients gate on semanticTokensProvider plus the exact lingSemanticTokens discovery object.
- Authority: `RFC-0048`, `RFC-0047`, `RFC-0046`, `RFC-0004`, `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0031`, `DEC-0071`, `DEC-0084`, `DEC-0085`, `DEC-0086`, `DEC-0087`
- Sources: [`docs/RFC-0048.md`](../RFC-0048.md), [`docs/RFC-0047.md`](../RFC-0047.md), [`docs/RFC-0046.md`](../RFC-0046.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0031-lsp-internal-cancellation-boundary.md`](../decisions/0031-lsp-internal-cancellation-boundary.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0084-lsp-lexical-token-source-index.md`](../decisions/0084-lsp-lexical-token-source-index.md), [`docs/decisions/0085-lsp-checked-token-identity-observation.md`](../decisions/0085-lsp-checked-token-identity-observation.md), [`docs/decisions/0086-lsp-checked-token-snapshot-identity.md`](../decisions/0086-lsp-checked-token-snapshot-identity.md), [`docs/decisions/0087-lsp-checked-token-source-fixtures.md`](../decisions/0087-lsp-checked-token-source-fixtures.md), [`crates/ling-db/src/semantic_token_index.rs`](../../crates/ling-db/src/semantic_token_index.rs), [`crates/ling-lsp/src/semantic_tokens.rs`](../../crates/ling-lsp/src/semantic_tokens.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-db/tests/semantic_tokens.rs`](../../crates/ling-db/tests/semantic_tokens.rs), [`crates/ling-lsp/tests/semantic_tokens.rs`](../../crates/ling-lsp/tests/semantic_tokens.rs), [`tests/protocols/lsp-semantic-tokens/fixtures/v1.json`](../../tests/protocols/lsp-semantic-tokens/fixtures/v1.json), [`tests/protocols/lsp-semantic-tokens/README.md`](../../tests/protocols/lsp-semantic-tokens/README.md), [`docs/status/LSP-2403-IMPLEMENTATION-REPORT.md`](../status/LSP-2403-IMPLEMENTATION-REPORT.md), [`docs/status/LSP-2404-IMPLEMENTATION-REPORT.md`](../status/LSP-2404-IMPLEMENTATION-REPORT.md)
- Notes: The Preview exposes no source text, VFS revision, path, diagnostic, compiler, Semantic or Definition identity, type, Effect, Capability, debug data, range/refresh/dynamic registration, partial/work-done result, wire cancellation, persistent history, Semantic Transaction, or Stable claim. Future language categories remain excluded by RFC-0046.

### `PROTO-LSP-WORKSPACE` — Ling LSP atomic workspace reload

- Producer: ling lsp --stdio; ling-lsp workspace reload adapter
- Consumer: Preview LSP clients; editor hosts; integration test harnesses
- Reader policy: Accept only Ready-state ling/workspace/reload requests with an exact canonical-decimal base revision, one to 1029 unique logical source/project-input deltas, per-text and aggregate bounds, and restricted non-temporary Ling URIs; reject stale, malformed, duplicate, unsupported, missing-source-removal, and open-overlay-removal batches without mutation.
- Writer policy: Canonicalize source URIs and declared input kinds, apply the batch to a private VFS/server candidate, preserve open-overlay visibility and document version history, publish exactly once after complete success, and return only changed plus the canonical-decimal session revision.
- Unknown-field policy: Ignore unknown ordinary request and entry fields in 0.1 while validating every required field and exact type; unknown input names and incompatible identity, limit, result, or revision evolution require a new protocol version.
- Migration tool: None; ling.lsp.workspace/0.1 is Experimental with no predecessor and is discovered through the exact initialize capability.
- Authority: `RFC-0030`, `RFC-0002`, `RFC-0004`, `RFC-0023`, `RFC-0025`, `DEC-0019`, `DEC-0071`
- Sources: [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0025.md`](../RFC-0025.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs), [`crates/ling-source/src/vfs.rs`](../../crates/ling-source/src/vfs.rs), [`crates/ling-db/src/lib.rs`](../../crates/ling-db/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/workspace_reload.rs`](../../crates/ling-lsp/tests/workspace_reload.rs), [`tests/protocols/lsp-workspace/README.md`](../../tests/protocols/lsp-workspace/README.md), [`docs/status/LSP-2105-IMPLEMENTATION-REPORT.md`](../status/LSP-2105-IMPLEMENTATION-REPORT.md)
- Notes: Client/host publication keeps watcher timing, paths, symlinks, and filesystem reads outside Ling semantics. Reload revisions invalidate exact source and manifest/lock/config/profile/target inputs without eager full compilation. Diagnostics, cancellation, compiler-result staleness, file URI mapping, Workspace Edits, Semantic Transactions, and Stable compatibility remain deferred.

### `PROTO-LSP-WORKSPACE-SYMBOL` — Ling snapshot-indexed LSP workspace symbols

- Producer: ling lsp --stdio; ling-lsp workspace-symbol provider
- Consumer: LSP 3.17 clients; editor hosts; integration test harnesses
- Reader policy: Accept only Ready-state workspace/symbol requests with one required string query of at most 256 UTF-8 bytes and no NUL; match the exact source spelling or a case-sensitive prefix, ignore ordinary unknown members, and perform no work for notifications.
- Writer policy: Capture and revalidate one complete immutable RequestSnapshot, resolve user definitions from tracked non-temporary writable workspace sources, reuse at most one exact snapshot-keyed plan, project original spans through the negotiated encoding, sort exact matches before prefixes, and return the first 256 standard SymbolInformation items or one atomic fixed failure.
- Unknown-field policy: Ignore ordinary unknown request members, including progress tokens; incompatible provider, discovery, scope, matching, kind, field, ordering, limit, cache-key, cancellation, position, freshness, or failure behavior requires a new marker and migration evidence.
- Migration tool: None; ling.lsp.workspace-symbol/0.1 is Preview with no predecessor and clients gate on workspaceSymbolProvider plus the exact lingWorkspaceSymbols discovery object.
- Authority: `RFC-0045`, `RFC-0036`, `RFC-0030`, `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0031`, `DEC-0071`, `DEC-0073`, `DEC-0082`
- Sources: [`docs/RFC-0045.md`](../RFC-0045.md), [`docs/RFC-0004.md`](../RFC-0004.md), [`docs/RFC-0023.md`](../RFC-0023.md), [`docs/RFC-0029.md`](../RFC-0029.md), [`docs/RFC-0030.md`](../RFC-0030.md), [`docs/RFC-0036.md`](../RFC-0036.md), [`docs/decisions/0002-source-position-units.md`](../decisions/0002-source-position-units.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/decisions/0019-incremental-query-boundary.md`](../decisions/0019-incremental-query-boundary.md), [`docs/decisions/0029-lsp-position-encoding-projection.md`](../decisions/0029-lsp-position-encoding-projection.md), [`docs/decisions/0031-lsp-internal-cancellation-boundary.md`](../decisions/0031-lsp-internal-cancellation-boundary.md), [`docs/decisions/0071-lsp-workspace-state-snapshot.md`](../decisions/0071-lsp-workspace-state-snapshot.md), [`docs/decisions/0073-ide-resolved-definition-index.md`](../decisions/0073-ide-resolved-definition-index.md), [`docs/decisions/0082-ide-workspace-symbol-lookups.md`](../decisions/0082-ide-workspace-symbol-lookups.md), [`crates/ling-db/src/definition_index.rs`](../../crates/ling-db/src/definition_index.rs), [`crates/ling-lsp/src/workspace_symbols.rs`](../../crates/ling-lsp/src/workspace_symbols.rs), [`crates/ling-lsp/src/lib.rs`](../../crates/ling-lsp/src/lib.rs)
- Fixtures: [`crates/ling-lsp/tests/workspace_symbols.rs`](../../crates/ling-lsp/tests/workspace_symbols.rs), [`tests/protocols/lsp-workspace-symbol/README.md`](../../tests/protocols/lsp-workspace-symbol/README.md), [`docs/status/IDE-2311-IMPLEMENTATION-REPORT.md`](../status/IDE-2311-IMPLEMENTATION-REPORT.md)
- Notes: The Preview publishes no Semantic ID or host path and does not claim dependency, generated, temporary, builtin, Prelude, filesystem-discovered, fuzzy, normalized, persistent, partial-result, resolve, work-done, stdio cancellation, Semantic Transaction, or Stable behavior.

### `PROTO-HUMAN-OUTPUT` — Human-readable CLI and diagnostic output

- Producer: ling CLI; ling-diagnostics human renderer; Ling REPL
- Consumer: humans
- Reader policy: Human output is not a machine-readable input protocol; automation must use the versioned JSON or Audit interfaces and process exit code.
- Writer policy: Public diagnostics always retain Chinese and English and preserve stable codes and meanings. DEC-0254 permits bilingual, Chinese-first, or English-first human order and ANSI color only for human diagnostics on stderr; wording, punctuation, layout, prompts, and optional context may improve.
- Unknown-field policy: Not applicable because human output has no field schema.
- Migration tool: Not applicable; no byte-for-byte compatibility is promised.
- Authority: `DEC-0001`, `DEC-0002`, `DEC-0013`, `DEC-0015`, `DEC-0016`, `DEC-0254`
- Sources: [`Cargo.toml`](../../Cargo.toml), [`docs/decisions/0254-cli-output-policy.md`](../decisions/0254-cli-output-policy.md), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/src/output_policy.rs`](../../crates/ling-cli/src/output_policy.rs)
- Fixtures: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/tests/output_policy.rs`](../../crates/ling-cli/tests/output_policy.rs), [`tests/protocols/cli-output-policy/README.md`](../../tests/protocols/cli-output-policy/README.md)
- Notes: Stable diagnostic code meanings are a compatibility subset; the surrounding human bytes, message order, and ANSI decoration are Preview, non-canonical human presentation.

### `PROTO-CLI-INIT` — Ling project initialization report

- Producer: ling init
- Consumer: shell scripts; project tooling; humans
- Reader policy: Consumers must gate on the exact ling.init/0.1 marker; the current repository provides a writer but no public reader, and unknown core fields are rejected by the schema.
- Writer policy: On success, emit exactly one report with the requested directory operand, fixed template version 1, package coordinates, and the sorted four-file scaffold list; failures remain Diagnostic JSON on stderr.
- Unknown-field policy: Reject unknown core fields; incompatible template or output changes require a new protocol version and accepted decision.
- Migration tool: None; ling.init/0.1 is current-writer-only.
- Authority: `DEC-0038`, `DEC-0254`, `DEC-0255`, `RFC-0002`, `DEC-0003`, `DEC-0013`
- Sources: [`docs/decisions/0038-cli-init-command.md`](../decisions/0038-cli-init-command.md), [`docs/decisions/0255-current-project-initialization-command.md`](../decisions/0255-current-project-initialization-command.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/src/init.rs`](../../crates/ling-cli/src/init.rs)
- Fixtures: [`crates/ling-cli/tests/init.rs`](../../crates/ling-cli/tests/init.rs), [`tests/protocols/init/README.md`](../../tests/protocols/init/README.md), [`schemas/init/0.1/schema.json`](../../schemas/init/0.1/schema.json), [`schemas/init/0.1/valid`](../../schemas/init/0.1/valid), [`schemas/init/0.1/invalid`](../../schemas/init/0.1/invalid)
- Notes: DEC-0255 accepts the existing four-file offline scaffold as complete CLI-1703. The report template version is metadata and does not add an unregistered field to RFC-0002's ling.toml manifest-v1 shape; no .zed or overwrite mode is generated.

### `PROTO-CLI-TEST` — Ling explicit test-file runner report

- Producer: ling test
- Consumer: shell scripts; CI jobs; humans
- Reader policy: Consumers must gate on the exact ling.test/0.1 marker; the current repository provides a writer but no public reader, and unknown core fields are rejected by the schema.
- Writer policy: On discovery success, emit exactly one report with the requested root operand, sorted logical .ling test names, captured stdout, and total/passed/failed counts; compilation and runtime diagnostics remain Diagnostic JSON on stderr.
- Unknown-field policy: Reject unknown core fields; project-test, source-declaration, or report extensions require a new accepted decision and protocol version.
- Migration tool: None; ling.test/0.1 is current-writer-only.
- Authority: `DEC-0039`, `DEC-0254`, `DEC-0256`, `DEC-0003`, `DEC-0013`
- Sources: [`docs/decisions/0039-cli-test-file-runner.md`](../decisions/0039-cli-test-file-runner.md), [`docs/decisions/0256-current-test-command.md`](../decisions/0256-current-test-command.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/src/test_runner.rs`](../../crates/ling-cli/src/test_runner.rs)
- Fixtures: [`crates/ling-cli/tests/test.rs`](../../crates/ling-cli/tests/test.rs), [`schemas/test/0.1/schema.json`](../../schemas/test/0.1/schema.json), [`schemas/test/0.1/valid`](../../schemas/test/0.1/valid), [`schemas/test/0.1/invalid`](../../schemas/test/0.1/invalid), [`tests/protocols/test/README.md`](../../tests/protocols/test/README.md)
- Notes: This remains the explicit standalone report; DEC-0256 composes it with RFC-0025 project smoke-test behavior without merging schemas or defining test syntax, manifest targets, workspaces, filters, assertions, snapshots, property tests, parallelism, or cancellation.

### `PROTO-DIAGNOSTIC-JSON` — Structured bilingual Diagnostic JSON

- Producer: ling-diagnostics JSON renderer; ling CLI --format json; Ling REPL JSON events
- Consumer: CLI integrations; future LSP/editor integrations; test harnesses
- Reader policy: The repository currently provides a writer but no public Diagnostic JSON reader; consumers must gate on the exact schema version.
- Writer policy: Emit schema, stable code, severity, Chinese and English messages, optional UTF-8 byte span and Semantic ID, ordered Facts, and structured Repairs.
- Unknown-field policy: Optional Facts and Repair candidates are compatible extensions; no compatibility promise currently exists for unknown top-level fields.
- Migration tool: None; breaking field changes require a new diagnostic schema with migration guidance, while changed code meaning requires a new code.
- Authority: `DEC-0001`, `DEC-0002`
- Sources: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/governance/error-code-lock.toml`](../governance/error-code-lock.toml), [`tools/xtask/src/error_codes.rs`](../../tools/xtask/src/error_codes.rs), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/diagnostic/0.1/schema.json`](../../schemas/diagnostic/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`tests/conformance/m2-invalid-number/expect.toml`](../../tests/conformance/m2-invalid-number/expect.toml), [`docs/governance/error-code-lock.toml`](../governance/error-code-lock.toml), [`tools/xtask/src/error_codes.rs`](../../tools/xtask/src/error_codes.rs), [`schemas/diagnostic/0.1/schema.json`](../../schemas/diagnostic/0.1/schema.json), [`schemas/diagnostic/0.1/valid`](../../schemas/diagnostic/0.1/valid), [`schemas/diagnostic/0.1/invalid`](../../schemas/diagnostic/0.1/invalid)
- Notes: Code meaning, error/warning classification, and existing Facts types are the documented stable subset; the 0.1 container remains Preview until 1.0 gates close; The Markdown registry is the sole handwritten allocation source; the generated lock and offline checker reject drift, retired reuse, and unregistered implementation/test codes.

### `PROTO-FORMAT-CLI` — Ling formatter CLI and report

- Producer: ling fmt
- Consumer: shell scripts; CI jobs; formatter integrations
- Reader policy: Consumers must gate on the exact ling.format/0.1 marker; no standalone reader is provided and unknown core fields are rejected by the schema.
- Writer policy: Emit exactly one report object in JSON mode with source, check, changed, disposition, and optional formatted text or diagnostics; human mode writes only formatted Author Source bytes when not checking.
- Unknown-field policy: Reject unknown core fields; no extension namespace or compatibility promise exists for this Preview schema.
- Migration tool: None; an incompatible report or write-in-place behavior requires a new schema and accepted decision.
- Authority: `DEC-0003`, `DEC-0023`, `DEC-0028`
- Sources: [`docs/decisions/0028-formatter-cli-contract.md`](../decisions/0028-formatter-cli-contract.md), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-format/src/author.rs`](../../crates/ling-format/src/author.rs), [`schemas/format/0.1/schema.json`](../../schemas/format/0.1/schema.json)
- Fixtures: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`schemas/format/0.1/schema.json`](../../schemas/format/0.1/schema.json), [`schemas/format/0.1/valid`](../../schemas/format/0.1/valid), [`schemas/format/0.1/invalid`](../../schemas/format/0.1/invalid)
- Notes: Preview, current-writer-only stdout contract; it does not claim in-place writing, range formatting, format-on-save, LSP Workspace Edits, or Semantic Transactions.

### `PROTO-LOCKFILE` — Ling dependency lockfile

- Producer: ling-project lock writer
- Consumer: Ling offline package and build tooling
- Reader policy: Accept only byte-valid canonical ling.lock/1 with exact required fields, identities, ordering, reachability, and acyclic references; reject unknown fields and every incompatible format without guessing.
- Writer policy: Emit compact UTF-8 JSON with ascending object keys, canonical package/dependency order, and exactly one trailing LF only after the complete local graph validates; unchanged locks are not rewritten.
- Unknown-field policy: ling.lock/1 rejects every unknown field.
- Migration tool: An incompatible lock change uses a new format value and explicit migration; no legacy Ling lock exists.
- Authority: `RFC-0002`
- Sources: [`crates/ling-project/src/lockfile.rs`](../../crates/ling-project/src/lockfile.rs), [`docs/RFC-0002.md`](../RFC-0002.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/lock/1/schema.json`](../../schemas/lock/1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`schemas/lock/1/valid/basic.json`](../../schemas/lock/1/valid/basic.json), [`schemas/lock/1/canonical/basic.bin`](../../schemas/lock/1/canonical/basic.bin), [`schemas/lock/1/invalid/whitespace.json`](../../schemas/lock/1/invalid/whitespace.json), [`crates/ling-project/tests/lockfile_fixtures.rs`](../../crates/ling-project/tests/lockfile_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/path-dependency/expected.ling.lock`](../../tests/projects/path-dependency/expected.ling.lock), [`tests/projects/offline-lock/ling.lock`](../../tests/projects/offline-lock/ling.lock)
- Notes: PRJ-1105 implements the library reader, writer, Update/Locked policy, local-only offline guarantee, and corruption corpus. PRJ-1106 adds end-to-end update, failure-atomicity, and checked-in offline-lock fixtures. PRJ-1108 adds generated model/canonical-byte round trips and enumeration-invariant lock evidence. Accepted DEC-0228 keeps publication, installation, and registry distribution Unsupported through Ling 1.0.

### `PROTO-PACKAGE-SEMANTIC-GRAPH-JSON` — Package-aware Semantic Graph JSON

- Producer: ling-semantic package snapshot writer
- Consumer: ling-semantic package-aware isolated reader; future project IDE and build integrations
- Reader policy: Require the exact 0.2, language, and Unicode versions; validate package graph/root identities, package-local module coordinates, IDs, ownership, imports, and cross-package references; decoded data cannot enter evaluation.
- Writer policy: Emit deterministic package-aware JSON only from checked Typed Core produced by the exact resolved PackageGraph; include path-free PackageIdentity coordinates and use v2 Semantic ID domains without changing file-mode 0.1 bytes.
- Unknown-field policy: Accept x-* extension fields at checked object levels and reject unknown core fields.
- Migration tool: None; this is a context-specific package protocol, not a silent replacement or migration claim for file-mode ling.semantic/0.1.
- Authority: `RFC-0002`, `DEC-0012`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/semantic/0.2/schema.json`](../../schemas/semantic/0.2/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-semantic/tests/project_snapshot.rs`](../../crates/ling-semantic/tests/project_snapshot.rs), [`tests/projects/resolution-v1/valid-cross-package`](../../tests/projects/resolution-v1/valid-cross-package), [`schemas/semantic/0.2/schema.json`](../../schemas/semantic/0.2/schema.json), [`schemas/semantic/0.2/valid`](../../schemas/semantic/0.2/valid), [`schemas/semantic/0.2/invalid`](../../schemas/semantic/0.2/invalid), [`schemas/semantic/0.2/canonical`](../../schemas/semantic/0.2/canonical)
- Notes: File-oriented Seed commands remain on ling.semantic/0.1; PRJ-1107 must explicitly select project mode before any CLI can emit this protocol.; No package-aware Audit Source is claimed because accepted ling.audit/0.1 and ling.audit/0.2 have no package coordinate model.

### `PROTO-REPL-JSON` — REPL submission event JSON

- Producer: ling repl --format json
- Consumer: scripted REPL clients; test harnesses
- Reader policy: The repository provides no standalone reader; consumers must gate on the exact schema and interpret each line as one submission or console event.
- Writer policy: Emit one JSON object per line with status, committed, submission, and status-specific value/type/effect/capability/diagnostic/console data; never mix raw Console text into JSON mode.
- Unknown-field policy: No unknown-field compatibility is promised for 0.1; consumers must not infer semantics from unrecognized fields.
- Migration tool: None; incompatible event changes require a new schema and migration notes.
- Authority: `DEC-0016`
- Sources: [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`docs/decisions/0016-repl-session-semantics.md`](../decisions/0016-repl-session-semantics.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/repl/0.1/schema.json`](../../schemas/repl/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs), [`schemas/repl/0.1/schema.json`](../../schemas/repl/0.1/schema.json), [`schemas/repl/0.1/valid`](../../schemas/repl/0.1/valid), [`schemas/repl/0.1/invalid`](../../schemas/repl/0.1/invalid)
- Notes: Deterministic scripted output is tested, but byte-canonical JSON and an N-1 reader are not claimed.

### `PROTO-SEMANTIC-GRAPH-JSON` — Semantic Graph JSON

- Producer: ling-semantic snapshot writer; ling semantic
- Consumer: ling-semantic isolated reader; ling audit projection; AI and editor tooling experiments
- Reader policy: Require the exact semantic, language, and Unicode versions; validate IDs, kinds, ownership, references, Prelude invariants, and ordering-independent structure; the returned graph is data only and cannot enter evaluation.
- Writer policy: Emit deterministic JSON from checked Typed Core with canonical ordering and no source paths, hash-map order, arena indices, allocation addresses, or Rust debug data in identity.
- Unknown-field policy: Accept x-* extension fields at checked object levels and reject unknown core fields.
- Migration tool: None; schema or identity changes require an explicit version upgrade, migration notes, and regenerated fixtures.
- Authority: `DEC-0012`, `RFC-0022`
- Sources: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/RFC-0022.md`](../RFC-0022.md), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`schemas/registry.toml`](../../schemas/registry.toml), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`tools/xtask/src/schema.rs`](../../tools/xtask/src/schema.rs)
- Fixtures: [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`schemas/semantic/0.1/schema.json`](../../schemas/semantic/0.1/schema.json), [`schemas/semantic/0.1/valid`](../../schemas/semantic/0.1/valid), [`schemas/semantic/0.1/invalid`](../../schemas/semantic/0.1/invalid), [`schemas/semantic/0.1/canonical`](../../schemas/semantic/0.1/canonical)
- Notes: GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001 keeps Stable versus Experimental fields and cross-version migration open.; RFC-0022 defines the optional Experimental x-ling-trait-ide witness/member projection; it does not add a core field or an LSP wire method.

### `PROTO-SEMANTIC-QUERY` — Semantic Query response

- Producer: ling query
- Consumer: CLI users and semantic tooling experiments
- Reader policy: No public reader is implemented; consumers must validate and gate on the exact 0.1 schema marker.
- Writer policy: Emit one deterministic path-free exact-NFC definition projection from the checked canonical Semantic Graph.
- Unknown-field policy: Reject unknown core fields; version 0.1 defines no extension namespace.
- Migration tool: None; no predecessor exists and incompatible changes require a new version.
- Authority: `RFC-0027`
- Sources: [`docs/RFC-0027.md`](../RFC-0027.md), [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`schemas/registry.toml`](../../schemas/registry.toml)
- Fixtures: [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`crates/ling-cli/tests/semantic_commands.rs`](../../crates/ling-cli/tests/semantic_commands.rs), [`tests/protocols/semantic-query/README.md`](../../tests/protocols/semantic-query/README.md), [`schemas/semantic-query/0.1/schema.json`](../../schemas/semantic-query/0.1/schema.json), [`schemas/semantic-query/0.1/valid`](../../schemas/semantic-query/0.1/valid), [`schemas/semantic-query/0.1/invalid`](../../schemas/semantic-query/0.1/invalid)
- Notes: Exact user-definition lookup only; no project, reference, fuzzy, paginated, or general graph-query behavior is implied.

### `PROTO-SEMANTIC-TRANSACTION` — Semantic Transaction proposal request

- Producer: Semantic tooling clients and repository conformance fixtures
- Consumer: ling patch exact-version proposal validator
- Reader policy: Require exact 0.1 schema, fields, canonical target/constraint order, bounds, current Program ID, and checked one-file scope before candidate validation.
- Writer policy: Writers provide one full-source replacement, explicit sorted target IDs, all four preserve constraints, and bounded provenance; the repository provides no general request writer.
- Unknown-field policy: Reject every unknown or duplicate core field.
- Migration tool: None; no predecessor exists and incompatible changes require a new version.
- Authority: `RFC-0027`, `DEC-0012`
- Sources: [`docs/RFC-0027.md`](../RFC-0027.md), [`docs/SEMANTICS.md`](../SEMANTICS.md), [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`schemas/registry.toml`](../../schemas/registry.toml)
- Fixtures: [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`crates/ling-cli/tests/semantic_commands.rs`](../../crates/ling-cli/tests/semantic_commands.rs), [`tests/protocols/semantic-query/README.md`](../../tests/protocols/semantic-query/README.md), [`schemas/semantic-transaction/0.1/schema.json`](../../schemas/semantic-transaction/0.1/schema.json), [`schemas/semantic-transaction/0.1/valid`](../../schemas/semantic-transaction/0.1/valid), [`schemas/semantic-transaction/0.1/invalid`](../../schemas/semantic-transaction/0.1/invalid)
- Notes: This version grants Graph.Read and Graph.Propose only; it cannot write source, emit an edit, or claim Graph.Commit, atomic publication, or LSP WorkspaceEdit.

### `PROTO-SEMANTIC-TRANSACTION-RESULT` — Semantic Transaction proposal result

- Producer: ling patch
- Consumer: CLI users and semantic tooling experiments
- Reader policy: No public reader is implemented; consumers must validate and gate on the exact 0.1 schema marker.
- Writer policy: Emit a deterministic path-free validated/not-committed result after stale, target, checked-candidate, preserve, and authorization checks succeed.
- Unknown-field policy: Reject unknown core fields; version 0.1 defines no extension namespace.
- Migration tool: None; no predecessor exists and incompatible changes require a new version.
- Authority: `RFC-0027`, `DEC-0012`
- Sources: [`docs/RFC-0027.md`](../RFC-0027.md), [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`schemas/registry.toml`](../../schemas/registry.toml)
- Fixtures: [`crates/ling-cli/src/semantic_commands.rs`](../../crates/ling-cli/src/semantic_commands.rs), [`crates/ling-cli/tests/semantic_commands.rs`](../../crates/ling-cli/tests/semantic_commands.rs), [`tests/protocols/semantic-query/README.md`](../../tests/protocols/semantic-query/README.md), [`schemas/semantic-transaction-result/0.1/schema.json`](../../schemas/semantic-transaction-result/0.1/schema.json), [`schemas/semantic-transaction-result/0.1/valid`](../../schemas/semantic-transaction-result/0.1/valid), [`schemas/semantic-transaction-result/0.1/invalid`](../../schemas/semantic-transaction-result/0.1/invalid)
- Notes: committed is always false in version 0.1; a commit result requires separate Accepted atomic-publication authority and a new protocol version.

### `PROTO-CANONICAL-BYTES` — Canonical bytes for semantic identities

- Producer: ling-resolve identity encoder; ling-semantic identity encoders
- Consumer: DefinitionId, REPL DefinitionId, BodyId, ProgramId, and semantic node ID hashers in file and project modes
- Reader policy: No general decoder is exposed; each identity class consumes only its own domain-separated, length-prefixed canonical input.
- Writer policy: Use distinct file-mode v1 or package-aware v2 ASCII domains, version inputs, normalized checked semantics, explicit lengths and values, and canonical collection ordering; v2 Definition/node/Program inputs include path-free package or graph identity while all modes exclude spans, host paths, comments, spelling, arena indices, and hash-map iteration.
- Unknown-field policy: Closed binary projection: unrecognized semantic inputs cannot be appended without a domain/schema version change.
- Migration tool: None; an encoding or normalization change requires a Semantic Schema or ID-prefix upgrade and migration explanation.
- Authority: `DEC-0012`, `RFC-0002`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md)
- Fixtures: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs)
- Notes: The domains are versioned separately in current code; no invented umbrella wire identifier is claimed.; The v2 domains are selected only for package-aware ling.semantic/0.2 snapshots; file mode remains byte-stable on v1.

### `PROTO-PACKAGE-IDENTITY` — Ling package content and dependency-graph identities

- Producer: ling-project local dependency resolver
- Consumer: ling-project package graph and lock writer; ling-resolve and ling-semantic package-aware identity; future build planner
- Reader policy: PackageSourceId and PackageGraphId are opaque, distinct Rust types; no general byte-stream decoder is exposed. Text identities emitted by the resolver use exactly sha256: plus 64 lowercase hexadecimal digits.
- Writer policy: Hash RFC-0002's exact unsigned-64-bit big-endian length-prefixed streams with SHA-256 under the separate ling.package-content/1 and ling.package-graph/1 domains; sort every declared collection by its specified canonical key and exclude host paths, cosmetic manifest text, dependency locators, permissions, timestamps, and unordered iteration.
- Unknown-field policy: Closed binary projection: changing included fields, framing, ordering, normalization, or algorithms requires a new domain version and migration evidence.
- Migration tool: None; incompatible identity evolution requires new content and graph domains and must not reuse existing v1 text identities.
- Authority: `RFC-0002`
- Sources: [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`docs/RFC-0002.md`](../RFC-0002.md)
- Fixtures: [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/valid-transitive/ling.toml`](../../tests/projects/dependency-v1/valid-transitive/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml), [`tests/projects/path-dependency/expected-graph.json`](../../tests/projects/path-dependency/expected-graph.json), [`tests/projects/cycle/expected-diagnostics.json`](../../tests/projects/cycle/expected-diagnostics.json), [`tests/projects/unicode-names/expected-graph.json`](../../tests/projects/unicode-names/expected-graph.json)
- Notes: PRJ-1104 implements recursive local path resolution and freezes independent content/graph vectors; PRJ-1103 consumes those identities for cross-package resolution and ling.semantic/0.2; PRJ-1105 projects them into canonical ling.lock/1 bytes; PRJ-1106 freezes the named end-to-end project graph and failure matrix; PRJ-1108 verifies generated cycle/oracle and filesystem-enumeration invariance properties. Accepted DEC-0228 keeps registry/network sources, installation, and publication Unsupported through Ling 1.0.

### `PROTO-SEMANTIC-ID` — Experimental semantic ID text form

- Producer: ling-resolve and ling-semantic BLAKE3 hashers
- Consumer: file-mode and package-aware Semantic Graphs; Audit Source; REPL events; snapshot validation
- Reader policy: Accept exactly the experimental:blake3: prefix followed by 64 lowercase hexadecimal digits in the identity positions allowed by the current schema.
- Writer policy: Hash the appropriate file-mode v1 or package-aware v2 domain-separated canonical bytes and emit lowercase BLAKE3 hexadecimal text with the experimental prefix.
- Unknown-field policy: Not field-based; unknown algorithms, prefixes, lengths, or non-hex text are rejected by current readers.
- Migration tool: None; algorithm, prefix, dependency propagation, or canonical-input changes require an explicit schema/ID upgrade and cannot silently reuse the current prefix.
- Authority: `DEC-0012`, `RFC-0002`
- Sources: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0012-semantic-identity-and-canonical-bytes.md`](../decisions/0012-semantic-identity-and-canonical-bytes.md), [`docs/RFC-0002.md`](../RFC-0002.md)
- Fixtures: [`crates/ling-resolve/src/lib.rs`](../../crates/ling-resolve/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`crates/ling-semantic/tests/project_snapshot.rs`](../../crates/ling-semantic/tests/project_snapshot.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs)
- Notes: GAP-SEMANTIC-HASH-UPGRADE-001 blocks stabilization and migration policy.

### `PROTO-AUDIT-SOURCE` — Canonical Audit Source

- Producer: ling-format Audit renderer; ling audit
- Consumer: ling-format isolated Audit parser; independent audit tooling
- Reader policy: Accept exact ling.audit/0.1 and ling.audit/0.2 headers; 0.1 rejects Handler fields, while 0.2 validates Handler expression identity, byte span, Core body/type, input/eliminated/residual rows, operation labels, and resume mode/use. Parse only into an isolated AuditModel and never convert it to CheckedProgram or evaluator input.
- Writer policy: Emit one BOM-free UTF-8/LF/two-space canonical document with fixed ordering, JSON string escaping, and exactly one final LF. Models without handlers retain ling.audit/0.1 bytes; models with checked handlers emit ling.audit/0.2 Handler blocks.
- Unknown-field policy: Accept and discard x-* extension fields, accept input field reordering, and reject unknown core fields.
- Migration tool: None; 0.1 remains accepted for handler-free models, 0.2 is selected only when checked Handler evidence is present, and future incompatible changes must upgrade ling.audit/*.
- Authority: `DEC-0015`, `DEC-0260`
- Sources: [`crates/ling-format/src/lib.rs`](../../crates/ling-format/src/lib.rs), [`crates/ling-semantic/src/lib.rs`](../../crates/ling-semantic/src/lib.rs), [`docs/decisions/0015-audit-source-format.md`](../decisions/0015-audit-source-format.md), [`docs/decisions/0260-checked-handler-lowering.md`](../decisions/0260-checked-handler-lowering.md)
- Fixtures: [`crates/ling-format/src/lib.rs`](../../crates/ling-format/src/lib.rs), [`crates/ling-cli/tests/conformance.rs`](../../crates/ling-cli/tests/conformance.rs), [`crates/ling-cli/tests/handler_boundary.rs`](../../crates/ling-cli/tests/handler_boundary.rs)
- Notes: Both accepted revisions are Preview and embed Experimental semantic identities; 0.2 is a bounded Handler projection, not executable Core or a package-aware Audit protocol.

### `PROTO-CLI-COMPLETION` — Ling canonical shell completion scripts

- Producer: ling completion
- Consumer: Bash; Zsh; Fish; PowerShell; shell integration tooling
- Reader policy: Select exactly one supported shell operand and consume only the matching script; the embedded ling.cli-completion/0.1 marker identifies this exact current inventory and byte contract.
- Writer policy: Emit the matching BOM-free UTF-8/LF canonical fixture with one final LF, fixed accepted command/option/value ordering, no filesystem or environment discovery, and no stderr on success.
- Unknown-field policy: Not field-based: unsupported shells and extra operands or flags are rejected with usage exit 2; scripts contain no extension namespace.
- Migration tool: None; incompatible command inventory, quoting, registration, or byte changes require a new protocol version and fixtures.
- Authority: `RFC-0028`, `DEC-0003`, `DEC-0040`, `DEC-0253`, `DEC-0254`, `RFC-0025`, `RFC-0027`
- Sources: [`docs/RFC-0028.md`](../RFC-0028.md), [`crates/ling-cli/src/command_catalog.rs`](../../crates/ling-cli/src/command_catalog.rs), [`crates/ling-cli/src/completion.rs`](../../crates/ling-cli/src/completion.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`tests/protocols/cli-completion/ling.bash`](../../tests/protocols/cli-completion/ling.bash), [`tests/protocols/cli-completion/_ling`](../../tests/protocols/cli-completion/_ling), [`tests/protocols/cli-completion/ling.fish`](../../tests/protocols/cli-completion/ling.fish), [`tests/protocols/cli-completion/ling.ps1`](../../tests/protocols/cli-completion/ling.ps1), [`crates/ling-cli/tests/completion.rs`](../../crates/ling-cli/tests/completion.rs)
- Notes: Version 0.1 is static Preview completion only. It does not scan paths/projects/symbols, install shell startup files, localize descriptions, advertise planned commands, or freeze ordinary help bytes.

### `PROTO-BUILD-METADATA` — Ling checked project artifact

- Producer: ling build --profile explore --target semantic
- Consumer: local checked-semantic tooling; future build and IDE integrations
- Reader policy: Consumers must gate on the exact ling.project.artifact/0.1 canonical JSON envelope with graph, explore profile, package-aware ProgramId, embedded ling.semantic/0.2 object, and semantic target; the repository provides the canonical writer but no standalone public artifact reader.
- Writer policy: Emit exact UTF-8 canonical JSON plus LF only after the complete locked project checks; publish with create-new semantics, include no host path or ambient input, and report SHA-256 of the complete bytes.
- Unknown-field policy: Unknown or reordered top-level fields are not accepted as canonical 0.1 bytes; incompatible extensions require a new artifact version.
- Migration tool: None; ling.project.artifact/0.1 has no predecessor and is Experimental.
- Authority: `RFC-0025`, `RFC-0002`, `DEC-0058`, `DEC-0083`
- Sources: [`docs/RFC-0025.md`](../RFC-0025.md), [`crates/ling-cli/src/project.rs`](../../crates/ling-cli/src/project.rs), [`crates/ling-cli/src/main.rs`](../../crates/ling-cli/src/main.rs)
- Fixtures: [`crates/ling-cli/src/project.rs`](../../crates/ling-cli/src/project.rs), [`crates/ling-cli/tests/project_commands.rs`](../../crates/ling-cli/tests/project_commands.rs), [`tests/protocols/project-command/README.md`](../../tests/protocols/project-command/README.md)
- Notes: This is a checked semantic artifact, not executable bytecode, native/Wasm output, a dependency-substitution cache key, a publication package, or a Stable 1.0 format.

### `PROTO-PACKAGE-MANIFEST` — Ling package/project manifest

- Producer: Ling project authors and future project tooling
- Consumer: ling-project manifest, module-discovery, and local dependency-graph readers; future build planner
- Reader policy: ling-project accepts exact UTF-8 ling.toml inputs using TOML 1.0, requires manifest-version = 1, validates the complete RFC-0002 model and limits, preserves original byte spans, discovers deterministic module/import graphs, and recursively resolves only explicitly declared vendored path dependencies beneath each referring package root; it performs no ambient project search.
- Writer policy: A future writer emits only the RFC-0002 version-1 model and never infers environment-dependent defaults; no writer is implemented yet.
- Unknown-field policy: Version 1 rejects every unknown top-level key, table, and field.
- Migration tool: No legacy Ling manifest exists; incompatible evolution requires a new manifest-version and explicit migration.
- Authority: `RFC-0002`, `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`crates/ling-project/src/lib.rs`](../../crates/ling-project/src/lib.rs), [`crates/ling-project/src/discovery.rs`](../../crates/ling-project/src/discovery.rs), [`crates/ling-project/src/package_graph.rs`](../../crates/ling-project/src/package_graph.rs), [`fuzz/fuzz_targets/manifest_bytes.rs`](../../fuzz/fuzz_targets/manifest_bytes.rs), [`crates/ling-diagnostics/src/lib.rs`](../../crates/ling-diagnostics/src/lib.rs), [`docs/ERROR-CODES.md`](../ERROR-CODES.md), [`docs/RFC-0002.md`](../RFC-0002.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: [`crates/ling-project/tests/manifest_fixtures.rs`](../../crates/ling-project/tests/manifest_fixtures.rs), [`crates/ling-project/tests/discovery_fixtures.rs`](../../crates/ling-project/tests/discovery_fixtures.rs), [`crates/ling-project/tests/package_graph_fixtures.rs`](../../crates/ling-project/tests/package_graph_fixtures.rs), [`crates/ling-project/tests/project_fixtures.rs`](../../crates/ling-project/tests/project_fixtures.rs), [`crates/ling-project/tests/project_properties.rs`](../../crates/ling-project/tests/project_properties.rs), [`tests/projects/README.md`](../../tests/projects/README.md), [`tests/projects/manifest-v1/valid-minimal/ling.toml`](../../tests/projects/manifest-v1/valid-minimal/ling.toml), [`tests/projects/manifest-v1/valid-unicode/ling.toml`](../../tests/projects/manifest-v1/valid-unicode/ling.toml), [`tests/projects/discovery-v1/valid-multi-root/ling.toml`](../../tests/projects/discovery-v1/valid-multi-root/ling.toml), [`tests/projects/discovery-v1/import-cycle/ling.toml`](../../tests/projects/discovery-v1/import-cycle/ling.toml), [`tests/projects/dependency-v1/valid-basic/ling.toml`](../../tests/projects/dependency-v1/valid-basic/ling.toml), [`tests/projects/dependency-v1/package-cycle/ling.toml`](../../tests/projects/dependency-v1/package-cycle/ling.toml), [`tests/projects/single-package/ling.toml`](../../tests/projects/single-package/ling.toml), [`tests/projects/multi-module/ling.toml`](../../tests/projects/multi-module/ling.toml), [`tests/projects/path-dependency/ling.toml`](../../tests/projects/path-dependency/ling.toml), [`tests/projects/cycle/ling.toml`](../../tests/projects/cycle/ling.toml), [`tests/projects/visibility/ling.toml`](../../tests/projects/visibility/ling.toml), [`tests/projects/offline-lock/ling.toml`](../../tests/projects/offline-lock/ling.toml), [`tests/projects/unicode-names/ling.toml`](../../tests/projects/unicode-names/ling.toml), [`tests/projects/manifest-v1/duplicate-field/ling.toml`](../../tests/projects/manifest-v1/duplicate-field/ling.toml), [`tests/projects/manifest-v1/path-traversal/ling.toml`](../../tests/projects/manifest-v1/path-traversal/ling.toml), [`tests/projects/manifest-v1/unsupported-language/ling.toml`](../../tests/projects/manifest-v1/unsupported-language/ling.toml), [`fuzz/corpus/manifest_bytes/malformed`](../../fuzz/corpus/manifest_bytes/malformed), [`fuzz/corpus/manifest_bytes/minimal`](../../fuzz/corpus/manifest_bytes/minimal), [`fuzz/corpus/manifest_bytes/path-traversal`](../../fuzz/corpus/manifest_bytes/path-traversal), [`fuzz/corpus/manifest_bytes/unicode`](../../fuzz/corpus/manifest_bytes/unicode)
- Notes: PRJ-1101 through PRJ-1106 plus PRJ-1108 implement the isolated reader/model, explicit-root source discovery, deterministic module/import graphs, recursive vendored dependency traversal, content/package-graph identities, exported-module visibility, checked package-aware resolution, the canonical local lock protocol, the complete named project fixture matrix, generated path/cycle/order properties, and deterministic manifest fuzz coverage. Manifest writing and build integration remain later tasks. Accepted DEC-0228 keeps publication, installation, and registry distribution Unsupported through Ling 1.0.

### `PROTO-BYTECODE` — Portable bytecode and verifier format

- Producer: VM-1202 ling-bytecode checked Typed Core lowerer and deterministic writer; VM-1203 canonical VerifiedProgramV1 re-encoder; VM-1205 closure/recursion lowerer and ling.bytecode/1.1 writer; VM-1206 aggregate/match lowerer and ling.bytecode/1.2 writer; VM-1208 checked Effect/Capability metadata boundary; VM-1209 table-driven Interpreter–VM differential harness; VM-1210 decoder/resource/cancellation evidence
- Consumer: VM-1203 bounded independent decoder/verifier; VM-1204 verifier-gated ling-vm executor; VM-1205 closure/partial-application VM execution; VM-1206 aggregate/match VM execution; VM-1208 explicit host-capability preflight and Runtime Fault boundary; VM-1209 differential oracle over checked snapshots and verified bytecode; VM-1210 robustness and host-control test harness
- Reader policy: The 1.2 reader accepts valid format (1, 0), (1, 1), and (1, 2) artifacts, dispatches on the exact version before decoding version-specific tables or instructions, validates hard and caller artifact bounds before allocation, rejects unknown executable content, and produces only untrusted decoded models. The independent verifier is the sole constructor of VerifiedProgramV1, and ling-vm accepts only that verified state plus explicit limits and injected host Capabilities.
- Writer policy: The library-only writers accept checked-source lowering output or independently verified models. The 1.0 writer emits RFC-0014; the 1.1 writer emits RFC-0015 closure/recursion; the 1.2 writer emits RFC-0016 aggregate/match records and instructions with canonical type, field, case, update, and source-map order, zero reserved bytes, and path-free metadata under hard and caller-supplied limits. No CLI artifact contract is published.
- Unknown-field policy: Each revision rejects unknown tags, opcodes, flags, fields, nonzero reserved bytes, incompatible versions, and trailing bytes. A reader rejects every newer revision and accepts only the explicitly supported earlier records.
- Migration tool: No previous format exists. A future migration must decode and verify the old version before encoding the new version; no tool is implemented.
- Authority: `RFC-0014`, `RFC-0015`, `RFC-0016`, `RFC-0018`, `RFC-0019`, `RFC-0020`, `RFC-0021`, `DEC-0261`, `DEC-0262`
- Sources: [`docs/RFC-0014.md`](../RFC-0014.md), [`docs/RFC-0015.md`](../RFC-0015.md), [`docs/RFC-0016.md`](../RFC-0016.md), [`docs/RFC-0018.md`](../RFC-0018.md), [`docs/RFC-0019.md`](../RFC-0019.md), [`docs/RFC-0020.md`](../RFC-0020.md), [`docs/RFC-0021.md`](../RFC-0021.md), [`docs/decisions/0261-handler-runtime-and-bytecode.md`](../decisions/0261-handler-runtime-and-bytecode.md), [`docs/decisions/0262-handler-cell-state-bytecode.md`](../decisions/0262-handler-cell-state-bytecode.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml), [`crates/ling-bytecode/src/lib.rs`](../../crates/ling-bytecode/src/lib.rs), [`crates/ling-bytecode/src/lower/v1_1.rs`](../../crates/ling-bytecode/src/lower/v1_1.rs), [`crates/ling-vm/src/lib.rs`](../../crates/ling-vm/src/lib.rs), [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/src/fault.rs`](../../crates/ling-vm/src/fault.rs), [`fuzz/fuzz_targets/bytecode_bytes.rs`](../../fuzz/fuzz_targets/bytecode_bytes.rs), [`tests/bytecode/README.md`](../../tests/bytecode/README.md)
- Fixtures: [`tests/bytecode/v1/golden/hello.lbc.hex`](../../tests/bytecode/v1/golden/hello.lbc.hex), [`tests/bytecode/v1/golden/hello.dis`](../../tests/bytecode/v1/golden/hello.dis), [`tests/bytecode/v1/malformed-cases.tsv`](../../tests/bytecode/v1/malformed-cases.tsv), [`crates/ling-bytecode/tests/decode_verify.rs`](../../crates/ling-bytecode/tests/decode_verify.rs), [`crates/ling-bytecode/tests/lowering.rs`](../../crates/ling-bytecode/tests/lowering.rs), [`crates/ling-vm/tests/execution.rs`](../../crates/ling-vm/tests/execution.rs), [`crates/ling-vm/tests/differential.rs`](../../crates/ling-vm/tests/differential.rs)
- Notes: VM-1201 through VM-1204 implement the unverified data model, typed index/digest domains, fixed tags/opcodes/limits, checked-snapshot minimal lowering, deterministic writing, debug disassembly, bounded independent decoding, failure-atomic verification, canonical VerifiedProgramV1 re-encoding, verifier-gated execution, registered bilingual diagnostics, and valid/corrupt/fuzz/differential evidence. The protocol is Experimental, is not a DEC-0012 semantic canonical-byte format, and has no CLI artifact command, default backend, or N-1 compatibility claim.; Accepted RFC-0015 is implemented by VM-1205 as the backward-compatible ling.bytecode/1.1 closure/recursion extension. Both 1.0 and 1.1 remain Experimental; no Stable, CLI artifact, default-backend, or general N-1 release promise is implied.; Accepted RFC-0016 is implemented by VM-1206 as the backward-compatible ling.bytecode/1.2 aggregate and checked-match extension. All revisions remain Experimental; no Stable, CLI artifact, default-backend, or general N-1 release promise is implied.; Accepted RFC-0018 is implemented by VM-1208: Effect closure, explicit Capability preflight, source-mapped L-RUNTIME-0001 host Faults, and host-panic containment use the existing wire revisions. The protocol remains Experimental; VM-1209 differential corpus and VM-1210 fuzz/resource work remain separate.; Accepted RFC-0019 is implemented by VM-1209: the table-driven harness compares checked-interpreter and verifier-created VM logical events, Unit results, stable Fault projections, source spans, committed state, and deterministic ProgramId values.; Accepted RFC-0020 is implemented by VM-1210: the existing bytecode protocol gains bounded decoder/resource/cancellation evidence, while the experimental ling.vm.control/0.1 host API is inventoried separately and makes no wire or CLI promise.; Accepted RFC-0021 is implemented by the checked Trait member lowering slice: selected implementation DefinitionIds reuse existing direct-call instructions and do not add a wire revision or serialized dictionary table.; Accepted DEC-0261 reserves planned ling.bytecode/1.3 for verified first-order Handler execution. The published current revision remains 1.2; an implementation-active 1.3 immutable/irrefutable slice has writer, reader, verifier, VM, malformed-input, resource, cancellation, and differential evidence, but GAP-EFFECT-HANDLER-BYTECODE-001 blocks shared Cell/refutable-pattern completion and any EFF-2104 Done or current-version claim.; Accepted DEC-0262 authorizes a future backward-reading ling.bytecode/1.4 Cell<T>/State<T> representation and irrefutable Handler operation inputs. Revision 1.4 is not yet implemented or current; the published current revision remains 1.2 until the complete conformance slice passes.

### `PROTO-VM-CONTROL` — Experimental VM host control API

- Producer: ling-vm execute_v1_with_cancellation
- Consumer: host orchestration and VM robustness tests
- Reader policy: No wire reader exists; host code links the explicit Rust API and owns token lifetime and cancellation requests.
- Writer policy: No serialized writer exists; cancellation is a host-memory request and is never inferred from source, Capability, wall clock, or thread state.
- Unknown-field policy: Not applicable because the API has no field-based wire schema.
- Migration tool: None; incompatible API changes require a new ling.vm.control version and an accepted specification.
- Authority: `RFC-0020`, `DEC-0013`
- Sources: [`docs/RFC-0020.md`](../RFC-0020.md), [`crates/ling-vm/src/lib.rs`](../../crates/ling-vm/src/lib.rs), [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/src/fault.rs`](../../crates/ling-vm/src/fault.rs)
- Fixtures: [`crates/ling-vm/src/cancel.rs`](../../crates/ling-vm/src/cancel.rs), [`crates/ling-vm/src/execute.rs`](../../crates/ling-vm/src/execute.rs), [`crates/ling-vm/tests/execution.rs`](../../crates/ling-vm/tests/execution.rs), [`fuzz/fuzz_targets/bytecode_bytes.rs`](../../fuzz/fuzz_targets/bytecode_bytes.rs)
- Notes: Experimental host control only: execute_v1 remains non-cancellable, cancellation is cooperative and source-mapped, committed effects remain visible, and structured Task/LSP cancellation is separately unresolved.

### `PROTO-INTERNAL-INCIDENT` — Local internal-incident reproduction report

- Producer: ling-cli internal incident capture
- Consumer: local compiler debugging and incident triage
- Reader policy: No public reader or compatibility contract exists; reports are local debugging data under the OS temporary directory.
- Writer policy: Write versioned pretty JSON containing the incident ID, compiler version, internal stage/detail, and bounded reproduction inputs; expose only a logical report label in public diagnostics.
- Unknown-field policy: Internal-only and unspecified; consumers must not treat fields as a Ling public protocol.
- Migration tool: None; internal reports may evolve with the compiler while keeping public L-INTERNAL facts within their documented compatibility boundary.
- Authority: `DEC-0001`, `DEC-0013`
- Sources: [`crates/ling-cli/src/incident.rs`](../../crates/ling-cli/src/incident.rs)
- Fixtures: [`crates/ling-cli/src/incident.rs`](../../crates/ling-cli/src/incident.rs)
- Notes: This record prevents a versioned implementation artifact from being mistaken for a public 1.x commitment; it is not the Future evidence-bundle protocol.

### `PROTO-REPLAY` — Deterministic replay log

- Producer: Future effect/runtime recorder
- Consumer: Future replay verifier and debugging tooling
- Reader policy: Not defined; no replay decoder or equivalence verifier exists.
- Writer policy: Not defined; recorded effects, ordering, redaction, corruption handling, and divergence semantics remain unresolved.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: Blocked by GAP-DETERMINISTIC-REPLAY-001.

### `PROTO-ABI` — Native/FFI binary ABI

- Producer: Future native backend and target packages
- Consumer: Future linker, foreign interfaces, and deployment tooling
- Reader policy: Not defined; no public ABI decoder, verifier, or compatibility checker exists.
- Writer policy: Not defined; layouts, calling convention, ownership transfer, exceptions/Faults, target identity, and symbol versioning require accepted RFCs.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `LANGUAGE`, `ROADMAP-1.0`
- Sources: [`docs/LANGUAGE.md`](../LANGUAGE.md), [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md)
- Fixtures: —
- Notes: No Rust ABI, allocation detail, or host calling convention is exposed as Ling semantics.

### `PROTO-EVIDENCE` — Critical/release evidence bundle

- Producer: Future build, test, proof, and release evidence pipeline
- Consumer: Future independent evidence verifier and Critical/release tooling
- Reader policy: Not defined; existing Markdown release reports are project records, not a versioned Ling evidence bundle.
- Writer policy: Not defined; identity, provenance, checksums, signatures, proof/test linkage, redaction, and verification rules require accepted specifications.
- Unknown-field policy: Not defined.
- Migration tool: Not defined.
- Authority: `ROADMAP-1.0`, `GAP-REGISTER`
- Sources: [`docs/ROADMAP-1.0.md`](../ROADMAP-1.0.md), [`docs/governance/gap-register.toml`](../governance/gap-register.toml)
- Fixtures: —
- Notes: The versioned internal incident report is not this future public evidence-bundle protocol.

## Machine source

The machine-readable source is [`protocol-inventory.toml`](protocol-inventory.toml). Run `cargo xtask governance check-protocols` to reject duplicate or missing required records, unversioned implemented/public schemas, invalid stability claims, Preview/Stable protocols without Accepted authority, Stable protocols without fixtures, missing paths/version markers, and generated-report drift.
