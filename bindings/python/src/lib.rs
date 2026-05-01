mod cpf;

use pyo3::prelude::*;

#[pymodule]
fn stdbr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    cpf::register(m)
}
