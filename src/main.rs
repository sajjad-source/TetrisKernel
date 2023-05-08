#![no_std]
#![no_main]

mod vga_buffer;
mod interrupts;
use core::panic::PanicInfo;
use crate::vga_buffer::{change_color, Color};


#[no_mangle]
pub extern "C" fn _start() -> ! {
    change_color(Color::Red);
    println!("Booting...\n");
    let line = interrupts::getline();
    println!("\nQuitting...\n");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}