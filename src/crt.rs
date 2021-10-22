#[no_mangle]
pub extern "C" fn strcmp(first: *const u8, second: *const u8) -> bool {
    print("crt::strcmp");
    todo!()
}

#[no_mangle]
pub extern "C" fn strtoul(s: *const u8) -> u32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn strtoull(s: *const u8) -> u64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn copysignf(first: f32, second: f32) -> f32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn copysign(first: f64, second: f64) -> f64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn ceilf(f: f32) -> f32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn ceil(f: f64) -> f64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn floorf(f: f32) -> f32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn floor(f: f64) -> f64 {
    todo!()
}

#[no_mangle]
pub extern "C" fn truncf(f: f32) -> f32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn trunc(f: f64) -> f64 {
    print("crt::trunc");
    todo!()
}

#[no_mangle]
pub extern "C" fn sqrtf(f: f32) -> f32 {
    print("crt::sqrtf");
    todo!()
}

#[no_mangle]
pub extern "C" fn sqrt(f: f64) -> f64 {
    print("crt::sqrt");
    todo!()
}

#[no_mangle]
pub extern "C" fn rintf(f: f32) -> f32 {
    print("crt::rintf");
    todo!()
}

#[no_mangle]
pub extern "C" fn rint(f: f64) -> f64 {
    print("crt::rint");
    todo!()
}

#[no_mangle]
pub extern "C" fn __popcountdi2(num: u32) -> u32 {
    print("crt::popcount");
    todo!()
}

#[no_mangle]
pub extern "C" fn abort() {
    print("crt::abort");
    todo!()
}

#[no_mangle]
pub extern "C" fn vsnprintf(buffer: *mut u8, count: usize, format: *const u8, args: *const u8 /* TODO: va_list */) -> u32 {
    print("crt::vsnprinf");
    todo!()
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
