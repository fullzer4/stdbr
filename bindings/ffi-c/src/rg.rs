use core::ffi::c_char;
use core::ptr;

use stdbr_core::rg::{self, RgError};

use crate::uf::StdbrState;
use crate::{cstr_to_str, to_c_string};

/// Error codes for RG validation. `STDBR_RG_ERROR_OK` (0) = success.
#[repr(u8)]
pub enum StdbrRgError {
    Ok = 0,
    InvalidLength = 1,
    InvalidCharacter = 2,
    InvalidFormat = 3,
    InvalidCheckDigit = 4,
    UnsupportedUfForGeneration = 5,
}

impl StdbrRgError {
    fn from_core(e: &RgError) -> Self {
        match e {
            RgError::InvalidLength => Self::InvalidLength,
            RgError::InvalidCharacter => Self::InvalidCharacter,
            RgError::InvalidFormat => Self::InvalidFormat,
            RgError::InvalidCheckDigit => Self::InvalidCheckDigit,
            RgError::UnsupportedUfForGeneration => Self::UnsupportedUfForGeneration,
        }
    }
}

pub struct StdbrRg(rg::Rg);

/// Parses an RG string for the given UF. Returns `NULL` on failure.
///
/// # Safety
/// `raw` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_parse(
    raw: *const c_char,
    uf: StdbrState,
    err: *mut StdbrRgError,
) -> *mut StdbrRg {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        if !err.is_null() {
            unsafe { *err = StdbrRgError::InvalidLength };
        }
        return ptr::null_mut();
    };

    match rg::parse_strict(s, uf.into_core()) {
        Result::Ok(r) => {
            if !err.is_null() {
                unsafe { *err = StdbrRgError::Ok };
            }
            Box::into_raw(Box::new(StdbrRg(r)))
        }
        Err(e) => {
            if !err.is_null() {
                unsafe { *err = StdbrRgError::from_core(&e) };
            }
            ptr::null_mut()
        }
    }
}

/// Generates a random valid RG for the given UF (currently SP only).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_create_for_uf(
    uf: StdbrState,
    err: *mut StdbrRgError,
) -> *mut StdbrRg {
    match rg::generate_for_uf(uf.into_core()) {
        Result::Ok(r) => {
            if !err.is_null() {
                unsafe { *err = StdbrRgError::Ok };
            }
            Box::into_raw(Box::new(StdbrRg(r)))
        }
        Err(e) => {
            if !err.is_null() {
                unsafe { *err = StdbrRgError::from_core(&e) };
            }
            ptr::null_mut()
        }
    }
}

/// Destroys an RG handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_destroy(rg: *mut StdbrRg) {
    if !rg.is_null() {
        unsafe { drop(Box::from_raw(rg)) };
    }
}

/// Unformatted body. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_as_str(rg: *const StdbrRg) -> *mut c_char {
    if rg.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*rg }.0.as_str().into())
}

/// Formatted per the UF mask. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_formatted(rg: *const StdbrRg) -> *mut c_char {
    if rg.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*rg }.0.formatted())
}

/// Returns the issuing UF.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_uf(rg: *const StdbrRg) -> StdbrState {
    if rg.is_null() {
        return StdbrState::AC;
    }
    StdbrState::from_core(unsafe { &*rg }.0.uf())
}

/// Writes the check digit to `*out`. Returns `true` if a digit exists
/// (SP only). For UFs without a verified algorithm, returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_check_digit(rg: *const StdbrRg, out: *mut u8) -> bool {
    if rg.is_null() || out.is_null() {
        return false;
    }
    match unsafe { &*rg }.0.check_digit() {
        Some(d) => {
            unsafe { *out = d };
            true
        }
        None => false,
    }
}

/// Lenient validation strips separators before checking.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_is_valid(raw: *const c_char, uf: StdbrState) -> bool {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return false;
    };
    rg::is_valid(s, uf.into_core())
}

/// Strict validation. Returns a `StdbrRgError` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_is_valid_strict(
    raw: *const c_char,
    uf: StdbrState,
) -> StdbrRgError {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return StdbrRgError::InvalidLength;
    };
    match rg::is_valid_strict(s, uf.into_core()) {
        Result::Ok(()) => StdbrRgError::Ok,
        Err(ref e) => StdbrRgError::from_core(e),
    }
}

/// Formats per the UF mask. Returns `NULL` if length is wrong.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_format(raw: *const c_char, uf: StdbrState) -> *mut c_char {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return ptr::null_mut();
    };
    rg::format_rg(s, uf.into_core()).map_or(ptr::null_mut(), to_c_string)
}

/// Strips separators per UF rules. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_remove_symbols(
    raw: *const c_char,
    uf: StdbrState,
) -> *mut c_char {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return ptr::null_mut();
    };
    to_c_string(rg::remove_symbols(s, uf.into_core()))
}

/// SP-only: compute the check digit. Returns `true` and writes to `*out`
/// (10 = `'X'` terminator). Returns `false` for non-SP or wrong length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_rg_compute_check_digit(
    base: *const c_char,
    uf: StdbrState,
    out: *mut u8,
) -> bool {
    if out.is_null() {
        return false;
    }
    let Some(s) = (unsafe { cstr_to_str(base) }) else {
        return false;
    };
    match rg::compute_check_digit(s, uf.into_core()) {
        Some(d) => {
            unsafe { *out = d };
            true
        }
        None => false,
    }
}

/// Random valid SP RG. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_rg_generate_sp() -> *mut c_char {
    to_c_string(rg::generate_sp().as_str().into())
}
