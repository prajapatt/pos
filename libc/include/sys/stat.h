#ifndef CHUT_SYS_STAT_H
#define CHUT_SYS_STAT_H

#include <sys/types.h>

#define S_IFMT 00170000
#define S_IFSOCK 0140000
#define S_IFLNK 0120000
#define S_IFREG 0100000
#define S_IFBLK 0060000
#define S_IFDIR 0040000
#define S_IFCHR 0020000
#define S_IFIFO 0010000

#define S_ISDIR(m) (((m) & S_IFMT) == S_IFDIR)
#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)

struct stat {
  dev_t st_dev;
  off_t st_ino;
  mode_t st_mode;
  off_t st_size;
  long st_atime;
  long st_mtime;
  long st_ctime;
};

int stat(const char *path, struct stat *st);
int fstat(int fd, struct stat *st);

#endif
