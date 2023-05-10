use core::arch::asm;

pub fn read_cmos(register: u8) -> u8 {
    let mut value: u8;

    unsafe {
        asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") register,
            lateout("al") value,
            options(nostack, preserves_flags)
        );
    }

    value
}

// binary-coded decimal to binary
fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}
