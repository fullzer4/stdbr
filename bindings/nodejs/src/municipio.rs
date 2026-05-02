use napi_derive::napi;

use stdbr_core::municipio as core_mun;

use crate::uf::State;

#[napi]
pub struct Municipio {
    inner: &'static core_mun::Municipio,
}

#[napi]
impl Municipio {
    /// IBGE 7-digit code.
    #[napi(getter)]
    pub fn ibge_code(&self) -> u32 {
        self.inner.ibge_code
    }

    /// Municipality name.
    #[napi(getter)]
    pub fn name(&self) -> String {
        self.inner.name.to_owned()
    }

    /// State this municipality belongs to.
    #[napi(getter)]
    pub fn state(&self) -> State {
        self.inner.state.into()
    }

    /// Whether this municipality is a state capital.
    #[napi(getter)]
    pub fn is_capital(&self) -> bool {
        self.inner.is_capital()
    }
}

/// Find a municipality by its IBGE code.
#[napi]
pub fn municipio_from_ibge_code(code: u32) -> Option<Municipio> {
    core_mun::Municipio::from_ibge_code(code).map(|m| Municipio { inner: m })
}

/// Get the capital of a given state.
#[napi]
pub fn municipio_capital_of(state: State) -> Municipio {
    Municipio {
        inner: core_mun::Municipio::capital_of(state.into()),
    }
}

/// Returns all municipalities in a given state.
#[napi]
pub fn municipios_by_state(state: State) -> Vec<Municipio> {
    core_mun::Municipio::by_state(state.into())
        .iter()
        .map(|m| Municipio { inner: m })
        .collect()
}

/// Find municipalities whose name contains the given substring (case-insensitive).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn municipio_search_by_name(query: String) -> Vec<Municipio> {
    core_mun::Municipio::search_by_name(&query)
        .map(|m| Municipio { inner: m })
        .collect()
}

/// Total number of municipalities.
#[napi]
#[allow(clippy::cast_possible_truncation)]
pub fn municipio_count() -> u32 {
    core_mun::ALL.len() as u32
}
