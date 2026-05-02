use pyo3::prelude::*;
use stdbr_core::cep as core_cep;

use crate::uf::State;

fn cep_err(e: &core_cep::CepError) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(e.to_string())
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PostalRegion {
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

impl From<PostalRegion> for core_cep::PostalRegion {
    fn from(r: PostalRegion) -> Self {
        // SAFETY: core PostalRegion is repr(u8) 0..=9, same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

impl From<core_cep::PostalRegion> for PostalRegion {
    fn from(r: core_cep::PostalRegion) -> Self {
        // SAFETY: both enums have repr(u8) 0..=9, same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

#[pyclass]
pub struct Cep {
    inner: core_cep::Cep,
}

#[pymethods]
impl Cep {
    /// Parse a CEP string (accepts `#####-###` or `########`).
    #[staticmethod]
    fn parse(raw: &str) -> PyResult<Self> {
        let cep: core_cep::Cep = raw.parse().map_err(|e| cep_err(&e))?;
        Ok(Self { inner: cep })
    }

    /// Generate a random valid CEP.
    #[staticmethod]
    fn generate() -> Self {
        Self {
            inner: core_cep::generate_cep(),
        }
    }

    /// Generate a random CEP for a given postal region.
    #[staticmethod]
    fn generate_for_region(region: PostalRegion) -> Self {
        Self {
            inner: core_cep::generate_for_region(region.into()),
        }
    }

    /// Generate a random CEP within the range of a given state.
    #[staticmethod]
    fn generate_for_state(state: State) -> Self {
        Self {
            inner: core_cep::generate_for_state(state.into()),
        }
    }

    /// Unformatted 8-digit string.
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Formatted as `#####-###`.
    fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked as `#####-***`.
    fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Postal region derived from the 1st digit.
    #[getter]
    fn postal_region(&self) -> PostalRegion {
        self.inner.postal_region().into()
    }

    /// State lookup by CEP range.
    #[getter]
    fn state(&self) -> Option<State> {
        self.inner.state().map(State::from)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Cep('{}')", self.inner)
    }
}

/// Lenient CEP validation (strips non-digit characters first).
#[pyfunction]
fn cep_is_valid(cep: &str) -> bool {
    core_cep::is_valid(cep)
}

/// Strict CEP validation (accepts only `#####-###` or `########`).
#[pyfunction]
fn cep_is_valid_strict(cep: &str) -> PyResult<()> {
    core_cep::is_valid_strict(cep).map_err(|e| cep_err(&e))
}

/// Format a CEP string as `#####-###`.
/// Returns `None` if the input doesn't contain exactly 8 digits.
#[pyfunction]
fn cep_format(cep: &str) -> Option<String> {
    core_cep::format_cep(cep)
}

/// Remove all non-digit characters from a CEP string.
#[pyfunction]
fn cep_remove_symbols(cep: &str) -> String {
    core_cep::remove_symbols(cep)
}

/// Generate a random valid CEP as an 8-digit string.
#[pyfunction]
fn cep_generate() -> String {
    core_cep::generate()
}

/// Generate a random CEP for a given postal region.
#[pyfunction]
fn cep_generate_for_region(region: PostalRegion) -> String {
    core_cep::generate_for_region(region.into()).as_str().into()
}

/// Generate a random CEP within the range of a given state.
#[pyfunction]
fn cep_generate_for_state(state: State) -> String {
    core_cep::generate_for_state(state.into()).as_str().into()
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PostalRegion>()?;
    m.add_class::<Cep>()?;
    m.add_function(wrap_pyfunction!(cep_is_valid, m)?)?;
    m.add_function(wrap_pyfunction!(cep_is_valid_strict, m)?)?;
    m.add_function(wrap_pyfunction!(cep_format, m)?)?;
    m.add_function(wrap_pyfunction!(cep_remove_symbols, m)?)?;
    m.add_function(wrap_pyfunction!(cep_generate, m)?)?;
    m.add_function(wrap_pyfunction!(cep_generate_for_region, m)?)?;
    m.add_function(wrap_pyfunction!(cep_generate_for_state, m)?)?;
    Ok(())
}
