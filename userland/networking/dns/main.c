#include <arpa/inet.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  struct addrinfo hints;
  struct addrinfo *result;
  int status;

  if (argc < 2) {
    fprintf(stderr, "usage: dns <hostname>\n");
    return 2;
  }

  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_UNSPEC;
  hints.ai_socktype = SOCK_STREAM;

  status = getaddrinfo(argv[1], NULL, &hints, &result);
  if (status != 0) {
    fprintf(stderr, "dns: %s: %s\n", argv[1], gai_strerror(status));
    return 1;
  }

  for (struct addrinfo *it = result; it != NULL; it = it->ai_next) {
    char host[INET6_ADDRSTRLEN];
    void *addr;

    if (it->ai_family == AF_INET) {
      struct sockaddr_in *ipv4 = (struct sockaddr_in *)it->ai_addr;
      addr = &(ipv4->sin_addr);
    } else if (it->ai_family == AF_INET6) {
      struct sockaddr_in6 *ipv6 = (struct sockaddr_in6 *)it->ai_addr;
      addr = &(ipv6->sin6_addr);
    } else {
      continue;
    }

    if (inet_ntop(it->ai_family, addr, host, sizeof(host)) == NULL) {
      continue;
    }

    printf("%s -> %s\n", argv[1], host);
  }

  freeaddrinfo(result);
  return 0;
}
