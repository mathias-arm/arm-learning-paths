core::arch::global_asm!(
    "
    .macro adr_l, reg, sym
        adrp \\reg, \\sym
        add \\reg, \\reg, :lo12:\\sym
    .endm

    .section .entry, \"ax\"
    .global entry
entry:
    /* Set stack pointer */
    adr_l x4, __stack_top
    mov sp, x4

    /* Enable Floating-point */
    mrs x4, cpacr_el1
    orr x4, x4, #(0x3 << 20)
    msr cpacr_el1, x4
    isb

    /* Zero out the bss section. */
    adr_l x4, __bss_begin
    adr_l x5, __bss_end
0:  cmp x4, x5
    b.hs 1f
    stp xzr, xzr, [x4], #16
    b 0b
1:

    /* Call into Rust code. */
    bl init
"
);

#[no_mangle]
unsafe extern "C" fn init(x0: u64, x1: u64, x2: u64, x3: u64) -> ! {
    crate::main(x0, x1, x2, x3);
}
