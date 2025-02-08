#![no_std]
#![no_main]

mod entry;
mod pl011;
use core::fmt::Write;

#[no_mangle]
unsafe extern "C" fn main(_x0: u64, _x1: u64, _x2: u64, _x3: u64) -> ! {
    let _ = writeln!(pl011::Pl011::default(), "Hello World!");

    // Uncomment next line to see panic handler
    // panic!("Panic");

    core::arch::asm!("hlt #0xf000", in("x0") 0x18u64, options(noreturn))
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(pl011::Pl011::default(), "Panic: {}", info);
    unsafe { core::arch::asm!("hlt #0xf000", in("x0") 0x18u64) };
    loop {}
}
