# DEC-0061: Seed type value classification / Seed 类型 Value 分类

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: ownership-design  
> Related authority/gap: `DEC-0008`, `DEC-0009`, `GAP-OWNERSHIP-MODEL-001`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision records the existing v0.0.1 Seed boundary as a read-only type
classification. It does not choose the future Value/Managed/Resource memory
model or authorize ownership, borrowing, Native, or FFI behavior.

## Question

Downstream Seed checks need to distinguish a completed type from an unresolved
or error sentinel without exposing Rust layout or inventing future memory
categories. What classification is authorized while RFC-N301/RFC-0007 remain
absent?

## Decision

1. `ling-types` exposes `SeedTypeClass::Value` as the sole classification
   authorized for completed v0.0.1 Seed type forms.
2. `Type::seed_type_class` returns `Some(Value)` for the existing primitive,
   aggregate, nominal, function, and collection forms. It returns `None` for
   unresolved type variables and the internal error sentinel.
3. The classification describes Seed value semantics only. It does not encode
   copy/move, aliasing, layout, allocation, identity, Drop, Managed, Resource,
   Borrow, lifetime, profile, ABI, FFI, or serialization behavior.
4. The method is an in-process Rust observation. It changes no source syntax,
   Typed Core field, Semantic Graph schema, diagnostic, Semantic ID, runtime,
   bytecode, VM, CLI, protocol, or Unicode behavior.

## Conformance plan

- Verify primitive, tuple, function, nominal, variant, list, and other
  completed Seed forms classify as `Value`.
- Verify unresolved type variables and error sentinels return no class rather
  than silently becoming a future memory kind.
- Verify repeated calls are deterministic and carry no Rust layout, pointer,
  allocator, source-path, or host-state information.
- Keep Managed/Resource, Copy/Move, Borrow, region, Drop, profile, Native, and
  FFI fixtures deferred until RFC-N301/RFC-0007 authority is Accepted.

## Compatibility impact

- Adds only an in-process `ling-types` enum and method over the accepted Seed
  type/value boundary. Existing language, diagnostics, schemas, Semantic IDs,
  source spans, CLI, runtime, bytecode, VM, protocols, and Unicode 17.0.0
  remain unchanged.
- No new diagnostic code or protocol inventory entry is required.

## Unresolved alternatives

The Value/Managed/Resource lattice, kind constraints, Copy/Move/Clone,
ownership and borrowing, region/drop, identity/equality/serialization,
profile/Native/FFI rules, and memory diagnostics remain governed by the blocked
`MEM-3101` parent and `GAP-OWNERSHIP-MODEL-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
