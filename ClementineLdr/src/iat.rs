use crate::reloc::IMAGE_DATA_DIRECTORY;

#[link_section = ".text"]
pub unsafe fn fix_iat(data_directory: *const IMAGE_DATA_DIRECTORY, base_address: usize) -> bool {



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