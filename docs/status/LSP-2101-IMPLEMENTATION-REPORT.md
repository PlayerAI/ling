# LSP-2101 Implementation Report: Current lifecycle skeleton

## Result

Accepted and verified the existing RFC-0004 implementation as the complete
bounded LSP-2101 lifecycle skeleton under DEC-0257. No duplicate code was
needed: `ling lsp --stdio`, framed JSON-RPC transport, lifecycle states, server
information, position negotiation, workspace-folder validation, deterministic
errors, and stdout purity are already implemented and tested.

## Normative clauses covered

- RFC-0004 §§1–6: command/channels, framing, lifecycle state machine,
  initialization fields, deterministic errors, and explicit non-claims;
- DEC-0029: position-encoding negotiation and projection ownership;
- DEC-0257: parent composition, later-method isolation, workspace opacity, and
  no duplicate transport/lifecycle implementation;
- RFC-0023 and RFC-0026 only for their independently accepted overlay and
  formatting capability composition; they do not broaden this parent.

## Current implementation evidence

- `crates/ling-lsp/src/lib.rs` owns bounded framing, JSON-RPC validation,
  lifecycle state, initialization metadata, response rendering, and the stdio
  loop.
- `crates/ling-lsp/tests/lifecycle.rs` covers positive, negative, ordering,
  Unicode workspace, size, malformed transport, and deterministic lifecycle
  cases.
- `crates/ling-cli/tests/lsp.rs` exercises the real process and proves framed
  stdout, quiet stderr, negotiated encoding, shutdown response, and exit 0.
- `PROTO-LSP-LIFECYCLE` inventories the Preview/current-writer-only protocol;
  later LSP protocols remain separate records and authorities.

## Verification

Focused evidence executed before the parent acceptance commit:

```text
cargo test -p ling-lsp --all-targets --locked --offline --quiet
cargo test -p ling-cli --test lsp --locked --offline --quiet
```

All lifecycle and CLI LSP tests passed. Repository-wide tests, Clippy, CI,
governance, support, status, RC0, traceability, formatting, and diff gates are
run before commit and again after status binding.

## Compatibility impact

No observable command, frame, method, field, error, channel, exit, source
semantic, diagnostic, schema, Semantic ID, span, runtime, bytecode, VM, ABI,
filesystem, network, determinism, or Unicode 17.0.0 behavior changes. This
milestone adds authority and traceability only.

## Specification gaps encountered

The original audit predated RFC-0004 and the completed CLI-1701 dependency.
DEC-0257 resolves that registry drift. Open transaction, snapshot, edit,
diagnostic, cancellation, and Stable-lifecycle gaps do not block the narrower
execution-plan lifecycle skeleton.

## Intentionally deferred

All LSP document/edit/analysis methods beyond their own accepted slices,
workspace discovery/reload, concurrent request scheduling, cancellation,
Workspace Edits, Semantic Transactions, and Stable editor compatibility remain
outside LSP-2101.
