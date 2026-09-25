#ifndef CHUT_BOOT_PROTOCOL_H
#define CHUT_BOOT_PROTOCOL_H

#include "boot_info.h"

#define CHUT_BOOT_MAGIC 0x434855544f53424full

typedef struct chut_boot_protocol {
  uint64_t magic;
  uint32_t version;
  uint32_t reserved;
  const chut_boot_info_t *info;
} chut_boot_protocol_t;

#endif
