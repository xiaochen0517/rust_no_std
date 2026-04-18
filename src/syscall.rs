pub fn sys_exit(code: usize) -> ! {
    unsafe {
        core::arch::asm!("syscall", in("rax") 60, in("rdi") code, options(noreturn));
    }
}

pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

pub fn sys_mmap(length: usize) -> *mut u8 {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 9usize,       // sys_mmap
            in("rdi") 0usize,       // addr = NULL，由内核选择地址
            in("rsi") length,       // 申请大小（需页对齐，4096 的倍数）
            in("rdx") 0x3usize,     // PROT_READ | PROT_WRITE
            in("r10") 0x22usize,    // MAP_PRIVATE | MAP_ANONYMOUS
            in("r8")  usize::MAX,   // fd = -1（匿名映射）
            in("r9")  0usize,       // offset = 0
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    result as *mut u8 // 失败时返回 MAP_FAILED = -1usize as *mut u8
}
