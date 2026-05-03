//! Generates `tests/parity/golden.json` the single source of truth for parity tests.
//!
//! ```sh
//! cargo run -p parity-gen > tests/parity/golden.json
//! ```

use serde_json::{Value, json};
use stdbr_core::{cep, cnpj, cpf, municipio, uf};

const IBGE_API_URL: &str =
    "https://servicodados.ibge.gov.br/api/v1/localidades/municipios?orderBy=nome";

fn main() {
    let ibge = fetch_ibge();

    let golden = json!({
        "cpf": cpf_cases(),
        "cnpj": cnpj_cases(),
        "cep": cep_cases(),
        "uf": uf_cases(),
        "municipio": municipio_cases(&ibge),
    });

    print!("{}", serde_json::to_string_pretty(&golden).unwrap());
}

fn strict_cases<E: std::fmt::Display + std::fmt::Debug>(
    validate: impl Fn(&str) -> Result<(), E>,
    cases: &[(&str, Option<&str>)],
) -> Value {
    Value::Array(
        cases
            .iter()
            .map(|&(input, expected_err)| {
                let actual = validate(input);
                match expected_err {
                    None => {
                        assert!(actual.is_ok(), "expected Ok for {input}, got {actual:?}");
                        json!({ "input": input, "valid": true })
                    }
                    Some(msg) => {
                        let err = actual.unwrap_err();
                        assert_eq!(err.to_string(), msg, "error mismatch for {input}");
                        json!({ "input": input, "valid": false, "error": msg })
                    }
                }
            })
            .collect(),
    )
}

fn fetch_ibge() -> Vec<Value> {
    eprintln!("Fetching IBGE municipality data from API...");

    let body = ureq::get(IBGE_API_URL)
        .call()
        .unwrap_or_else(|e| panic!("failed to fetch IBGE API: {e}"))
        .body_mut()
        .read_to_string()
        .unwrap_or_else(|e| panic!("failed to read IBGE response body: {e}"));

    let raw: Value =
        serde_json::from_str(&body).unwrap_or_else(|e| panic!("invalid IBGE JSON: {e}"));
    let arr = raw
        .as_array()
        .unwrap_or_else(|| panic!("IBGE JSON root must be an array"));

    let entries: Vec<Value> = arr
        .iter()
        .map(|m| {
            let id = u32::try_from(m["id"].as_u64().expect("id must be u64"))
                .expect("IBGE id exceeds u32 range");
            let nome = m["nome"].as_str().expect("nome must be string");
            let uf_sigla = extract_uf_sigla(m, nome, id);

            json!({ "id": id, "nome": nome, "uf_sigla": uf_sigla })
        })
        .collect();

    eprintln!("Loaded {} IBGE municipalities", entries.len());
    entries
}

fn extract_uf_sigla<'a>(m: &'a Value, nome: &str, id: u32) -> &'a str {
    m["microrregiao"]["mesorregiao"]["UF"]["sigla"]
        .as_str()
        .or_else(|| m["regiao-imediata"]["regiao-intermediaria"]["UF"]["sigla"].as_str())
        .unwrap_or_else(|| panic!("cannot determine UF for {nome} (id={id})"))
}

fn cpf_cases() -> Value {
    let samples: Vec<cpf::Cpf> = ["52998224725", "34706612004", "00123456797"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let parse: Vec<Value> = ["529.982.247-25", "347.066.120-04", "00123456797"]
        .iter()
        .zip(&samples)
        .map(|(&input, parsed)| {
            json!({
                "input": input,
                "digits_only": parsed.as_str(),
                "formatted": parsed.to_string(),
                "masked": parsed.masked(),
                "fiscal_region": parsed.fiscal_region() as u8,
                "check_digits": [parsed.check_digits().0, parsed.check_digits().1],
            })
        })
        .collect();

    json!({
        "parse": parse,
        "is_valid": [
            { "input": "529.982.247-25", "expected": true },
            { "input": "52998224725",    "expected": true },
            { "input": "000.000.000-00", "expected": false },
            { "input": "11111111111",    "expected": false },
            { "input": "123",            "expected": false },
            { "input": "",               "expected": false },
        ],
        "is_valid_strict": strict_cases(cpf::is_valid_strict, &[
            ("529.982.247-25", None),
            ("52998224725",    None),
            ("000.000.000-00", Some("CPF with all equal digits is invalid")),
            ("11111111111",    Some("CPF with all equal digits is invalid")),
            ("123",            Some("CPF must contain exactly 11 digits")),
            ("",               Some("CPF must contain exactly 11 digits")),
            ("529.982.247-26", Some("CPF check digits are invalid")),
            ("529x982x247-25", Some("CPF format must be ###.###.###-## or 11 digits")),
        ]),
        "format": [
            { "input": "52998224725", "expected": "529.982.247-25" },
            { "input": "123",         "expected": null },
        ],
        "remove_symbols": [
            { "input": "529.982.247-25", "expected": "52998224725" },
            { "input": "52998224725",    "expected": "52998224725" },
        ],
        "compute_check_digits": [
            { "base": "529982247", "expected": [2, 5] },
            { "base": "347066120", "expected": [0, 4] },
            { "base": "001234567", "expected": [9, 7] },
            { "base": "12345",     "expected": null },
            { "base": "",          "expected": null },
        ],
        "generate": { "test_roundtrip": true },
    })
}

fn cnpj_cases() -> Value {
    let samples: Vec<cnpj::Cnpj> = ["11222333000181", "11444777000161"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let parse: Vec<Value> = ["11.222.333/0001-81", "11444777000161"]
        .iter()
        .zip(&samples)
        .map(|(&input, parsed)| {
            json!({
                "input": input,
                "digits_only": parsed.as_str(),
                "formatted": parsed.to_string(),
                "masked": parsed.masked(),
                "raiz": parsed.raiz(),
                "ordem": parsed.ordem(),
                "kind": parsed.kind() as u8,
                "establishment_type": parsed.establishment_type() as u8,
                "check_digits": [parsed.check_digits().0, parsed.check_digits().1],
            })
        })
        .collect();

    json!({
        "parse": parse,
        "is_valid": [
            { "input": "11.222.333/0001-81", "expected": true },
            { "input": "11222333000181",     "expected": true },
            { "input": "00.000.000/0000-00", "expected": false },
            { "input": "123",                "expected": false },
        ],
        "is_valid_strict": strict_cases(cnpj::is_valid_strict, &[
            ("11.222.333/0001-81", None),
            ("11222333000181",     None),
            ("00.000.000/0000-00", Some("CNPJ with all equal characters is invalid")),
            ("00000000000000",     Some("CNPJ with all equal characters is invalid")),
            ("123",                Some("CNPJ must contain exactly 14 characters")),
            ("",                   Some("CNPJ must contain exactly 14 characters")),
            ("11.222.333/0001-82", Some("CNPJ check digits are invalid")),
            ("11.222.333|0001-81", Some("CNPJ format must be XX.XXX.XXX/XXXX-DD or 14 characters")),
        ]),
        "format": [
            { "input": "11222333000181", "expected": "11.222.333/0001-81" },
            { "input": "123",            "expected": null },
        ],
        "remove_symbols": [
            { "input": "11.222.333/0001-81", "expected": "11222333000181" },
        ],
        "compute_check_digits": [
            { "base": "112223330001", "expected": [8, 1] },
            { "base": "114447770001", "expected": [6, 1] },
            { "base": "12345",        "expected": null },
            { "base": "",             "expected": null },
        ],
        "generate": { "test_roundtrip": true },
    })
}

fn cep_cases() -> Value {
    let samples: Vec<cep::Cep> = ["01310100", "20040020", "90010000"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let parse: Vec<Value> = ["01310-100", "20040020", "90010-000"]
        .iter()
        .zip(&samples)
        .map(|(&input, parsed)| {
            json!({
                "input": input,
                "digits_only": parsed.as_str(),
                "formatted": parsed.formatted(),
                "masked": parsed.masked(),
                "postal_region": parsed.postal_region() as u8,
                "state": parsed.state().map(uf::State::abbreviation),
            })
        })
        .collect();

    json!({
        "parse": parse,
        "is_valid": [
            { "input": "01310-100", "expected": true },
            { "input": "01310100",  "expected": true },
            { "input": "1234567",   "expected": false },
            { "input": "",          "expected": false },
        ],
        "is_valid_strict": strict_cases(cep::is_valid_strict, &[
            ("01310-100", None),
            ("01310100",  None),
            ("1234567",   Some("CEP must contain exactly 8 digits")),
            ("",          Some("CEP must contain exactly 8 digits")),
            ("0131010A",  Some("CEP contains invalid characters")),
            ("01310.100", Some("CEP format must be #####-### or 8 digits")),
        ]),
        "format": [
            { "input": "01310100", "expected": "01310-100" },
            { "input": "1234",     "expected": null },
        ],
        "remove_symbols": [
            { "input": "01310-100", "expected": "01310100" },
        ],
        "generate": { "test_roundtrip": true },
    })
}

fn uf_cases() -> Value {
    let states: Vec<Value> = uf::ALL
        .iter()
        .map(|s| {
            json!({
                "abbreviation": s.abbreviation(),
                "name": s.name(),
                "region": s.region().to_string(),
            })
        })
        .collect();

    json!({
        "states": states,
        "from_abbreviation": [
            { "input": "SP", "expected": "SP" },
            { "input": "sp", "expected": "SP" },
            { "input": "Rj", "expected": "RJ" },
            { "input": "XX", "expected": null },
            { "input": "",   "expected": null },
        ],
    })
}

fn municipio_cases(ibge_source: &[Value]) -> Value {
    json!({
        "count": municipio::ALL.len(),
        "from_ibge_code": from_ibge_cases(),
        "capital_of": capital_cases(),
        "search_by_name": search_cases(),
        "by_state_count": by_state_cases(),
        "ibge_source": ibge_source,
    })
}

fn from_ibge_cases() -> Vec<Value> {
    let valid = [3_550_308, 3_304_557, 3_509_502, 3_518_800];
    let invalid = [9_999_999, 0];

    let mut cases: Vec<Value> = valid
        .iter()
        .map(|&code| {
            let m = municipio::Municipio::from_ibge_code(code).unwrap();
            json!({
                "code": code,
                "name": m.name,
                "state": m.state.abbreviation(),
                "is_capital": m.is_capital(),
            })
        })
        .collect();

    cases.extend(
        invalid
            .iter()
            .map(|&code| json!({ "code": code, "expected": null })),
    );

    cases
}

fn capital_cases() -> Vec<Value> {
    uf::ALL
        .iter()
        .map(|&s| {
            let cap = municipio::Municipio::capital_of(s);
            json!({
                "state": s.abbreviation(),
                "name": cap.name,
                "ibge_code": cap.ibge_code,
            })
        })
        .collect()
}

fn search_cases() -> Vec<Value> {
    [
        "Florianópolis",
        "Campinas",
        "São Paulo",
        "Porto",
        "ZZZZNOTFOUND",
    ]
    .iter()
    .map(|&query| {
        let results: Vec<Value> = municipio::Municipio::search_by_name(query)
            .map(|m| {
                json!({
                    "ibge_code": m.ibge_code,
                    "name": m.name,
                    "state": m.state.abbreviation(),
                })
            })
            .collect();
        json!({ "query": query, "results": results })
    })
    .collect()
}

fn by_state_cases() -> Vec<Value> {
    uf::ALL
        .iter()
        .map(|&s| {
            json!({
                "state": s.abbreviation(),
                "count": municipio::Municipio::by_state(s).len(),
            })
        })
        .collect()
}
