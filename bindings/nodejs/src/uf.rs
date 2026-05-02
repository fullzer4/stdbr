use napi_derive::napi;

use stdbr_core::uf as core_uf;

#[napi]
#[derive(PartialEq, Eq)]
pub enum Region {
    Norte,
    Nordeste,
    CentroOeste,
    Sudeste,
    Sul,
}

impl From<Region> for core_uf::Region {
    fn from(r: Region) -> Self {
        // SAFETY: core Region is repr(u8) 0..=4, same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

impl From<core_uf::Region> for Region {
    fn from(r: core_uf::Region) -> Self {
        const REGIONS: [Region; 5] = [
            Region::Norte,
            Region::Nordeste,
            Region::CentroOeste,
            Region::Sudeste,
            Region::Sul,
        ];
        REGIONS[r as u8 as usize]
    }
}

#[napi]
#[derive(PartialEq, Eq)]
pub enum State {
    AC,
    AL,
    AM,
    AP,
    BA,
    CE,
    DF,
    ES,
    GO,
    MA,
    MG,
    MS,
    MT,
    PA,
    PB,
    PE,
    PI,
    PR,
    RJ,
    RN,
    RO,
    RR,
    RS,
    SC,
    SE,
    SP,
    TO,
}

impl From<State> for core_uf::State {
    fn from(s: State) -> Self {
        core_uf::ALL[s as usize]
    }
}

impl From<core_uf::State> for State {
    fn from(s: core_uf::State) -> Self {
        const STATES: [State; 27] = [
            State::AC,
            State::AL,
            State::AM,
            State::AP,
            State::BA,
            State::CE,
            State::DF,
            State::ES,
            State::GO,
            State::MA,
            State::MG,
            State::MS,
            State::MT,
            State::PA,
            State::PB,
            State::PE,
            State::PI,
            State::PR,
            State::RJ,
            State::RN,
            State::RO,
            State::RR,
            State::RS,
            State::SC,
            State::SE,
            State::SP,
            State::TO,
        ];
        STATES[s as u8 as usize]
    }
}

/// Get the abbreviation of a state (e.g. "SP").
#[napi]
pub fn state_abbreviation(state: State) -> String {
    core_uf::State::from(state).abbreviation().to_owned()
}

/// Get the full name of a state (e.g. "São Paulo").
#[napi]
pub fn state_name(state: State) -> String {
    core_uf::State::from(state).name().to_owned()
}

/// Get the geographic region of a state.
#[napi]
pub fn state_region(state: State) -> Region {
    core_uf::State::from(state).region().into()
}

/// Parse a state from its two-letter abbreviation (case-insensitive).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn state_from_abbreviation(abbr: String) -> Option<State> {
    core_uf::State::from_abbreviation(&abbr).map(State::from)
}

/// Returns all 27 Brazilian states.
#[napi]
pub fn all_states() -> Vec<State> {
    core_uf::ALL.iter().copied().map(State::from).collect()
}
