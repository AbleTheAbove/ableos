ENTRY(_start)
SECTIONS
{
    . = 0x40000000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(8);
    . = . + 0x4000;
    LD_STACK_PTR = .;
}
