#ifndef CHUT_IPC_H
#define CHUT_IPC_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  int channel_id;
  int sender;
  int receiver;
  uint64_t tag;
  uint32_t length;
  uint8_t payload[256];
} ipc_message_t;

int ipc_send(int channel_id, const ipc_message_t *msg);
int ipc_recv(int channel_id, ipc_message_t *msg);
int ipc_open_channel(int channel_id);
int ipc_close_channel(int channel_id);

#ifdef __cplusplus
}
#endif

#endif
