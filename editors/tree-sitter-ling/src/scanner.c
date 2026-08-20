#include "tree_sitter/alloc.h"
#include "tree_sitter/array.h"
#include "tree_sitter/parser.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

enum TokenType {
  NEWLINE,
  INDENT,
  DEDENT,
  SOFT_NEWLINE,
  LINE_LEADING_BAR,
  LINE_LEADING_PIPELINE,
  BLOCK_COMMENT,
  DELIMITER_OPEN,
  DELIMITER_CLOSE,
  ERROR_SENTINEL,
  ROOT_DECLARATION_BOUNDARY,
};

enum {
  MAX_LAYOUT_DEPTH = 256,
  MAX_DELIMITER_DEPTH = 256,
  MAX_COMMENT_DEPTH = 256,
  SERIALIZATION_VERSION = 2,
};

typedef struct {
  Array(uint16_t) indents;
  uint16_t delimiter_depth;
} Scanner;

typedef struct {
  uint16_t indentation;
  bool eof;
  bool starts_bar;
  bool starts_pipeline;
  bool skipped_comment_line;
} LayoutProbe;

static inline bool is_newline(int32_t character) {
  return character == '\n' || character == '\r';
}

static inline bool is_root_keyword_separator(int32_t character) {
  return character == 0 || character == ' ' || character == '\t' ||
         character == '\f' || character == '/' || is_newline(character);
}

static inline void advance(TSLexer *lexer) {
  lexer->advance(lexer, false);
}

static inline void skip(TSLexer *lexer) {
  lexer->advance(lexer, true);
}

static bool scan_ascii_word(TSLexer *lexer, const char *word) {
  for (const char *character = word; *character != '\0'; ++character) {
    if (lexer->lookahead != *character) {
      return false;
    }
    advance(lexer);
  }
  return is_root_keyword_separator(lexer->lookahead);
}

static bool scan_root_declaration_start(TSLexer *lexer) {
  if (lexer->get_column(lexer) != 0) {
    return false;
  }
  switch (lexer->lookahead) {
  case 'i':
    return scan_ascii_word(lexer, "import");
  case 'l':
    return scan_ascii_word(lexer, "let");
  case 'm':
    return scan_ascii_word(lexer, "module");
  case 't':
    return scan_ascii_word(lexer, "type");
  default:
    return false;
  }
}

static void skip_newline(TSLexer *lexer) {
  if (lexer->lookahead == '\r') {
    skip(lexer);
    if (lexer->lookahead == '\n') {
      skip(lexer);
    }
  } else if (lexer->lookahead == '\n') {
    skip(lexer);
  }
}

static uint16_t read_u16(const char *buffer) {
  return (uint16_t)(uint8_t)buffer[0] |
         (uint16_t)((uint16_t)(uint8_t)buffer[1] << 8);
}

static void write_u16(char *buffer, uint16_t value) {
  buffer[0] = (char)(value & UINT16_C(0xff));
  buffer[1] = (char)(value >> 8);
}

static void reset_scanner(Scanner *scanner) {
  array_clear(&scanner->indents);
  array_push(&scanner->indents, 0);
  scanner->delimiter_depth = 0;
}

static bool scan_block_comment(TSLexer *lexer) {
  if (lexer->lookahead != '/') {
    return false;
  }
  advance(lexer);
  if (lexer->lookahead != '*') {
    return false;
  }
  advance(lexer);

  unsigned depth = 1;
  bool exceeded_depth = false;
  while (!lexer->eof(lexer)) {
    if (lexer->lookahead == '/') {
      advance(lexer);
      if (lexer->lookahead == '*') {
        advance(lexer);
        depth++;
        if (depth > MAX_COMMENT_DEPTH) {
          exceeded_depth = true;
        }
      }
      continue;
    }
    if (lexer->lookahead == '*') {
      advance(lexer);
      if (lexer->lookahead == '/') {
        advance(lexer);
        depth--;
        if (depth == 0) {
          if (exceeded_depth) {
            return false;
          }
          lexer->mark_end(lexer);
          lexer->result_symbol = BLOCK_COMMENT;
          return true;
        }
      }
      continue;
    }
    advance(lexer);
  }
  return false;
}

static uint16_t scan_indentation(TSLexer *lexer) {
  uint32_t indentation = 0;
  while (lexer->lookahead == ' ' || lexer->lookahead == '\t' ||
         lexer->lookahead == '\f') {
    if (lexer->lookahead != '\f' && indentation < UINT16_MAX) {
      indentation++;
    }
    skip(lexer);
  }
  return (uint16_t)indentation;
}

static void skip_horizontal_trivia(TSLexer *lexer) {
  while (lexer->lookahead == ' ' || lexer->lookahead == '\t' ||
         lexer->lookahead == '\f') {
    advance(lexer);
  }
}

static bool probe_block_comment(TSLexer *lexer) {
  if (lexer->lookahead != '*') {
    return false;
  }
  advance(lexer);

  unsigned depth = 1;
  while (!lexer->eof(lexer)) {
    if (lexer->lookahead == '/') {
      advance(lexer);
      if (lexer->lookahead == '*') {
        advance(lexer);
        if (depth < UINT32_MAX) {
          depth++;
        }
      }
      continue;
    }
    if (lexer->lookahead == '*') {
      advance(lexer);
      if (lexer->lookahead == '/') {
        advance(lexer);
        depth--;
        if (depth == 0) {
          return true;
        }
      }
      continue;
    }
    advance(lexer);
  }
  return true;
}

static LayoutProbe probe_layout(TSLexer *lexer, uint16_t indentation) {
  LayoutProbe result = {
      .indentation = indentation,
      .eof = false,
      .starts_bar = false,
      .starts_pipeline = false,
      .skipped_comment_line = false,
  };
  bool saw_comment = false;
  bool can_extend_token = true;

  for (;;) {
    if (lexer->eof(lexer)) {
      result.eof = true;
      result.skipped_comment_line |= saw_comment;
      return result;
    }

    if (is_newline(lexer->lookahead)) {
      result.skipped_comment_line |= saw_comment;
      skip_newline(lexer);
      result.indentation = scan_indentation(lexer);
      saw_comment = false;
      if (can_extend_token) {
        lexer->mark_end(lexer);
      }
      continue;
    }

    if (lexer->lookahead == '/') {
      advance(lexer);
      if (lexer->lookahead == '/') {
        saw_comment = true;
        can_extend_token = false;
        while (!lexer->eof(lexer) && !is_newline(lexer->lookahead)) {
          advance(lexer);
        }
        continue;
      }
      if (probe_block_comment(lexer)) {
        saw_comment = true;
        can_extend_token = false;
        skip_horizontal_trivia(lexer);
        continue;
      }

      result.starts_bar = false;
      return result;
    }

    result.starts_bar = lexer->lookahead == '|';
    if (result.starts_bar) {
      advance(lexer);
      result.starts_pipeline = lexer->lookahead == '>';
    }
    return result;
  }
}

static bool emit_dedent(Scanner *scanner, TSLexer *lexer) {
  if (scanner->indents.size <= 1) {
    return false;
  }
  (void)array_pop(&scanner->indents);
  lexer->result_symbol = DEDENT;
  return true;
}

static bool scan_layout(Scanner *scanner, TSLexer *lexer,
                        const bool *valid_symbols) {
  if (lexer->eof(lexer)) {
    return valid_symbols[DEDENT] && emit_dedent(scanner, lexer);
  }

  if (!is_newline(lexer->lookahead)) {
    if (scanner->delimiter_depth == 0 && valid_symbols[DEDENT] &&
        scanner->indents.size > 1) {
      const uint32_t column = lexer->get_column(lexer);
      const uint16_t current = *array_back(&scanner->indents);
      if (column < current) {
        return emit_dedent(scanner, lexer);
      }
    }
    return false;
  }

  lexer->mark_end(lexer);
  skip_newline(lexer);
  const uint16_t indentation = scan_indentation(lexer);
  lexer->mark_end(lexer);

  // Synchronization is intentionally conservative: the ordinary lexer still
  // owns keywords and Unicode identifiers, while this token ends before them.
  if (indentation == 0 && valid_symbols[ROOT_DECLARATION_BOUNDARY] &&
      scan_root_declaration_start(lexer)) {
    while (scanner->indents.size > 1) {
      (void)array_pop(&scanner->indents);
    }
    scanner->delimiter_depth = 0;
    lexer->result_symbol = ROOT_DECLARATION_BOUNDARY;
    return true;
  }

  if (scanner->delimiter_depth > 0) {
    if (indentation == 0 && scan_root_declaration_start(lexer)) {
      return false;
    }
    if (valid_symbols[SOFT_NEWLINE]) {
      lexer->result_symbol = SOFT_NEWLINE;
      return true;
    }
    return false;
  }

  const uint16_t current = *array_back(&scanner->indents);
  const bool can_close_layout =
      current > 0 && indentation < current && valid_symbols[DEDENT];
  if (indentation == 0 && !can_close_layout &&
      scan_root_declaration_start(lexer)) {
    return false;
  }

  const LayoutProbe probe = probe_layout(lexer, indentation);
  if (probe.skipped_comment_line && valid_symbols[NEWLINE]) {
    lexer->result_symbol = NEWLINE;
    return true;
  }

  if (probe.eof) {
    if (valid_symbols[DEDENT] && current > 0) {
      return emit_dedent(scanner, lexer);
    }
    if (valid_symbols[NEWLINE]) {
      lexer->result_symbol = NEWLINE;
      return true;
    }
    return false;
  }

  const uint16_t next = probe.indentation;
  if (valid_symbols[LINE_LEADING_PIPELINE] && next == current &&
      probe.starts_pipeline) {
    lexer->result_symbol = LINE_LEADING_PIPELINE;
    return true;
  }
  if (valid_symbols[LINE_LEADING_BAR] && next == current &&
      probe.starts_bar && !probe.starts_pipeline) {
    lexer->result_symbol = LINE_LEADING_BAR;
    return true;
  }
  if (next > current) {
    if (valid_symbols[INDENT] && scanner->indents.size < MAX_LAYOUT_DEPTH) {
      array_push(&scanner->indents, next);
      lexer->result_symbol = INDENT;
      return true;
    }
    if (valid_symbols[NEWLINE]) {
      lexer->result_symbol = NEWLINE;
      return true;
    }
    return false;
  }
  if (next < current) {
    if (valid_symbols[DEDENT]) {
      return emit_dedent(scanner, lexer);
    }
    if (valid_symbols[NEWLINE]) {
      lexer->result_symbol = NEWLINE;
      return true;
    }
    return false;
  }
  if (valid_symbols[NEWLINE]) {
    lexer->result_symbol = NEWLINE;
    return true;
  }
  return false;
}

void *tree_sitter_ling_external_scanner_create(void) {
  Scanner *scanner = ts_calloc(1, sizeof(Scanner));
  if (scanner == NULL) {
    return NULL;
  }
  array_init(&scanner->indents);
  reset_scanner(scanner);
  return scanner;
}

void tree_sitter_ling_external_scanner_destroy(void *payload) {
  Scanner *scanner = (Scanner *)payload;
  if (scanner == NULL) {
    return;
  }
  array_delete(&scanner->indents);
  ts_free(scanner);
}

unsigned tree_sitter_ling_external_scanner_serialize(void *payload,
                                                     char *buffer) {
  const Scanner *scanner = (const Scanner *)payload;
  if (scanner == NULL || scanner->indents.size == 0 ||
      scanner->indents.size > MAX_LAYOUT_DEPTH) {
    return 0;
  }

  if (scanner->delimiter_depth > MAX_DELIMITER_DEPTH) {
    return 0;
  }

  const size_t required = 5 + scanner->indents.size * sizeof(uint16_t);
  if (required > TREE_SITTER_SERIALIZATION_BUFFER_SIZE) {
    return 0;
  }

  buffer[0] = SERIALIZATION_VERSION;
  write_u16(&buffer[1], (uint16_t)scanner->indents.size);
  write_u16(&buffer[3], scanner->delimiter_depth);
  size_t offset = 5;
  for (uint32_t index = 0; index < scanner->indents.size; ++index) {
    write_u16(&buffer[offset], *array_get(&scanner->indents, index));
    offset += sizeof(uint16_t);
  }
  return (unsigned)offset;
}

void tree_sitter_ling_external_scanner_deserialize(void *payload,
                                                   const char *buffer,
                                                   unsigned length) {
  Scanner *scanner = (Scanner *)payload;
  reset_scanner(scanner);
  if (buffer == NULL || length < 5 ||
      (uint8_t)buffer[0] != SERIALIZATION_VERSION) {
    return;
  }

  const uint16_t count = read_u16(&buffer[1]);
  const uint16_t delimiter_depth = read_u16(&buffer[3]);
  const size_t required = 5 + (size_t)count * sizeof(uint16_t);
  if (count == 0 || count > MAX_LAYOUT_DEPTH ||
      delimiter_depth > MAX_DELIMITER_DEPTH || required != length) {
    return;
  }

  uint16_t previous = 0;
  for (uint16_t index = 0; index < count; ++index) {
    const uint16_t indent = read_u16(&buffer[5 + index * 2]);
    if ((index == 0 && indent != 0) || (index > 0 && indent <= previous)) {
      return;
    }
    previous = indent;
  }

  array_clear(&scanner->indents);
  array_reserve(&scanner->indents, count);
  scanner->delimiter_depth = delimiter_depth;
  for (uint16_t index = 0; index < count; ++index) {
    array_push(&scanner->indents,
               read_u16(&buffer[5 + index * sizeof(uint16_t)]));
  }
}

bool tree_sitter_ling_external_scanner_scan(void *payload, TSLexer *lexer,
                                            const bool *valid_symbols) {
  Scanner *scanner = (Scanner *)payload;
  if (valid_symbols[ROOT_DECLARATION_BOUNDARY] &&
      is_newline(lexer->lookahead)) {
    return scan_layout(scanner, lexer, valid_symbols);
  }
  if (valid_symbols[ERROR_SENTINEL]) {
    return false;
  }
  if (valid_symbols[DELIMITER_OPEN]) {
    if (scanner->delimiter_depth >= MAX_DELIMITER_DEPTH) {
      return false;
    }
    scanner->delimiter_depth++;
    lexer->result_symbol = DELIMITER_OPEN;
    return true;
  }
  if (valid_symbols[DELIMITER_CLOSE] && scanner->delimiter_depth > 0) {
    while (lexer->lookahead == ' ' || lexer->lookahead == '\t' ||
           lexer->lookahead == '\f') {
      skip(lexer);
    }
    if (lexer->lookahead == ')' || lexer->lookahead == ']' ||
        lexer->lookahead == '}') {
      scanner->delimiter_depth--;
      lexer->result_symbol = DELIMITER_CLOSE;
      return true;
    }
  }
  if (valid_symbols[BLOCK_COMMENT] && lexer->lookahead == '/') {
    return scan_block_comment(lexer);
  }
  return scan_layout(scanner, lexer, valid_symbols);
}
