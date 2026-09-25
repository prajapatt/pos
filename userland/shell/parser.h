#ifndef CHUT_SHELL_PARSER_H
#define CHUT_SHELL_PARSER_H

#include <stddef.h>

#define CHUT_SHELL_MAX_ARGUMENTS 32

typedef enum shell_parse_error {
  SHELL_PARSE_OK = 0,
  SHELL_PARSE_EMPTY,
  SHELL_PARSE_TOO_MANY_ARGUMENTS,
  SHELL_PARSE_UNTERMINATED_QUOTE,
  SHELL_PARSE_TRAILING_ESCAPE,
  SHELL_PARSE_ARGUMENT_TOO_LONG
} shell_parse_error_t;

typedef struct shell_command {
  size_t argument_count;
  char *arguments[CHUT_SHELL_MAX_ARGUMENTS];
} shell_command_t;

shell_parse_error_t shell_parse_line(char *line, shell_command_t *command);
const char *shell_parse_error_string(shell_parse_error_t error);

#endif