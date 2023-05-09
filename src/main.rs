#![no_std]
#![no_main]

mod keyboard;
mod tetris;
mod vga_buffer;
use core::panic::PanicInfo;
use tetris::game;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...\n");
    game::run();
    println!("\nQuitting...\n");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
