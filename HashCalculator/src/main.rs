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
    unsafe {
        let functions: [&str; 2] = ["NtAllocateVirtualMemory", "NtProtectVirtualMemory"];
        let modules: [&str; 1] = ["ntdll"];

        println!("Functions ---------------\r\n");
        for func in functions.iter() {
            println!("{}: {:#X}\n", func, dbj2_hash(func.as_bytes()));
        }

        println!("Modules ---------------\r\n");
        for string in modules.iter() {
            println!("{}: {:#X}\r\n", string, dbj2_hash(string.as_bytes()));
        }
    }
}