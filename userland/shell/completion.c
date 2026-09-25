#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *shell_completion_candidates[] = {
    "cd",    "echo", "exit", "help", "history", "ls",
    "mkdir", "ps",   "pwd",  "rm",   "top",     NULL};

void shell_complete(const char *prefix) {
  if (prefix == NULL) {
    return;
  }

  for (size_t i = 0; shell_completion_candidates[i] != NULL; ++i) {
    if (strncmp(prefix, shell_completion_candidates[i], strlen(prefix)) == 0) {
      printf("%s\n", shell_completion_candidates[i]);
    }
  }
}
