#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

static int copy_stream(FILE *in, FILE *out) {
  char buffer[4096];
  size_t nread;

  while ((nread = fread(buffer, 1, sizeof(buffer), in)) > 0) {
    if (fwrite(buffer, 1, nread, out) != nread) {
      return 1;
    }
  }

  return ferror(in) ? 1 : 0;
}

int main(int argc, char **argv) {
  int exit_code = 0;

  if (argc < 2) {
    return copy_stream(stdin, stdout) ? 1 : 0;
  }

  for (int i = 1; i < argc; ++i) {
    FILE *fp = fopen(argv[i], "rb");
    if (!fp) {
      fprintf(stderr, "cat: %s: %s\n", argv[i], strerror(errno));
      exit_code = 1;
      continue;
    }

    if (copy_stream(fp, stdout) != 0) {
      fprintf(stderr, "cat: %s: write error\n", argv[i]);
      exit_code = 1;
    }

    fclose(fp);
  }

  return exit_code;
}
