use core::mem::size_of;

#[link_section = ".text"]
pub unsafe fn fix_reloc(data_directory: *const IMAGE_DATA_DIRECTORY, base_address: usize, relocation_address: usize) -> bool {
    let mut image_base_relocation = (base_address + (*data_directory).VirtualAddress as usize) as *mut IMAGE_BASE_RELOCATION;
    let delta: usize = base_address - relocation_address;

    while (*image_base_relocation).VirtualAddress {
        let base_relocation_entry = (image_base_relocation as usize + 1) as *mut BASE_RELOCATION_ENTRY;
        let number_of_entries: usize = (*image_base_relocation).SizeOfBlock as usize - (size_of::<IMAGE_BASE_RELOCATION>()) / size_of::<u16>();

        for _ in 0..number_of_entries {

            match (*base_relocation_entry).Type {

                10 => *((base_address + (*image_base_relocation).VirtualAddress as usize + (*base_relocation_entry).Offset as usize) as *mut u64) as usize += delta,
                3 => *((base_address + (*image_base_relocation).VirtualAddress as usize + (*base_relocation_entry).Offset as usize) as *mut u32) as usize += delta >> 16,
                1 => *((base_address + (*image_base_relocation).VirtualAddress as usize + (*base_relocation_entry).Offset as usize) as *mut u16) as usize += delta,
                0 => (),
                _ => { return false;}
            }

            base_relocation_entry as usize += 1;
        }
        image_base_relocation = (image_base_relocation as usize + (*image_base_relocation).SizeOfBlock as usize) as *mut IMAGE_BASE_RELOCATION;
    }

    return true;
}


#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}

#[repr(C)]
pub struct IMAGE_BASE_RELOCATION
{
    pub VirtualAddress: u32, // Page where the relocations will be performed
    pub SizeOfBlock: u32
}

#[repr(C)]
pub struct BASE_RELOCATION_ENTRY
{
    pub Offset: u32, // Offset to the pointer who will be relocated
    pub Type: u32
}