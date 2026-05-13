use stdbr_core::rg as core_rg;
use wasm_bindgen::prelude::*;

use crate::uf::State;

fn rg_err(e: &core_rg::RgError) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen]
pub struct Rg {
    inner: core_rg::Rg,
}

#[wasm_bindgen]
impl Rg {
    /// Parse an RG string for the given UF.
    #[wasm_bindgen]
    pub fn parse(raw: &str, uf: State) -> Result<Rg, JsError> {
        let inner = core_rg::parse_strict(raw, uf.into()).map_err(|e| rg_err(&e))?;
        Ok(Self { inner })
    }

    /// Generate a random valid RG. Currently SP only.
    #[wasm_bindgen]
    pub fn generate(uf: State) -> Result<Rg, JsError> {
        core_rg::generate(uf.into())
            .map(|inner| Self { inner })
            .map_err(|e| rg_err(&e))
    }

    /// Unformatted body.
    #[wasm_bindgen(js_name = "asStr")]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted per the UF mask.
    #[wasm_bindgen]
    pub fn formatted(&self) -> String {
        self.inner.formatted()
    }

    /// Masked representation — first 2 digits visible, rest masked.
    #[wasm_bindgen]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Body without check digit (SP: 8-digit base; others: full string).
    #[wasm_bindgen]
    pub fn body(&self) -> String {
        self.inner.body().to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn uf(&self) -> State {
        self.inner.uf().into()
    }

    /// Check digit (`0..=9`, `10` for SP `'X'`, `undefined` otherwise).
    #[wasm_bindgen(getter, js_name = "checkDigit")]
    pub fn check_digit(&self) -> Option<u8> {
        self.inner.check_digit()
    }
}

#[wasm_bindgen(js_name = "rgIsValid")]
pub fn rg_is_valid(rg: &str, uf: State) -> bool {
    core_rg::is_valid(rg, uf.into())
}

#[wasm_bindgen(js_name = "rgIsValidStrict")]
pub fn rg_is_valid_strict(rg: &str, uf: State) -> Result<(), JsError> {
    core_rg::is_valid_strict(rg, uf.into()).map_err(|e| rg_err(&e))
}

#[wasm_bindgen(js_name = "rgFormat")]
pub fn rg_format(rg: &str, uf: State) -> Option<String> {
    core_rg::format_rg(rg, uf.into())
}

#[wasm_bindgen(js_name = "rgRemoveSymbols")]
pub fn rg_remove_symbols(rg: &str, uf: State) -> String {
    core_rg::remove_symbols(rg, uf.into())
}

#[wasm_bindgen(js_name = "rgComputeCheckDigit")]
pub fn rg_compute_check_digit(base: &str, uf: State) -> Option<u8> {
    core_rg::compute_check_digit(base, uf.into())
}

#[wasm_bindgen(js_name = "rgGenerate")]
pub fn rg_generate(uf: State) -> Result<String, JsError> {
    core_rg::generate(uf.into())
        .map(|rg| rg.as_str().to_owned())
        .map_err(|e| rg_err(&e))
}
