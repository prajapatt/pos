#include <stddef.h>
#include <stdint.h>
#include <unistd.h>

long syscall0(long number) { return 0; }

long syscall1(long number, long arg0) {
  (void)number;
  (void)arg0;
  return 0;
}

long syscall2(long number, long arg0, long arg1) {
  (void)number;
  (void)arg0;
  (void)arg1;
  return 0;
}

long syscall3(long number, long arg0, long arg1, long arg2) {
  (void)number;
  (void)arg0;
  (void)arg1;
  (void)arg2;
  return 0;
}

int write(int fd, const void *buf, size_t count) {
  (void)fd;
  (void)buf;
  (void)count;
  return 0;
}

int read(int fd, void *buf, size_t count) {
  (void)fd;
  (void)buf;
  (void)count;
  return 0;
}

int close(int fd) {
  (void)fd;
  return 0;
}

int open(const char *path, int flags, int mode) {
  (void)path;
  (void)flags;
  (void)mode;
  return -1;
}

void _exit(int status) {
  (void)status;
  for (;;) {
  }
}
