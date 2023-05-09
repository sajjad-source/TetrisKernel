#![no_std]
#![no_main]

mod vga_buffer;
mod keyboard;
mod tetris;
use tetris::tetris;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...\n");
    tetris::run();
    println!("\nQuitting...\n");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}