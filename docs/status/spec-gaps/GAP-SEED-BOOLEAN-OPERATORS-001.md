# GAP-SEED-BOOLEAN-OPERATORS-001: Seed boolean operator boundary

> Status when discovered: Open (the machine registry is authoritative)
> Discovered by: TS-3101
> Date: 2026-08-20
> Type: Language syntax and evaluation
> Machine record: [`docs/governance/gap-register.toml`](../../governance/gap-register.toml) under the same ID; this file preserves discovery evidence and is not a second status authority

## Trigger

[Draft SEMANTICS §8.3](../../SEMANTICS.md) specifies short-circuit `a && b` and `a || b`. [Draft RFC-0001 §8](../../RFC-0001.md) omits boolean operators from the Seed expression precedence grammar. The compiler lexer recognizes `&&` and `||`, but the parser has no production for either token and rejects them.

The execution plan's TS-3105 task expects boolean precedence coverage. A Tree-sitter grammar cannot choose whether these operators are valid, where they bind, or how they lower.

## Missing decision

An Accepted syntax decision must choose one of these boundaries:

1. include `&&` and `||` in v0.1, define their precedence and associativity, specify short-circuit Typed Core lowering and evaluation order, and add compiler/Tree-sitter differential evidence; or
2. explicitly defer them beyond v0.1, retain deterministic syntax errors, and correct the Draft semantics and execution-plan expectations.

This audit does not choose either option.

## Observable impact

Accepting the operators changes which source files parse and how mixed operator expressions group. Their short-circuit behavior changes whether the right operand is evaluated and which Effects or Faults are observable. Deferring them preserves current compiler behavior but requires documentation and editor grammar to avoid implying support.

## Work boundary

- TS-3102 may build the bounded Seed skeleton and must treat `&&`/`||` as finite error input.
- TS-3105 is blocked from adding boolean precedence until an Accepted decision resolves this gap.
- The compiler lexer tokens are implementation evidence only; they do not authorize a parser, AST, Typed Core, evaluator, formatter, or editor feature.
- No snapshot or Tree-sitter corpus may be used to decide the language behavior.

## Required evidence for resolution

- positive and negative syntax cases;
- every neighboring precedence pair and associativity case;
- short-circuit Effect/Fault behavior;
- compiler parser ↔ Tree-sitter differential cases;
- source migration impact for the rejected alternative.

## Resolution

Accepted [`DEC-0017`](../../decisions/0017-seed-boolean-operators.md) includes `&&` and `||` in Seed, fixes `||` below `&&` below equality and above pipeline, requires `Bool` operands/results, and defines checked left-to-right short-circuit behavior. The machine gap registry records the Accepted state; TS-3105 owns the implementation and conformance evidence.
