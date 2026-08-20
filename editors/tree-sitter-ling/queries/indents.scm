; Zed indentation ranges are editing hints over the Experimental CST. They do
; not select an indentation width and do not replace compiler layout checks or
; a formatter. Capturing the containing declaration makes the first offside
; body line part of the range; capturing `block` itself would start too late.

; Offside declaration bodies
(module_declaration) @indent
(type_declaration) @indent
(function_definition) @indent
(let_declaration) @indent

; DEC-0006 keeps each case marker aligned with `match`. Only the arm itself
; introduces another level, so `match_expression` is deliberately not an
; indentation range.
(match_case) @indent

; Each branch has its own range. The consequence ends before `else`, and the
; alternative starts at `else`, keeping that keyword aligned with `if`.
(if_expression
  "else" @end) @indent

(if_expression
  "else" @start) @indent

; Delimiter contents indent until, but not including, their closing token.
(record_type
  "}" @end) @indent

(record_pattern
  "}" @end) @indent

(record_expression
  "}" @end) @indent

(record_update_expression
  "}" @end) @indent

(tuple_type
  ")" @end) @indent

(tuple_pattern
  ")" @end) @indent

(tuple_expression
  ")" @end) @indent

(list_expression
  "]" @end) @indent

; DEC-0004 and DEC-0006 align a line-leading `|>` with the pipeline start.
; Starting the range at the operator indents only a right operand continued on
; a later line; it does not push the operator itself to another level.
(pipeline_expression
  "|>" @start
  right: (_) @end) @indent
