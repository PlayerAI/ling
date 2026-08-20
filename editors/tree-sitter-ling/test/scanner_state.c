#include <assert.h>
#include <stdint.h>
#include <string.h>

#include "../src/scanner.c"

static Scanner *new_scanner(void) {
  Scanner *scanner = tree_sitter_ling_external_scanner_create();
  assert(scanner != NULL);
  return scanner;
}

static bool never_eof(const TSLexer *lexer) {
  (void)lexer;
  return false;
}

typedef struct {
  TSLexer lexer;
  const char *source;
  size_t offset;
  size_t marked_end;
  uint32_t column;
} TestLexer;

static void test_advance(TSLexer *lexer, bool skip) {
  (void)skip;
  TestLexer *test = (TestLexer *)lexer;
  if (test->source[test->offset] == '\0') {
    return;
  }
  if (test->source[test->offset] == '\n' ||
      test->source[test->offset] == '\r') {
    test->column = 0;
  } else {
    test->column += 1;
  }
  test->offset += 1;
  lexer->lookahead = (unsigned char)test->source[test->offset];
}

static void test_mark_end(TSLexer *lexer) {
  TestLexer *test = (TestLexer *)lexer;
  test->marked_end = test->offset;
}

static uint32_t test_get_column(TSLexer *lexer) {
  return ((TestLexer *)lexer)->column;
}

static bool test_eof(const TSLexer *lexer) {
  const TestLexer *test = (const TestLexer *)lexer;
  return test->source[test->offset] == '\0';
}

static TestLexer test_lexer(const char *source) {
  TestLexer test = {
      .lexer =
          {
              .lookahead = (unsigned char)source[0],
              .advance = test_advance,
              .mark_end = test_mark_end,
              .get_column = test_get_column,
              .eof = test_eof,
          },
      .source = source,
  };
  return test;
}

static void assert_root_state(const Scanner *scanner) {
  assert(scanner->indents.size == 1);
  assert(*array_get(&scanner->indents, 0) == 0);
  assert(scanner->delimiter_depth == 0);
}

static void round_trips_complete_state(void) {
  Scanner *source = new_scanner();
  array_push(&source->indents, 2);
  array_push(&source->indents, 9);
  array_push(&source->indents, UINT16_MAX);
  source->delimiter_depth = 17;

  char buffer[TREE_SITTER_SERIALIZATION_BUFFER_SIZE] = {0};
  const unsigned length =
      tree_sitter_ling_external_scanner_serialize(source, buffer);
  assert(length > 0);
  assert(length <= TREE_SITTER_SERIALIZATION_BUFFER_SIZE);

  Scanner *restored = new_scanner();
  tree_sitter_ling_external_scanner_deserialize(restored, buffer, length);
  assert(restored->indents.size == source->indents.size);
  assert(memcmp(restored->indents.contents, source->indents.contents,
                source->indents.size * sizeof(uint16_t)) == 0);
  assert(restored->delimiter_depth == source->delimiter_depth);

  tree_sitter_ling_external_scanner_destroy(restored);
  tree_sitter_ling_external_scanner_destroy(source);
}

static void serializes_the_maximum_delimiter_depth(void) {
  Scanner *source = new_scanner();
  source->delimiter_depth = MAX_DELIMITER_DEPTH;

  char buffer[TREE_SITTER_SERIALIZATION_BUFFER_SIZE] = {0};
  const unsigned length =
      tree_sitter_ling_external_scanner_serialize(source, buffer);
  assert(length > 0);

  Scanner *restored = new_scanner();
  tree_sitter_ling_external_scanner_deserialize(restored, buffer, length);
  assert(restored->delimiter_depth == MAX_DELIMITER_DEPTH);

  tree_sitter_ling_external_scanner_destroy(restored);
  tree_sitter_ling_external_scanner_destroy(source);
}

static void enforces_the_delimiter_depth_boundary(void) {
  Scanner *scanner = new_scanner();
  TSLexer lexer = {.lookahead = '(', .eof = never_eof};
  bool valid_symbols[ROOT_DECLARATION_BOUNDARY + 1] = {false};
  valid_symbols[DELIMITER_OPEN] = true;

  for (uint16_t depth = 0; depth < MAX_DELIMITER_DEPTH; ++depth) {
    assert(tree_sitter_ling_external_scanner_scan(
        scanner, &lexer, valid_symbols));
    assert(lexer.result_symbol == DELIMITER_OPEN);
  }
  assert(scanner->delimiter_depth == MAX_DELIMITER_DEPTH);
  assert(!tree_sitter_ling_external_scanner_scan(scanner, &lexer,
                                                 valid_symbols));

  valid_symbols[DELIMITER_OPEN] = false;
  valid_symbols[DELIMITER_CLOSE] = true;
  lexer.lookahead = ')';
  for (uint16_t depth = MAX_DELIMITER_DEPTH; depth > 0; --depth) {
    assert(tree_sitter_ling_external_scanner_scan(
        scanner, &lexer, valid_symbols));
    assert(lexer.result_symbol == DELIMITER_CLOSE);
  }
  assert(scanner->delimiter_depth == 0);
  assert(!tree_sitter_ling_external_scanner_scan(scanner, &lexer,
                                                 valid_symbols));

  tree_sitter_ling_external_scanner_destroy(scanner);
}

static void serializes_the_maximum_layout_depth(void) {
  Scanner *source = new_scanner();
  for (uint16_t indent = 1; indent < MAX_LAYOUT_DEPTH; ++indent) {
    array_push(&source->indents, indent);
  }

  char buffer[TREE_SITTER_SERIALIZATION_BUFFER_SIZE] = {0};
  const unsigned length =
      tree_sitter_ling_external_scanner_serialize(source, buffer);
  assert(source->indents.size == MAX_LAYOUT_DEPTH);
  assert(length <= TREE_SITTER_SERIALIZATION_BUFFER_SIZE);

  Scanner *restored = new_scanner();
  tree_sitter_ling_external_scanner_deserialize(restored, buffer, length);
  assert(restored->indents.size == MAX_LAYOUT_DEPTH);
  assert(*array_back(&restored->indents) == MAX_LAYOUT_DEPTH - 1);

  tree_sitter_ling_external_scanner_destroy(restored);
  tree_sitter_ling_external_scanner_destroy(source);
}

static void rejects_corrupt_or_non_monotonic_state(void) {
  Scanner *scanner = new_scanner();
  const char bad_version[] = {99, 1, 0, 0, 0, 0, 0};
  tree_sitter_ling_external_scanner_deserialize(
      scanner, bad_version, sizeof(bad_version));
  assert_root_state(scanner);

  const char truncated[] = {2, 2, 0, 0, 0, 0, 0};
  tree_sitter_ling_external_scanner_deserialize(scanner, truncated,
                                                sizeof(truncated));
  assert_root_state(scanner);

  const char non_monotonic[] = {2, 3, 0, 0, 0, 0, 0, 4, 0, 2, 0};
  tree_sitter_ling_external_scanner_deserialize(
      scanner, non_monotonic, sizeof(non_monotonic));
  assert_root_state(scanner);

  const char excessive_delimiters[] = {2, 1, 0, 1, 1, 0, 0};
  tree_sitter_ling_external_scanner_deserialize(
      scanner, excessive_delimiters, sizeof(excessive_delimiters));
  assert_root_state(scanner);

  tree_sitter_ling_external_scanner_destroy(scanner);
}

static void root_boundary_resynchronizes_scanner_state(void) {
  Scanner *scanner = new_scanner();
  array_push(&scanner->indents, 4);
  scanner->delimiter_depth = 2;
  TestLexer test = test_lexer("\ntype After = Text");
  bool valid_symbols[ROOT_DECLARATION_BOUNDARY + 1] = {false};
  valid_symbols[ROOT_DECLARATION_BOUNDARY] = true;

  assert(tree_sitter_ling_external_scanner_scan(
      scanner, &test.lexer, valid_symbols));
  assert(test.lexer.result_symbol == ROOT_DECLARATION_BOUNDARY);
  assert(test.marked_end == 1);
  assert_root_state(scanner);

  tree_sitter_ling_external_scanner_destroy(scanner);
}

static void boundary_probe_preserves_normal_newlines(void) {
  Scanner *scanner = new_scanner();
  TestLexer test = test_lexer("\n");
  bool valid_symbols[ROOT_DECLARATION_BOUNDARY + 1] = {false};
  valid_symbols[NEWLINE] = true;
  valid_symbols[ROOT_DECLARATION_BOUNDARY] = true;

  assert(tree_sitter_ling_external_scanner_scan(
      scanner, &test.lexer, valid_symbols));
  assert(test.lexer.result_symbol == NEWLINE);
  assert(test.marked_end == 1);
  assert_root_state(scanner);

  tree_sitter_ling_external_scanner_destroy(scanner);
}

static void boundary_probe_does_not_split_unicode_identifiers(void) {
  Scanner *scanner = new_scanner();
  array_push(&scanner->indents, 4);
  scanner->delimiter_depth = 2;
  TestLexer test = test_lexer("\ntype\xE4\xBA\xBA = 1");
  bool valid_symbols[ROOT_DECLARATION_BOUNDARY + 1] = {false};
  valid_symbols[ROOT_DECLARATION_BOUNDARY] = true;

  assert(!tree_sitter_ling_external_scanner_scan(
      scanner, &test.lexer, valid_symbols));
  assert(scanner->indents.size == 2);
  assert(*array_back(&scanner->indents) == 4);
  assert(scanner->delimiter_depth == 2);

  tree_sitter_ling_external_scanner_destroy(scanner);
}

int main(void) {
  round_trips_complete_state();
  serializes_the_maximum_layout_depth();
  serializes_the_maximum_delimiter_depth();
  enforces_the_delimiter_depth_boundary();
  rejects_corrupt_or_non_monotonic_state();
  root_boundary_resynchronizes_scanner_state();
  boundary_probe_preserves_normal_newlines();
  boundary_probe_does_not_split_unicode_identifiers();
  return 0;
}
