# TS-3102 known differences

This file records deliberate differences between the first Tree-sitter skeleton and the authoritative Ling compiler. None of these differences changes Ling syntax or semantics.

| Area | TS-3102 behavior | Authoritative behavior / owner |
| --- | --- | --- |
| Offside layout | `_indent` atomically recognizes a newline followed by one or more ASCII spaces. It does not compare indentation depths or emit dedents. Single-level blocks are covered. | `ling-syntax` implements DEC-0006. TS-3103 owns a stateful, serializable incremental layout strategy and its ADR. |
| Layout-sensitive shared examples | `../../examples/hello.ling` parses without `ERROR`; `adt-match.ling`, `pipeline.ling`, and `人物.ling` expose nested-dedent or continuation errors under the provisional token. | TS-3103 must remove these layout errors before TS-3108 claims shared-corpus parity. |
| Block comments | The internal lexer token recognizes ordinary non-nested `/* ... */` comments. | DEC-0006 requires nesting through depth 256. TS-3103 must include this in its scanner decision or record a separate bounded implementation. |
| Unicode identifiers | The token uses Tree-sitter CLI Unicode `XID_Start` / `XID_Continue` properties and accepts `_`; it does not perform NFC, mixed-script, confusable, or security checks. | `ling-unicode` remains pinned to Unicode 17.0.0 and authoritative. TS-3104 owns generated-range parity and lexer differential evidence. |
| Reserved `and` | Because `and` has no reachable Seed production, the scanner-free word token can still classify the isolated spelling as an identifier in identifier positions. An `and` binding group remains finite error input. | The compiler reserves `and`; DEC-0012 defers recursive groups. TS-3104 must exclude the exact spelling while retaining identifiers such as `and_then`. |
| Numeric and Text validation | The grammar recognizes the accepted literal shapes and escape forms, but it does not validate finite `f64`, Unicode scalar values, or every invalid-number recovery boundary. | `ling-syntax` diagnostics remain authoritative. TS-3104/TS-3107 own lexical differential and malformed-input coverage. |
| Expressions | The current Seed arithmetic, comparison, equality, application, projection, pipeline, assignment, and unary layers have explicit associativity and a shallow `binary_expression` node. `&&` and `||` are finite error input. | TS-3105 owns exhaustive pairwise precedence evidence and remains blocked by `GAP-SEED-BOOLEAN-OPERATORS-001` for boolean operators. |
| Patterns and types | Major forms are represented. A bare identifier uses syntax-neutral `identifier_pattern`; only a qualified or payload-bearing form becomes `constructor_pattern`. | Name resolution determines binding versus zero-payload constructor. TS-3106 owns complete pattern/type edge coverage. |
| Error recovery | Eight malformed/future cases terminate with an `ERROR` tree and preserve the surrounding source-file parse where covered. | TS-3107 owns systematic incomplete-edit, delimiter, indentation, and randomized edit-state recovery. |
| Compiler differential | No synchronized compiler corpus or differential runner is claimed by TS-3102. | TS-3108 owns deterministic corpus synchronization and compiler/Tree-sitter comparison. |

Tree-sitter node names remain implementation evidence, not public Ling protocol or Semantic ID inputs.
