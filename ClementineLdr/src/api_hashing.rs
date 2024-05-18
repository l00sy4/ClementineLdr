use crate::{
    HANDLE,
    BOOLEAN,
    asm,
    from_raw_parts,
    IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_NT_SIGNATURE,
    IMAGE_NT_HEADERS64,
    IMAGE_DOS_HEADER,
    HMODULE,
    LIST_ENTRY,
    c_void,
    IMAGE_EXPORT_DIRECTORY,
    RTL_USER_PROCESS_PARAMETERS,
    PPS_POST_PROCESS_INIT_ROUTINE,
    UNICODE_STRING
};

 #[link_section = ".text"]
pub unsafe fn get_function_address(dll: HMODULE, function_hash: u32) -> Option<usize> {

     #[cfg(target_arch = "x86_64")]
         let nt_header = (dll as usize + (*(dll as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut IMAGE_NT_HEADERS64;
     #[cfg(target_arch = "x86")]
         let nt_header = (dll as usize + (*(dll as *mut IMAGE_DOS_HEADER)).e_lfanew as usize) as *mut crate::IMAGE_NT_HEADERS32;

     if (*nt_header).Signature != IMAGE_NT_SIGNATURE
     {
         return None;
     }

     let export_directory = (dll as usize + (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT as usize].VirtualAddress as usize) as *mut IMAGE_EXPORT_DIRECTORY;
     let names = from_raw_parts((dll as usize + (*export_directory).AddressOfNames as usize) as *const u32, (*export_directory).NumberOfNames as _);
     let functions = from_raw_parts((dll as usize + (*export_directory).AddressOfFunctions as usize) as *const u32, (*export_directory).NumberOfFunctions as _,);
     let ordinals = from_raw_parts((dll as usize + (*export_directory).AddressOfNameOrdinals as usize) as *const u16, (*export_directory).NumberOfNames as _);

     for i in 0..(*export_directory).NumberOfFunctions {

         let function_name = (dll as usize + names[i as usize] as usize) as *const u8;
         let function_address = dll as usize + functions[ordinals[i]] as usize;
         let name_slice: &[u8] = from_raw_parts(function_name,get_cstring_length(function_name as *const char));

         if dbj2_hash(name_slice) == function_hash {
             return Some(*function_address as usize);
         }
     }

     None
}

#[link_section = ".text"]
pub fn dbj2_hash(buffer: &[u8]) -> u32
{
    let mut hash: u32 = 5441;
    let mut i: usize = 0;
    let mut cur: u8;

    while i < buffer.len()
    {
        cur = buffer[i];

        if cur == 0
        {
            i += 1;
            continue;
        }

        if cur >= ('a' as u8)
        {
            cur -= 0x20;
        }

        hash = ((hash << 5).wrapping_add(hash)) + cur as u32;
        i += 1;
    }

    return hash;
}

#[link_section = ".text"]
pub unsafe fn get_cstring_length(string: *const char) -> usize
{
    let mut temp: usize = string as usize;

    while *(temp as *const u8) != 0
    {
        temp += 1;
    }

    temp - string as usize
}

#[link_section = ".text"]
pub unsafe fn GetPEB() -> *mut PEB {
    let address: *mut PEB;
    #[cfg(target_arch = "x86_64")]
        asm!(
        "xor rax, rax",
        "mov rdx, rax",
        "add dl, 0x65",
        "sub rdx, 0x5",
        "mov {address}, QWORD PTR gs:[rax + rdx * 1]",
        address = out(reg) address
        );
    #[cfg(target_arch = "x86")]
        asm!(
        "xor eax, eax",
        "mov ebx, eax",
        "add bl, 0x47",
        "sub ebx, 0x17",
        "mov {address}, DWORD PTR fs:[eax + ebx * 1]",
        address = out(reg) address
        );
        address
}

#[repr(C)]
pub struct PEB {

    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut c_void; 3],
    pub AtlThunkSListPtr: *mut c_void,
    pub Reserved5: *mut c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut c_void; 1],
    pub SessionId: u32,
}

#[repr(C)]
pub struct PEB_LDR_DATA {
    pub Length: u32,
    pub Initialized: BOOLEAN,
    pub SsHandle: HANDLE,
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
    pub EntryInProgress: *mut c_void,
    pub ShutdownInProgress: BOOLEAN,
    pub ShutdownThreadId: HANDLE,
}

#[repr(C)]
pub union LDR_DATA_TABLE_ENTRY_u1 {
    pub InInitializationOrderLinks: LIST_ENTRY,
    pub InProgressLinks: LIST_ENTRY,
}

pub type PLDR_INIT_ROUTINE = Option<unsafe extern "system" fn(DllHandle: *mut c_void, Reason: u32, Context: *mut c_void) -> BOOLEAN>;

#[repr(C)]
pub struct LDR_DATA_TABLE_ENTRY
{
    pub InLoadOrderLinks: LIST_ENTRY,
    pub InMemoryOrderLinks: LIST_ENTRY,
    pub u1: LDR_DATA_TABLE_ENTRY_u1,
    pub DllBase: *mut c_void,
    pub EntryPoint: PLDR_INIT_ROUTINE,
    pub SizeOfImage: u32,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
}