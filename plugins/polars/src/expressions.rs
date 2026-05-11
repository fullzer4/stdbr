use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use stdbr_core::{cep, cnpj, cpf};


#[polars_expr(output_type=Boolean)]
fn cpf_is_valid(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: BooleanChunked = ca
        .into_iter()
        .map(|opt| opt.map(cpf::is_valid))
        .collect_ca(ca.name().clone());
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cpf_format(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        if let Some(formatted) = cpf::format_cpf(val) {
            buf.push_str(&formatted);
        } else {
            buf.push_str(val);
        }
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cpf_remove_symbols(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        buf.push_str(&cpf::remove_symbols(val));
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=Boolean)]
fn cnpj_is_valid(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: BooleanChunked = ca
        .into_iter()
        .map(|opt| opt.map(cnpj::is_valid))
        .collect_ca(ca.name().clone());
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cnpj_format(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        if let Some(formatted) = cnpj::format_cnpj(val) {
            buf.push_str(&formatted);
        } else {
            buf.push_str(val);
        }
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cnpj_remove_symbols(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        buf.push_str(&cnpj::remove_symbols(val));
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=Boolean)]
fn cep_is_valid(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: BooleanChunked = ca
        .into_iter()
        .map(|opt| opt.map(cep::is_valid))
        .collect_ca(ca.name().clone());
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cep_format(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        if let Some(formatted) = cep::format_cep(val) {
            buf.push_str(&formatted);
        } else {
            buf.push_str(val);
        }
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn cep_remove_symbols(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|val, buf| {
        buf.push_str(&cep::remove_symbols(val));
    });
    Ok(out.into_series())
}
