# CLI-1706 Implementation Report: Shell completion and help fixtures

## Result

Implemented the complete bounded CLI-1706 surface authorized by Accepted
RFC-0028:

- `ling completion bash` emits the canonical Bash registration script;
- `ling completion zsh` emits the canonical Zsh registration script;
- `ling completion fish` emits canonical Fish `complete` declarations;
- `ling completion powershell` emits one canonical native PowerShell argument
  completer;
- `ling --help` and `ling -h` truthfully advertise the command without making
  ordinary help bytes a compatibility artifact.

All four scripts expose only the currently accepted parser inventory and are
offline, deterministic, BOM-free UTF-8/LF documents with one final LF.

## Normative clauses covered

- RFC-0028 §§1–4: exact command shape, exit/channel behavior, static inventory,
  canonical script bytes, shell registration, nonmutation, and help lifecycle;
- DEC-0003: hand-written parser and no placeholder commands;
- DEC-0040: truthful help semantic coverage without byte freezing;
- DEC-0253 and DEC-0254: one command catalog/dispatcher and preserved output
  policy boundary;
- RFC-0025 and RFC-0027: current project and semantic-tooling commands/options
  included without expanding their behavior.

## Implementation

- `crates/ling-cli/src/completion.rs` owns the supported shells, root commands,
  command-specific options, fixed values, and four isolated renderers.
- `crates/ling-cli/src/command_catalog.rs` includes `completion`; a unit
  invariant compares completion roots with the complete parser catalog,
  reducing inventory drift without introducing a public reflection API.
- `crates/ling-cli/src/main.rs` parses completion before ordinary output-policy
  options, enforces exactly one shell operand, writes protocol bytes directly,
  and maps stdout failures through the existing host-fault path.
- `tests/protocols/cli-completion/` stores the four canonical scripts and a
  bilingual protocol-boundary description.
- Protocol, schema-boundary, support, authority, and lifecycle registries
  identify `ling.cli-completion/0.1` as canonical Preview text.

## Tests and verification

Focused evidence executed successfully before the implementation commit:

```text
cargo test -p ling-cli --bin ling --locked --offline
cargo test -p ling-cli --test completion --test help --locked --offline
cargo clippy -p ling-cli --all-targets --locked --offline -- -D warnings
cargo xtask governance check-all
cargo xtask support verify
```

The process suite verifies exact bytes for all shells, repeated-process
determinism, UTF-8/LF framing, usage failures for missing/unknown/extra/output-
policy operands, empty stdout on failure, and shell syntax where a parser is
installed. PowerShell parsing was exercised on the implementation host; other
shell parsers remain conditional cross-platform evidence rather than an
unverified claim.

Repository-wide tests, Clippy, CI, schema, support, status, RC0, traceability,
rustfmt, and diff checks are executed again immediately before committing and
after task-status binding.

## Compatibility impact

- **CLI:** adds one Preview root and four fixed operands; existing command
  forms and exit meanings are unchanged.
- **Help:** adds the truthful completion form; wording and layout remain
  non-canonical.
- **Diagnostics:** no new code; invalid usage remains exit `2`, and stdout host
  failure remains the registered runtime-host diagnostic and exit `4`.
- **Protocols/schemas:** adds canonical non-JSON text protocol
  `ling.cli-completion/0.1`; no JSON schema or predecessor exists.
- **Semantic IDs/spans/runtime:** no identity, span, compiler, evaluator,
  bytecode, VM, ABI, artifact, cache, or source mutation.
- **Determinism/Unicode:** output uses a fixed ASCII inventory and UTF-8/LF
  framing with no clock/environment/filesystem/map-order input; Unicode remains
  17.0.0 elsewhere.

## Specification gaps encountered

The original audit correctly blocked generation while the parent CLI command
and option contracts were incomplete. Those dependencies are now Done, and
RFC-0028 closes the remaining completion-specific authority. No residual
specification gap blocks the bounded CLI-1706 acceptance criteria.

## Intentionally deferred

Filesystem/path, package/module/symbol, history, daemon, plugin, or LSP-backed
dynamic completion; descriptions/localization; shell installation or startup-
file mutation; completion for future commands; and Stable 1.x compatibility
remain outside this milestone.
