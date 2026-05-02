use pyo3::prelude::*;
use stdbr_core::uf as core_uf;

#[pyclass(eq, eq_int)]
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

#[pyclass(eq, eq_int)]
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
#[pyfunction]
fn state_abbreviation(state: State) -> &'static str {
    core_uf::State::from(state).abbreviation()
}

/// Get the full name of a state (e.g. "São Paulo").
#[pyfunction]
fn state_name(state: State) -> &'static str {
    core_uf::State::from(state).name()
}

/// Get the geographic region of a state.
#[pyfunction]
fn state_region(state: State) -> Region {
    core_uf::State::from(state).region().into()
}

/// Parse a state from its two-letter abbreviation (case-insensitive).
#[pyfunction]
fn state_from_abbreviation(abbr: &str) -> Option<State> {
    core_uf::State::from_abbreviation(abbr).map(State::from)
}

/// Returns all 27 Brazilian states.
#[pyfunction]
fn all_states() -> Vec<State> {
    core_uf::ALL.iter().copied().map(State::from).collect()
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Region>()?;
    m.add_class::<State>()?;
    m.add_function(wrap_pyfunction!(state_abbreviation, m)?)?;
    m.add_function(wrap_pyfunction!(state_name, m)?)?;
    m.add_function(wrap_pyfunction!(state_region, m)?)?;
    m.add_function(wrap_pyfunction!(state_from_abbreviation, m)?)?;
    m.add_function(wrap_pyfunction!(all_states, m)?)?;
    Ok(())
}
