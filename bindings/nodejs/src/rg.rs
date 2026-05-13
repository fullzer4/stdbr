use napi::bindgen_prelude::*;
use napi_derive::napi;

use stdbr_core::rg as core_rg;

use crate::uf::State;

fn rg_err(e: &core_rg::RgError) -> Error {
    Error::new(Status::InvalidArg, e.to_string())
}

#[napi]
pub struct Rg {
    inner: core_rg::Rg,
}

#[napi]
impl Rg {
    /// Parse an RG string for the given UF (accepts the canonical mask or
    /// unformatted body).
    #[napi(factory)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn parse(raw: String, uf: State) -> Result<Rg> {
        let inner = core_rg::parse_strict(&raw, uf.into()).map_err(|e| rg_err(&e))?;
        Ok(Rg { inner })
    }

    /// Generate a random valid RG. Currently SP only; other UFs return an error.
    #[napi(factory)]
    pub fn generate(uf: State) -> Result<Rg> {
        core_rg::generate(uf.into())
            .map(|inner| Rg { inner })
            .map_err(|e| rg_err(&e))
    }

    /// Unformatted body (digits, with optional trailing 'X' for SP).
    #[napi]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted per the UF mask (or raw body if no mask is defined).
    #[napi]
    pub fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked representation — first 2 digits visible, rest masked.
    #[napi]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Body without check digit (SP: 8-digit base; others: full string).
    #[napi]
    pub fn body(&self) -> String {
        self.inner.body().to_owned()
    }

    /// Issuing UF.
    #[napi(getter)]
    pub fn uf(&self) -> State {
        self.inner.uf().into()
    }

    /// Check digit (`Some(0..=9)` for digits, `Some(10)` for SP `'X'`,
    /// `null` for UFs without a verified algorithm).
    #[napi(getter)]
    pub fn check_digit(&self) -> Option<u8> {
        self.inner.check_digit()
    }
}

/// Lenient RG validation (strips symbols first).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn rg_is_valid(rg: String, uf: State) -> bool {
    core_rg::is_valid(&rg, uf.into())
}

/// Strict RG validation (canonical mask or unformatted body only).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn rg_is_valid_strict(rg: String, uf: State) -> Result<()> {
    core_rg::is_valid_strict(&rg, uf.into()).map_err(|e| rg_err(&e))
}

/// Format an RG with the per-UF mask. Returns `null` if length is wrong.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn rg_format(rg: String, uf: State) -> Option<String> {
    core_rg::format_rg(&rg, uf.into())
}

/// Strip separators. For SP, preserves a trailing 'X'.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn rg_remove_symbols(rg: String, uf: State) -> String {
    core_rg::remove_symbols(&rg, uf.into())
}

/// SP-only: compute the check digit for an 8-digit base. Returns 10 for the
/// `'X'` terminator. `null` for non-SP UFs or wrong length.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn rg_compute_check_digit(base: String, uf: State) -> Option<u8> {
    core_rg::compute_check_digit(&base, uf.into())
}

/// Generate a random valid RG (unformatted body). Currently SP only.
#[napi]
pub fn rg_generate(uf: State) -> Result<String> {
    core_rg::generate(uf.into())
        .map(|rg| rg.as_str().to_owned())
        .map_err(|e| rg_err(&e))
}
