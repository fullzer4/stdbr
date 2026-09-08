mod cep;
mod cnpj;
mod cpf;
mod municipio;
mod rg;
pub(crate) mod uf;

use pyo3::pymodule;

#[pymodule]
mod stdbr {
    use pyo3::prelude::*;

    use super::{cep, cnpj, cpf, municipio, rg, uf};

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        uf::register(m)?;
        cpf::register(m)?;
        cnpj::register(m)?;
        cep::register(m)?;
        rg::register(m)?;
        municipio::register(m)
    }
}
