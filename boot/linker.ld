/* Linker script for bootloader */
OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

SECTIONS
{
    . = 0x7C00;  /* BIOS loads boot sector at 0x7C00 */

    .text : ALIGN(4)
    {
        *(.text .text.*)
    }

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4)
    {
        *(.data .data.*)
    }

    .bss : ALIGN(4)
    {
        *(.bss .bss.*)
        *(COMMON)
    }

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.comment)
        *(.note*)
        *(.gnu*)
    }
}
