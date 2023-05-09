use core::arch::asm;

pub fn rdtsc() -> u64 {
    let a: usize;
    let b: usize;

    unsafe {
        asm!(
            "rdtsc",
            out("eax") a,
            out("edx") b,
        );
    }

    ((b as u64) << 32) | (a as u64)
}

pub fn rand() -> u64 {
    let a = 1664525u64;
    let c = 1013904223u64;
    let m = 2u64.pow(32);
    let seed = rdtsc();

    (a * seed + c) % m
}
