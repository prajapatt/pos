#ifndef CHUT_SYS_TYPES_H
#define CHUT_SYS_TYPES_H

#include <stddef.h>
#include <stdint.h>

typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef long off_t;
typedef unsigned long size_t;
typedef long ssize_t;
typedef unsigned int mode_t;
typedef signed long pid_t;

typedef struct {
  int major;
  int minor;
} dev_t;

#endif
