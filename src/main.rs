#![feature(asm)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[link_section = ".start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Hello\r\n
    outb(42, 72);
    outb(42, 101);
    outb(42, 108);
    outb(42, 108);
    outb(42, 111);
    outb(42, 13);
    outb(42, 10);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn outb(_port: u8, value: u8) {
    unsafe {
        asm!("out 42, al", /* in(reg_byte) port, */ in("al") value);
    }
}
