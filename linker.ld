ENTRY(_start)

SECTIONS
{
    /* Start at 0x40000000 which is the default load address for QEMU virt machine */
    . = 0x40000000;
    
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    . = ALIGN(16);
    __bss_start = .;
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
    __bss_end = .;
    
    /* Reserve 64KB for stack */
    . = ALIGN(16);
    __stack_start = .;
    .stack (NOLOAD) : {
        . = . + 0x10000; /* 64KB stack */
    }
    __stack_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}
