#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  DIR *proc_dir = opendir("/proc");
  struct dirent *entry;

  if (!proc_dir) {
    fprintf(stderr, "ps: unable to open /proc: %s\n", strerror(errno));
    return 1;
  }

  printf("PID\tCOMMAND\n");
  while ((entry = readdir(proc_dir)) != NULL) {
    char path[256];
    FILE *fp;
    char name[128];

    if (entry->d_name[0] < '0' || entry->d_name[0] > '9') {
      continue;
    }

    snprintf(path, sizeof(path), "/proc/%s/comm", entry->d_name);
    fp = fopen(path, "r");
    if (!fp) {
      continue;
    }

    if (fgets(name, sizeof(name), fp) != NULL) {
      size_t len = strlen(name);
      if (len > 0 && name[len - 1] == '\n') {
        name[len - 1] = '\0';
      }
      printf("%s\t%s\n", entry->d_name, name);
    }

    fclose(fp);
  }

  closedir(proc_dir);
  return 0;
}
