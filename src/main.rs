#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use rust_no_std::{
    allocator::{BlockHeader, align_up},
    println,
    syscall::sys_exit,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    // println!("Hello, world!");
    // let header_align = core::mem::align_of::<BlockHeader>();
    // println!("BlockHeader alignment: {}", header_align);
    // let header_size = core::mem::size_of::<BlockHeader>();
    // println!("BlockHeader size: {}", header_size);

    // let box_u32 = Box::new(42u32);
    // println!("Boxed value: {}", box_u32);

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
    // let aligned = align_up(0x101, 8);
    // println!("Aligned address: {:#x}", aligned);
    let aligned = align_up(0x101, 1);
    println!("Aligned address: {:#x}", aligned);

    let num = Box::new(42u8);
    println!("Boxed u8 value: {}", num);
    let num = Box::new(84u16);
    println!("Boxed u16 value: {}", num);
    let num = Box::new(168u32);
    println!("Boxed u32 value: {}", num);
    let num = Box::new(336u64);
    println!("Boxed u64 value: {}", num);

    println!("\nDone!");
    sys_exit(0);
}
