use wasm_bindgen::prelude::*;
use stdbr_core::uf as core_uf;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Norte = 0,
    Nordeste = 1,
    CentroOeste = 2,
    Sudeste = 3,
    Sul = 4,
}

impl From<Region> for core_uf::Region {
    fn from(r: Region) -> Self {
        // SAFETY: both enums are repr(u8) 0..=4 with same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

impl From<core_uf::Region> for Region {
    fn from(r: core_uf::Region) -> Self {
        // SAFETY: both enums are repr(u8) 0..=4 with same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
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

impl From<State> for core_uf::State {
    fn from(s: State) -> Self {
        // SAFETY: both enums are repr(u8) 0..=26 with same variant order.
        unsafe { core::mem::transmute(s as u8) }
    }
}

impl From<core_uf::State> for State {
    fn from(s: core_uf::State) -> Self {
        // SAFETY: both enums are repr(u8) 0..=26 with same variant order.
        unsafe { core::mem::transmute(s as u8) }
    }
}

/// Get the abbreviation of a state (e.g. "SP").
#[wasm_bindgen(js_name = "stateAbbreviation")]
pub fn state_abbreviation(state: State) -> String {
    core_uf::State::from(state).abbreviation().to_owned()
}

/// Get the full name of a state (e.g. "São Paulo").
#[wasm_bindgen(js_name = "stateName")]
pub fn state_name(state: State) -> String {
    core_uf::State::from(state).name().to_owned()
}

/// Get the geographic region of a state.
#[wasm_bindgen(js_name = "stateRegion")]
pub fn state_region(state: State) -> Region {
    core_uf::State::from(state).region().into()
}

/// Parse a state from its two-letter abbreviation (case-insensitive).
#[wasm_bindgen(js_name = "stateFromAbbreviation")]
pub fn state_from_abbreviation(abbr: &str) -> Option<State> {
    core_uf::State::from_abbreviation(abbr).map(State::from)
}

/// Returns all 27 Brazilian states.
#[wasm_bindgen(js_name = "allStates")]
pub fn all_states() -> Vec<State> {
    core_uf::ALL.iter().copied().map(State::from).collect()
}
