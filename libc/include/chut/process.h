#ifndef CHUT_PROCESS_H
#define CHUT_PROCESS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  int pid;
  int parent;
  int status;
  unsigned long cpu_time;
} process_info_t;

int spawn_process(const char *path, const char *const argv[],
                  const char *const envp[]);
int wait_process(int pid, int *status);
int kill_process(int pid, int signal);
int getpid(void);

#ifdef __cplusplus
}
#endif

#endif
