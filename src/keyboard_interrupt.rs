use x86_64::instructions::interrupts;
use core::arch::asm;

pub fn getch() -> u8 {
    let mut inchar: u32 = 0;

    unsafe {
        interrupts::without_interrupts(|| {
            asm!("int 0x16", inout("ax") 0x0000 => inchar);
        });
    }

    (inchar & 0xff) as u8
}