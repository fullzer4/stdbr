use pyo3::prelude::*;
use stdbr_core::cpf as core_cpf;

fn cpf_err(e: &core_cpf::CpfError) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(e.to_string())
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FiscalRegion {
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

impl From<FiscalRegion> for core_cpf::FiscalRegion {
    fn from(r: FiscalRegion) -> Self {
        match r {
            FiscalRegion::Rs => Self::Rs,
            FiscalRegion::DfGoMsMtTo => Self::DfGoMsMtTo,
            FiscalRegion::AcAmApPaRoRr => Self::AcAmApPaRoRr,
            FiscalRegion::CeMaPi => Self::CeMaPi,
            FiscalRegion::AlPbPeRn => Self::AlPbPeRn,
            FiscalRegion::BaSe => Self::BaSe,
            FiscalRegion::Mg => Self::Mg,
            FiscalRegion::EsRj => Self::EsRj,
            FiscalRegion::Sp => Self::Sp,
            FiscalRegion::PrSc => Self::PrSc,
        }
    }
}

impl From<core_cpf::FiscalRegion> for FiscalRegion {
    fn from(r: core_cpf::FiscalRegion) -> Self {
        match r {
            core_cpf::FiscalRegion::Rs => Self::Rs,
            core_cpf::FiscalRegion::DfGoMsMtTo => Self::DfGoMsMtTo,
            core_cpf::FiscalRegion::AcAmApPaRoRr => Self::AcAmApPaRoRr,
            core_cpf::FiscalRegion::CeMaPi => Self::CeMaPi,
            core_cpf::FiscalRegion::AlPbPeRn => Self::AlPbPeRn,
            core_cpf::FiscalRegion::BaSe => Self::BaSe,
            core_cpf::FiscalRegion::Mg => Self::Mg,
            core_cpf::FiscalRegion::EsRj => Self::EsRj,
            core_cpf::FiscalRegion::Sp => Self::Sp,
            core_cpf::FiscalRegion::PrSc => Self::PrSc,
        }
    }
}

#[pyclass]
pub struct Cpf {
    inner: core_cpf::Cpf,
}

#[pymethods]
impl Cpf {
    /// Parse a CPF string (accepts `###.###.###-##` or `###########`).
    #[staticmethod]
    fn parse(raw: &str) -> PyResult<Self> {
        let cpf: core_cpf::Cpf = raw.parse().map_err(|e| cpf_err(&e))?;
        Ok(Self { inner: cpf })
    }

    /// Generate a random valid CPF.
    #[staticmethod]
    fn generate() -> Self {
        Self {
            inner: core_cpf::generate_cpf(),
        }
    }

    /// Generate a random valid CPF for a specific fiscal region.
    #[staticmethod]
    fn generate_for_region(region: FiscalRegion) -> Self {
        Self {
            inner: core_cpf::generate_for_region(region.into()),
        }
    }

    /// Unformatted 11-digit string.
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Formatted as `###.###.###-##`.
    fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XXX.***.***-XX`.
    fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Fiscal region derived from the 9th digit.
    #[getter]
    fn fiscal_region(&self) -> FiscalRegion {
        self.inner.fiscal_region().into()
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
        format!("Cpf('{}')", self.inner)
    }
}

/// Lenient CPF validation (strips non-digit characters first).
#[pyfunction]
fn cpf_is_valid(cpf: &str) -> bool {
    core_cpf::is_valid(cpf)
}

/// Strict CPF validation (accepts only `###.###.###-##` or `###########`).
#[pyfunction]
fn cpf_is_valid_strict(cpf: &str) -> PyResult<()> {
    core_cpf::is_valid_strict(cpf).map_err(|e| cpf_err(&e))
}

/// Format a CPF string as `###.###.###-##`.
/// Returns `None` if the input doesn't contain exactly 11 digits.
#[pyfunction]
fn cpf_format(cpf: &str) -> Option<String> {
    core_cpf::format_cpf(cpf)
}

/// Remove all non-digit characters from a CPF string.
#[pyfunction]
fn cpf_remove_symbols(cpf: &str) -> String {
    core_cpf::remove_symbols(cpf)
}

/// Generate a random valid CPF as an 11-digit string.
#[pyfunction]
fn cpf_generate() -> String {
    core_cpf::generate()
}

/// Compute check digits for a 9-digit CPF base.
/// Returns `None` if the input is invalid.
#[pyfunction]
fn cpf_compute_check_digits(base: &str) -> Option<(u8, u8)> {
    core_cpf::compute_check_digits(base)
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FiscalRegion>()?;
    m.add_class::<Cpf>()?;
    m.add_function(wrap_pyfunction!(cpf_is_valid, m)?)?;
    m.add_function(wrap_pyfunction!(cpf_is_valid_strict, m)?)?;
    m.add_function(wrap_pyfunction!(cpf_format, m)?)?;
    m.add_function(wrap_pyfunction!(cpf_remove_symbols, m)?)?;
    m.add_function(wrap_pyfunction!(cpf_generate, m)?)?;
    m.add_function(wrap_pyfunction!(cpf_compute_check_digits, m)?)?;
    Ok(())
}
