mod cep;
mod cnpj;
mod cpf;
mod municipio;
pub(crate) mod uf;

use pyo3::prelude::*;

#[pymodule]
fn stdbr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    uf::register(m)?;
    cpf::register(m)?;
    cnpj::register(m)?;
    cep::register(m)?;
    municipio::register(m)
}
