/* Adafruit nRF52 bootloader layout. FLASH ends at 0xA0000, where main.rs puts the keymap storage. */
MEMORY
{
  FLASH : ORIGIN = 0x00001000, LENGTH = 636K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K
}
