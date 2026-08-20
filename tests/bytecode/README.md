# Bytecode conformance staging area

This directory contains the failing-first corpus for TEST-VM-0001 and VM-1201 through VM-1204. RFC-0014 is the normative bytecode authority.

Current evidence is intentionally asymmetric:

- `v1/programs/` runs through the existing checked interpreter and freezes the observable baseline;
- `v1/malformed-cases.tsv` freezes verifier scenarios and stable reason tags before a decoder/verifier exists;
- the VM half of the differential test is explicitly ignored with a VM-1204 reason.

No file here is executable merely because it is called bytecode. VM-1202 will add deterministic encoded/disassembly goldens, VM-1203 will add binary valid/corrupt fixtures and an independent verifier, and VM-1204 will enable the differential half only for `VerifiedProgramV1`.

The corpus must preserve original UTF-8 bytes. Physical checkout paths, Rust `Debug` output, allocation details, and hash-map order are never expected values.
