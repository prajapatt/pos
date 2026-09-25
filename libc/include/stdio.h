#ifndef CHUT_STDIO_H
#define CHUT_STDIO_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef EOF
#define EOF (-1)
#endif

typedef struct FILE {
  int fd;
  int flags;
} FILE;

extern FILE *stdout;
extern FILE *stderr;

int putchar(int c);
int puts(const char *s);
int printf(const char *fmt, ...);
int snprintf(char *buf, size_t size, const char *fmt, ...);

#ifdef __cplusplus
}
#endif

#endif
