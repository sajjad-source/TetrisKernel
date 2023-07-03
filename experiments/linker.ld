ENTRY(_start)

SECTIONS
{
    . = 0x8000;   /* Where you want to load your kernel */

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }
}
