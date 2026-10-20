#!/bin/bash
# Build and flash: enter the bootloader (Fn+U or double-tap reset), then run
# ./flash.sh, or ./flash.sh file.uf2 to flash another firmware.
set -eu

FW="${1:-target/rmk.uf2}"
if [ $# -eq 0 ]; then
  cargo objcopy --release -- -O ihex target/rmk.hex
  cargo hex-to-uf2 --input-path target/rmk.hex --output-path "$FW" --family nrf52840
fi

[ -d /Volumes/NICENANO ] || { echo "No NICENANO drive: enter the bootloader first."; exit 1; }
# -X skips extended attributes, which macOS would try to write after the
# bootloader has already rebooted.
cp -X "$FW" /Volumes/NICENANO/
