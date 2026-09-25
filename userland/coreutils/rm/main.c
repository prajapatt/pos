#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "usage: rm <path> [<path> ...]\n");
    return 2;
  }

  for (int i = 1; i < argc; ++i) {
    if (unlink(argv[i]) != 0) {
      fprintf(stderr, "rm: %s: %s\n", argv[i], strerror(errno));
      return 1;
    }
  }

  return 0;
}
