use wasm_bindgen::prelude::*;
use stdbr_core::municipio as core_mun;

use crate::uf::State;

#[wasm_bindgen]
pub struct Municipio {
    inner: &'static core_mun::Municipio,
}

#[wasm_bindgen]
impl Municipio {
    /// IBGE 7-digit code.
    #[wasm_bindgen(getter, js_name = "ibgeCode")]
    pub fn ibge_code(&self) -> u32 {
        self.inner.ibge_code
    }

    /// Municipality name.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.to_owned()
    }

    /// State this municipality belongs to.
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> State {
        self.inner.state.into()
    }

    /// Whether this municipality is a state capital.
    #[wasm_bindgen(getter, js_name = "isCapital")]
    pub fn is_capital(&self) -> bool {
        self.inner.is_capital()
    }
}

/// Opaque list of municipalities for collection results.
#[wasm_bindgen]
pub struct MunicipioList {
    items: Vec<&'static core_mun::Municipio>,
}

#[wasm_bindgen]
impl MunicipioList {
    /// Number of municipalities in the list.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> u32 {
        #[allow(clippy::cast_possible_truncation)]
        { self.items.len() as u32 }
    }

    /// Get a municipality by index. Returns `undefined` if out of bounds.
    pub fn get(&self, index: u32) -> Option<Municipio> {
        self.items
            .get(index as usize)
            .map(|m| Municipio { inner: m })
    }
}

/// Find a municipality by its IBGE code.
#[wasm_bindgen(js_name = "municipioFromIbgeCode")]
pub fn municipio_from_ibge_code(code: u32) -> Option<Municipio> {
    core_mun::Municipio::from_ibge_code(code).map(|m| Municipio { inner: m })
}

/// Get the capital of a given state.
#[wasm_bindgen(js_name = "municipioCapitalOf")]
pub fn municipio_capital_of(state: State) -> Municipio {
    Municipio {
        inner: core_mun::Municipio::capital_of(state.into()),
    }
}

/// Returns all municipalities in a given state.
#[wasm_bindgen(js_name = "municipiosByState")]
pub fn municipios_by_state(state: State) -> MunicipioList {
    MunicipioList {
        items: core_mun::Municipio::by_state(state.into()).iter().collect(),
    }
}

/// Find municipalities whose name contains the given substring (case-insensitive).
#[wasm_bindgen(js_name = "municipioSearchByName")]
pub fn municipio_search_by_name(query: &str) -> MunicipioList {
    MunicipioList {
        items: core_mun::Municipio::search_by_name(query).collect(),
    }
}

/// Total number of municipalities.
#[wasm_bindgen(js_name = "municipioCount")]
#[allow(clippy::cast_possible_truncation)]
pub fn municipio_count() -> u32 {
    core_mun::ALL.len() as u32
}
