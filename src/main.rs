#![no_std]
#![no_main]

mod vga_buffer;
mod interrupts;
use core::panic::PanicInfo;
use crate::interrupts::PICS;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...\n");
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    println!("Quitting...\n");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}