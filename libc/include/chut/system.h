#ifndef CHUT_SYSTEM_H
#define CHUT_SYSTEM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

long chsys_call(long number, long arg0, long arg1, long arg2);
int chsys_write(int fd, const void *buf, unsigned long count);
int chsys_read(int fd, void *buf, unsigned long count);
int chsys_open(const char *path, int flags, int mode);
int chsys_close(int fd);
int chsys_exit(int status);

#ifdef __cplusplus
}
#endif

#endif
