# TRAIT-1309-TERMINATION Implementation Report

**Status:** Done (bounded termination-evidence child)  
**Decision:** Accepted `DEC-0068`  
**Implementation:** `crates/ling-types/src/solver.rs`

## Delivered

- Added a finite solver corpus case that runs the same concrete Trait fixture
  under distinct logical source names.
- Compared only the deterministic selected Trait/implementation/member
  projection, proving source evidence does not affect selection.
- Retained the existing active-cycle and depth-64 negative tests without
  changing the accepted RFC-0005/DEC-0026 limit or error categories.

## Verification

```text
cargo test -p ling-types --lib --offline
40 passed; 0 failed
```

No timing, allocation, filesystem-order, cancellation, or public-protocol
assertion is made. Full TRAIT-1309 performance and LSP work remains deferred.
