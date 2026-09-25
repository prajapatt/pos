#include "parser.h"

static int is_space(char character) {
  return character == ' ' || character == '\t' || character == '\n' ||
         character == '\r';
}

shell_parse_error_t shell_parse_line(char *line, shell_command_t *command) {
  if (line == NULL || command == NULL) {
    return SHELL_PARSE_EMPTY;
  }

  command->argument_count = 0;
  char *read = line;
  char *write = line;

  while (*read != '\0') {
    while (is_space(*read)) {
      ++read;
    }
    if (*read == '\0') {
      break;
    }
    if (command->argument_count == CHUT_SHELL_MAX_ARGUMENTS) {
      return SHELL_PARSE_TOO_MANY_ARGUMENTS;
    }

    command->arguments[command->argument_count++] = write;
    char quote = '\0';
    int token_started = 0;
    while (*read != '\0') {
      if (quote == '\0' && is_space(*read)) {
        break;
      }
      if (*read == '\\') {
        ++read;
        if (*read == '\0') {
          return SHELL_PARSE_TRAILING_ESCAPE;
        }
        *write++ = *read++;
        token_started = 1;
        continue;
      }
      if (*read == '\'' || *read == '"') {
        if (quote == '\0') {
          token_started = 1;
          quote = *read++;
          continue;
        }
        if (quote == *read) {
          quote = '\0';
          ++read;
          continue;
        }
      }
      *write++ = *read++;
      token_started = 1;
    }
    if (quote != '\0') {
      return SHELL_PARSE_UNTERMINATED_QUOTE;
    }
    if (!token_started) {
      --command->argument_count;
    }
    *write++ = '\0';
  }

  return command->argument_count == 0 ? SHELL_PARSE_EMPTY : SHELL_PARSE_OK;
}

const char *shell_parse_error_string(shell_parse_error_t error) {
  switch (error) {
  case SHELL_PARSE_OK:
    return "ok";
  case SHELL_PARSE_EMPTY:
    return "empty command";
  case SHELL_PARSE_TOO_MANY_ARGUMENTS:
    return "too many arguments";
  case SHELL_PARSE_UNTERMINATED_QUOTE:
    return "unterminated quote";
  case SHELL_PARSE_TRAILING_ESCAPE:
    return "trailing escape";
  case SHELL_PARSE_ARGUMENT_TOO_LONG:
    return "argument too long";
  default:
    return "unknown parse error";
  }
}
