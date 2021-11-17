OUTPUT_ARCH(riscv64gc)

ENTRY(_boot);


SECTIONS {
    . = 0x80200000;
    .text : {
        PROVIDE(__text_start = .);
        PROVIDE(KERNEL_START = .);
        *(.init.boot)
        *(.init.rust)
        *(.text .text.*)
        . = ALIGN(4K);
        PROVIDE(__text_end = .);
    }

    .data : {
        PROVIDE(__data_start = .);
        *(.data .data.* .rodata .rodata.*)
        . = ALIGN(8);
        PROVIDE(__tmp_stack_bottom = .);
        . += 1024 * 1024 * 4;
        PROVIDE(__tmp_stack_top = .);
        . += 4096;
        PROVIDE(__scratch_stack = .);
        . = ALIGN(8);
    }

    . = ALIGN(8);

    .sdata : {
        PROVIDE(__global_pointer$ = .);
        *(.sdata .sdata.*)       
        . = ALIGN(4K);
        PROVIDE(__data_end = .); 
    }

    PROVIDE(__bss_start = .);
    .sbss : {
        *(.sbss .sbss.*);
    }

    .bss : {
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    PROVIDE(__bss_end = .);

    .tdata : {
        . = ALIGN(4K);
        PROVIDE(__tdata_start = .);
        
        *(.tdata .tdata.*)

        . = ALIGN(4K);
        PROVIDE(__tdata_end = .);
    }

    . = ALIGN(2M);
    PROVIDE(KERNEL_END = .);

    /DISCARD/ : { *(.eh_frame_hdr .eh_frame) }
}
