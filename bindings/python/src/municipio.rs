use pyo3::prelude::*;
use stdbr_core::municipio as core_mun;

use crate::uf::State;

#[pyclass]
pub struct Municipio {
    inner: &'static core_mun::Municipio,
}

#[pymethods]
impl Municipio {
    /// IBGE 7-digit code.
    #[getter]
    fn ibge_code(&self) -> u32 {
        self.inner.ibge_code
    }

    /// Municipality name.
    #[getter]
    fn name(&self) -> &str {
        self.inner.name
    }

    /// State this municipality belongs to.
    #[getter]
    fn state(&self) -> State {
        self.inner.state.into()
    }

    /// Whether this municipality is a state capital.
    #[getter]
    fn is_capital(&self) -> bool {
        self.inner.is_capital()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Municipio({}, '{}')", self.inner.ibge_code, self.inner.name)
    }
}

/// Find a municipality by its IBGE code.
#[pyfunction]
fn municipio_from_ibge_code(code: u32) -> Option<Municipio> {
    core_mun::Municipio::from_ibge_code(code).map(|m| Municipio { inner: m })
}

/// Get the capital of a given state.
#[pyfunction]
fn municipio_capital_of(state: State) -> Municipio {
    Municipio {
        inner: core_mun::Municipio::capital_of(state.into()),
    }
}

/// Returns all municipalities in a given state.
#[pyfunction]
fn municipios_by_state(state: State) -> Vec<Municipio> {
    core_mun::Municipio::by_state(state.into())
        .iter()
        .map(|m| Municipio { inner: m })
        .collect()
}

/// Find municipalities whose name contains the given substring (case-insensitive).
#[pyfunction]
fn municipio_search_by_name(query: &str) -> Vec<Municipio> {
    core_mun::Municipio::search_by_name(query)
        .map(|m| Municipio { inner: m })
        .collect()
}

/// Total number of municipalities.
#[pyfunction]
#[allow(clippy::cast_possible_truncation)]
fn municipio_count() -> u32 {
    core_mun::ALL.len() as u32
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Municipio>()?;
    m.add_function(wrap_pyfunction!(municipio_from_ibge_code, m)?)?;
    m.add_function(wrap_pyfunction!(municipio_capital_of, m)?)?;
    m.add_function(wrap_pyfunction!(municipios_by_state, m)?)?;
    m.add_function(wrap_pyfunction!(municipio_search_by_name, m)?)?;
    m.add_function(wrap_pyfunction!(municipio_count, m)?)?;
    Ok(())
}
