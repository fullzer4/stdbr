use wasm_bindgen::prelude::*;
use stdbr_core::cep as core_cep;

use crate::uf::State;

fn cep_err(e: &core_cep::CepError) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct Cep {
    inner: core_cep::Cep,
}

#[wasm_bindgen]
impl Cep {
    /// Parse a CEP string (accepts `#####-###` or `########`).
    #[wasm_bindgen]
    pub fn parse(raw: &str) -> Result<Cep, JsError> {
        let cep: core_cep::Cep = raw.parse().map_err(|e| cep_err(&e))?;
        Ok(Self { inner: cep })
    }

    /// Generate a random valid CEP.
    #[wasm_bindgen]
    pub fn generate() -> Self {
        Self {
            inner: core_cep::generate_cep(),
        }
    }

    /// Generate a random CEP for a given postal region.
    #[wasm_bindgen(js_name = "generateForRegion")]
    pub fn generate_for_region(region: PostalRegion) -> Self {
        Self {
            inner: core_cep::generate_for_region(region.into()),
        }
    }

    /// Generate a random CEP within the range of a given state.
    #[wasm_bindgen(js_name = "generateForState")]
    pub fn generate_for_state(state: State) -> Self {
        Self {
            inner: core_cep::generate_for_state(state.into()),
        }
    }

    /// Unformatted 8-digit string.
    #[wasm_bindgen(js_name = "asStr")]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `#####-###`.
    #[wasm_bindgen]
    pub fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked as `#####-***`.
    #[wasm_bindgen]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Postal region derived from the 1st digit.
    #[wasm_bindgen(getter, js_name = "postalRegion")]
    pub fn postal_region(&self) -> PostalRegion {
        self.inner.postal_region().into()
    }

    /// State lookup by CEP range. Returns the state abbreviation or undefined.
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> Option<State> {
        self.inner.state().map(State::from)
    }
}

/// Lenient CEP validation (strips non-digit characters first).
#[wasm_bindgen(js_name = "cepIsValid")]
pub fn cep_is_valid(cep: &str) -> bool {
    core_cep::is_valid(cep)
}

/// Strict CEP validation (accepts only `#####-###` or `########`).
#[wasm_bindgen(js_name = "cepIsValidStrict")]
pub fn cep_is_valid_strict(cep: &str) -> Result<(), JsError> {
    core_cep::is_valid_strict(cep).map_err(|e| cep_err(&e))
}

/// Format a CEP string as `#####-###`.
/// Returns `undefined` if the input doesn't contain exactly 8 digits.
#[wasm_bindgen(js_name = "cepFormat")]
pub fn cep_format(cep: &str) -> Option<String> {
    core_cep::format_cep(cep)
}

/// Remove all non-digit characters from a CEP string.
#[wasm_bindgen(js_name = "cepRemoveSymbols")]
pub fn cep_remove_symbols(cep: &str) -> String {
    core_cep::remove_symbols(cep)
}

/// Generate a random valid CEP as an 8-digit string.
#[wasm_bindgen(js_name = "cepGenerate")]
pub fn cep_generate() -> String {
    core_cep::generate()
}

/// Generate a random CEP for a given postal region.
#[wasm_bindgen(js_name = "cepGenerateForRegion")]
pub fn cep_generate_for_region(region: PostalRegion) -> String {
    core_cep::generate_for_region(region.into()).as_str().into()
}

/// Generate a random CEP within the range of a given state.
#[wasm_bindgen(js_name = "cepGenerateForState")]
pub fn cep_generate_for_state(state: State) -> String {
    core_cep::generate_for_state(state.into()).as_str().into()
}
