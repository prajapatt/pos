#include <assert.h>
#include <string.h>

#include "userland/shell/parser.h"

static void parses_quotes_and_escapes(void) {
  char line[] = "open \"my file\" plain\\ argument 'quoted'";
  shell_command_t command;

  assert(shell_parse_line(line, &command) == SHELL_PARSE_OK);
  assert(command.argument_count == 4);
  assert(strcmp(command.arguments[0], "open") == 0);
  assert(strcmp(command.arguments[1], "my file") == 0);
  assert(strcmp(command.arguments[2], "plain argument") == 0);
  assert(strcmp(command.arguments[3], "quoted") == 0);
}

static void rejects_incomplete_input(void) {
  char quote[] = "open \"file";
  char escape[] = "open file\\";
  shell_command_t command;

  assert(shell_parse_line(quote, &command) == SHELL_PARSE_UNTERMINATED_QUOTE);
  assert(shell_parse_line(escape, &command) == SHELL_PARSE_TRAILING_ESCAPE);
}

static void handles_empty_lines(void) {
  char line[] = " \t\r\n";
  shell_command_t command;

  assert(shell_parse_line(line, &command) == SHELL_PARSE_EMPTY);
  assert(command.argument_count == 0);
}

static void preserves_empty_quoted_arguments(void) {
  char line[] = "emit \"\" ''";
  shell_command_t command;

  assert(shell_parse_line(line, &command) == SHELL_PARSE_OK);
  assert(command.argument_count == 3);
  assert(strcmp(command.arguments[1], "") == 0);
  assert(strcmp(command.arguments[2], "") == 0);
}

int main(void) {
  parses_quotes_and_escapes();
  rejects_incomplete_input();
  handles_empty_lines();
  preserves_empty_quoted_arguments();
  return 0;
}