use crate::reloc::IMAGE_DATA_DIRECTORY;
use crate::callback::functions_enum;

#[link_section = ".text"]
pub unsafe fn fix_iat(data_directory: *const IMAGE_DATA_DIRECTORY, base_address: usize) -> bool {

    if data_directory.is_null() {
        return false;
    }

    let mut import_descriptor =(base_address + (*data_directory).VirtualAddress as usize) as *mut IMAGE_IMPORT_DESCRIPTOR;

    if import_descriptor.is_null() {
        return false;
    }

    while (*import_descriptor).Name != 0 {

        let dll_name = (base_address + (*import_descriptor).Name as usize) as *const u8;

        if dll_name.is_null() {
            return false;
        }

        let load_library = functions_enum::load_library_args;


    }

    false
}

#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    pub Name: u32,
    pub FirstThunk: u32,
}

#[repr(C)]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    pub Characteristics: u32,
    pub OriginalFirstThunk: u32,
}