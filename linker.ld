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
    
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
    
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}
