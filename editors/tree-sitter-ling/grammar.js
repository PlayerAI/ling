/**
 * @file Tree-sitter grammar for the Ling programming language
 * @license Apache-2.0
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  ASSIGNMENT: 1,
  PIPELINE: 2,
  BOOLEAN_OR: 3,
  BOOLEAN_AND: 4,
  EQUALITY: 5,
  COMPARISON: 6,
  ADDITIVE: 7,
  MULTIPLICATIVE: 8,
  APPLICATION: 9,
  PROJECTION: 10,
  UNARY: 11,
  TYPE_FUNCTION: 1,
  TYPE_PRODUCT: 2,
  TYPE_APPLICATION: 3,
};

const { IDENTIFIER_PATTERN } = require("./src/unicode-identifiers.js");

function binaryOperation($, precedence, left, operator, right) {
  return prec.left(
    precedence,
    seq(
      field("left", left),
      field("operator", operator),
      field(
        "right",
        choice(right, seq($._indent, right, $._dedent)),
      ),
    ),
  );
}

module.exports = grammar({
  name: "ling",

  word: ($) => $.identifier,

  externals: ($) => [
    $._newline,
    $._indent,
    $._dedent,
    $._soft_newline,
    $._line_leading_bar,
    $._line_leading_pipeline,
    $.block_comment,
    $._delimiter_open,
    $._delimiter_close,
    $._error_sentinel,
    $._root_declaration_boundary,
  ],

  extras: ($) => [
    /[ \t\f]+/,
    $.line_comment,
    $.doc_comment,
    $.block_comment,
    $._soft_newline,
  ],

  reserved: {
    global: ($) => [
      "let",
      "mutable",
      "rec",
      "and",
      "type",
      "of",
      "match",
      "with",
      "when",
      "if",
      "then",
      "else",
      "true",
      "false",
      "module",
      "import",
      "as",
      "requires",
    ],
  },

  rules: {
    source_file: ($) =>
      seq(optional($._bom), repeat(choice($._newline, $._root_declaration))),

    _bom: (_) => "\uFEFF",

    // The boundary is hidden and consumes only the preceding newline. A small
    // dynamic preference keeps synchronized declarations available in error
    // branches without changing the shape of valid declaration nodes.
    _root_declaration: ($) =>
      choice(
        $._declaration,
        prec.dynamic(1, seq($._root_declaration_boundary, $._declaration)),
      ),

    _and_keyword: (_) => token("and"),

    _declaration: ($) =>
      choice(
        $.module_declaration,
        $.import_declaration,
        $.function_definition,
        $.let_declaration,
        $.type_declaration,
        $._reserved_and_error,
        $._stray_left_arrow_error,
      ),

    // Keep `and` reachable as a terminal for global keyword extraction without
    // accepting recursive binding groups before their language design exists.
    _reserved_and_error: ($) => seq($._and_keyword, $._error_sentinel),

    _stray_left_arrow_error: ($) => seq("<-", $._error_sentinel),

    // Retain one complete declaration after an incomplete binding. The
    // scanner never emits the sentinel; its alias therefore leaves a built-in
    // `MISSING "="` marker instead of accepting the edit as valid source.
    _missing_body_recovery: ($) =>
      seq(
        $._root_declaration_boundary,
        choice(
          $.module_declaration,
          $.import_declaration,
          alias($._complete_function_definition, $.function_definition),
          alias($._complete_let_declaration, $.let_declaration),
          $.type_declaration,
        ),
        alias($._error_sentinel, "="),
      ),

    module_declaration: ($) =>
      seq(
        "module",
        field("name", $.qualified_name),
        optional($.capability_block),
      ),

    capability_block: ($) =>
      seq(
        $._indent,
        repeat($._newline),
        "requires",
        field("capability", $.qualified_name),
        repeat(seq(",", field("capability", $.qualified_name))),
        repeat($._newline),
        $._dedent,
      ),

    import_declaration: ($) =>
      seq(
        "import",
        field("module", $.qualified_name),
        optional(seq("as", field("alias", $.identifier))),
      ),

    qualified_name: ($) =>
      seq($.identifier, repeat(seq(".", $.identifier))),

    _qualified_constructor_name: ($) =>
      seq($.identifier, repeat1(seq(".", $.identifier))),

    let_declaration: ($) =>
      choice(
        $._complete_let_declaration,
        seq(
          "let",
          optional("rec"),
          optional("mutable"),
          field("pattern", $._binding_pattern),
          optional(seq(":", field("type", $._type))),
          $._missing_body_recovery,
        ),
      ),

    _complete_let_declaration: ($) =>
      seq(
        "let",
        optional("rec"),
        optional("mutable"),
        field("pattern", $._binding_pattern),
        optional(seq(":", field("type", $._type))),
        "=",
        field("value", $._body_expression),
      ),

    function_definition: ($) =>
      choice(
        $._complete_function_definition,
        seq(
          "let",
          optional("rec"),
          optional("mutable"),
          field("name", $._binding_pattern),
          repeat1(field("parameter", $._parameter_pattern)),
          optional(seq(":", field("return_type", $._type))),
          $._missing_body_recovery,
        ),
      ),

    _complete_function_definition: ($) =>
      seq(
        "let",
        optional("rec"),
        optional("mutable"),
        field("name", $._binding_pattern),
        repeat1(field("parameter", $._parameter_pattern)),
        optional(seq(":", field("return_type", $._type))),
        "=",
        field("body", $._body_expression),
      ),

    type_declaration: ($) =>
      seq(
        "type",
        field("name", $.identifier),
        optional($.type_parameter_list),
        "=",
        field(
          "body",
          choice(
            $._type_definition,
            seq(
              $._indent,
              repeat($._newline),
              $._type_definition,
              repeat($._newline),
              $._dedent,
            ),
          ),
        ),
      ),

    type_parameter_list: ($) =>
      seq(
        "<",
        $.type_parameter,
        repeat(seq(",", $.type_parameter)),
        ">",
      ),

    type_parameter: ($) => seq("'", field("name", $.identifier)),

    _type_definition: ($) =>
      choice($.record_type, $.variant_type, $.type_alias),

    record_type: ($) =>
      seq(
        "{",
        $._delimiter_open,
        optional($._member_separator),
        $.field_declaration,
        repeat(seq($._member_separator, $.field_declaration)),
        optional($._member_separator),
        $._delimiter_close,
        "}",
      ),

    field_declaration: ($) =>
      seq(
        optional("mutable"),
        field("name", $.identifier),
        ":",
        field("type", $._type),
      ),

    variant_type: ($) =>
      prec.right(
        seq(
          $.variant_case,
          repeat(
            choice(
              $._newline,
              $.variant_case,
              seq($._line_leading_bar, $.variant_case),
            ),
          ),
        ),
      ),

    variant_case: ($) =>
      seq(
        "|",
        field("name", $.identifier),
        optional(seq("of", field("payload", $._type))),
      ),

    type_alias: ($) => field("value", $._type),

    _type: ($) =>
      choice(
        $.function_type,
        $.product_type,
        $.applied_type,
        $.qualified_type,
        $.named_type,
        $.type_variable,
        $.parenthesized_type,
        $.tuple_type,
      ),

    function_type: ($) =>
      prec.right(
        PREC.TYPE_FUNCTION,
        seq(field("parameter", $._type), "->", field("result", $._type)),
      ),

    product_type: ($) =>
      prec.left(
        PREC.TYPE_PRODUCT,
        seq(field("left", $._type), "*", field("right", $._type)),
      ),

    applied_type: ($) =>
      prec(
        PREC.TYPE_APPLICATION,
        seq(
          field("constructor", choice($.qualified_type, $.named_type)),
          $.type_argument_list,
        ),
      ),

    type_argument_list: ($) =>
      seq("<", $._type, repeat(seq(",", $._type)), ">"),

    qualified_type: ($) =>
      seq(
        field("module", $.identifier),
        repeat1(seq(".", field("member", $.identifier))),
      ),

    named_type: ($) => field("name", $.identifier),

    type_variable: ($) => seq("'", field("name", $.identifier)),

    parenthesized_type: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._type,
        $._delimiter_close,
        ")",
      ),

    tuple_type: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._type,
        ",",
        $._type,
        repeat(seq(",", $._type)),
        $._delimiter_close,
        ")",
      ),

    _pattern: ($) => choice($.constructor_pattern, $._atomic_pattern),

    _binding_pattern: ($) => $._atomic_pattern,

    _parameter_pattern: ($) => $._atomic_pattern,

    _atomic_pattern: ($) =>
      choice(
        $.identifier_pattern,
        $.wildcard_pattern,
        $.literal_pattern,
        $.unit_pattern,
        $.parenthesized_pattern,
        $.tuple_pattern,
        $.record_pattern,
      ),

    identifier_pattern: ($) => $.identifier,

    wildcard_pattern: (_) => token(prec(2, "_")),

    literal_pattern: ($) =>
      choice(
        $.integer_literal,
        $.float_literal,
        $.string_literal,
        $.boolean_literal,
      ),

    constructor_pattern: ($) =>
      choice(
        seq(
          field(
            "constructor",
            alias($._qualified_constructor_name, $.qualified_name),
          ),
          repeat(field("argument", $._atomic_pattern)),
        ),
        seq(
          field("constructor", $.identifier),
          repeat1(field("argument", $._atomic_pattern)),
        ),
      ),

    unit_pattern: ($) =>
      seq("(", $._delimiter_open, $._delimiter_close, ")"),

    parenthesized_pattern: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._pattern,
        $._delimiter_close,
        ")",
      ),

    tuple_pattern: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._pattern,
        ",",
        $._pattern,
        repeat(seq(",", $._pattern)),
        $._delimiter_close,
        ")",
      ),

    record_pattern: ($) =>
      seq(
        "{",
        $._delimiter_open,
        optional($._member_separator),
        $.record_pattern_field,
        repeat(seq($._member_separator, $.record_pattern_field)),
        optional($._member_separator),
        $._delimiter_close,
        "}",
      ),

    record_pattern_field: ($) =>
      seq(
        field("name", $.identifier),
        "=",
        field("pattern", $._pattern),
      ),

    block: ($) =>
      prec.right(
        seq(
          repeat($._newline),
          $._indent,
          repeat($._newline),
          $._sequence_element,
          repeat(choice($._newline, $._sequence_element)),
          $._dedent,
        ),
      ),

    _sequence_element: ($) =>
      choice(
        $.function_definition,
        $.let_declaration,
        $._expression,
        $._stray_left_arrow_error,
      ),

    _body_expression: ($) => choice($._expression, $.block),

    _expression: ($) =>
      choice($.assignment_expression, $._pipeline_expression),

    if_expression: ($) =>
      seq(
        "if",
        field("condition", $._expression),
        "then",
        field("consequence", $._body_expression),
        "else",
        field("alternative", $._body_expression),
      ),

    match_expression: ($) =>
      prec.right(
        seq(
          "match",
          field("value", $._expression),
          "with",
          field(
            "cases",
            seq(
              repeat($._newline),
              optional($._line_leading_bar),
              $.match_case,
              repeat(
                choice(
                  $._newline,
                  $.match_case,
                  seq($._line_leading_bar, $.match_case),
                ),
              ),
            ),
          ),
        ),
      ),

    match_case: ($) =>
      seq(
        "|",
        field("pattern", $._pattern),
        optional(seq("when", field("guard", $._expression))),
        "->",
        field("body", $._body_expression),
      ),

    assignment_expression: ($) =>
      prec.right(
        PREC.ASSIGNMENT,
        seq(
          field("left", choice($.name_expression, $.projection_expression)),
          "<-",
          field(
            "right",
            choice(
              $._pipeline_expression,
              seq($._indent, $._pipeline_expression, $._dedent),
            ),
          ),
        ),
      ),

    _pipeline_expression: ($) =>
      choice($.pipeline_expression, $._boolean_or_expression),

    pipeline_expression: ($) =>
      prec.left(
        PREC.PIPELINE,
        seq(
          field(
            "left",
            choice($.pipeline_expression, $._boolean_or_expression),
          ),
          choice("|>", seq($._line_leading_pipeline, "|>")),
          field(
            "right",
            choice(
              $._boolean_or_expression,
              seq($._indent, $._boolean_or_expression, $._dedent),
            ),
          ),
        ),
      ),

    _boolean_or_expression: ($) =>
      choice(
        alias($._boolean_or_operation, $.binary_expression),
        $._boolean_and_expression,
      ),

    _boolean_or_operation: ($) =>
      binaryOperation(
        $,
        PREC.BOOLEAN_OR,
        $._boolean_or_expression,
        "||",
        $._boolean_and_expression,
      ),

    _boolean_and_expression: ($) =>
      choice(
        alias($._boolean_and_operation, $.binary_expression),
        $._equality_expression,
      ),

    _boolean_and_operation: ($) =>
      binaryOperation(
        $,
        PREC.BOOLEAN_AND,
        $._boolean_and_expression,
        "&&",
        $._equality_expression,
      ),

    _equality_expression: ($) =>
      choice(
        alias($._equality_operation, $.binary_expression),
        $._comparison_expression,
      ),

    _equality_operation: ($) =>
      binaryOperation(
        $,
        PREC.EQUALITY,
        $._equality_expression,
        choice("==", "!="),
        $._comparison_expression,
      ),

    _comparison_expression: ($) =>
      choice(
        alias($._comparison_operation, $.binary_expression),
        $._additive_expression,
      ),

    _comparison_operation: ($) =>
      binaryOperation(
        $,
        PREC.COMPARISON,
        $._comparison_expression,
        choice("<", "<=", ">", ">="),
        $._additive_expression,
      ),

    _additive_expression: ($) =>
      choice(
        alias($._additive_operation, $.binary_expression),
        $._multiplicative_expression,
      ),

    _additive_operation: ($) =>
      binaryOperation(
        $,
        PREC.ADDITIVE,
        $._additive_expression,
        choice("+", "-"),
        $._multiplicative_expression,
      ),

    _multiplicative_expression: ($) =>
      choice(
        alias($._multiplicative_operation, $.binary_expression),
        $._application_expression,
      ),

    _multiplicative_operation: ($) =>
      binaryOperation(
        $,
        PREC.MULTIPLICATIVE,
        $._multiplicative_expression,
        choice("*", "/", "%"),
        $._application_expression,
      ),

    _application_expression: ($) =>
      choice($.application_expression, $._projection_expression),

    application_expression: ($) =>
      prec.left(
        PREC.APPLICATION,
        seq(
          field(
            "function",
            choice($.application_expression, $._projection_expression),
          ),
          field("argument", $._application_argument),
        ),
      ),

    _application_argument: ($) => $._argument_projection_expression,

    _argument_projection_expression: ($) =>
      choice(
        alias($._argument_projection_operation, $.projection_expression),
        $._primary_expression,
      ),

    _argument_projection_operation: ($) =>
      prec.left(
        PREC.PROJECTION,
        seq(
          field("value", $._argument_projection_expression),
          ".",
          field("field", $.identifier),
        ),
      ),

    _projection_expression: ($) =>
      choice($.projection_expression, $._unary_expression),

    projection_expression: ($) =>
      prec.left(
        PREC.PROJECTION,
        seq(
          field(
            "value",
            choice($.projection_expression, $._unary_expression),
          ),
          ".",
          field("field", $.identifier),
        ),
      ),

    _unary_expression: ($) =>
      choice($.unary_expression, $._primary_expression),

    unary_expression: ($) =>
      prec.right(
        PREC.UNARY,
        seq(
          field("operator", choice("+", "-")),
          field("operand", $._unary_expression),
        ),
      ),

    _primary_expression: ($) =>
      choice(
        $.name_expression,
        $.literal_expression,
        $.unit_expression,
        $.parenthesized_expression,
        $.tuple_expression,
        $.record_update_expression,
        $.record_expression,
        $.list_expression,
        $.if_expression,
        $.match_expression,
      ),

    name_expression: ($) => field("name", $.identifier),

    literal_expression: ($) =>
      choice(
        $.integer_literal,
        $.float_literal,
        $.string_literal,
        $.boolean_literal,
      ),

    unit_expression: ($) =>
      seq("(", $._delimiter_open, $._delimiter_close, ")"),

    parenthesized_expression: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._expression,
        $._delimiter_close,
        ")",
      ),

    tuple_expression: ($) =>
      seq(
        "(",
        $._delimiter_open,
        $._expression,
        ",",
        $._expression,
        repeat(seq(",", $._expression)),
        $._delimiter_close,
        ")",
      ),

    record_expression: ($) =>
      seq(
        "{",
        $._delimiter_open,
        optional($._member_separator),
        $.record_field,
        repeat(seq($._member_separator, $.record_field)),
        optional($._member_separator),
        $._delimiter_close,
        "}",
      ),

    record_update_expression: ($) =>
      seq(
        "{",
        $._delimiter_open,
        optional($._member_separator),
        field("base", $.name_expression),
        "with",
        optional($._member_separator),
        $.record_field,
        repeat(seq($._member_separator, $.record_field)),
        optional($._member_separator),
        $._delimiter_close,
        "}",
      ),

    record_field: ($) =>
      seq(
        field("name", $.identifier),
        "=",
        field("value", $._expression),
      ),

    list_expression: ($) =>
      seq(
        "[",
        $._delimiter_open,
        optional(
          seq(
            optional($._member_separator),
            $._expression,
            repeat(seq($._member_separator, $._expression)),
            optional($._member_separator),
          ),
        ),
        $._delimiter_close,
        "]",
      ),

    _member_separator: ($) =>
      repeat1(choice(";", $._soft_newline)),

    identifier: (_) => token(new RustRegex(IDENTIFIER_PATTERN)),

    integer_literal: (_) =>
      token(
        choice(
          new RustRegex("(?:0|[1-9](?:[0-9]|_[0-9])*)"),
          new RustRegex("0b[01](?:[01]|_[01])*"),
          new RustRegex("0o[0-7](?:[0-7]|_[0-7])*"),
          new RustRegex("0x[0-9A-Fa-f](?:[0-9A-Fa-f]|_[0-9A-Fa-f])*")
        ),
      ),

    float_literal: (_) =>
      token(
        prec(
          1,
          new RustRegex(
            "(?:0|[1-9](?:[0-9]|_[0-9])*)(?:\\.[0-9](?:[0-9]|_[0-9])*(?:[eE][+-]?[0-9](?:[0-9]|_[0-9])*)?|[eE][+-]?[0-9](?:[0-9]|_[0-9])*)",
          ),
        ),
      ),

    string_literal: ($) =>
      seq(
        '"',
        repeat(
          choice(
            $.escape_sequence,
            token.immediate(new RustRegex('[^"\\\\\\r\\n]+')),
          ),
        ),
        '"',
      ),

    escape_sequence: (_) =>
      token.immediate(
        new RustRegex('\\\\(?:["\\\\nrt0]|u\\{[0-9A-Fa-f]{1,6}\\})'),
      ),

    boolean_literal: (_) => choice("true", "false"),

    doc_comment: (_) => token(prec(2, seq("///", /[^\r\n]*/))),

    line_comment: (_) => token(prec(1, seq("//", /[^\r\n]*/))),

  },
});
