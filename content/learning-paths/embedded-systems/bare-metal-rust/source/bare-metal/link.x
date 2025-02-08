ENTRY(entry)

SECTIONS
{
    . = 0x84000000;
    .entry : { *(.entry.entry) }
    .text : { *(.text.*) *(.rodata.*) }
    .got : { *(.got.*) }
    .data : ALIGN(16) { *(.data.*) }

    __bss_begin = .;
    .bss : ALIGN(16) { *(.bss.*) *(COMMON) }
    __bss_end = .;

    .stack (NOLOAD) : ALIGN(4096) { . += 16 * 4096; }
    __stack_top = .;

    . = ALIGN(4096);
    PROVIDE(__free_memory = .);
}
