# PKG-6403-DEFERMENT Implementation Report

## Scope

Implemented Accepted `DEC-0228` by making registry deferment through Ling 1.0
an explicit support decision, proving the absence of a package-registry
protocol, preserving the exact Experimental local package protocols, and
adding a sixty-category test-local deferment inventory.

## Verification

- `cargo test -p xtask --bin xtask registry_deferment --locked --offline`
- `cargo test -p ling-types --test registry_deferment_evidence --locked --offline`
- `cargo xtask support verify`
- `cargo xtask governance check-protocols`
- `cargo clippy -p ling-types -p xtask --all-targets --locked --offline -- -D warnings`
- `cargo xtask governance check-all`
- `cargo xtask status verify`

## Compatibility and deferral

The support roadmap now explicitly defers registry behavior through 1.0.
Manifest, lock, package identity, resolver, compiler, diagnostic, CLI,
Semantic ID, span, dependency, Unicode, and protocol stability behavior remain
unchanged. Public `PKG-6403` remains `BlockedSpec`.
