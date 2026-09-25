#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include "parser.h"

int shell_builtin_cd(shell_command_t *command);
int shell_builtin_echo(shell_command_t *command);
int shell_builtin_pwd(shell_command_t *command);
int shell_builtin_help(shell_command_t *command);
int shell_builtin_history(shell_command_t *command);
int shell_builtin_exit(shell_command_t *command);

static int shell_is_builtin(const char *name) {
  static const char *builtins[] = {"cd",   "echo",    "pwd", "help",
                                   "exit", "history", NULL};
  int i = 0;

  while (builtins[i] != NULL) {
    if (strcmp(name, builtins[i]) == 0) {
      return 1;
    }
    ++i;
  }

  return 0;
}

static int shell_run_builtin(shell_command_t *command) {
  if (strcmp(command->arguments[0], "cd") == 0) {
    return shell_builtin_cd(command);
  }
  if (strcmp(command->arguments[0], "echo") == 0) {
    return shell_builtin_echo(command);
  }
  if (strcmp(command->arguments[0], "pwd") == 0) {
    return shell_builtin_pwd(command);
  }
  if (strcmp(command->arguments[0], "help") == 0) {
    return shell_builtin_help(command);
  }
  if (strcmp(command->arguments[0], "history") == 0) {
    return shell_builtin_history(command);
  }
  if (strcmp(command->arguments[0], "exit") == 0) {
    return shell_builtin_exit(command);
  }

  fprintf(stderr, "shell: unknown builtin: %s\n", command->arguments[0]);
  return 1;
}

static int shell_run_external(shell_command_t *command) {
  pid_t pid = fork();

  if (pid < 0) {
    perror("fork");
    return 1;
  }

  if (pid == 0) {
    char *argv[CHUT_SHELL_MAX_ARGUMENTS + 1];
    for (size_t i = 0; i < command->argument_count; ++i) {
      argv[i] = command->arguments[i];
    }
    argv[command->argument_count] = NULL;

    execvp(argv[0], argv);
    fprintf(stderr, "shell: %s: %s\n", argv[0], strerror(errno));
    _exit(127);
  }

  int status = 0;
  if (waitpid(pid, &status, 0) < 0) {
    perror("waitpid");
    return 1;
  }

  if (WIFEXITED(status)) {
    return WEXITSTATUS(status);
  }
  if (WIFSIGNALED(status)) {
    fprintf(stderr, "shell: %s terminated by signal %d\n",
            command->arguments[0], WTERMSIG(status));
  }
  return 1;
}

int shell_execute_command(shell_command_t *command) {
  if (command == NULL || command->argument_count == 0) {
    return 0;
  }

  if (shell_is_builtin(command->arguments[0])) {
    return shell_run_builtin(command);
  }

  return shell_run_external(command);
}
