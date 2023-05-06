#![no_std]
#![no_main]

mod vga_buffer;
mod interrupts;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...\n");
    let line = interrupts::getline();
    println!("{:?}", line);
    println!("Quitting...\n");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}