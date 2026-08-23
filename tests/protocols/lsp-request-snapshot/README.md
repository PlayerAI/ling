# LSP request-snapshot evidence

LSP-2501 is an internal analysis boundary composed by current Accepted LSP
protocols. It is not a JSON-RPC method or standalone public protocol.

| Request family | Authority | Executable evidence |
| --- | --- | --- |
| Overlay and incremental publication | RFC-0023, RFC-0029 | `crates/ling-lsp/tests/overlay.rs`, `crates/ling-lsp/tests/incremental_changes.rs` |
| Complete source/project-input capture | DEC-0030, RFC-0030 | `crates/ling-lsp/tests/request_snapshot.rs`, `crates/ling-lsp/tests/workspace_reload.rs` |
| Formatting | RFC-0026 | `crates/ling-lsp/tests/formatting.rs` |
| Push/pull diagnostics | RFC-0032, RFC-0033 | `crates/ling-lsp/tests/push_diagnostics.rs`, `crates/ling-lsp/tests/pull_diagnostics.rs` |
| Document symbols and hover | RFC-0036, RFC-0037 | `crates/ling-lsp/tests/document_symbols.rs`, `crates/ling-lsp/tests/hover.rs` |
| Navigation and references | RFC-0038, RFC-0039 | `crates/ling-lsp/tests/navigation.rs`, `crates/ling-lsp/tests/references.rs` |
| Prepare rename and rename | RFC-0039, RFC-0041 | `crates/ling-lsp/tests/prepare_rename.rs`, `crates/ling-lsp/tests/rename.rs` |
| Completion and resolve | RFC-0042, RFC-0043 | `crates/ling-lsp/tests/completion.rs`, `crates/ling-lsp/tests/completion_resolve.rs` |
| Code actions | RFC-0044 | `crates/ling-lsp/tests/code_action.rs` |
| Workspace symbols | RFC-0045 | `crates/ling-lsp/tests/workspace_symbols.rs` |
| Semantic tokens | RFC-0048 | `crates/ling-lsp/tests/semantic_tokens.rs` |

Every compiler-backed family consumes immutable captured bytes and current
workspace inputs. Long-running or independently compiled work revalidates the
complete snapshot before publication. RFC-0026 formatting is synchronously
bounded by the single-threaded dispatcher. Lifecycle, overlay, and workspace
reload calls negotiate or publish state and are not analysis requests.

Internal VFS/query revisions remain distinct from client document versions and
are never serialized as snapshot identities, Semantic IDs, or cache keys.
