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

/// Finds a municipio, or `NULL` if absent. Caller destroys it with `stdbr_municipio_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_from_ibge_code(code: u32) -> *mut StdbrMunicipio {
    core_mun::Municipio::from_ibge_code(code).map_or(ptr::null_mut(), |m| {
        Box::into_raw(Box::new(StdbrMunicipio(m)))
    })
}

/// Gets a capital, or `NULL` for an invalid state. Caller destroys it with `stdbr_municipio_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_capital_of(state: u8) -> *mut StdbrMunicipio {
    let Some(state) = StdbrState::core_from_raw(state) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(StdbrMunicipio(core_mun::Municipio::capital_of(
        state,
    ))))
}

/// Destroys a municipio handle. `NULL`-safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_destroy(m: *mut StdbrMunicipio) {
    if !m.is_null() {
        unsafe { drop(Box::from_raw(m)) };
    }
}

/// Returns the IBGE code, or the invalid code 0 when `m` is `NULL`.
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

/// Returns the state, or `STDBR_STATE_INVALID` when `m` is `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_state(m: *const StdbrMunicipio) -> StdbrState {
    if m.is_null() {
        return StdbrState::Invalid;
    }
    StdbrState::from_core(unsafe { &*m }.0.state)
}

/// Returns whether a municipio is a capital; returns `false` when `m` is `NULL`.
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

/// Returns a state list, or `NULL` for invalid state. Caller uses `stdbr_municipio_list_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn stdbr_municipio_by_state(state: u8) -> *mut StdbrMunicipioList {
    let Some(state) = StdbrState::core_from_raw(state) else {
        return ptr::null_mut();
    };
    let items = core_mun::Municipio::by_state(state).iter().collect();
    Box::into_raw(Box::new(StdbrMunicipioList { items }))
}

/// Searches by name, or returns `NULL` for a null/invalid query. Caller destroys the list with `stdbr_municipio_list_destroy`.
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

/// Returns the item count, or `UINT32_MAX` when `list` is `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stdbr_municipio_list_count(list: *const StdbrMunicipioList) -> u32 {
    if list.is_null() {
        return u32::MAX;
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        unsafe { &*list }.items.len() as u32
    }
}

/// Returns an independently owned municipio, or `NULL` for a null list/out-of-range index; destroy it with `stdbr_municipio_destroy`.
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
