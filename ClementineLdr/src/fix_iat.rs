use crate::{
    reloc::IMAGE_DATA_DIRECTORY,
    callback::{
        exec_callback,
        load_library_args,
        PTP_WORK_CALLBACK
    },
    api_hashing::{
        get_function_address, dbj2_hash
    },
    LOAD_LIBRARY_A_HASH,
    callback::loadlibrary_callback,
    IMAGE_THUNK_DATA64,
    IMAGE_THUNK_DATA32,
    IMAGE_ORDINAL_FLAG64,
    IMAGE_IMPORT_BY_NAME,
    c_void
};

#[link_section = ".text"]
pub unsafe fn fix_iat(data_directory: *const IMAGE_DATA_DIRECTORY, base_address: usize, kernel32_address: isize,
                      ntdll_address: isize) -> bool {

    if data_directory.is_null() {
        return false;
    }

    let mut import_descriptor = (base_address + (*data_directory).VirtualAddress as usize) as *mut IMAGE_IMPORT_DESCRIPTOR;

    let load_library_ptr = get_function_address(kernel32_address, LOAD_LIBRARY_A_HASH).unwrap();

    if import_descriptor.is_null() || load_library_ptr == 0 {
        return false;
    }


    while (*import_descriptor).Name != 0 {
        let dll_name = (base_address + (*import_descriptor).Name as usize) as *const i8;

        if dll_name.is_null()
        {
            return false;
        }

        let mut args = load_library_args {
            function_pointer: load_library_ptr,
            library_name: dll_name
        };

        let ptr: *mut load_library_args = &mut args;

        exec_callback(*(loadlibrary_callback) as *mut PTP_WORK_CALLBACK, ptr as *mut c_void, ntdll_address);

        #[cfg(target_arch = "x86_64")]
            let mut original_first_thunk = if (base_address + (*import_descriptor).Anonymous.OriginalFirstThunk as usize) != 0 {
            let temp = (base_address + (*import_descriptor).Anonymous.OriginalFirstThunk as usize) as *mut IMAGE_THUNK_DATA64;
            temp
        } else {
            let temp = (base_address + (*import_descriptor).FirstThunk as usize) as *mut IMAGE_THUNK_DATA64;
            temp
        };
        let mut thunk = (base_address + (*import_descriptor).FirstThunk as usize) as *mut IMAGE_THUNK_DATA64;

        #[cfg(target_arch = "x86")]
        {
            let mut original_first_thunk = if (base_address + (*import_descriptor).Anonymous.OriginalFirstThunk as usize) != 0 {
                let temp = (base_address + (*import_descriptor).Anonymous.OriginalFirstThunk as usize) as *mut IMAGE_THUNK_DATA32;
                temp
            } else {
                let temp = (base_address + (*import_descriptor).FirstThunk as usize) as *mut IMAGE_THUNK_DATA32;
                temp
            };
            let mut thunk = (base_address + (*import_descriptor).FirstThunk as usize) as *mut IMAGE_THUNK_DATA32;
        }

        while (*original_first_thunk).u1.Function != 0 {
            let snap = ((*original_first_thunk).u1.Ordinal & IMAGE_ORDINAL_FLAG64) != 0;

            if snap {
                let ordinal = ((*original_first_thunk).u1.Ordinal & 0xffff) as *const u8;

            } else {
                let thunk_data = (base_address + (*original_first_thunk).u1.AddressOfData as usize) as *mut IMAGE_IMPORT_BY_NAME;
                let name = dbj2_hash((*thunk_data).Name.as_slice());
            }

            thunk = thunk.add(1);
            original_first_thunk = original_first_thunk.add(1);
        }

        import_descriptor = import_descriptor.add(1);
    }

   return true;
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