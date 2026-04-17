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
