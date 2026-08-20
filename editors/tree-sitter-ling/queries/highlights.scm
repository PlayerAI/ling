; Keywords are limited to current lexical tokens with grammar productions or
; an explicit Seed reservation. Boolean literals have their own capture.
[
  "and"
  "as"
  "else"
  "if"
  "import"
  "let"
  "match"
  "module"
  "mutable"
  "of"
  "rec"
  "requires"
  "then"
  "type"
  "when"
  "with"
] @keyword

; Comments
(line_comment) @comment
(block_comment) @comment
(doc_comment) @comment.doc

; Literals
(string_literal) @string
(escape_sequence) @string.escape

[
  (integer_literal)
  (float_literal)
] @number

(boolean_literal) @boolean

; Type declarations and references. Primitive and intrinsic Seed types are
; spelling-based only inside a syntactic type position. Prelude nominal types
; such as Option and Result remain ordinary type references.
(type_declaration
  name: (identifier) @type)

(type_parameter
  name: (identifier) @type)

(type_variable
  name: (identifier) @type)

((named_type
   name: (identifier) @type.builtin)
 (#any-of? @type.builtin "Unit" "Bool" "Int" "f64" "Text" "List"))

((named_type
   name: (identifier) @type)
 (#not-any-of? @type "Unit" "Bool" "Int" "f64" "Text" "List"))

(qualified_type
  module: (identifier) @type)

(qualified_type
  member: (identifier) @type)

; Constructors are captured only where the CST proves the role. A bare name
; expression can be shadowed, so semantic tokens must refine those references.
(variant_case
  name: (identifier) @constructor)

(constructor_pattern
  constructor: (identifier) @constructor)

(constructor_pattern
  constructor: (qualified_name
    (identifier) @constructor .))

; Conservative syntactic variables and record/member properties. More
; specific callable and parameter patterns follow these generic patterns.
(identifier_pattern
  (identifier) @variable)

(name_expression
  name: (identifier) @variable)

(field_declaration
  name: (identifier) @property)

(record_pattern_field
  name: (identifier) @property)

(record_field
  name: (identifier) @property)

(projection_expression
  field: (identifier) @property)

; Calls have a syntactically known callable role. Definitions prefer the
; definition-specific capture, with the broadly supported function fallback
; immediately to its left as required by Zed's right-to-left resolution.
(application_expression
  function: (name_expression
    name: (identifier) @function))

(application_expression
  function: (projection_expression
    field: (identifier) @function))

(function_definition
  name: (identifier_pattern
    (identifier) @function @function.definition))

(function_definition
  parameter: (identifier_pattern
    (identifier) @variable.parameter))

; Operators
[
  "="
  "<-"
  "|>"
  "->"
  "||"
  "&&"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "+"
  "-"
  "*"
  "/"
  "%"
] @operator

; Delimiters and brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(type_parameter_list
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(type_argument_list
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

[
  ","
  ";"
  ":"
  "."
  "|"
  "'"
] @punctuation.delimiter
