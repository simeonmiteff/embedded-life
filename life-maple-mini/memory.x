/* Linker script for the STM32F103C8T6

Copied from: https://github.com/jamwaffles/ssd1306/blob/master/memory.x

*/
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
