
use windows_sys::Win32::System::Threading::{PTP_CALLBACK_INSTANCE, PTP_WORK, PTP_WORK_CALLBACK, TP_CALLBACK_ENVIRON_V3};
use crate::{
    api_hashing::{
        get_function_address,
        get_module_handle
    },
    NTDLL_HASH,
    TP_ALLOC_WORK_HASH,
    TP_POST_WORK_HASH,
    TP_RELEASE_WORK_HASH,
    NTSTATUS,
    c_void,
    asm
};

type TpAllocWork = unsafe extern "system" fn(*PTP_WORK, PTP_WORK_CALLBACK, *mut c_void, *mut TP_CALLBACK_ENVIRON_V3) -> NTSTATUS;
type TpPostWork = unsafe extern "system" fn(PTP_WORK);
type TpReleaseWork = unsafe extern "system" fn(PTP_WORK);


#[link_section = ".text"]
pub unsafe fn exec_callback(function_pointer: usize) -> bool {

    let ntdll_address = get_module_handle(NTDLL_HASH).unwrap();

    let tp_alloc_work = (*(get_function_address(ntdll_address, TP_ALLOC_WORK_HASH).unwrap())) as TpAllocWork;
    let tp_post_work = (*(get_function_address(ntdll_address, TP_POST_WORK_HASH).unwrap())) as TpPostWork;
    let tp_release_work = (*(get_function_address(ntdll_address, TP_RELEASE_WORK_HASH).unwrap())) as TpReleaseWork;

    let work_return: PTP_WORK = 0;

    tp_post_work(work_return);
    tp_release_work(work_return);

    return true;
}
#[link_section = ".text"]
pub unsafe fn work_callback() {
    asm!()
}