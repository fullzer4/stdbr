use napi::bindgen_prelude::*;
use napi_derive::napi;

use stdbr_core::cep as core_cep;

use crate::uf::State;

fn cep_err(e: &core_cep::CepError) -> Error {
    Error::new(Status::InvalidArg, e.to_string())
}

#[napi]
#[derive(PartialEq, Eq)]
pub enum PostalRegion {
    GranSaoPaulo,
    InteriorSaoPaulo,
    RjEs,
    Mg,
    BaSe,
    PeAlPbRn,
    CePiMaPaAmAcApRr,
    DfGoToMtMsRo,
    PrSc,
    Rs,
}

impl From<PostalRegion> for core_cep::PostalRegion {
    fn from(r: PostalRegion) -> Self {
        // SAFETY: core PostalRegion is repr(u8) 0..=9, same variant order.
        unsafe { core::mem::transmute(r as u8) }
    }
}

impl From<core_cep::PostalRegion> for PostalRegion {
    fn from(r: core_cep::PostalRegion) -> Self {
        const REGIONS: [PostalRegion; 10] = [
            PostalRegion::GranSaoPaulo,
            PostalRegion::InteriorSaoPaulo,
            PostalRegion::RjEs,
            PostalRegion::Mg,
            PostalRegion::BaSe,
            PostalRegion::PeAlPbRn,
            PostalRegion::CePiMaPaAmAcApRr,
            PostalRegion::DfGoToMtMsRo,
            PostalRegion::PrSc,
            PostalRegion::Rs,
        ];
        REGIONS[r as u8 as usize]
    }
}

#[napi]
pub struct Cep {
    inner: core_cep::Cep,
}

#[napi]
impl Cep {
    /// Parse a CEP string (accepts `#####-###` or `########`).
    #[napi(factory)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn parse(raw: String) -> Result<Cep> {
        let inner: core_cep::Cep = raw.parse().map_err(|e| cep_err(&e))?;
        Ok(Cep { inner })
    }

    /// Generate a random valid CEP.
    #[napi(factory)]
    pub fn generate() -> Cep {
        Cep {
            inner: core_cep::generate_cep(),
        }
    }

    /// Generate a random CEP for a given postal region.
    #[napi(factory)]
    pub fn generate_for_region(region: PostalRegion) -> Cep {
        Cep {
            inner: core_cep::generate_for_region(region.into()),
        }
    }

    /// Generate a random CEP within the range of a given state.
    #[napi(factory)]
    pub fn generate_for_state(state: State) -> Cep {
        Cep {
            inner: core_cep::generate_for_state(state.into()),
        }
    }

    /// Unformatted 8-digit string.
    #[napi]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `#####-###`.
    #[napi]
    pub fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked as `#####-***`.
    #[napi]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Postal region derived from the 1st digit.
    #[napi(getter)]
    pub fn postal_region(&self) -> PostalRegion {
        self.inner.postal_region().into()
    }

    /// State lookup by CEP range.
    #[napi(getter)]
    pub fn state(&self) -> Option<State> {
        self.inner.state().map(State::from)
    }
}

/// Lenient CEP validation (strips non-digit characters first).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cep_is_valid(cep: String) -> bool {
    core_cep::is_valid(&cep)
}

/// Strict CEP validation (accepts only `#####-###` or `########`).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cep_is_valid_strict(cep: String) -> Result<()> {
    core_cep::is_valid_strict(&cep).map_err(|e| cep_err(&e))
}

/// Format a CEP string as `#####-###`.
/// Returns `null` if the input doesn't contain exactly 8 digits.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cep_format(cep: String) -> Option<String> {
    core_cep::format_cep(&cep)
}

/// Remove all non-digit characters from a CEP string.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cep_remove_symbols(cep: String) -> String {
    core_cep::remove_symbols(&cep)
}

/// Generate a random valid CEP as an 8-digit string.
#[napi]
pub fn cep_generate() -> String {
    core_cep::generate()
}

/// Generate a random CEP for a given postal region.
#[napi]
pub fn cep_generate_for_region(region: PostalRegion) -> String {
    core_cep::generate_for_region(region.into()).as_str().into()
}

/// Generate a random CEP within the range of a given state.
#[napi]
pub fn cep_generate_for_state(state: State) -> String {
    core_cep::generate_for_state(state.into()).as_str().into()
}
