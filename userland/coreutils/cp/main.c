#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

static int copy_file(const char *src, const char *dst) {
  int in_fd = open(src, O_RDONLY);
  int out_fd;
  char buffer[8192];
  ssize_t bytes_read;

  if (in_fd < 0) {
    fprintf(stderr, "cp: %s: %s\n", src, strerror(errno));
    return 1;
  }

  out_fd = open(dst, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  if (out_fd < 0) {
    fprintf(stderr, "cp: %s: %s\n", dst, strerror(errno));
    close(in_fd);
    return 1;
  }

  while ((bytes_read = read(in_fd, buffer, sizeof(buffer))) > 0) {
    if (write(out_fd, buffer, (size_t)bytes_read) != bytes_read) {
      fprintf(stderr, "cp: %s: write error\n", dst);
      close(in_fd);
      close(out_fd);
      return 1;
    }
  }

  close(in_fd);
  close(out_fd);
  return 0;
}

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: cp <src> <dst>\n");
    return 2;
  }
  return copy_file(argv[1], argv[2]);
}
