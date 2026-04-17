#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

// #[unsafe(no_mangle)]
// pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
//     for i in 0..n {
//         unsafe {
//             *dest.add(i) = *src.add(i);
//         }
//     }
//     dest
// }

#[unsafe(no_mangle)]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *s.add(i) = c as u8;
        }
    }
    s
}

// #[unsafe(no_mangle)]
// pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
//     if dest < src as *mut u8 {
//         for i in 0..n {
//             unsafe {
//                 *dest.add(i) = *src.add(i);
//             }
//         }
//     } else {
//         for i in (0..n).rev() {
//             unsafe {
//                 *dest.add(i) = *src.add(i);
//             }
//         }
//     }
//     dest
// }
