use core::ffi::c_char;
use core::ptr;

use stdbr_core::cpf::{self, CpfError, FiscalRegion};

use crate::{cstr_to_str, to_c_string};

/// Fiscal region mapped by the 9th digit of a CPF.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrFiscalRegion {
    Rs = 0,
    DfGoMsMtTo = 1,
    AcAmApPaRoRr = 2,
    CeMaPi = 3,
    AlPbPeRn = 4,
    BaSe = 5,
    Mg = 6,
    EsRj = 7,
    Sp = 8,
    PrSc = 9,
}

impl StdbrFiscalRegion {
    fn from_core(r: FiscalRegion) -> Self {
        // SAFETY: both enums are repr(u8) with identical discriminants 0..=9.
        unsafe { core::mem::transmute::<u8, Self>(r as u8) }
    }

    fn into_core(self) -> FiscalRegion {
        unsafe { core::mem::transmute::<u8, FiscalRegion>(self as u8) }
    }
}

/// Error codes for CPF validation. `STDBR_CPF_ERROR_OK` (0) = success.
#[repr(u8)]
pub enum StdbrCpfError {
    Ok = 0,
    InvalidLength = 1,
    InvalidCharacter = 2,
    InvalidFormat = 3,
    AllDigitsEqual = 4,
    InvalidCheckDigits = 5,
}

impl StdbrCpfError {
    fn from_core(e: &CpfError) -> Self {
        match e {
            CpfError::InvalidLength => Self::InvalidLength,
            CpfError::InvalidCharacter => Self::InvalidCharacter,
            CpfError::InvalidFormat => Self::InvalidFormat,
            CpfError::AllDigitsEqual => Self::AllDigitsEqual,
            CpfError::InvalidCheckDigits => Self::InvalidCheckDigits,
        }
    }
}

pub struct StdbrCpf(cpf::Cpf);

/// Parses a CPF string (strict). Returns `NULL` on failure.
/// Writes the error code to `*err` when `err` is not `NULL`.
///
/// # Safety
/// `raw` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_parse(
    raw: *const c_char,
    err: *mut StdbrCpfError,
) -> *mut StdbrCpf {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        if !err.is_null() {
            unsafe { *err = StdbrCpfError::InvalidLength };
        }
        return ptr::null_mut();
    };

    match s.parse::<cpf::Cpf>() {
        Result::Ok(c) => {
            if !err.is_null() {
                unsafe { *err = StdbrCpfError::Ok };
            }
            Box::into_raw(Box::new(StdbrCpf(c)))
        }
        Err(e) => {
            if !err.is_null() {
                unsafe { *err = StdbrCpfError::from_core(&e) };
            }
            ptr::null_mut()
        }
    }
}

/// Generates a random valid CPF.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cpf_create() -> *mut StdbrCpf {
    Box::into_raw(Box::new(StdbrCpf(cpf::generate_cpf())))
}

/// Generates a random valid CPF for a fiscal region.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cpf_create_for_region(region: StdbrFiscalRegion) -> *mut StdbrCpf {
    Box::into_raw(Box::new(StdbrCpf(cpf::generate_for_region(
        region.into_core(),
    ))))
}

/// Destroys a CPF handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_destroy(cpf: *mut StdbrCpf) {
    if !cpf.is_null() {
        unsafe { drop(Box::from_raw(cpf)) };
    }
}

/// Unformatted 11-digit string. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_as_str(cpf: *const StdbrCpf) -> *mut c_char {
    if cpf.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cpf }.0.as_str().into())
}

/// Formatted `###.###.###-##`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_formatted(cpf: *const StdbrCpf) -> *mut c_char {
    if cpf.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cpf }.0.to_string())
}

/// Masked `XXX.***.***-XX`. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_masked(cpf: *const StdbrCpf) -> *mut c_char {
    if cpf.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*cpf }.0.masked())
}

/// Returns the fiscal region.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_fiscal_region(cpf: *const StdbrCpf) -> StdbrFiscalRegion {
    if cpf.is_null() {
        return StdbrFiscalRegion::Rs;
    }
    StdbrFiscalRegion::from_core(unsafe { &*cpf }.0.fiscal_region())
}

/// Writes the two check digits to `*d1` and `*d2`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_check_digits(cpf: *const StdbrCpf, d1: *mut u8, d2: *mut u8) {
    if cpf.is_null() || d1.is_null() || d2.is_null() {
        return;
    }
    let (a, b) = unsafe { &*cpf }.0.check_digits();
    unsafe {
        *d1 = a;
        *d2 = b;
    }
}

/// Lenient validation strips non-digits before checking.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_is_valid(raw: *const c_char) -> bool {
    unsafe { cstr_to_str(raw) }.is_some_and(cpf::is_valid)
}

/// Strict validation. Returns a `StdbrCpfError` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_is_valid_strict(raw: *const c_char) -> StdbrCpfError {
    let Some(s) = (unsafe { cstr_to_str(raw) }) else {
        return StdbrCpfError::InvalidLength;
    };
    match cpf::is_valid_strict(s) {
        Result::Ok(()) => StdbrCpfError::Ok,
        Err(ref e) => StdbrCpfError::from_core(e),
    }
}

/// Random valid CPF as 11-digit string. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_cpf_generate() -> *mut c_char {
    to_c_string(cpf::generate())
}

/// Formats as `###.###.###-##`, or `NULL` if invalid. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_format(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .and_then(cpf::format_cpf)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Strips non-digit characters. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_remove_symbols(raw: *const c_char) -> *mut c_char {
    unsafe { cstr_to_str(raw) }
        .map(cpf::remove_symbols)
        .map_or(ptr::null_mut(), to_c_string)
}

/// Computes check digits for a 9-digit base. Returns `false` if invalid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_cpf_compute_check_digits(
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
    match cpf::compute_check_digits(s) {
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
