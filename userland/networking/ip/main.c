#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int show_interfaces(void) {
  FILE *fp = fopen("/proc/net/dev", "r");
  char line[256];
  int first = 1;

  if (!fp) {
    fprintf(stderr, "ip: unable to read /proc/net/dev: %s\n", strerror(errno));
    return 1;
  }

  fgets(line, sizeof(line), fp);
  fgets(line, sizeof(line), fp);

  printf("Interface\tMTU\tRX-OK\tTX-OK\n");
  while (fgets(line, sizeof(line), fp)) {
    char name[64];
    unsigned long rx_bytes = 0, tx_bytes = 0;
    unsigned int mtu = 0;

    if (sscanf(line,
               "%63[^:]: %*d %*d %*d %*d %*d %*d %*d %*d %*d %*d %lu %*d %*d "
               "%*d %*d %*d %*d %*d %*d %*d %lu",
               name, &rx_bytes, &tx_bytes) != 3) {
      continue;
    }

    if (first) {
      first = 0;
    }

    mtu = 1500;
    printf("%s\t%u\t%lu\t%lu\n", name, mtu, rx_bytes, tx_bytes);
  }

  fclose(fp);
  return 0;
}

static int show_address_info(void) {
  FILE *fp = fopen("/proc/net/fib_trie", "r");
  char line[256];
  int found = 0;

  if (!fp) {
    fprintf(stderr, "ip: unable to read /proc/net/fib_trie: %s\n",
            strerror(errno));
    return 1;
  }

  printf("Address info:\n");
  while (fgets(line, sizeof(line), fp)) {
    if (strstr(line, "32 host") != NULL || strstr(line, "32 host") != NULL) {
      printf("  %s", line);
      found = 1;
    }
  }

  if (!found) {
    printf("  loopback/127.0.0.1\n");
  }

  fclose(fp);
  return 0;
}

int main(int argc, char **argv) {
  if (argc < 2) {
    return show_interfaces();
  }

  if (strcmp(argv[1], "addr") == 0 || strcmp(argv[1], "address") == 0) {
    return show_address_info();
  }

  if (strcmp(argv[1], "link") == 0 || strcmp(argv[1], "iface") == 0) {
    return show_interfaces();
  }

  fprintf(stderr, "usage: ip [addr|link]\n");
  return 2;
}
