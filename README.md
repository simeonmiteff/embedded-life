# embedded-life

`embedded-life` is Conway's Game Of Life rendered on an OLED display, and built for an ARM Cortex-M (STM32F103), Raspberry Pi and a
SDL-based display simulator. It demonstrates using the power of Rust embedded-hal abstraction to write portable code.

For an in-depth discussion, please see the [detailed article on my blog](https://fdi.sk/posts/embedded-life). This document will focus on
instructions for building and running the code only.

## Build

### Install build dependencies

- The simulator app requires SDL2.
- The ARM targets needs the linker from ARM GCC and binutils.

This will work for Ubuntu/Debian/Mint (other platforms will have similar packages):
```terminal
$ sudo apt-get -y install libsdl2-dev gcc-arm-linux-gnueabihf binutils-arm-none-eabi
```

Depending on what programming hardware you have, you may also need `openocd`, `stlink-tools` or `stm32flash`. I'm using
`stm32flash` with a FTDI FT232RL (3.3v USB UART):

```terminal
$ sudo apt-get -y install stm32flash
```

You'll also need `rustup` (get it [here](https://rustup.rs/)).

From this point on everything is in Rust-land:

```terminal
$ rustup target add armv7-unknown-linux-gnueabihf
$ rustup target add thumbv7m-none-eabi
```

### Build all the targets

Finally, run the build script (this is required because cargo's workspaces don't work well with multiple architectures):

```terminal
$ ./build.sh
```

## Running

### The simulator

Run:
```terminal
$ ./target/release/life-simulator
```

### ARM Cortex-M (maple mini)

If you have a maple mini clone, and an SSD1306 OLED display, you can wire them up as follows:

<insert diagram>

The script in `life-maple-mini/build-and-flash.sh` takes a USB serial device file argument, and uses `stm32flash` to
program the STM32 (remember to press the reset button first):

```terminal
$ ./build-and-flash.sh /dev/ttyUSB0 
<snip>
    Finished release [optimized] target(s) in 24.41s
stm32flash 0.5

http://stm32flash.sourceforge.net/

Using Parser : Intel HEX
Interface serial_posix: 57600 8E1
Version      : 0x22
Option 1     : 0x00
Option 2     : 0x00
Device ID    : 0x0410 (STM32F10xxx Medium-density)
- RAM        : 20KiB  (512b reserved by bootloader)
- Flash      : 128KiB (size first sector: 4x1024)
- Option RAM : 16b
- System RAM : 2KiB
Write to memory
Erasing memory
Wrote address 0x080056e8 (100.00%) Done.

Starting execution at address 0x08000000... done.
```

### The Raspberry Pi

To try this on a Raspberry Pi, wire the SSD1306 OLED display as follows:

<insert diagram>

The I2C device file is hard coded (to `/dev/i2c-0`) in `life-raspi/src/main.rs`. If you're not using the first
I2C bus, edit as required.

Running the code is as simple as copying `target/armv7-unknown-linux-gnueabihf/release/life-raspi` to the Pi 
(`scp` or transfer by USB drive) and then running it with `./life-raspi`.

## License and attribution
 
`embedded-life` is largely derivative of James Waples' embedded graphics crates, so it is reproduced under the same license
(Apache 2.0) - see `LICENSE`.

The Rust implementation of Game Of Life is loosely based on the [version at Rosetta code](https://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust).
For details please see the notes in `life/src/lib.rs`.
