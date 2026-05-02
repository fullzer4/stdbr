use core::ffi::c_char;
use core::ptr;

use stdbr_core::cep::{self, CepError};

use crate::uf::StdbrState;
use crate::{cstr_to_str, to_c_string};

/// Postal region mapped by the first digit of a CEP.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrPostalRegion {
    GranSaoPaulo = 0,
    InteriorSaoPaulo = 1,
    RjEs = 2,
    Mg = 3,
    BaSe = 4,
    PeAlPbRn = 5,
    CePiMaPaAmAcApRr = 6,
    DfGoToMtMsRo = 7,
    PrSc = 8,
    Rs = 9,
}

impl StdbrPostalRegion {
    fn from_core(r: cep::PostalRegion) -> Self {
        // SAFETY: both enums are repr(u8) 0..=9 with same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }

    fn into_core(self) -> cep::PostalRegion {
        // SAFETY: both enums are repr(u8) 0..=9 with same variant order.
        unsafe { core::mem::transmute(self as u8) }
    }
}

/// Error codes for CEP validation.
#[repr(u8)]
pub enum StdbrCepError {
    Ok = 0,
    InvalidLength = 1,
    InvalidCharacter = 2,
    InvalidFormat = 3,
}

impl StdbrCepError {
    fn from_core(e: &CepError) -> Self {
        match e {
            CepError::InvalidLength => Self::InvalidLength,
            CepError::InvalidCharacter => Self::InvalidCharacter,
            CepError::InvalidFormat => Self::InvalidFormat,
        }
    }
}

pub struct StdbrCep(cep::Cep);

/// Parses a CEP string (strict). Returns `NULL` on failure.
/// Writes the error code to `*err` when `err` is not `NULL`.
///
/// # Safety
/// `raw` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_parse(
    raw: *const c_char,
    err: *mut StdbrCepError,
) -> *mut StdbrCep {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        if !err.is_null() {
            unsafe { *err = StdbrCepError::InvalidLength };
        }
        return ptr::null_mut();
    };

    match s.parse::<cep::Cep>() {
        Result::Ok(c) => {
            if !err.is_null() {
                unsafe { *err = StdbrCepError::Ok };
            }
            Box::into_raw(Box::new(StdbrCep(c)))
        }
        Err(e) => {
            if !err.is_null() {
                unsafe { *err = StdbrCepError::from_core(&e) };
            }
            ptr::null_mut()
        }
    }
}

/// Generates a random valid CEP.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cep_create() -> *mut StdbrCep {
    Box::into_raw(Box::new(StdbrCep(cep::generate_cep())))
}

/// Generates a random CEP for a postal region.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cep_create_for_region(region: StdbrPostalRegion) -> *mut StdbrCep {
    Box::into_raw(Box::new(StdbrCep(cep::generate_for_region(
        region.into_core(),
    ))))
}

/// Generates a random CEP within the range of a given state.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cep_create_for_state(state: StdbrState) -> *mut StdbrCep {
    Box::into_raw(Box::new(StdbrCep(cep::generate_for_state(
        state.into_core(),
    ))))
}

/// Destroys a CEP handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_destroy(cep: *mut StdbrCep) {
    if !cep.is_null() {
        unsafe { drop(Box::from_raw(cep)) };
    }
}

/// Unformatted 8-digit string. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_as_str(cep: *const StdbrCep) -> *mut c_char {
    if cep.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cep }.0.as_str().into())
}

/// Formatted `#####-###`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_formatted(cep: *const StdbrCep) -> *mut c_char {
    if cep.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cep }.0.formatted())
}

/// Masked `#####-***`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_masked(cep: *const StdbrCep) -> *mut c_char {
    if cep.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cep }.0.masked())
}

/// Returns the postal region.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_postal_region(cep: *const StdbrCep) -> StdbrPostalRegion {
    if cep.is_null() {
        return StdbrPostalRegion::GranSaoPaulo;
    }
    StdbrPostalRegion::from_core(unsafe { &*cep }.0.postal_region())
}

/// Returns the state. Writes the result to `*out` and returns `true`.
/// Returns `false` if the CEP doesn't map to a known state.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_state(cep: *const StdbrCep, out: *mut StdbrState) -> bool {
    if cep.is_null() || out.is_null() {
        return false;
    }
    match unsafe { &*cep }.0.state() {
        Some(s) => {
            unsafe { *out = StdbrState::from_core(s) };
            true
        }
        None => false,
    }
}

/// Lenient validation strips non-digits before checking.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_is_valid(raw: *const c_char) -> bool {
    unsafe { cstr_to_str(raw) }.is_some_and(cep::is_valid)
}

/// Strict validation. Returns a `StdbrCepError` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_is_valid_strict(raw: *const c_char) -> StdbrCepError {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return StdbrCepError::InvalidLength;
    };
    match cep::is_valid_strict(s) {
        Result::Ok(()) => StdbrCepError::Ok,
        Err(ref e) => StdbrCepError::from_core(e),
    }
}

/// Random valid CEP as 8-digit string. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cep_generate() -> *mut c_char {
    to_c_string(cep::generate())
}

/// Formats as `#####-###`, or `NULL` if invalid. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_format(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .and_then(cep::format_cep)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Strips non-digit characters. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cep_remove_symbols(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .map(cep::remove_symbols)
        .map_or(ptr::null_mut(), to_c_string)
}
