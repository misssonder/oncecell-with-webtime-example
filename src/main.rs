#![no_std]
#![feature(alloc_error_handler, start, core_intrinsics, lang_items, link_cfg)]


extern crate alloc;
extern crate wee_alloc;

use alloc::sync::Arc;
use once_cell::unsync::Lazy;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _data: Lazy<Arc<isize>> = Lazy::new(|| Arc::new(42));
    1
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort();
}

#[panic_handler]
#[lang = "panic_impl"]
fn rust_begin_panic(_: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

#[no_mangle]
extern "C" fn _rust_eh_personality() {}

#[no_mangle]
extern "C" fn rust_eh_personality() {}

#[no_mangle]
extern "C" fn rust_eh_register_frames() {}

#[no_mangle]
extern "C" fn rust_eh_unregister_frames() {}

#[no_mangle]
extern "C" fn _Unwind_Resume() {}
