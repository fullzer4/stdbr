use core::ffi::c_char;
use core::ptr;

use stdbr_core::municipio as core_mun;

use crate::uf::StdbrState;
use crate::{cstr_to_str, to_c_string};

pub struct StdbrMunicipio(&'static core_mun::Municipio);

/// Opaque list of municipalities.
pub struct StdbrMunicipioList {
    items: Vec<&'static core_mun::Municipio>,
}

/// Find a municipality by its IBGE code. Returns `NULL` if not found.
/// Caller frees with `stdbr_municipio_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_from_ibge_code(code: u32) -> *mut StdbrMunicipio {
    core_mun::Municipio::from_ibge_code(code).map_or(ptr::null_mut(), |m| {
        Box::into_raw(Box::new(StdbrMunicipio(m)))
    })
}

/// Get the capital of a given state.
/// Caller frees with `stdbr_municipio_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_capital_of(state: StdbrState) -> *mut StdbrMunicipio {
    Box::into_raw(Box::new(StdbrMunicipio(core_mun::Municipio::capital_of(
        state.into_core(),
    ))))
}

/// Destroys a municipio handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_destroy(m: *mut StdbrMunicipio) {
    if !m.is_null() {
        unsafe { drop(Box::from_raw(m)) };
    }
}

/// Returns the IBGE code of a municipio.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_ibge_code(m: *const StdbrMunicipio) -> u32 {
    if m.is_null() {
        return 0;
    }
    unsafe { &*m }.0.ibge_code
}

/// Returns the name of a municipio. Caller frees with `stdbr_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_name(m: *const StdbrMunicipio) -> *mut c_char {
    if m.is_null() {
        return ptr::null_mut();
    }
    to_c_string(unsafe { &*m }.0.name.into())
}

/// Returns the state of a municipio.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_state(m: *const StdbrMunicipio) -> StdbrState {
    if m.is_null() {
        return StdbrState::AC;
    }
    StdbrState::from_core(unsafe { &*m }.0.state)
}

/// Returns whether a municipio is a state capital.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_is_capital(m: *const StdbrMunicipio) -> bool {
    if m.is_null() {
        return false;
    }
    unsafe { &*m }.0.is_capital()
}

/// Total number of municipalities in the database.
#[unsafe(no_mangle)]
#[allow(clippy::cast_possible_truncation)]
pub extern "C" fn stdbr_municipio_count() -> u32 {
    core_mun::ALL.len() as u32
}

// --- List API ---

/// Returns all municipalities in a given state as a list.
/// Caller frees with `stdbr_municipio_list_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_by_state(state: StdbrState) -> *mut StdbrMunicipioList {
    let items = core_mun::Municipio::by_state(state.into_core())
        .iter()
        .collect();
    Box::into_raw(Box::new(StdbrMunicipioList { items }))
}

/// Searches municipalities by name (case-insensitive substring match).
/// Caller frees with `stdbr_municipio_list_destroy`.
///
/// # Safety
/// `query` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_search_by_name(
    query: *const c_char,
) -> *mut StdbrMunicipioList {
    let Some(q) = (unsafe { cstr_to_str(query) }) else {
        return ptr::null_mut();
    };
    let items = core_mun::Municipio::search_by_name(q).collect();
    Box::into_raw(Box::new(StdbrMunicipioList { items }))
}

/// Returns the number of items in a municipio list.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_list_count(list: *const StdbrMunicipioList) -> u32 {
    if list.is_null() {
        return 0;
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        unsafe { &*list }.items.len() as u32
    }
}

/// Returns a municipio from the list at the given index.
/// The returned pointer is valid as long as the list is alive. Do NOT free it.
/// Returns `NULL` if out of bounds.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_list_get(
    list: *const StdbrMunicipioList,
    index: u32,
) -> *mut StdbrMunicipio {
    if list.is_null() {
        return ptr::null_mut();
    }
    unsafe { &*list }
        .items
        .get(index as usize)
        .map_or(ptr::null_mut(), |m| {
            Box::into_raw(Box::new(StdbrMunicipio(m)))
        })
}

/// Destroys a municipio list. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_list_destroy(list: *mut StdbrMunicipioList) {
    if !list.is_null() {
        unsafe { drop(Box::from_raw(list)) };
    }
}
