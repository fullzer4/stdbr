use pyo3::prelude::*;
use stdbr_core::cnpj as core_cnpj;

fn cnpj_err(e: &core_cnpj::CnpjError) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(e.to_string())
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CnpjKind {
    Numeric = 0,
    Alphanumeric = 1,
}

impl From<CnpjKind> for core_cnpj::CnpjKind {
    fn from(k: CnpjKind) -> Self {
        match k {
            CnpjKind::Numeric => Self::Numeric,
            CnpjKind::Alphanumeric => Self::Alphanumeric,
        }
    }
}

impl From<core_cnpj::CnpjKind> for CnpjKind {
    fn from(k: core_cnpj::CnpjKind) -> Self {
        match k {
            core_cnpj::CnpjKind::Numeric => Self::Numeric,
            core_cnpj::CnpjKind::Alphanumeric => Self::Alphanumeric,
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EstablishmentType {
    Matriz = 0,
    Filial = 1,
}

impl From<core_cnpj::EstablishmentType> for EstablishmentType {
    fn from(t: core_cnpj::EstablishmentType) -> Self {
        match t {
            core_cnpj::EstablishmentType::Matriz => Self::Matriz,
            core_cnpj::EstablishmentType::Filial => Self::Filial,
        }
    }
}

#[pyclass]
pub struct Cnpj {
    inner: core_cnpj::Cnpj,
}

#[pymethods]
impl Cnpj {
    /// Parse a CNPJ string (accepts `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
    #[staticmethod]
    fn parse(raw: &str) -> PyResult<Self> {
        let cnpj: core_cnpj::Cnpj = raw.parse().map_err(|e| cnpj_err(&e))?;
        Ok(Self { inner: cnpj })
    }

    /// Generate a random valid CNPJ.
    #[staticmethod]
    fn generate(kind: CnpjKind) -> Self {
        Self {
            inner: core_cnpj::generate_cnpj(kind.into()),
        }
    }

    /// Generate a random valid CNPJ with ordem "0001" (Matriz).
    #[staticmethod]
    fn generate_matriz(kind: CnpjKind) -> Self {
        Self {
            inner: core_cnpj::generate_matriz(kind.into()),
        }
    }

    /// Unformatted 14-character string.
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Formatted as `XX.XXX.XXX/XXXX-DD`.
    fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XX.XXX.XXX/****-**`.
    fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Root (positions 1-8): identifies the company.
    fn raiz(&self) -> &str {
        self.inner.raiz()
    }

    /// Order (positions 9-12): identifies the establishment.
    fn ordem(&self) -> &str {
        self.inner.ordem()
    }

    /// Whether the CNPJ is numeric or alphanumeric.
    #[getter]
    fn kind(&self) -> CnpjKind {
        self.inner.kind().into()
    }

    /// Whether this is Matriz or Filial.
    #[getter]
    fn establishment_type(&self) -> EstablishmentType {
        self.inner.establishment_type().into()
    }

    /// The two check digits as `(d1, d2)`.
    #[getter]
    fn check_digits(&self) -> (u8, u8) {
        self.inner.check_digits()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Cnpj('{}')", self.inner)
    }
}

/// Lenient CNPJ validation (strips non-alphanumeric characters first).
#[pyfunction]
fn cnpj_is_valid(cnpj: &str) -> bool {
    core_cnpj::is_valid(cnpj)
}

/// Strict CNPJ validation (accepts only `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
#[pyfunction]
fn cnpj_is_valid_strict(cnpj: &str) -> PyResult<()> {
    core_cnpj::is_valid_strict(cnpj).map_err(|e| cnpj_err(&e))
}

/// Format a CNPJ string as `XX.XXX.XXX/XXXX-DD`.
/// Returns `None` if the input is not a valid 14-character CNPJ.
#[pyfunction]
fn cnpj_format(cnpj: &str) -> Option<String> {
    core_cnpj::format_cnpj(cnpj)
}

/// Remove all non-alphanumeric characters from a CNPJ string, uppercase.
#[pyfunction]
fn cnpj_remove_symbols(cnpj: &str) -> String {
    core_cnpj::remove_symbols(cnpj)
}

/// Generate a random valid CNPJ as a 14-character string.
#[pyfunction]
fn cnpj_generate(kind: CnpjKind) -> String {
    core_cnpj::generate(kind.into())
}

/// Compute check digits for a 12-character CNPJ base.
/// Returns `None` if the input is invalid.
#[pyfunction]
fn cnpj_compute_check_digits(base: &str) -> Option<(u8, u8)> {
    core_cnpj::compute_check_digits(base)
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CnpjKind>()?;
    m.add_class::<EstablishmentType>()?;
    m.add_class::<Cnpj>()?;
    m.add_function(wrap_pyfunction!(cnpj_is_valid, m)?)?;
    m.add_function(wrap_pyfunction!(cnpj_is_valid_strict, m)?)?;
    m.add_function(wrap_pyfunction!(cnpj_format, m)?)?;
    m.add_function(wrap_pyfunction!(cnpj_remove_symbols, m)?)?;
    m.add_function(wrap_pyfunction!(cnpj_generate, m)?)?;
    m.add_function(wrap_pyfunction!(cnpj_compute_check_digits, m)?)?;
    Ok(())
}
