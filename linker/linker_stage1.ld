ENTRY(_start)

SECTIONS
{
    . = 0x7C00;

    .boot :
    {
        KEEP(*(.multiboot))
        KEEP(*(.text))
    }

    . = 0x7DFE;
    .signature :
    {
        SHORT(0xaa55);
    }
}
