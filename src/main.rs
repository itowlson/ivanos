#![no_main]
#![no_std]

// #![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(asm)]

extern crate alloc;

use core::panic::PanicInfo;

// use wasm3::Environment;
// use wasm3::Module;

mod allocator;
// mod crt;

#[link_section = ".start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Hello\r\n
    outb(42, 72);
    outb(42, 101);
    outb(42, 108);
    outb(42, 108);
    outb(42, 111);
    outb(42, 111);
    outb(42, 111);
    outb(42, 111);
    outb(42, 111);
    outb(42, 111);
    outb(42, 111);
    outb(42, 13);
    outb(42, 10);

    print("Initing allocator\r\n");
    allocator::init();
    print("Inited allocator\r\n");

    // let env = Environment::new()
    //     .expect("Unable to create environment");
    // let rt = env
    //     .create_runtime(1024)
    //     .expect("Unable to create runtime");

    // print("Created runtime\r\n");

    hlt();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// From https://os.phil-opp.com/heap-allocation/
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    // panic!("allocation error: {:?}", layout)
    outb(42, 97);
    outb(42, 108);
    outb(42, 108);
    outb(42, 111);
    outb(42, 99);
    outb(42, 13);
    outb(42, 10);
    loop {}
}

fn print(s: &str) {
    for c in s.as_bytes() {
        outb(42, *c);
    }
}

fn outb(_port: u8, value: u8) {
    unsafe {
        asm!("out 42, al", /* in(reg_byte) port, */ in("al") value);
    }
}

fn hlt() {
    unsafe {
        asm!("hlt");
    }
}