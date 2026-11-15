use core::fmt;

#[inline(always)]
fn sys_write(fd: i32, buf: &[u8]) -> isize {
    let ret: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1,              // SYS_write
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack)
        );
    }

    ret
}

pub struct StdoutWriter;

impl fmt::Write for StdoutWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut remaining = s.as_bytes();

        while !remaining.is_empty() {
            let written = sys_write(1, remaining);

            if written <= 0 {
                return Err(fmt::Error);
            }

            remaining = &remaining[written as usize..];
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let _ = StdoutWriter.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::logger::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

#[inline(always)]
pub fn exit_syscall(status: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60,       // SYS_exit
            in("rdi") status,        // Exit status
            options(noreturn)
        );
    }
}
