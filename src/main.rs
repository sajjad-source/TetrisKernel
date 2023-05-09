#![no_std]
#![no_main]

mod keyboard;
mod tetris;
mod vga_buffer;
mod random;
use core::panic::PanicInfo;
use tetris::game;

use crate::vga_buffer::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...\n");
    game::run();
    println!("\nQuitting...\n");
    WRITER.lock().flush();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
