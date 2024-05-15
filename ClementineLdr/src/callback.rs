use crate::{
    api_hashing::{
        get_function_address,
        get_module_handle
    },
    FARPROC,
    NTDLL_HASH,
    TP_ALLOC_WORK_HASH,
    TP_POST_WORK_HASH,
    TP_RELEASE_WORK_HASH,
    NTSTATUS,
    c_void
};

type TpAllocWork = unsafe extern "system" fn(*mut PTP_WORK, PTP_WORK_CALLBACK, *mut c_void, *mut PTP_CALLBACK_ENVIRON_V3) -> NTSTATUS;
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
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
#[repr(transparent)]
pub struct PTP_WORK(pub isize);
#[repr(transparent)]
pub struct PTP_POOL(pub isize);
#[repr(transparent)]
pub struct PTP_CLEANUP_GROUP(pub isize);
#[repr(transparent)]
pub struct PTP_CALLBACK_INSTANCE(pub isize);
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
#[repr(transparent)]
pub struct TP_CALLBACK_PRIORITY(pub i32);
pub type PTP_SIMPLE_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut c_void)>;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<unsafe extern "system" fn(objectcontext: *mut c_void, cleanupcontext: *mut c_void)>;
pub type PTP_WORK_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut c_void, work: PTP_WORK)>;