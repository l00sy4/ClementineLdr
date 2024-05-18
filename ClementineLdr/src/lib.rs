#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use core::{ffi::c_void, arch::asm, slice::from_raw_parts, mem::size_of};

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
                RTL_USER_PROCESS_PARAMETERS,
                PTP_CALLBACK_INSTANCE, PTP_WORK,
                PTP_WORK_CALLBACK,
                TP_CALLBACK_ENVIRON_V3,
            },
            Kernel::{
                LIST_ENTRY,
                NT_TIB
            },
            Diagnostics::Debug::{IMAGE_NT_HEADERS32, IMAGE_NT_HEADERS64, IMAGE_DIRECTORY_ENTRY_EXPORT},
            WindowsProgramming::CLIENT_ID
        },
        Foundation::{NTSTATUS, BOOL, BOOLEAN, HMODULE, FARPROC, UNICODE_STRING},
    }
};

mod api_hashing;
mod reloc;
mod iat;
mod memory_perms;
mod callback;
mod sleep;

pub const TP_ALLOC_WORK_HASH: u32 = 0xB8CF6EF3;
pub const TP_POST_WORK_HASH: u32 = 0x8F4BD5EE;
pub const TP_RELEASE_WORK_HASH: u32 = 0xAB78109;
pub const LOAD_LIBRARY_A_HASH: u32 =  0x514D6A17;
pub const NT_PROTECT_HASH: u32 = 0x3D7A5DC4;
pub const NT_ALLOC_HASH: u32 = 0x763B95A8;

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
pub unsafe extern "system" fn ClementineInit(dll_address: *mut c_void, kernel32_address: isize, ntdll_address: isize) {

    if dll_address.is_null() {
        return;
    }

    let base_address = dll_address as usize;
    #[cfg(target_arch = "x86_64")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    #[cfg(target_arch = "x86")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS32;

    let dll_size = (*nt_header).OptionalHeader.SizeOfImage as usize;
    let preferred_dll_address = (*nt_header).OptionalHeader.ImageBase as usize;

    /* If loaded at it's preferred address, skip relocation and IAT reparation */
}
