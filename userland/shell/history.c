#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SHELL_HISTORY_LIMIT 128

typedef struct shell_history_entry {
  char *line;
  struct shell_history_entry *next;
} shell_history_entry_t;

static shell_history_entry_t *g_history_head = NULL;
static shell_history_entry_t *g_history_tail = NULL;
static size_t g_history_count = 0;

void shell_history_init(void) {
  g_history_head = NULL;
  g_history_tail = NULL;
  g_history_count = 0;
}

void shell_history_add(const char *line) {
  if (line == NULL || *line == '\0') {
    return;
  }

  shell_history_entry_t *entry = calloc(1, sizeof(*entry));
  if (entry == NULL) {
    return;
  }

  entry->line = strdup(line);
  if (entry->line == NULL) {
    free(entry);
    return;
  }

  if (g_history_tail == NULL) {
    g_history_head = entry;
    g_history_tail = entry;
  } else {
    g_history_tail->next = entry;
    g_history_tail = entry;
  }

  ++g_history_count;

  while (g_history_count > SHELL_HISTORY_LIMIT) {
    shell_history_entry_t *old_head = g_history_head;
    g_history_head = old_head->next;
    if (g_history_head == NULL) {
      g_history_tail = NULL;
    }
    free(old_head->line);
    free(old_head);
    --g_history_count;
  }
}

void shell_history_list(void) {
  shell_history_entry_t *current = g_history_head;
  size_t index = 1;

  while (current != NULL) {
    printf("%zu  %s\n", index, current->line);
    current = current->next;
    ++index;
  }
}

void shell_history_free(void) {
  shell_history_entry_t *current = g_history_head;

  while (current != NULL) {
    shell_history_entry_t *next = current->next;
    free(current->line);
    free(current);
    current = next;
  }

  g_history_head = NULL;
  g_history_tail = NULL;
  g_history_count = 0;
}
