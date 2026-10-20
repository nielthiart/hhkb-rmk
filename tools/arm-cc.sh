#!/bin/sh
# C compiler for thumbv7em-none-eabihf build scripts. Homebrew's
# arm-none-eabi-gcc ships without C library headers, so use clang with its
# freestanding headers plus the string.h shim next to this script.
exec clang -ffreestanding -isystem "$(dirname "$0")/c-shim" "$@"
