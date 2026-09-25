#ifndef CHUT_UNISTD_H
#define CHUT_UNISTD_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SYS_EXIT 1
#define SYS_WRITE 2
#define SYS_READ 3
#define SYS_OPEN 4
#define SYS_CLOSE 5
#define SYS_FORK 6
#define SYS_WAITPID 7
#define SYS_EXEC 8
#define SYS_BRK 9

long syscall0(long number);
long syscall1(long number, long arg0);
long syscall2(long number, long arg0, long arg1);
long syscall3(long number, long arg0, long arg1, long arg2);

int write(int fd, const void *buf, size_t count);
int read(int fd, void *buf, size_t count);
int close(int fd);
int open(const char *path, int flags, int mode);
void _exit(int status);

#ifdef __cplusplus
}
#endif

#endif
