#include <stddef.h>
#include <stdint.h>

void *memcpy(void *dest, const void *src, size_t n) {
  unsigned char *d = (unsigned char *)dest;
  const unsigned char *s = (const unsigned char *)src;
  for (size_t i = 0; i < n; ++i) {
    d[i] = s[i];
  }
  return dest;
}

void *memmove(void *dest, const void *src, size_t n) {
  unsigned char *d = (unsigned char *)dest;
  const unsigned char *s = (const unsigned char *)src;
  if (d < s) {
    for (size_t i = 0; i < n; ++i)
      d[i] = s[i];
  } else {
    for (size_t i = n; i > 0; --i)
      d[i - 1] = s[i - 1];
  }
  return dest;
}

void *memset(void *s, int c, size_t n) {
  unsigned char *p = (unsigned char *)s;
  for (size_t i = 0; i < n; ++i)
    p[i] = (unsigned char)c;
  return s;
}

size_t strlen(const char *s) {
  size_t len = 0;
  while (s[len] != '\0')
    ++len;
  return len;
}

int strcmp(const char *a, const char *b) {
  while (*a && (*a == *b)) {
    ++a;
    ++b;
  }
  return (unsigned char)*a - (unsigned char)*b;
}

int strncmp(const char *a, const char *b, size_t n) {
  for (size_t i = 0; i < n; ++i) {
    if (a[i] != b[i])
      return (unsigned char)a[i] - (unsigned char)b[i];
    if (a[i] == '\0')
      return 0;
  }
  return 0;
}

char *strcpy(char *dest, const char *src) {
  char *out = dest;
  while ((*dest++ = *src++) != '\0') {
  }
  return out;
}

char *strncpy(char *dest, const char *src, size_t n) {
  char *out = dest;
  for (size_t i = 0; i < n; ++i) {
    dest[i] = src[i];
    if (src[i] == '\0')
      break;
  }
  return out;
}

char *strchr(const char *s, int c) {
  while (*s != '\0') {
    if (*s == (char)c)
      return (char *)s;
    ++s;
  }
  return (*s == (char)c) ? (char *)s : (char *)0;
}

char *strrchr(const char *s, int c) {
  const char *last = (char *)0;
  while (*s != '\0') {
    if (*s == (char)c)
      last = s;
    ++s;
  }
  return (char *)last;
}
