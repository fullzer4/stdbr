mod cep;
mod cnpj;
mod cpf;
mod municipio;
pub(crate) mod uf;

use core::ffi::{CStr, c_char};
use std::ffi::CString;

/// # Safety
/// `ptr` must be null or a valid null-terminated C string.
pub(crate) unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_str().ok())?
}

pub(crate) fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Frees a string previously returned by stdbr functions.
///
/// # Safety
/// `ptr` must have been returned by a stdbr function, or be `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)) };
    }
}
