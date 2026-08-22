# `ling.init/0.1` protocol fixture

The `ling init --format json` success report is the current-writer-only
`ling.init/0.1` protocol. The executable conformance tests in
`crates/ling-cli/tests/init.rs` create a fresh destination, verify the exact
generated scaffold and lock, and check rejection of existing destinations and
invalid package names.
