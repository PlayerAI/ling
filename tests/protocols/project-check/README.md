# `ling.project.check/0.1` fixture boundary

The `ling project check --manifest-path <path> --locked` Preview command
validates one explicit RFC-0002 project root using the existing module and
package graph readers. It is always local/offline and never writes a lock,
source, cache, or build artifact.

`crates/ling-cli/tests/project_check.rs` verifies deterministic path-free JSON,
missing-lock rejection without mutation, and invalid command-shape handling.
Semantic compilation, project `run`/`test`/`build`, parent-directory search,
registry/network access, and Stable 1.0 compatibility are intentionally out of
scope.
