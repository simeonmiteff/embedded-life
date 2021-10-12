#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <programmer serial port device file>"
    exit 1
fi

if [ ! -c "$1" ]; then
    echo "Error: $1 is not a character device. Try something like /dev/ttyUSBX where X is 0, 1, etc."
    exit 2
fi

cargo build --release

arm-none-eabi-objcopy -O ihex ../target/thumbv7m-none-eabi/release/life-maple-mini life-maple-mini.hex

stm32flash -b 57600 -w life-maple-mini.hex -g 0x8000000 $1
