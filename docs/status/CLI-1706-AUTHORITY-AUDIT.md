# CLI-1706 Authority Audit: Shell completion and help fixtures

## Outcome

The original `BlockedSpec` finding is now closed. CLI-1701 through CLI-1705
established the accepted current command, output, project, formatter, test, and
semantic-tooling surfaces. Accepted RFC-0028 now defines the remaining bounded
shell-completion contract and authorizes implementation of CLI-1706.

The accepted scope is deliberately static: `ling completion <shell>` emits one
canonical script for Bash, Zsh, Fish, or PowerShell from the implemented parser
inventory. Ordinary help remains truthful under DEC-0040 but is not a canonical
byte protocol.

## Authority closure

- DEC-0003 authorizes the hand-written parser boundary and no placeholder
  commands.
- DEC-0040 authorizes semantic help coverage without freezing help bytes.
- DEC-0253 and DEC-0254 accept the current command model, option policy, and
  exit composition.
- RFC-0025 accepts project `run`/`check`/`test`/`build`; RFC-0027 accepts
  `query` and proposal-only `patch`.
- RFC-0028 accepts `ling.cli-completion/0.1`, its four operands, exact static
  inventory, shell quoting/registration boundaries, canonical UTF-8/LF bytes,
  usage/host-output exits, fixtures, and nonmutation rules.

These authorities cover every item the original audit required: command and
flag inventory, Preview lifecycle, help compatibility boundary, deterministic
ordering, shell-specific output, unsupported/deprecated exclusion, exit
behavior, and conformance evidence.

## Current implementation evidence

- `crates/ling-cli/src/command_catalog.rs` lists every implemented command once
  and now includes `completion`.
- `crates/ling-cli/src/completion.rs` owns one static inventory and renders all
  four scripts without reading ambient state.
- `crates/ling-cli/tests/completion.rs` compares independent process output to
  exact fixtures, verifies repeatability and invalid usage, and invokes every
  corresponding shell parser available on the host.
- `crates/ling-cli/tests/help.rs` verifies both help aliases advertise
  `completion` and continue to reject planned/stale command names.
- `PROTO-CLI-COMPLETION` and its non-JSON canonical boundary register the
  public `ling.cli-completion/0.1` protocol.

## Compatibility and residual boundary

The closure adds one Preview CLI root and four canonical text artifacts. It
adds no Ling syntax or semantics, JSON schema, diagnostic code, Semantic ID,
source span, runtime behavior, bytecode, VM, ABI, filesystem mutation, network
access, or Unicode-version change.

Dynamic path/package/module/symbol completion, startup-file installation,
localized descriptions, plugins, future command discovery, and Stable 1.x
compatibility remain explicitly outside CLI-1706 rather than hidden partial
implementations.
