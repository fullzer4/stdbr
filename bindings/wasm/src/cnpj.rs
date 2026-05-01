use stdbr_core::cnpj as core_cnpj;
use wasm_bindgen::prelude::*;

fn cnpj_err(e: &core_cnpj::CnpjError) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct Cnpj {
    inner: core_cnpj::Cnpj,
}

#[wasm_bindgen]
impl Cnpj {
    /// Parse a CNPJ string (accepts `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
    #[wasm_bindgen]
    pub fn parse(raw: &str) -> Result<Cnpj, JsError> {
        let cnpj: core_cnpj::Cnpj = raw.parse().map_err(|e| cnpj_err(&e))?;
        Ok(Self { inner: cnpj })
    }

    /// Generate a random valid CNPJ.
    #[wasm_bindgen]
    pub fn generate(kind: CnpjKind) -> Self {
        Self {
            inner: core_cnpj::generate_cnpj(kind.into()),
        }
    }

    /// Generate a random valid CNPJ with ordem "0001" (Matriz).
    #[wasm_bindgen(js_name = "generateMatriz")]
    pub fn generate_matriz(kind: CnpjKind) -> Self {
        Self {
            inner: core_cnpj::generate_matriz(kind.into()),
        }
    }

    /// Unformatted 14-character string.
    #[wasm_bindgen(js_name = "asStr")]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `XX.XXX.XXX/XXXX-DD`.
    #[wasm_bindgen]
    pub fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XX.XXX.XXX/****-**`.
    #[wasm_bindgen]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Root (positions 1-8): identifies the company.
    #[wasm_bindgen]
    pub fn raiz(&self) -> String {
        self.inner.raiz().to_owned()
    }

    /// Order (positions 9-12): identifies the establishment.
    #[wasm_bindgen]
    pub fn ordem(&self) -> String {
        self.inner.ordem().to_owned()
    }

    /// Whether the CNPJ is numeric or alphanumeric.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> CnpjKind {
        self.inner.kind().into()
    }

    /// Whether this is Matriz or Filial.
    #[wasm_bindgen(getter, js_name = "establishmentType")]
    pub fn establishment_type(&self) -> EstablishmentType {
        self.inner.establishment_type().into()
    }

    /// The two check digits as `Uint8Array`.
    #[wasm_bindgen(getter, js_name = "checkDigits")]
    pub fn check_digits(&self) -> Vec<u8> {
        let (d1, d2) = self.inner.check_digits();
        vec![d1, d2]
    }
}

/// Lenient CNPJ validation (strips non-alphanumeric characters first).
#[wasm_bindgen(js_name = "cnpjIsValid")]
pub fn cnpj_is_valid(cnpj: &str) -> bool {
    core_cnpj::is_valid(cnpj)
}

/// Strict CNPJ validation (accepts only `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
#[wasm_bindgen(js_name = "cnpjIsValidStrict")]
pub fn cnpj_is_valid_strict(cnpj: &str) -> Result<(), JsError> {
    core_cnpj::is_valid_strict(cnpj).map_err(|e| cnpj_err(&e))
}

/// Format a CNPJ string as `XX.XXX.XXX/XXXX-DD`.
/// Returns `undefined` if the input is not a valid 14-character CNPJ.
#[wasm_bindgen(js_name = "cnpjFormat")]
pub fn cnpj_format(cnpj: &str) -> Option<String> {
    core_cnpj::format_cnpj(cnpj)
}

/// Remove all non-alphanumeric characters from a CNPJ string, uppercase.
#[wasm_bindgen(js_name = "cnpjRemoveSymbols")]
pub fn cnpj_remove_symbols(cnpj: &str) -> String {
    core_cnpj::remove_symbols(cnpj)
}

/// Generate a random valid CNPJ as a 14-character string.
#[wasm_bindgen(js_name = "cnpjGenerate")]
pub fn cnpj_generate(kind: CnpjKind) -> String {
    core_cnpj::generate(kind.into())
}

/// Compute check digits for a 12-character CNPJ base.
/// Returns `undefined` if the input is invalid.
#[wasm_bindgen(js_name = "cnpjComputeCheckDigits")]
pub fn cnpj_compute_check_digits(base: &str) -> Option<Vec<u8>> {
    core_cnpj::compute_check_digits(base).map(|(d1, d2)| vec![d1, d2])
}
