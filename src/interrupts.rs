use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use core::arch::asm;

use crate::print;

pub fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") data);
    }
    data
}

pub fn getch(prev_scancode: &mut u8) -> Option<char> {
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    let scancode = inb(0x60);
    if scancode == *prev_scancode {
        return None;
    }
    *prev_scancode = scancode;

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(DecodedKey::Unicode(character)) =
            keyboard.process_keyevent(key_event)
        {
            return Some(character);
        }
    }
    None
}

const BUFFER_SIZE: usize = 80;

pub fn getline() -> ! {
    let mut prev_scancode: u8 = 0;
    loop {
        match getch(&mut prev_scancode) {
            Some(ch) => {
                print!("{}", ch);
            }
            None => {}
        }
    }
}
