use std::ffi::CString;
use windows_sys::{
    Win32::{
        System::{
            SystemServices::{
                IMAGE_DOS_HEADER,
                IMAGE_NT_SIGNATURE
            },
            Diagnostics::{
                Debug::{IMAGE_NT_HEADERS32, IMAGE_NT_HEADERS64},
                Debug::IMAGE_SECTION_HEADER
            },
            LibraryLoader::LoadLibraryA
        }
    }
};

pub unsafe fn rva2offset(rva: usize, base_address: usize) -> Option<usize> {

    #[cfg(target_arch = "x86_64")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    #[cfg(target_arch = "x86")]
        let nt_header = (dll as usize + (*(dll as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS32;

    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        return None;
    }

    let section_header = (&(*nt_header).OptionalHeader as *const _ as usize + (*nt_header).FileHeader.SizeOfOptionalHeader as usize) as *mut IMAGE_SECTION_HEADER;

    for i in 0..(*nt_header).FileHeader.NumberOfSections as usize {
        if rva > (*section_header).VirtualAddress[i] && rva < ((*section_header).VirtualAddress[i] + (*section_header).Misc.VirtualSize[i]) {
            Some(((rva - (*section_header).VirtualAddress[i]) + (*section_header).PointerToRawData[i]));
        }
    }

    return None;
}

fn main() {

    unsafe {
    let ntdll_name = CString::new("ntdll").unwrap().as_bytes().as_ptr();
    let kernel32_name = CString::new("kernel32").unwrap().as_bytes().as_ptr();

    let ntdll_address: isize = LoadLibraryA(ntdll_name);
    let kernel32_address: isize = LoadLibraryA(kernel32_name);


    if ntdll_address == 0 || kernel32_address == 0 {
        return;
    }
}
}
