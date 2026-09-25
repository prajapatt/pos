#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: mv <src> <dst>\n");
    return 2;
  }

  if (rename(argv[1], argv[2]) != 0) {
    fprintf(stderr, "mv: %s -> %s: %s\n", argv[1], argv[2], strerror(errno));
    return 1;
  }

  return 0;
}
