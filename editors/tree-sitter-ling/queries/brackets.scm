; Structural delimiters
("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)

; Quotes participate in matching, but a string is one logical pair rather
; than another rainbow-bracket nesting level. Escaped quotes are named escape
; nodes, not anonymous quote tokens, and therefore do not match this pattern.
(("\"" @open "\"" @close)
 (#set! rainbow.exclude))
