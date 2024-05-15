#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use core::{ffi::c_void, arch::asm, slice::from_raw_parts};

pub use windows_sys::{
    Win32:: {
        System::{
            SystemServices::{
                DLL_PROCESS_ATTACH,
                IMAGE_DOS_HEADER,
                IMAGE_DOS_SIGNATURE,
                IMAGE_NT_SIGNATURE,
                IMAGE_EXPORT_DIRECTORY,
            },
            Threading::{
                PPS_POST_PROCESS_INIT_ROUTINE,
                RTL_USER_PROCESS_PARAMETERS
            },
            Kernel::{
                LIST_ENTRY,
                NT_TIB
            },
            Diagnostics::Debug::{IMAGE_NT_HEADERS32, IMAGE_NT_HEADERS64, IMAGE_DIRECTORY_ENTRY_EXPORT},
            WindowsProgramming::CLIENT_ID
        },
        Foundation::{BOOL, BOOLEAN, HANDLE, HMODULE, FARPROC, UNICODE_STRING},
    }
};

mod api_hashing;
mod reloc;
mod iat;
mod memory_perms;
mod callback;

pub const NTDLL_HASH: u32 = 0x99A7385F;
pub const TP_ALLOC_WORK_HASH: u32 = 0xB8CF6EF3;
pub const TP_POST_WORK_HASH: u32 = 0x8F4BD5EE;
pub const TP_RELEASE_WORK_HASH: u32 =0xAB78109;

#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub unsafe extern "system" fn _DllMainCRTStartup(
    _module: HMODULE,
    _call_reason: u32,
    _reserved: *mut c_void,
) -> BOOL {
    1
}

#[link_section = ".text"]
#[no_mangle]
pub unsafe extern "system" fn ClementineInit() {

}
