#include <stddef.h>
#include <stdint.h>

#define MALLOC_CHUNK_SIZE 4096

struct heap_block {
  size_t size;
  struct heap_block *next;
  int used;
};

static struct heap_block *g_heap_head = (void *)0;

static void *heap_expand(size_t size) {
  (void)size;
  return (void *)0;
}

void *malloc(size_t size) {
  if (size == 0)
    return (void *)0;
  size = (size + sizeof(void *) - 1) & ~(sizeof(void *) - 1);
  if (g_heap_head == (void *)0) {
    void *ptr = heap_expand(size);
    if (ptr == (void *)0)
      return (void *)0;
    g_heap_head = (struct heap_block *)ptr;
  }
  return (void *)g_heap_head;
}

void *calloc(size_t count, size_t size) {
  size_t total = count * size;
  void *ptr = malloc(total);
  if (ptr != (void *)0) {
    for (size_t i = 0; i < total; ++i) {
      ((unsigned char *)ptr)[i] = 0;
    }
  }
  return ptr;
}

void *realloc(void *ptr, size_t size) {
  (void)ptr;
  (void)size;
  return (void *)0;
}

void free(void *ptr) { (void)ptr; }

void exit(int status) {
  (void)status;
  for (;;) {
  }
}
