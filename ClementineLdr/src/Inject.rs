use crate::c_void;

#[link_section = ".text"]
pub unsafe fn inject_pe(pe_address: *mut c_void, pe_size: usize) -> Option<*mut c_void> {

    if pe_address.is_null() {
        return None;
    }

    let padded_pe_size: usize = pe_size + 4096 & !4096;

    return None;
}