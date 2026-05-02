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
}

impl StdbrState {
    pub(crate) fn into_core(self) -> core_uf::State {
        // SAFETY: both enums are repr(u8) 0..=26 with same variant order.
        unsafe { core::mem::transmute(self as u8) }
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
}

impl StdbrRegion {
    fn from_core(r: core_uf::Region) -> Self {
        // SAFETY: both enums are repr(u8) 0..=4 with same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

/// Returns the two-letter abbreviation of a state. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_abbreviation(state: StdbrState) -> *mut c_char {
    to_c_string(state.into_core().abbreviation().into())
}

/// Returns the full name of a state. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_name(state: StdbrState) -> *mut c_char {
    to_c_string(state.into_core().name().into())
}

/// Returns the geographic region of a state.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_state_region(state: StdbrState) -> StdbrRegion {
    StdbrRegion::from_core(state.into_core().region())
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

/// Writes all 27 states to `buf`. `buf` must point to an array of at least 27 elements.
/// Returns the number of states written (always 27).
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
