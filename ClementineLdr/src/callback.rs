
use crate::{api_hashing::get_function_address,
            api_hashing::get_module_handle,
            FARPROC, NTDLL_HASH, TP_ALLOC_WORK_HASH, TP_POST_WORK_HASH, TP_RELEASE_WORK_HASH};


#[repr(C)]
type TpAllocWork = unsafe extern "system" fn(*mut PTP_WORK, PTP_WORK_CALLBACK, *const c_void, PTP_CALLBACK_ENVIRON) -> NTSTATUS;
type TpPostWork = unsafe extern "system" fn(*mut PTP_WORK);
type TpReleaseWork = unsafe extern "system" fn(*mut PTP_WORK);


#[link_section = ".text"]
pub unsafe fn exec_callback() -> bool {

     let ntdll_address = get_module_handle(NTDLL_HASH).unwrap();

     let tp_alloc_work= get_function_address(ntdll_address, TP_ALLOC_WORK_HASH).unwrap();
     let tp_post_work= get_function_address(ntdll_address, TP_POST_WORK_HASH).unwrap();
     let tp_release_work= get_function_address(ntdll_address, TP_RELEASE_WORK_HASH).unwrap();


    return true;
}

#[repr(C)]
pub struct NtProtectVirtualMemory_Args {

}

#[repr(C)]
pub type PTP_WORK = isize;