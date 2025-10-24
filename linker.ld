ENTRY(_start)

PHDRS
{
    text PT_LOAD FLAGS(5);  /* R-X */
    data PT_LOAD FLAGS(6);  /* RW- */
}

SECTIONS
{
    /* Start at 0x40000000 which is the default load address for QEMU virt machine */
    . = 0x40000000;
    
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    } :text
    
    .rodata : {
        *(.rodata .rodata.*)
    } :text
    
    .data : {
        *(.data .data.*)
    } :data
    
    . = ALIGN(16);
    __bss_start = .;
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data
    __bss_end = .;
    
    /* Reserve 256KB for stack */
    . = ALIGN(16);
    __stack_start = .;
    .stack (NOLOAD) : {
        . = . + 0x40000; /* 256KB stack */
    } :data
    __stack_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}
