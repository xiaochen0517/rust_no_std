#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

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

#[repr(C)]
pub struct UnwindException {
    pub private: [u64; 2],
    pub cleanup: extern "C" fn(code: i32, obj: *mut UnwindException),
}

/// LLVM unwinding ABI 中的异常对象类型
#[repr(C)]
pub struct UnwindContext;

/// 继续异常展开
///
/// # Safety
/// 这个函数由 LLVM 生成的展开代码调用，
/// 参数必须是有效的 _Unwind_Exception 指针
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume(ex: *mut UnwindException) -> ! {
    // 这个函数不应该正常返回
    // 在异常展开失败时，程序应该中止
    unsafe {
        // 可以调用 abort 或其他终止函数
        // core::intrinsics::abort();
        panic!("Unwind resume called with exception: {:?}", ex);
    }
}
