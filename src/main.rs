#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::boxed::Box;
use rust_no_std::{allocator::BlockHeader, println, syscall::sys_exit};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    println!("Hello, world!");
    let header_align = core::mem::align_of::<BlockHeader>();
    println!("BlockHeader alignment: {}", header_align);
    let header_size = core::mem::size_of::<BlockHeader>();
    println!("BlockHeader size: {}", header_size);

    let box_u32 = Box::new(42u32);
    println!("Boxed value: {}", box_u32);

    // let max_num = 10;

    // for i in 0..100 {
    //     let str_unit = alloc::format!("'unit string'");
    //     let mut str = alloc::format!("Hello, heap! {}", i);
    //     if max_num > 0 && i % max_num != 0 {
    //         for _ in 0..(i % max_num) {
    //             str += str_unit.as_str();
    //         }
    //     }
    //     println!("{}", str);
    // }
    println!("\nDone!");
    sys_exit(0);
}
