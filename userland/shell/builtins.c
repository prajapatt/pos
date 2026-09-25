#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "parser.h"

extern void shell_history_list(void);

int shell_builtin_cd(shell_command_t *command) {
  const char *target =
      command->argument_count > 1 ? command->arguments[1] : "/";

  if (chdir(target) != 0) {
    fprintf(stderr, "cd: %s: %s\n", target, strerror(errno));
    return 1;
  }
  return 0;
}

int shell_builtin_echo(shell_command_t *command) {
  for (size_t i = 1; i < command->argument_count; ++i) {
    printf("%s", command->arguments[i]);
    if (i + 1 < command->argument_count) {
      putchar(' ');
    }
  }
  putchar('\n');
  return 0;
}

int shell_builtin_pwd(shell_command_t *command) {
  (void)command;
  char buffer[4096];

  if (getcwd(buffer, sizeof(buffer)) == NULL) {
    perror("pwd");
    return 1;
  }

  printf("%s\n", buffer);
  return 0;
}

int shell_builtin_help(shell_command_t *command) {
  (void)command;
  static const char *help_text[] = {"Builtins:",
                                    "  cd <path>",
                                    "  echo [args]",
                                    "  pwd",
                                    "  help",
                                    "  history",
                                    "  exit",
                                    "External commands are executed via PATH.",
                                    NULL};

  for (int i = 0; help_text[i] != NULL; ++i) {
    puts(help_text[i]);
  }
  return 0;
}

int shell_builtin_history(shell_command_t *command) {
  (void)command;
  shell_history_list();
  return 0;
}

int shell_builtin_exit(shell_command_t *command) {
  (void)command;
  return 2;
}
