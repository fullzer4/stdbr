use core::ffi::c_char;
use core::ptr;

use stdbr_core::uf as core_uf;

use crate::to_c_string;

/// Brazilian state.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrState {
    AC = 0,
    AL = 1,
    AM = 2,
    AP = 3,
    BA = 4,
    CE = 5,
    DF = 6,
    ES = 7,
    GO = 8,
    MA = 9,
    MG = 10,
    MS = 11,
    MT = 12,
    PA = 13,
    PB = 14,
    PE = 15,
    PI = 16,
    PR = 17,
    RJ = 18,
    RN = 19,
    RO = 20,
    RR = 21,
    RS = 22,
    SC = 23,
    SE = 24,
    SP = 25,
    TO = 26,
    /// Sentinel returned when a state cannot be read.
    Invalid = 255,
}

impl StdbrState {
    pub(crate) fn core_from_raw(value: u8) -> Option<core_uf::State> {
        core_uf::ALL.get(value as usize).copied()
    }

    pub(crate) fn from_core(s: core_uf::State) -> Self {
        // SAFETY: both enums are repr(u8) 0..=26 with same variant order.
        unsafe { core::mem::transmute(s as u8) }
    }
}

/// Geographic region.
#[repr(u8)]
#[allow(dead_code)]
pub enum StdbrRegion {
    Norte = 0,
    Nordeste = 1,
    CentroOeste = 2,
    Sudeste = 3,
    Sul = 4,
    /// Sentinel returned for an invalid state.
    Invalid = 255,
}

impl StdbrRegion {
    fn from_core(r: core_uf::Region) -> Self {
        // SAFETY: both enums are repr(u8) 0..=4 with same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

/// Returns the two-letter abbreviation, or `NULL` for an invalid state. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_abbreviation(state: u8) -> *mut c_char {
    StdbrState::core_from_raw(state).map_or(ptr::null_mut(), |state| {
        to_c_string(state.abbreviation().into())
    })
}

/// Returns the full name, or `NULL` for an invalid state. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_name(state: u8) -> *mut c_char {
    StdbrState::core_from_raw(state)
        .map_or(ptr::null_mut(), |state| to_c_string(state.name().into()))
}

/// Returns the geographic region, or `STDBR_REGION_INVALID` for an invalid state.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_region(state: u8) -> StdbrRegion {
    StdbrState::core_from_raw(state).map_or(StdbrRegion::Invalid, |state| {
        StdbrRegion::from_core(state.region())
    })
}

/// Parse a state from its two-letter abbreviation (case-insensitive).
/// Returns `true` on success and writes the result to `*out`.
///
/// # Safety
/// `abbr` must be a valid null-terminated UTF-8 string.
/// `out` must be a valid pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_state_from_abbreviation(
    abbr: *const c_char,
    out: *mut StdbrState,
) -> bool {
    if out.is_null() {
        return false;
    }
    let Some(s) = (unsafe { crate::cstr_to_str(abbr) }) else {
        return false;
    };
    match core_uf::State::from_abbreviation(s) {
        Some(state) => {
            unsafe { *out = StdbrState::from_core(state) };
            true
        }
        None => false,
    }
}

/// Writes all 27 states to `buf`; returns 27 on success or 0 when `buf` is `NULL`.
///
/// # Safety
/// `buf` must point to a valid array of at least 27 `StdbrState` elements.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_all_states(buf: *mut StdbrState) -> u32 {
    if buf.is_null() {
        return 0;
    }
    for (i, &state) in core_uf::ALL.iter().enumerate() {
        unsafe { ptr::write(buf.add(i), StdbrState::from_core(state)) };
    }
    27
}
