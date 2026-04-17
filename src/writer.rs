use crate::syscall::sys_write;
use core::fmt::{self, Write};

pub struct SyscallWriter;

impl Write for SyscallWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        sys_write(1, bytes.as_ptr(), bytes.len()); // fd=1 是 stdout
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    let mut writer = SyscallWriter;
    let _ = fmt::write(&mut writer, args);
}

// 实现 print! 和 println! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::writer::print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
