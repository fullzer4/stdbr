use stdbr_core::cpf as core_cpf;
use wasm_bindgen::prelude::*;

fn cpf_err(e: &core_cpf::CpfError) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct Cpf {
    inner: core_cpf::Cpf,
}

#[wasm_bindgen]
impl Cpf {
    /// Parse a CPF string (accepts `###.###.###-##` or `###########`).
    #[wasm_bindgen]
    pub fn parse(raw: &str) -> Result<Cpf, JsError> {
        let cpf: core_cpf::Cpf = raw.parse().map_err(|e| cpf_err(&e))?;
        Ok(Self { inner: cpf })
    }

    /// Generate a random valid CPF.
    #[wasm_bindgen]
    pub fn generate() -> Self {
        Self {
            inner: core_cpf::generate_cpf(),
        }
    }

    /// Generate a random valid CPF for a specific fiscal region.
    #[wasm_bindgen(js_name = "generateForRegion")]
    pub fn generate_for_region(region: FiscalRegion) -> Self {
        Self {
            inner: core_cpf::generate_for_region(region.into()),
        }
    }

    /// Unformatted 11-digit string.
    #[wasm_bindgen(js_name = "asStr")]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `###.###.###-##`.
    #[wasm_bindgen]
    pub fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XXX.***.***-XX`.
    #[wasm_bindgen]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Fiscal region derived from the 9th digit.
    #[wasm_bindgen(getter, js_name = "fiscalRegion")]
    pub fn fiscal_region(&self) -> FiscalRegion {
        self.inner.fiscal_region().into()
    }

    /// The two check digits as `Uint8Array`.
    #[wasm_bindgen(getter, js_name = "checkDigits")]
    pub fn check_digits(&self) -> Vec<u8> {
        let (d1, d2) = self.inner.check_digits();
        vec![d1, d2]
    }
}

/// Lenient CPF validation (strips non-digit characters first).
#[wasm_bindgen(js_name = "cpfIsValid")]
pub fn cpf_is_valid(cpf: &str) -> bool {
    core_cpf::is_valid(cpf)
}

/// Strict CPF validation (accepts only `###.###.###-##` or `###########`).
#[wasm_bindgen(js_name = "cpfIsValidStrict")]
pub fn cpf_is_valid_strict(cpf: &str) -> Result<(), JsError> {
    core_cpf::is_valid_strict(cpf).map_err(|e| cpf_err(&e))
}

/// Format a CPF string as `###.###.###-##`.
/// Returns `undefined` if the input doesn't contain exactly 11 digits.
#[wasm_bindgen(js_name = "cpfFormat")]
pub fn cpf_format(cpf: &str) -> Option<String> {
    core_cpf::format_cpf(cpf)
}

/// Remove all non-digit characters from a CPF string.
#[wasm_bindgen(js_name = "cpfRemoveSymbols")]
pub fn cpf_remove_symbols(cpf: &str) -> String {
    core_cpf::remove_symbols(cpf)
}

/// Generate a random valid CPF as an 11-digit string.
#[wasm_bindgen(js_name = "cpfGenerate")]
pub fn cpf_generate() -> String {
    core_cpf::generate()
}

/// Compute check digits for a 9-digit CPF base.
/// Returns `undefined` if the input is invalid.
#[wasm_bindgen(js_name = "cpfComputeCheckDigits")]
pub fn cpf_compute_check_digits(base: &str) -> Option<Vec<u8>> {
    core_cpf::compute_check_digits(base).map(|(d1, d2)| vec![d1, d2])
}
