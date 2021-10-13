// This is currently based on https://os.phil-opp.com/heap-allocation/

pub const HEAP_START: usize = 0x0030_0000;
pub const HEAP_SIZE: usize =  0x0050_0000;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
// static  ALLOCATOR: Dummy = Dummy {};

pub fn init() {
    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); }
}

// use alloc::alloc::{GlobalAlloc, Layout};
// use core::ptr::null_mut;

// pub struct Dummy;

// unsafe impl GlobalAlloc for Dummy {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
//         null_mut()
//     }

//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//         panic!("dealloc should be never called")
//     }
// }
