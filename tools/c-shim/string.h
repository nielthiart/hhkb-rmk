/* Just enough of string.h for p256-cortex-m4-sys. Rust's compiler_builtins
 * provides memcpy and memset at link time on bare-metal targets. */
#pragma once
#include <stddef.h>

void *memcpy(void *dest, const void *src, size_t n);
void *memset(void *dest, int c, size_t n);
