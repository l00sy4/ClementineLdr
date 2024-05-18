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
        }
    }
};

pub unsafe fn rva2offset(rva: u32, base_address: usize) -> Option<u32> {

    #[cfg(target_arch = "x86_64")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    #[cfg(target_arch = "x86")]
        let nt_header = (dll as usize + (*(dll as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS32;

    if (*nt_header).Signature != IMAGE_NT_SIGNATURE {
        return None;
    }

    let section_header = (*(&nt_header).OptionalHeader as usize + (*nt_header).FileHeader.SizeOfOptionalHeader as usize) as *mut IMAGE_SECTION_HEADER;

    for i in 0..(*nt_header).FileHeader.NumberOfSections {
        if rva > (*section_header).VirtualAddress[i] && rva < ((*section_header).VirtualAddress[i] + (*section_header).Misc.VirtualSize[i]) {
            Some((rva - (*section_header).VirtualAddress[i]) + (*section_header).PointerToRawData[i]) as u32;
        }
    }

    return None;
}

pub unsafe fn get_dll_address(dll_name: *const u8) -> Option<isize> {

    None;
}

fn main() {
}
