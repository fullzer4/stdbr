use pyo3::prelude::*;
use stdbr_core::rg as core_rg;

use crate::uf::State;

fn rg_err(e: &core_rg::RgError) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(e.to_string())
}

#[pyclass]
pub struct Rg {
    inner: core_rg::Rg,
}

#[pymethods]
impl Rg {
    /// Parse an RG string for the given UF.
    #[staticmethod]
    fn parse(raw: &str, uf: State) -> PyResult<Self> {
        let inner = core_rg::parse_strict(raw, uf.into()).map_err(|e| rg_err(&e))?;
        Ok(Self { inner })
    }

    /// Generate a random valid RG. Currently SP only; other UFs raise `ValueError`.
    #[staticmethod]
    fn generate(uf: State) -> PyResult<Self> {
        core_rg::generate(uf.into())
            .map(|inner| Self { inner })
            .map_err(|e| rg_err(&e))
    }

    /// Unformatted body.
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Formatted per the UF mask.
    fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked representation — first 2 digits visible, rest masked.
    fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Body without check digit (SP: 8-digit base; others: full string).
    fn body(&self) -> &str {
        self.inner.body()
    }

    #[getter]
    fn uf(&self) -> State {
        self.inner.uf().into()
    }

    #[getter]
    fn check_digit(&self) -> Option<u8> {
        self.inner.check_digit()
    }

    fn __str__(&self) -> String {
        self.inner.formatted()
    }

    fn __repr__(&self) -> String {
        format!(
            "Rg('{}', {})",
            self.inner.formatted(),
            self.inner.uf().abbreviation()
        )
    }
}

#[pyfunction]
fn rg_is_valid(rg: &str, uf: State) -> bool {
    core_rg::is_valid(rg, uf.into())
}

#[pyfunction]
fn rg_is_valid_strict(rg: &str, uf: State) -> PyResult<()> {
    core_rg::is_valid_strict(rg, uf.into()).map_err(|e| rg_err(&e))
}

#[pyfunction]
fn rg_format(rg: &str, uf: State) -> Option<String> {
    core_rg::format_rg(rg, uf.into())
}

#[pyfunction]
fn rg_remove_symbols(rg: &str, uf: State) -> String {
    core_rg::remove_symbols(rg, uf.into())
}

#[pyfunction]
fn rg_compute_check_digit(base: &str, uf: State) -> Option<u8> {
    core_rg::compute_check_digit(base, uf.into())
}

#[pyfunction]
fn rg_generate(uf: State) -> PyResult<String> {
    core_rg::generate(uf.into())
        .map(|rg| rg.as_str().to_owned())
        .map_err(|e| rg_err(&e))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rg>()?;
    m.add_function(wrap_pyfunction!(rg_is_valid, m)?)?;
    m.add_function(wrap_pyfunction!(rg_is_valid_strict, m)?)?;
    m.add_function(wrap_pyfunction!(rg_format, m)?)?;
    m.add_function(wrap_pyfunction!(rg_remove_symbols, m)?)?;
    m.add_function(wrap_pyfunction!(rg_compute_check_digit, m)?)?;
    m.add_function(wrap_pyfunction!(rg_generate, m)?)?;
    Ok(())
}
