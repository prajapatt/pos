#ifndef CHUT_BOOT_INFO_H
#define CHUT_BOOT_INFO_H

#include <stdint.h>

#define CHUT_BOOT_INFO_VERSION 1u
#define CHUT_MEMORY_USABLE 7u

typedef struct chut_memory_region {
  uint64_t base;
  uint64_t length;
  uint32_t kind;
  uint32_t reserved;
} chut_memory_region_t;

typedef struct chut_framebuffer_info {
  uint64_t address;
  uint32_t width;
  uint32_t height;
  uint32_t stride;
  uint32_t format;
} chut_framebuffer_info_t;

typedef struct chut_boot_info {
  uint32_t version;
  uint32_t size;
  uint64_t kernel_base;
  uint64_t kernel_size;
  const chut_memory_region_t *memory_regions;
  uint32_t memory_region_count;
  uint32_t reserved;
  chut_framebuffer_info_t framebuffer;
} chut_boot_info_t;

#endif
