#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "parser.h"

int shell_execute_command(shell_command_t *command);
void shell_history_init(void);
void shell_history_add(const char *line);
void shell_history_list(void);
void shell_history_free(void);

static void print_prompt(void) { printf("pos> "); }

int main(void) {
  char *line = NULL;
  size_t capacity = 0;
  ssize_t length;
  shell_command_t command;
  shell_parse_error_t parse_error;

  shell_history_init();

  while (1) {
    print_prompt();
    length = getline(&line, &capacity, stdin);
    if (length < 0) {
      if (feof(stdin)) {
        puts("");
        break;
      }
      perror("getline");
      break;
    }

    while (length > 0 &&
           (line[length - 1] == '\n' || line[length - 1] == '\r')) {
      line[--length] = '\0';
    }

    if (length == 0) {
      continue;
    }

    shell_history_add(line);
    parse_error = shell_parse_line(line, &command);
    if (parse_error != SHELL_PARSE_OK) {
      fprintf(stderr, "shell: %s\n", shell_parse_error_string(parse_error));
      continue;
    }

    if (shell_execute_command(&command) == 2) {
      break;
    }
  }

  free(line);
  shell_history_free();
  return 0;
}
