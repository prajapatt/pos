#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int read_uint_value(const char *path, unsigned long *value) {
  FILE *fp = fopen(path, "r");
  if (!fp) {
    return -1;
  }
  int ok = fscanf(fp, "%lu", value) == 1;
  fclose(fp);
  return ok ? 0 : -1;
}

int main(void) {
  DIR *proc_dir = opendir("/proc");
  struct dirent *entry;

  if (!proc_dir) {
    fprintf(stderr, "top: unable to open /proc: %s\n", strerror(errno));
    return 1;
  }

  printf("PID\tSTATE\tCPU\n");
  while ((entry = readdir(proc_dir)) != NULL) {
    char stat_path[256];
    char state_path[256];
    unsigned long pid;
    unsigned long utime, stime;

    if (entry->d_name[0] < '0' || entry->d_name[0] > '9') {
      continue;
    }

    snprintf(stat_path, sizeof(stat_path), "/proc/%s/stat", entry->d_name);
    snprintf(state_path, sizeof(state_path), "/proc/%s/stat", entry->d_name);

    if (read_uint_value(stat_path, &pid) != 0) {
      continue;
    }

    FILE *fp = fopen(state_path, "r");
    if (!fp) {
      continue;
    }

    char name[128];
    char state;
    if (fscanf(fp, "%lu %127s %c %*d %*d %*d %*d %*d %*d %*d %*d %*d %lu %lu",
               &pid, name, &state, &utime, &stime) == 5) {
      printf("%lu\t%c\t%lu\n", pid, state, utime + stime);
    }
    fclose(fp);
  }

  closedir(proc_dir);
  return 0;
}
