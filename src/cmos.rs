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

// Assumes that midnight is not crossed
pub fn get_time() -> usize {
    let seconds = bcd_to_bin(read_cmos(0x00)) as usize;
    let minutes = bcd_to_bin(read_cmos(0x02)) as usize;
    let hours = bcd_to_bin(read_cmos(0x04)) as usize;

    // Convert hours and minutes to seconds
    (hours * 60 * 60) + (minutes * 60) + seconds
}