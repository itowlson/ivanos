#![no_main]
#![no_std]

// #![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(asm)]

#![feature(fmt_internals)]

extern crate alloc;

use core::{ops::Add, panic::PanicInfo};

use alloc::{borrow::ToOwned, format};

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

    let mut s = "foo\r\n".to_owned();
    s.make_ascii_uppercase();
    let s1 = s.add("bar\r\n");
    print(&s1);

    let t = "arse\r\n";
    // let t1 = format!("{}:{}", s1, t);

    print("ARGING ALL THE THINGS\r\n");

    let s1_ = ::core::fmt::ArgumentV1::new(&s1, ::core::fmt::Display::fmt);
    print("made first args arg\r\n");
    let t_ = ::core::fmt::ArgumentV1::new(&t, ::core::fmt::Display::fmt);
    print("made args args\r\n");

    let slicey = [s1_, t_];

    let fmtargs = ::core::fmt::Arguments::new_v1(&["", ":"], &slicey);
    print("made args struct\r\n");

    print("CALLING FORMAT\r\n");
    let t1 = ::alloc::fmt::format(fmtargs);

    print("PRINTING THE THING\r\n");
    print(&t1);

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