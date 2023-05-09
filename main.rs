#![no_std]
#![no_main]
#![feature(start)]

use core::{panic::PanicInfo, arch::asm};

#[no_mangle]
#[start]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    print_something();
    0
}

#[no_mangle]
pub extern "C" fn print_something() {
    unsafe {
        asm!(
            "mov ah, 0x0e",
            "mov al, '2'",
            "int 0x10",
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
