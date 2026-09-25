#include <stddef.h>
#include <stdint.h>

int spawn_process(const char *path, const char *const argv[],
                  const char *const envp[]) {
  (void)path;
  (void)argv;
  (void)envp;
  return -1;
}

int wait_process(int pid, int *status) {
  (void)pid;
  (void)status;
  return -1;
}

int kill_process(int pid, int signal) {
  (void)pid;
  (void)signal;
  return -1;
}

int getpid(void) { return 0; }
