/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x00800000, LENGTH = 128K
}