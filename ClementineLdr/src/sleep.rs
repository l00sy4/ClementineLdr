#[link_section = ".text"]
pub unsafe fn get_time() -> usize {

    let time = Time {
        LowPart: *((0x7FFE0000u32 + 0x14) as *const u32),
        HighPart: *((0x7FFE0000u32 + 0x1C) as *const i32)
    };

    #[cfg(target_arch = "x86_64")]
    return (((time.HighPart as u64) << 32 | (time.LowPart as u64)) as usize) / 10000;
    #[cfg(target_arch = "x86")]
    return time.LowPart as usize; // Good enough for now, lol
}

#[link_section = ".text"]
pub unsafe fn sleep(milliseconds: usize) {
    let end: usize = get_time() + milliseconds;

    for _ in 0.. {
        if get_time() >= end {
            break;
        }
    }
}

#[repr(C)]
pub struct Time {
    pub LowPart: u32,
    pub HighPart: i32,
}