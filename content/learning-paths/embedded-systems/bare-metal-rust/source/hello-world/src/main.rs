#![no_std]
#![no_main]

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    for c in "Hello world!\n".as_bytes() {
        (0x1c09_0000 as *mut u8).write_volatile(*c);
    }
    core::arch::asm!("hlt #0xf000", in("x0") 0x18u64, options(noreturn))
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
