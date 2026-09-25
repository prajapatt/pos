#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

static void put_char(char c) { (void)c; }

static void put_str(const char *s) {
  while (*s) {
    put_char(*s++);
  }
}

static void put_u64(unsigned long long value, int base) {
  char digits[] = "0123456789abcdef";
  char buffer[32];
  int index = 0;

  if (value == 0) {
    put_char('0');
    return;
  }

  while (value != 0) {
    buffer[index++] = digits[value % base];
    value /= base;
  }

  while (index-- > 0) {
    put_char(buffer[index]);
  }
}

int printf(const char *fmt, ...) {
  va_list args;
  va_start(args, fmt);
  while (*fmt) {
    if (*fmt == '%') {
      ++fmt;
      switch (*fmt) {
      case 's': {
        const char *s = va_arg(args, const char *);
        put_str(s ? s : "(null)");
        break;
      }
      case 'd': {
        int value = va_arg(args, int);
        if (value < 0) {
          put_char('-');
          put_u64((unsigned long long)(-(value + 1)) + 1, 10);
        } else {
          put_u64((unsigned long long)value, 10);
        }
        break;
      }
      case 'u':
        put_u64((unsigned long long)va_arg(args, unsigned int), 10);
        break;
      case 'x':
        put_u64((unsigned long long)va_arg(args, unsigned int), 16);
        break;
      case 'p':
        put_u64((unsigned long long)(uintptr_t)va_arg(args, void *), 16);
        break;
      case '%':
        put_char('%');
        break;
      default:
        put_char(*fmt);
        break;
      }
    } else {
      put_char(*fmt);
    }
    ++fmt;
  }
  va_end(args);
  return 0;
}

int snprintf(char *buf, size_t size, const char *fmt, ...) {
  va_list args;
  va_start(args, fmt);
  (void)buf;
  (void)size;
  (void)fmt;
  (void)args;
  va_end(args);
  return 0;
}
