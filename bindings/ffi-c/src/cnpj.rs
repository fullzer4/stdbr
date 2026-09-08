use core::ffi::c_char;
use core::ptr;

use stdbr_core::cnpj::{self, CnpjError};

use crate::{cstr_to_str, to_c_string};

/// Whether the CNPJ uses only digits or also contains letters.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrCnpjKind {
    Numeric = 0,
    Alphanumeric = 1,
    /// Sentinel returned when a CNPJ kind cannot be read.
    Invalid = 255,
}

impl StdbrCnpjKind {
    fn core_from_raw(value: u8) -> Option<cnpj::CnpjKind> {
        match value {
            0 => Some(cnpj::CnpjKind::Numeric),
            1 => Some(cnpj::CnpjKind::Alphanumeric),
            _ => None,
        }
    }

    fn from_core(k: cnpj::CnpjKind) -> Self {
        match k {
            cnpj::CnpjKind::Numeric => Self::Numeric,
            cnpj::CnpjKind::Alphanumeric => Self::Alphanumeric,
        }
    }
}

/// Whether the establishment is Matriz or Filial.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrEstablishmentType {
    Matriz = 0,
    Filial = 1,
    /// Sentinel returned when an establishment type cannot be read.
    Invalid = 255,
}

impl StdbrEstablishmentType {
    fn from_core(t: cnpj::EstablishmentType) -> Self {
        match t {
            cnpj::EstablishmentType::Matriz => Self::Matriz,
            cnpj::EstablishmentType::Filial => Self::Filial,
        }
    }
}

/// Error codes for CNPJ validation. `STDBR_CNPJ_ERROR_OK` (0) = success.
#[repr(u8)]
pub enum StdbrCnpjError {
    Ok = 0,
    InvalidLength = 1,
    InvalidCharacter = 2,
    InvalidFormat = 3,
    AllCharsEqual = 4,
    InvalidCheckDigits = 5,
}

impl StdbrCnpjError {
    fn from_core(e: &CnpjError) -> Self {
        match e {
            CnpjError::InvalidLength => Self::InvalidLength,
            CnpjError::InvalidCharacter => Self::InvalidCharacter,
            CnpjError::InvalidFormat => Self::InvalidFormat,
            CnpjError::AllCharsEqual => Self::AllCharsEqual,
            CnpjError::InvalidCheckDigits => Self::InvalidCheckDigits,
        }
    }
}

pub struct StdbrCnpj(cnpj::Cnpj);

/// Parses a CNPJ string (strict). Returns `NULL` on failure.
/// Writes the error code to `*err` when `err` is not `NULL`.
///
/// # Safety
/// `raw` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_parse(
    raw: *const c_char,
    err: *mut StdbrCnpjError,
) -> *mut StdbrCnpj {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        if !err.is_null() {
            unsafe { *err = StdbrCnpjError::InvalidLength };
        }
        return ptr::null_mut();
    };

    match s.parse::<cnpj::Cnpj>() {
        Result::Ok(c) => {
            if !err.is_null() {
                unsafe { *err = StdbrCnpjError::Ok };
            }
            Box::into_raw(Box::new(StdbrCnpj(c)))
        }
        Err(e) => {
            if !err.is_null() {
                unsafe { *err = StdbrCnpjError::from_core(&e) };
            }
            ptr::null_mut()
        }
    }
}

/// Generates a random valid CNPJ, or `NULL` if `kind` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cnpj_create(kind: u8) -> *mut StdbrCnpj {
    let Some(kind) = StdbrCnpjKind::core_from_raw(kind) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(StdbrCnpj(cnpj::generate_cnpj(kind))))
}

/// Generates a Matriz CNPJ, or `NULL` if `kind` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cnpj_create_matriz(kind: u8) -> *mut StdbrCnpj {
    let Some(kind) = StdbrCnpjKind::core_from_raw(kind) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(StdbrCnpj(cnpj::generate_matriz(kind))))
}

/// Destroys a CNPJ handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_destroy(cnpj: *mut StdbrCnpj) {
    if !cnpj.is_null() {
        unsafe { drop(Box::from_raw(cnpj)) };
    }
}

/// Unformatted 14-character string. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_as_str(cnpj: *const StdbrCnpj) -> *mut c_char {
    if cnpj.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cnpj }.0.as_str().into())
}

/// Formatted `XX.XXX.XXX/XXXX-DD`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_formatted(cnpj: *const StdbrCnpj) -> *mut c_char {
    if cnpj.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cnpj }.0.to_string())
}

/// Masked `XX.XXX.XXX/****-**`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_masked(cnpj: *const StdbrCnpj) -> *mut c_char {
    if cnpj.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cnpj }.0.masked())
}

/// Returns the CNPJ kind, or `STDBR_CNPJ_KIND_INVALID` when `cnpj` is `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_kind(cnpj: *const StdbrCnpj) -> StdbrCnpjKind {
    if cnpj.is_null() {
        return StdbrCnpjKind::Invalid;
    }
    StdbrCnpjKind::from_core(unsafe { &*cnpj }.0.kind())
}

/// Returns the establishment type, or `STDBR_ESTABLISHMENT_TYPE_INVALID` when `cnpj` is `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_establishment_type(
    cnpj: *const StdbrCnpj,
) -> StdbrEstablishmentType {
    if cnpj.is_null() {
        return StdbrEstablishmentType::Invalid;
    }
    StdbrEstablishmentType::from_core(unsafe { &*cnpj }.0.establishment_type())
}

/// Writes both check digits and returns `true`; returns `false` for any `NULL` pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_check_digits(
    cnpj: *const StdbrCnpj,
    d1: *mut u8,
    d2: *mut u8,
) -> bool {
    if cnpj.is_null() || d1.is_null() || d2.is_null() {
        return false;
    }
    let (a, b) = unsafe { &*cnpj }.0.check_digits();
    unsafe {
        *d1 = a;
        *d2 = b;
    }
    true
}

/// Lenient validation strips non-alphanumeric chars before checking.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_is_valid(raw: *const c_char) -> bool {
    unsafe { cstr_to_str(raw) }.is_some_and(cnpj::is_valid)
}

/// Strict validation. Returns a `StdbrCnpjError` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_is_valid_strict(raw: *const c_char) -> StdbrCnpjError {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return StdbrCnpjError::InvalidLength;
    };
    match cnpj::is_valid_strict(s) {
        Result::Ok(()) => StdbrCnpjError::Ok,
        Err(ref e) => StdbrCnpjError::from_core(e),
    }
}

/// Returns a random CNPJ string, or `NULL` if `kind` is invalid. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cnpj_generate(kind: u8) -> *mut c_char {
    StdbrCnpjKind::core_from_raw(kind)
        .map(cnpj::generate)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Formats as `XX.XXX.XXX/XXXX-DD`, or `NULL` if invalid. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_format(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .and_then(cnpj::format_cnpj)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Strips non-alphanumeric characters, uppercases. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_remove_symbols(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .map(cnpj::remove_symbols)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Computes check digits for a 12-character base. Returns `false` if invalid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cnpj_compute_check_digits(
    base: *const c_char,
    d1: *mut u8,
    d2: *mut u8,
) -> bool {
    if d1.is_null() || d2.is_null() {
        return false;
    }
    let Some(s) = (unsafe { cstr_to_str(base) }) else {
        return false;
    };
    match cnpj::compute_check_digits(s) {
        Some((a, b)) => {
            unsafe {
                *d1 = a;
                *d2 = b;
            }
            true
        }
        None => false,
    }
}
