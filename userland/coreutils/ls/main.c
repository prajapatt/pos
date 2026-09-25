#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  const char *path = argc > 1 ? argv[1] : ".";
  DIR *dir = opendir(path);
  struct dirent *entry;

  if (!dir) {
    fprintf(stderr, "ls: %s: %s\n", path, strerror(errno));
    return 1;
  }

  while ((entry = readdir(dir)) != NULL) {
    if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
      continue;
    }
    printf("%s\n", entry->d_name);
  }

  closedir(dir);
  return 0;
}
