#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;
pub mod logger;
mod ram;
use ram::CpuRam;
pub mod CPUMain;
use CPUMain::Bus;

/*
change the read and writes to volatile read and writes and
also check the mem functions one more time just in case
also check to make sure the stack isn't overflowing whenever I am doing RAM stuff (Check GPT)
*/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(ptr1: *const c_void, ptr2: *const c_void, count: usize) -> i32 {
    let p1 = ptr1 as *const u8;
    let p2 = ptr2 as *const u8;

    unsafe {
        for i in 0..count {
            let a = p1.add(i).read_volatile();
            let b = p2.add(i).read_volatile();

            if a != b {
                return a as i32 - b as i32;
            }
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;

    unsafe {
        if (d as usize) <= (s as usize) {
            for i in 0..len {
                d.add(i).write_volatile(s.add(i).read_volatile());
            }
        } else {
            for i in (0..len).rev() {
                d.add(i).write_volatile(s.add(i).read_volatile());
            }
        }
    }

    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(
    dest: *mut c_void,
    value: i32,
    count: usize,
) -> *mut c_void {
    let dest = dest as *mut u8;

    for i in 0..count {
        unsafe {
            dest.add(i).write(value as u8);
        }
    }
    dest as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *mut u8;
    unsafe {
        for i in 0..n {
            let byte = s.add(i).read_volatile();
            d.add(i).write_volatile(byte);
        }
    }
    return dest;
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut testram: CpuRam = CpuRam {
        ram_data: [0; 0x10000],
    };
    Bus::ram_write(0xAB23, 123, &mut testram);
    let data: u8 = Bus::ram_read(0xAB23, false, &testram);
    println!("testing_data: {}", data);
    logger::exit_syscall(0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    logger::exit_syscall(1);
}
