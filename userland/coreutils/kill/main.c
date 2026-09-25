#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  int sig = SIGTERM;
  long pid;
  char *end = NULL;

  if (argc < 2) {
    fprintf(stderr, "usage: kill [-signal] <pid>\n");
    return 2;
  }

  if (argc == 3) {
    if (argv[1][0] != '-') {
      fprintf(stderr, "usage: kill [-signal] <pid>\n");
      return 2;
    }
    sig = atoi(argv[1] + 1);
    if (sig <= 0) {
      fprintf(stderr, "kill: invalid signal %s\n", argv[1]);
      return 2;
    }
    pid = strtol(argv[2], &end, 10);
  } else {
    pid = strtol(argv[1], &end, 10);
  }

  if (*end != '\0' || pid <= 0) {
    fprintf(stderr, "kill: invalid pid: %s\n", argv[argc == 3 ? 2 : 1]);
    return 2;
  }

  if (kill((pid_t)pid, sig) != 0) {
    fprintf(stderr, "kill: %ld: %s\n", pid, strerror(errno));
    return 1;
  }

  return 0;
}
