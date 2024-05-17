
use crate::IMAGE_NT_HEADERS64;
#[link_section = ".text"]
pub unsafe fn fix_mem_perms(base_address: usize, nt_headers: *const IMAGE_NT_HEADERS64, section_header: *IMAGE_SECTION_HEADER) -> bool {

    if nt_headers.is_null() || section_header.is_null() {
        return false;
    }

    for i in (*nt_headers).FileHeader.NumberOfSections {

        let protection: u32 = 0;
        let old_protection: u32 = 0;

        match (*section_header).Characteristics[i] {

        }

    }



    return true;
}

#[repr(C)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    pub VirtualSize: u32,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: u32,
}