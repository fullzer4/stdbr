use napi::bindgen_prelude::*;
use napi_derive::napi;

use stdbr_core::cnpj as core_cnpj;

fn cnpj_err(e: &core_cnpj::CnpjError) -> Error {
    Error::new(Status::InvalidArg, e.to_string())
}

#[napi]
pub enum CnpjKind {
    Numeric,
    Alphanumeric,
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

#[napi]
pub enum EstablishmentType {
    Matriz,
    Filial,
}

impl From<core_cnpj::EstablishmentType> for EstablishmentType {
    fn from(t: core_cnpj::EstablishmentType) -> Self {
        match t {
            core_cnpj::EstablishmentType::Matriz => Self::Matriz,
            core_cnpj::EstablishmentType::Filial => Self::Filial,
        }
    }
}

#[napi]
pub struct Cnpj {
    inner: core_cnpj::Cnpj,
}

#[napi]
impl Cnpj {
    /// Parse a CNPJ string (accepts `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
    #[napi(factory)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn parse(raw: String) -> Result<Cnpj> {
        let inner: core_cnpj::Cnpj = raw.parse().map_err(|e| cnpj_err(&e))?;
        Ok(Cnpj { inner })
    }

    /// Generate a random valid CNPJ.
    #[napi(factory)]
    pub fn generate(kind: CnpjKind) -> Cnpj {
        Cnpj {
            inner: core_cnpj::generate_cnpj(kind.into()),
        }
    }

    /// Generate a random valid CNPJ with ordem "0001" (Matriz).
    #[napi(factory)]
    pub fn generate_matriz(kind: CnpjKind) -> Cnpj {
        Cnpj {
            inner: core_cnpj::generate_matriz(kind.into()),
        }
    }

    /// Unformatted 14-character string.
    #[napi]
    pub fn as_str(&self) -> String {
        self.inner.as_str().to_owned()
    }

    /// Formatted as `XX.XXX.XXX/XXXX-DD`.
    #[napi]
    pub fn formatted(&self) -> String {
        self.inner.to_string()
    }

    /// Masked as `XX.XXX.XXX/****-**`.
    #[napi]
    pub fn masked(&self) -> String {
        self.inner.masked()
    }

    /// Root (positions 1-8): identifies the company.
    #[napi]
    pub fn raiz(&self) -> String {
        self.inner.raiz().to_owned()
    }

    /// Order (positions 9-12): identifies the establishment.
    #[napi]
    pub fn ordem(&self) -> String {
        self.inner.ordem().to_owned()
    }

    /// Whether the CNPJ is numeric or alphanumeric.
    #[napi(getter)]
    pub fn kind(&self) -> CnpjKind {
        self.inner.kind().into()
    }

    /// Whether this is Matriz or Filial.
    #[napi(getter)]
    pub fn establishment_type(&self) -> EstablishmentType {
        self.inner.establishment_type().into()
    }

    /// The two check digits as `[d1, d2]`.
    #[napi(getter)]
    pub fn check_digits(&self) -> Vec<u8> {
        let (d1, d2) = self.inner.check_digits();
        vec![d1, d2]
    }
}

/// Lenient CNPJ validation (strips non-alphanumeric characters first).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cnpj_is_valid(cnpj: String) -> bool {
    core_cnpj::is_valid(&cnpj)
}

/// Strict CNPJ validation (accepts only `XX.XXX.XXX/XXXX-DD` or 14 raw characters).
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cnpj_is_valid_strict(cnpj: String) -> Result<()> {
    core_cnpj::is_valid_strict(&cnpj).map_err(|e| cnpj_err(&e))
}

/// Format a CNPJ string as `XX.XXX.XXX/XXXX-DD`.
/// Returns `null` if the input is not a valid 14-character CNPJ.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cnpj_format(cnpj: String) -> Option<String> {
    core_cnpj::format_cnpj(&cnpj)
}

/// Remove all non-alphanumeric characters from a CNPJ string, uppercase.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cnpj_remove_symbols(cnpj: String) -> String {
    core_cnpj::remove_symbols(&cnpj)
}

/// Generate a random valid CNPJ as a 14-character string.
#[napi]
pub fn cnpj_generate(kind: CnpjKind) -> String {
    core_cnpj::generate(kind.into())
}

/// Compute check digits for a 12-character CNPJ base.
/// Returns `null` if the input is invalid.
#[napi]
#[allow(clippy::needless_pass_by_value)]
pub fn cnpj_compute_check_digits(base: String) -> Option<Vec<u8>> {
    core_cnpj::compute_check_digits(&base).map(|(d1, d2)| vec![d1, d2])
}
