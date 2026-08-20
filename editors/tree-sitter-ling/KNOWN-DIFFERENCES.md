# TS-3103 known differences

This file records deliberate differences between the Tree-sitter editor parser after TS-3103 and the authoritative Ling compiler. None of these differences changes Ling syntax or semantics.

| Area | Tree-sitter behavior after TS-3103 | Authoritative behavior / owner |
| --- | --- | --- |
| Layout diagnostics | The serialized scanner compares relative indentation and recovers finitely, but counts a leading tab as one recovery column and falls back to the nearest lower stack entry for inconsistent dedents. It emits no diagnostic. | `ling-syntax` implements DEC-0006 and remains authoritative for registered bilingual tab, inconsistent-dedent, delimiter, comment, and depth diagnostics. |
| Layout-sensitive shared examples | All four root examples parse without `ERROR` or `MISSING`; this is a focused editor-parser check, not compiler equivalence. | TS-3108 owns synchronized shared-corpus differential evidence before parity can be claimed. |
| Block comments | The external scanner recognizes nested `/* ... */` through depth 256; unclosed and depth-257 inputs terminate with error trees. | Compiler diagnostics, original UTF-8 spans, and validity remain authoritative. TS-3107 owns broader malformed-edit recovery coverage. |
| Unicode identifiers | The token uses Tree-sitter CLI Unicode `XID_Start` / `XID_Continue` properties and accepts `_`; it does not perform NFC, mixed-script, confusable, or security checks. | `ling-unicode` remains pinned to Unicode 17.0.0 and authoritative. TS-3104 owns generated-range parity and lexer differential evidence. |
| Reserved `and` | Because `and` has no reachable Seed production, the scanner-free word token can still classify the isolated spelling as an identifier in identifier positions. An `and` binding group remains finite error input. | The compiler reserves `and`; DEC-0012 defers recursive groups. TS-3104 must exclude the exact spelling while retaining identifiers such as `and_then`. |
| Numeric and Text validation | The grammar recognizes the accepted literal shapes and escape forms, but it does not validate finite `f64`, Unicode scalar values, or every invalid-number recovery boundary. | `ling-syntax` diagnostics remain authoritative. TS-3104/TS-3107 own lexical differential and malformed-input coverage. |
| Expressions | The current Seed arithmetic, comparison, equality, application, projection, pipeline, assignment, and unary layers have explicit associativity and a shallow `binary_expression` node. `&&` and `||` are finite error input. | TS-3105 owns exhaustive pairwise precedence evidence and remains blocked by `GAP-SEED-BOOLEAN-OPERATORS-001` for boolean operators. |
| Patterns and types | Major forms are represented. A bare identifier uses syntax-neutral `identifier_pattern`; only a qualified or payload-bearing form becomes `constructor_pattern`. | Name resolution determines binding versus zero-payload constructor. TS-3106 owns complete pattern/type edge coverage. |
| Error recovery | Covered malformed/future cases, unclosed/over-depth block comments, and scanner-state corruption terminate finitely. | TS-3107 owns systematic incomplete-edit, delimiter, indentation, and randomized edit-state recovery. |
| Compiler differential | No synchronized compiler corpus or differential runner is claimed by TS-3103. | TS-3108 owns deterministic corpus synchronization and compiler/Tree-sitter comparison. |

Tree-sitter node names remain implementation evidence, not public Ling protocol or Semantic ID inputs.
