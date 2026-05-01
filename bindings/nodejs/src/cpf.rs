use napi::bindgen_prelude::*;
use napi_derive::napi;

use stdbr_core::cpf as core_cpf;

fn cpf_err(e: &core_cpf::CpfError) -> Error {
    Error::new(Status::InvalidArg, e.to_string())
}

#[napi]
pub enum FiscalRegion {
    Rs,
    DfGoMsMtTo,
    AcAmApPaRoRr,
    CeMaPi,
    AlPbPeRn,
    BaSe,
    Mg,
    EsRj,
    Sp,
    PrSc,
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

#[napi]
pub struct Cpf {
    inner: core_cpf::Cpf,
}

#[napi]
impl Cpf {
    /// Parse a CPF string (accepts `###.###.###-##` or `###########`).
    #[napi(factory)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn parse(raw: String) -> Result<Cpf> {
        let inner: core_cpf::Cpf = raw.parse().map_err(|e| cpf_err(&e))?;
        Ok(Cpf { inner })
    }

    /// Generate a random valid CPF.
    #[napi(factory)]
    pub fn generate() -> Cpf {
        Cpf {
            inner: core_cpf::generate_cpf(),
        }
    }

    /// Generate a random valid CPF for a specific fiscal region.
    #[napi(factory)]
    pub fn generate_for_region(region: FiscalRegion) -> Cpf {
        Cpf {
            inner: core_cpf::generate_for_region(region.into()),
        }
    }

    /// Unformatted 11-digit string.
    #[napi]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `###.###.###-##`.
    #[napi]
    pub fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XXX.***.***-XX`.
    #[napi]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Fiscal region derived from the 9th digit.
    #[napi(getter)]
    pub fn fiscal_region(&self) -> FiscalRegion {
        self.inner.fiscal_region().into()
    }

    /// The two check digits as `[d1, d2]`.
    #[napi(getter)]
    pub fn check_digits(&self) -> Vec<u8> {
        let (d1, d2) = self.inner.check_digits();
        vec![d1, d2]
    }
}

/// Lenient CPF validation (strips non-digit characters first).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cpf_is_valid(cpf: String) -> bool {
    core_cpf::is_valid(&cpf)
}

/// Strict CPF validation (accepts only `###.###.###-##` or `###########`).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cpf_is_valid_strict(cpf: String) -> Result<()> {
    core_cpf::is_valid_strict(&cpf).map_err(|e| cpf_err(&e))
}

/// Format a CPF string as `###.###.###-##`.
/// Returns `null` if the input doesn't contain exactly 11 digits.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cpf_format(cpf: String) -> Option<String> {
    core_cpf::format_cpf(&cpf)
}

/// Remove all non-digit characters from a CPF string.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cpf_remove_symbols(cpf: String) -> String {
    core_cpf::remove_symbols(&cpf)
}

/// Generate a random valid CPF as an 11-digit string.
#[napi]
pub fn cpf_generate() -> String {
    core_cpf::generate()
}

/// Compute check digits for a 9-digit CPF base.
/// Returns `null` if the input is invalid.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cpf_compute_check_digits(base: String) -> Option<Vec<u8>> {
    core_cpf::compute_check_digits(&base).map(|(d1, d2)| vec![d1, d2])
}
