/* Linker script for kernel */
OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kernel_main)

SECTIONS
{
    . = 1M;  /* Kernel loaded at 1MB */

    .text ALIGN(4K) : AT(ADDR(.text))
    {
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata))
    {
        *(.rodata .rodata.*)
    }

    .data ALIGN(4K) : AT(ADDR(.data))
    {
        *(.data .data.*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss))
    {
        *(.bss .bss.*)
        *(COMMON)
    }

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.comment)
    }
}
