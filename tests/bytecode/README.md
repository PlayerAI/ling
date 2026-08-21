# Bytecode conformance staging area

This directory contains the conformance corpus for TEST-VM-0001 and VM-1201 through VM-1204. RFC-0014 is the normative bytecode authority.

Current evidence is staged by trust boundary:

- `v1/programs/` runs through the existing checked interpreter and freezes the observable baseline;
- `v1/malformed-cases.tsv` maps all 22 required corrupt-model scenarios to executable VM-1203 mutation vectors and stable reason tags;
- `v1/golden/hello.lbc.hex` freezes the exact VM-1202 deterministic bytes for the first checked lowering slice;
- `v1/golden/hello.dis` freezes the matching non-contract debug disassembly;
- `crates/ling-bytecode/tests/decode_verify.rs` independently decodes, verifies, mutates, and exact-round-trips the valid artifacts without using VM execution;
- `fuzz/corpus/bytecode_bytes/` supplies reviewable exact valid/corrupt binary seeds to the bounded decoder/verifier fuzz target;
- the VM half of the differential test remains explicitly ignored with a VM-1204 reason.

No file here is executable merely because it is called bytecode. VM-1203 permits only the independent verifier to construct `VerifiedProgramV1`; VM-1204 will enable the differential half only through that state.

The corpus preserves original UTF-8 bytes with repository-enforced LF checkout for reviewed text fixtures. Separate tests exercise BOM/CRLF provenance. Physical checkout paths, Rust `Debug` output, allocation details, and hash-map order are never expected values.
