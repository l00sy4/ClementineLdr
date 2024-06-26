#![allow(non_snake_case)]

pub fn dbj2_hash(buffer: &[u8]) -> u32
{
    let mut hash: u32 = 5441;
    let mut i: usize = 0;
    let mut cur: u8;

    while i < buffer.len()
    {
        cur = buffer[i];

        if cur == 0
        {
            i += 1;
            continue;
        }

        if cur >= ('a' as u8)
        {
            cur -= 0x20;
        }

        hash = ((hash << 5).wrapping_add(hash)) + cur as u32;
        i += 1;
    }

    return hash;
}

fn main() {

    let functions: [&str; 6] = ["TpAllocWork", "TpPostWork", "TpReleaseWork", "LoadLibraryA",
        "NtProtectVirtualMemory", "NtAllocateVirtualMemory"];

    println!("Functions ---------------\r\n");
    for func in functions.iter() {
        println!("{}: {:#X}", func, dbj2_hash(func.as_bytes()));
    }
}