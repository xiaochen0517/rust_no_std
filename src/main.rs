#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use rust_no_std::{println, syscall::sys_exit};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let num = Box::new(42u8);
    println!("Boxed u8 value: {}", num);
    let num = Box::new(84u16);
    println!("Boxed u16 value: {}", num);
    let num = Box::new(168u32);
    println!("Boxed u32 value: {}", num);
    let num = Box::new(336u64);
    println!("Boxed u64 value: {}", num);

    let str = Box::new("Hello, heap!");
    println!("Boxed str value: {}", str);

    let mut vec = Vec::new();
    vec.push("test1");
    vec.push("test2");
    vec.push("test3");
    println!("Vec values: {:?}", vec);

    println!("\nDone!");
    sys_exit(0);
}
