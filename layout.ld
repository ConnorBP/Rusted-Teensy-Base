/* memory map implemented for teensy 3.6 (Cortex M4 MK66FX1M0) https://www.pjrc.com/teensy/K66P144M180SF5RMV2.pdf */

MEMORY
{
/*
    Teensy 3.6 has 1M flash, 256K Ram
    0x1C00_0000–0x1FFF_FFFF SRAM_L: Lower SRAM (ICODE/DCODE) All masters
    0x2000_0000–0x200F_FFFF2 SRAM_U: Upper SRAM bitband region All masters
*/
/*  From brenan src impl:
    • SRAM_L = [0x2000_0000–(SRAM_size/2)] to 0x1FFF_FFFF
    • SRAM_U = 0x2000_0000 to [0x2000_0000+(SRAM_size/2)-1]
    Origin was set at: 0x1FFF_8000 (not sure why it is set to 8000)
*/
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 1M
    RAM  (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}

SECTIONS
{
    .text : {
        . = 0;/* First 1024 bytes (0x400) are the exception vectors */
        KEEP(*(.vectors))
        . = 0x400;
        KEEP(*(.flashconfig*))
        . = ALIGN(4);
        *(.text*)
    } > FLASH = 0xFF

    .rodata : {
        *(.rodata*)
    } > FLASH

    _stack_top = ORIGIN(RAM) + LENGTH(RAM);

    /DISCARD/ : {
        *(.ARM.*)
    }
}
