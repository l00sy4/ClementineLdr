use crate::{
    api_hashing::get_function_address,
    FARPROC
};
use crate::api_hashing::get_module_handle;

#[repr(C)]
pub struct NtProtectVirtualMemory_Args {

}

#[link_section = ".text"]
pub unsafe fn exec_callback() -> bool {

    FARPROC tp_alloc_work = get_function_address(get_module_handle(), );



    false
}