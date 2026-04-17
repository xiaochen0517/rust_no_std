#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_no_std::{println, syscall::sys_exit};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct BlockHeader {
    size: usize,   // 8 bytes
    is_free: bool, // 1 byte, 7 bytes padding
} // Total: 16 bytes

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(0);
}
