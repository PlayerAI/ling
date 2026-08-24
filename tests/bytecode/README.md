# Bytecode conformance staging area

This directory contains the conformance corpus for TEST-VM-0001 and VM-1201 through VM-1204. RFC-0014 is the normative bytecode authority.

Current evidence is staged by trust boundary:

- `v1/programs/` runs through the existing checked interpreter and freezes the observable baseline;
- `v1/malformed-cases.tsv` maps all 22 required corrupt-model scenarios to executable VM-1203 mutation vectors and stable reason tags;
- `v1/golden/hello.lbc.hex` freezes the exact VM-1202 deterministic bytes for the first checked lowering slice;
- `v1/golden/hello.dis` freezes the matching non-contract debug disassembly;
- `v1/golden/cell-state-1.4.lbc.hex` freezes the exact DEC-0262 Cell/State 1.4 model bytes;
- `v1/golden/cell-state-1.4.dis` freezes the matching 1.4 debug disassembly;
- `crates/ling-bytecode/tests/decode_verify.rs` independently decodes, verifies, mutates, and exact-round-trips the valid artifacts without using VM execution;
- `fuzz/corpus/bytecode_bytes/` supplies reviewable exact valid/corrupt binary seeds to the bounded decoder/verifier fuzz target;
- `crates/ling-vm/tests/execution.rs` executes only independently verified state and covers every version-1.0 scalar operator, the isolated 1.4 Cell primitive, direct and recursive calls, both branch directions, jump/return, Capability preflight, source-mapped Runtime Faults, host commit state, and deterministic execution limits;
- the supported VM-1202 lowering slice is compared directly with the checked interpreter for exact logical Console output.

No file here is executable merely because it is called bytecode. VM-1203 permits only the independent verifier to construct `VerifiedProgramV1`; VM-1204 accepts only that state plus explicit limits and injected host Capabilities. No CLI bytecode command or backend selector is implied.

The corpus preserves original UTF-8 bytes with repository-enforced LF checkout for reviewed text fixtures. Separate tests exercise BOM/CRLF provenance. Physical checkout paths, Rust `Debug` output, allocation details, and hash-map order are never expected values.
