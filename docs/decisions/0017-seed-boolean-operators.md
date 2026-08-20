# DEC-0017: Seed boolean operators and expression precedence / Seed 布尔运算符与表达式优先级

> 状态：Accepted
> 提出日期：2026-08-20
> 决定日期：2026-08-20
> Owner role：language-syntax-design
> 相关 RFC/缺口：RFC-0001 | GAP-SEED-BOOLEAN-OPERATORS-001
> 生命周期记录：`docs/governance/lifecycle.toml`

## Question

Draft `SEMANTICS.md` requires short-circuit `&&` and `||`, while Draft RFC-0001 omits them from the Seed expression grammar. TS-3105 cannot make the editor grammar authoritative for their acceptance, precedence, associativity, type, or evaluation order. The same task also needs one explicit precedence order for the already accepted assignment, pipeline, arithmetic, comparison, equality, application, projection, and unary forms.

## Decision

`&&` and `||` are Seed Author Source binary operators. Both operands and the result have type `Bool`. No keyword aliases (`and` or `or`) and no unary boolean-negation operator are introduced by this decision.

From lowest to highest binding strength, Seed expressions use this order:

1. place assignment `<-`, non-associative at one unparenthesized layer;
2. pipeline `|>`, left-associative;
3. boolean disjunction `||`, left-associative;
4. boolean conjunction `&&`, left-associative;
5. equality `==` and `!=`, left-associative;
6. comparison `<`, `<=`, `>`, and `>=`, left-associative;
7. addition and subtraction `+` and `-`, left-associative;
8. multiplication, division, and remainder `*`, `/`, and `%`, left-associative;
9. whitespace function application, left-associative;
10. member projection `.identifier`, left-associative;
11. prefix numeric `+` and `-`, right-associative;
12. primary expressions.

An unparenthesized `+` or `-` following an expression is an infix operator, so a signed application argument must be parenthesized: `f (-x)`. This preserves the existing Seed parse of `f - x` as subtraction. Repeated comparison or equality syntax groups left; the type checker may reject the resulting operand types.

The compiler preserves each boolean operator and both operand spans through CST and AST lowering. Checked Typed Core uses distinct `BooleanAnd` and `BooleanOr` operators; it does not expose an unchecked evaluator path. Their runtime behavior is:

```text
a && b  ≡  if a then b else false
a || b  ≡  if a then true else b
```

The left operand is evaluated exactly once and first. `false && b` and `true || b` do not evaluate `b`; therefore effects and Faults from the skipped operand are not observed at runtime. Both operands are still resolved and type/effect/capability checked, and static effect rows conservatively include effects from either operand.

Assignment remains restricted to a checked Place on the left. `a <- b <- c` is not accepted as an unparenthesized chain. Parentheses remain the only way to request grouping that differs from the precedence table.

## Conformance plan

- Add positive compiler and Tree-sitter fixtures for every neighboring precedence pair, both boolean associativity cases, and explicit parentheses that reverse each default grouping.
- Add negative fixtures for non-`Bool` operands, missing operands, unsupported `and`/`or` aliases, and unparenthesized assignment chaining; assert registered bilingual diagnostics and original UTF-8 byte spans.
- Add runtime cases proving that each left operand is evaluated once, skipped right operands emit no Console effect and raise no Fault, and required right operands retain left-to-right behavior.
- Add compiler CST/AST versus Tree-sitter CST differential cases for mixed arithmetic, comparison, equality, boolean, pipeline, assignment, application, projection, and unary expressions.
- Verify old valid Seed fixtures retain their prior AST grouping, output, diagnostics, canonical bytes, and Semantic IDs.

## Compatibility impact

- **Source:** sources containing `&&` or `||` move from deterministic parse errors to accepted, typed syntax. Existing valid sources retain their grouping. The rejected deferral alternative therefore needs no valid-source rewrite.
- **CLI and diagnostics:** no command, exit class, diagnostic schema, or error-code meaning changes. Existing parser and type diagnostics cover malformed syntax and non-`Bool` operands.
- **Semantic identity:** existing programs retain their operator discriminants and canonical bytes. New boolean-expression programs include appended operator discriminants under the existing experimental Semantic ID version.
- **Schemas and protocols:** no public schema or protocol version changes. Tree-sitter node names remain Experimental editor CST surface.
- **Unicode and positions:** Unicode remains 17.0.0. Original UTF-8 byte spans and bilingual diagnostics remain authoritative.

## Unresolved alternatives

- Deferring both operators was rejected because it contradicts `SEMANTICS.md` §8.3 and leaves TS-3105 unable to satisfy its boolean-precedence requirement.
- Giving `&&` and `||` equal precedence was rejected because it makes common mixed conditions require avoidable parentheses and differs from the established conjunction-before-disjunction convention.
- Desugaring before type checking was rejected because it would fabricate source-less literals/branches and weaken direct operand-span diagnostics.
- Keyword operators, unary `!`, non-associative comparison syntax, chained assignment, and new boolean overloads remain outside this decision and require separate Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
