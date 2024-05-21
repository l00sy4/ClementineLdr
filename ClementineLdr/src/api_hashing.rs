use crate::{
    IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_DOS_HEADER,
    IMAGE_EXPORT_DIRECTORY,
    from_raw_parts,
    IMAGE_NT_HEADERS64,
    IMAGE_NT_SIGNATURE
};

#[link_section = ".text"]
pub unsafe fn get_function_address(base_address: isize, function_hash: u32) -> Option<usize> {

     #[cfg(target_arch = "x86_64")]
         let nt_header = (base_address as usize + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
     #[cfg(target_arch = "x86")]
         let nt_header = (base_address as usize + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut crate::IMAGE_NT_HEADERS32;

     if (*nt_header).Signature != IMAGE_NT_SIGNATURE
     {
         return None;
     }

     let export_directory = (base_address as usize + (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT as usize].VirtualAddress as usize) as *mut IMAGE_EXPORT_DIRECTORY;
     let names = from_raw_parts((base_address as usize + (*export_directory).AddressOfNames as usize) as *const u32, (*export_directory).NumberOfNames as _);
     let functions = from_raw_parts((base_address as usize + (*export_directory).AddressOfFunctions as usize) as *const u32, (*export_directory).NumberOfFunctions as _,);
     let ordinals = from_raw_parts((base_address as usize + (*export_directory).AddressOfNameOrdinals as usize) as *const u16, (*export_directory).NumberOfNames as _);

     for i in 0..(*export_directory).NumberOfFunctions {

         let function_name = (base_address as usize + names[i as usize] as usize) as *const u8;
         let name_slice: &[u8] = from_raw_parts(function_name,get_cstring_length(function_name as *const char));

         if dbj2_hash(name_slice) == function_hash {
             let ordinal = ordinals[i as usize] as usize;
             return Some(base_address as usize + functions[ordinal] as usize);
         }
     }

     None
}

#[link_section = ".text"]
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

#[link_section = ".text"]
pub unsafe fn get_cstring_length(string: *const char) -> usize
{
    let mut temp: usize = string as usize;

    while *(temp as *const u8) != 0
    {
        temp += 1;
    }

    temp - string as usize
}