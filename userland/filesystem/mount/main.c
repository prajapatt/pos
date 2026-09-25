#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef __linux__
#warning                                                                       \
    "mount is Linux-specific; this build will not mount devices outside Linux."
#endif

int main(int argc, char **argv) {
  const char *source;
  const char *target;
  const char *fstype;
  const char *options = "rw";

  if (argc < 4) {
    fprintf(stderr, "usage: mount <source> <target> <fstype> [options]\n");
    return 2;
  }

  source = argv[1];
  target = argv[2];
  fstype = argv[3];
  if (argc >= 5) {
    options = argv[4];
  }

#ifdef __linux__
  if (mount(source, target, fstype, 0, options) != 0) {
    fprintf(stderr, "mount: %s -> %s: %s\n", source, target, strerror(errno));
    return 1;
  }
#else
  (void)source;
  (void)target;
  (void)fstype;
  (void)options;
  fprintf(stderr, "mount: unsupported platform\n");
  return 1;
#endif

  printf("mounted %s on %s (%s)\n", source, target, fstype);
  return 0;
}
