#![allow(
    clippy::cast_possible_truncation,
    clippy::redundant_closure_for_method_calls
)]

use serde_json::Value;
use std::collections::HashMap;
use stdbr_core::{cep, cnpj, cpf, municipio, uf};

fn golden() -> Value {
    let path =
        std::env::var("GOLDEN_JSON").unwrap_or_else(|_| "tests/parity/golden.json".to_string());
    let json =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    serde_json::from_str(&json).expect("failed to parse golden.json")
}

// ── CPF ──────────────────────────────────────────────────────────────

#[test]
fn cpf_parse() {
    let cases = &golden()["cpf"]["parse"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let parsed: cpf::Cpf = input.parse().unwrap();

        assert_eq!(parsed.as_str(), case["digits_only"].as_str().unwrap());
        assert_eq!(parsed.to_string(), case["formatted"].as_str().unwrap());
        assert_eq!(parsed.masked(), case["masked"].as_str().unwrap());
        assert_eq!(
            parsed.fiscal_region() as u8,
            case["fiscal_region"].as_u64().unwrap() as u8
        );
        let cd = case["check_digits"].as_array().unwrap();
        assert_eq!(parsed.check_digits().0, cd[0].as_u64().unwrap() as u8);
        assert_eq!(parsed.check_digits().1, cd[1].as_u64().unwrap() as u8);
    }
}

#[test]
fn cpf_is_valid() {
    let cases = &golden()["cpf"]["is_valid"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_bool().unwrap();
        assert_eq!(cpf::is_valid(input), expected, "is_valid({input})");
    }
}

#[test]
fn cpf_is_valid_strict() {
    let cases = &golden()["cpf"]["is_valid_strict"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let valid = case["valid"].as_bool().unwrap();
        let result = cpf::is_valid_strict(input);
        if valid {
            assert!(result.is_ok(), "is_valid_strict({input}) expected Ok");
        } else {
            let err = result.unwrap_err();
            let expected_msg = case["error"].as_str().unwrap();
            assert_eq!(
                err.to_string(),
                expected_msg,
                "is_valid_strict({input}) error"
            );
        }
    }
}

#[test]
fn cpf_format() {
    let cases = &golden()["cpf"]["format"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str();
        assert_eq!(
            cpf::format_cpf(input).as_deref(),
            expected,
            "format({input})"
        );
    }
}

#[test]
fn cpf_remove_symbols() {
    let cases = &golden()["cpf"]["remove_symbols"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str().unwrap();
        assert_eq!(cpf::remove_symbols(input), expected);
    }
}

#[test]
fn cpf_compute_check_digits() {
    let cases = &golden()["cpf"]["compute_check_digits"];
    for case in cases.as_array().unwrap() {
        let base = case["base"].as_str().unwrap();
        let result = cpf::compute_check_digits(base);
        if case["expected"].is_null() {
            assert!(
                result.is_none(),
                "compute_check_digits({base}) expected None"
            );
        } else {
            let (d1, d2) = result.unwrap();
            let expected = case["expected"].as_array().unwrap();
            assert_eq!(d1, expected[0].as_u64().unwrap() as u8);
            assert_eq!(d2, expected[1].as_u64().unwrap() as u8);
        }
    }
}

#[test]
fn cpf_generate_roundtrip() {
    assert!(
        golden()["cpf"]["generate"]["test_roundtrip"]
            .as_bool()
            .unwrap()
    );
    let raw = cpf::generate();
    assert!(cpf::is_valid(&raw), "generated CPF {raw} is not valid");
    let parsed: cpf::Cpf = raw.parse().unwrap();
    assert_eq!(parsed.as_str(), raw);
}

// ── CNPJ ─────────────────────────────────────────────────────────────

#[test]
fn cnpj_parse() {
    let cases = &golden()["cnpj"]["parse"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let parsed: cnpj::Cnpj = input.parse().unwrap();

        assert_eq!(parsed.as_str(), case["digits_only"].as_str().unwrap());
        assert_eq!(parsed.to_string(), case["formatted"].as_str().unwrap());
        assert_eq!(parsed.masked(), case["masked"].as_str().unwrap());
        assert_eq!(parsed.raiz(), case["raiz"].as_str().unwrap());
        assert_eq!(parsed.ordem(), case["ordem"].as_str().unwrap());
        assert_eq!(parsed.kind() as u8, case["kind"].as_u64().unwrap() as u8);
        assert_eq!(
            parsed.establishment_type() as u8,
            case["establishment_type"].as_u64().unwrap() as u8
        );
        let cd = case["check_digits"].as_array().unwrap();
        assert_eq!(parsed.check_digits().0, cd[0].as_u64().unwrap() as u8);
        assert_eq!(parsed.check_digits().1, cd[1].as_u64().unwrap() as u8);
    }
}

#[test]
fn cnpj_is_valid() {
    let cases = &golden()["cnpj"]["is_valid"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_bool().unwrap();
        assert_eq!(cnpj::is_valid(input), expected, "is_valid({input})");
    }
}

#[test]
fn cnpj_is_valid_strict() {
    let cases = &golden()["cnpj"]["is_valid_strict"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let valid = case["valid"].as_bool().unwrap();
        let result = cnpj::is_valid_strict(input);
        if valid {
            assert!(result.is_ok(), "is_valid_strict({input}) expected Ok");
        } else {
            let err = result.unwrap_err();
            let expected_msg = case["error"].as_str().unwrap();
            assert_eq!(
                err.to_string(),
                expected_msg,
                "is_valid_strict({input}) error"
            );
        }
    }
}

#[test]
fn cnpj_format() {
    let cases = &golden()["cnpj"]["format"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str();
        assert_eq!(
            cnpj::format_cnpj(input).as_deref(),
            expected,
            "format({input})"
        );
    }
}

#[test]
fn cnpj_remove_symbols() {
    let cases = &golden()["cnpj"]["remove_symbols"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str().unwrap();
        assert_eq!(cnpj::remove_symbols(input), expected);
    }
}

#[test]
fn cnpj_compute_check_digits() {
    let cases = &golden()["cnpj"]["compute_check_digits"];
    for case in cases.as_array().unwrap() {
        let base = case["base"].as_str().unwrap();
        let result = cnpj::compute_check_digits(base);
        if case["expected"].is_null() {
            assert!(
                result.is_none(),
                "compute_check_digits({base}) expected None"
            );
        } else {
            let (d1, d2) = result.unwrap();
            let expected = case["expected"].as_array().unwrap();
            assert_eq!(d1, expected[0].as_u64().unwrap() as u8);
            assert_eq!(d2, expected[1].as_u64().unwrap() as u8);
        }
    }
}

#[test]
fn cnpj_generate_roundtrip() {
    assert!(
        golden()["cnpj"]["generate"]["test_roundtrip"]
            .as_bool()
            .unwrap()
    );
    let raw = cnpj::generate(cnpj::CnpjKind::Numeric);
    assert!(cnpj::is_valid(&raw), "generated CNPJ {raw} is not valid");
    let parsed: cnpj::Cnpj = raw.parse().unwrap();
    assert_eq!(parsed.as_str(), raw);
}

// ── CEP ──────────────────────────────────────────────────────────────

#[test]
fn cep_parse() {
    let cases = &golden()["cep"]["parse"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let parsed: cep::Cep = input.parse().unwrap();

        assert_eq!(parsed.as_str(), case["digits_only"].as_str().unwrap());
        assert_eq!(parsed.formatted(), case["formatted"].as_str().unwrap());
        assert_eq!(parsed.masked(), case["masked"].as_str().unwrap());
        assert_eq!(
            parsed.postal_region() as u8,
            case["postal_region"].as_u64().unwrap() as u8
        );
        let expected_state = case["state"].as_str();
        assert_eq!(
            parsed.state().map(|s| s.abbreviation()),
            expected_state,
            "state for {input}"
        );
    }
}

#[test]
fn cep_is_valid() {
    let cases = &golden()["cep"]["is_valid"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_bool().unwrap();
        assert_eq!(cep::is_valid(input), expected, "is_valid({input})");
    }
}

#[test]
fn cep_is_valid_strict() {
    let cases = &golden()["cep"]["is_valid_strict"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let valid = case["valid"].as_bool().unwrap();
        let result = cep::is_valid_strict(input);
        if valid {
            assert!(result.is_ok(), "is_valid_strict({input}) expected Ok");
        } else {
            let err = result.unwrap_err();
            let expected_msg = case["error"].as_str().unwrap();
            assert_eq!(
                err.to_string(),
                expected_msg,
                "is_valid_strict({input}) error"
            );
        }
    }
}

#[test]
fn cep_format() {
    let cases = &golden()["cep"]["format"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str();
        assert_eq!(
            cep::format_cep(input).as_deref(),
            expected,
            "format({input})"
        );
    }
}

#[test]
fn cep_remove_symbols() {
    let cases = &golden()["cep"]["remove_symbols"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str().unwrap();
        assert_eq!(cep::remove_symbols(input), expected);
    }
}

#[test]
fn cep_generate_roundtrip() {
    assert!(
        golden()["cep"]["generate"]["test_roundtrip"]
            .as_bool()
            .unwrap()
    );
    let raw = cep::generate();
    assert!(cep::is_valid(&raw), "generated CEP {raw} is not valid");
    let parsed: cep::Cep = raw.parse().unwrap();
    assert_eq!(parsed.as_str(), raw);
}

// ── UF ───────────────────────────────────────────────────────────────

#[test]
fn uf_states() {
    let cases = &golden()["uf"]["states"];
    let states = cases.as_array().unwrap();
    assert_eq!(states.len(), uf::ALL.len());

    for (i, case) in states.iter().enumerate() {
        let s = uf::ALL[i];
        assert_eq!(s.abbreviation(), case["abbreviation"].as_str().unwrap());
        assert_eq!(s.name(), case["name"].as_str().unwrap());
        assert_eq!(s.region().to_string(), case["region"].as_str().unwrap());
    }
}

#[test]
fn uf_from_abbreviation() {
    let cases = &golden()["uf"]["from_abbreviation"];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected"].as_str();
        let result = uf::State::from_abbreviation(input).map(|s| s.abbreviation());
        assert_eq!(result, expected, "from_abbreviation({input})");
    }
}

// ── Municipio ────────────────────────────────────────────────────────

#[test]
fn municipio_count() {
    let expected = golden()["municipio"]["count"].as_u64().unwrap() as usize;
    assert_eq!(municipio::ALL.len(), expected);
}

#[test]
fn municipio_from_ibge_code() {
    let cases = &golden()["municipio"]["from_ibge_code"];
    for case in cases.as_array().unwrap() {
        let code = case["code"].as_u64().unwrap() as u32;
        let m = municipio::Municipio::from_ibge_code(code);
        if case.get("expected") == Some(&Value::Null) {
            assert!(m.is_none(), "from_ibge_code({code}) should be None");
        } else {
            let m = m.unwrap();
            assert_eq!(m.name, case["name"].as_str().unwrap());
            assert_eq!(m.state.abbreviation(), case["state"].as_str().unwrap());
            assert_eq!(m.is_capital(), case["is_capital"].as_bool().unwrap());
        }
    }
}

#[test]
fn municipio_capital_of() {
    let cases = &golden()["municipio"]["capital_of"];
    for case in cases.as_array().unwrap() {
        let state_abbr = case["state"].as_str().unwrap();
        let state = uf::State::from_abbreviation(state_abbr).unwrap();
        let capital = municipio::Municipio::capital_of(state);
        assert_eq!(capital.name, case["name"].as_str().unwrap());
        assert_eq!(
            capital.ibge_code,
            case["ibge_code"].as_u64().unwrap() as u32
        );
    }
}

#[test]
fn municipio_search_by_name() {
    let cases = &golden()["municipio"]["search_by_name"];
    for case in cases.as_array().unwrap() {
        let query = case["query"].as_str().unwrap();
        let expected = case["results"].as_array().unwrap();
        let results: Vec<_> = municipio::Municipio::search_by_name(query).collect();

        assert_eq!(results.len(), expected.len(), "search({query}) count");
        for (m, e) in results.iter().zip(expected) {
            assert_eq!(m.ibge_code, e["ibge_code"].as_u64().unwrap() as u32);
            assert_eq!(m.name, e["name"].as_str().unwrap());
            assert_eq!(m.state.abbreviation(), e["state"].as_str().unwrap());
        }
    }
}

#[test]
fn municipio_by_state_count() {
    let cases = &golden()["municipio"]["by_state_count"];
    for case in cases.as_array().unwrap() {
        let state_abbr = case["state"].as_str().unwrap();
        let state = uf::State::from_abbreviation(state_abbr).unwrap();
        let expected = case["count"].as_u64().unwrap() as usize;
        assert_eq!(
            municipio::Municipio::by_state(state).len(),
            expected,
            "by_state({state_abbr})"
        );
    }
}

// ── Municipio IBGE sync ──────────────────────────────────────────────

fn ibge_source() -> Vec<(u32, String, String)> {
    let g = golden();
    let arr = g["municipio"]["ibge_source"]
        .as_array()
        .expect("ibge_source must be present in golden.json");
    arr.iter()
        .map(|m| {
            let id = m["id"].as_u64().unwrap() as u32;
            let nome = m["nome"].as_str().unwrap().to_string();
            let uf_sigla = m["uf_sigla"].as_str().unwrap().to_string();
            (id, nome, uf_sigla)
        })
        .collect()
}

#[test]
fn ibge_count_matches() {
    let ibge = ibge_source();
    assert_eq!(
        municipio::ALL.len(),
        ibge.len(),
        "municipality count: hardcoded={} vs IBGE={}",
        municipio::ALL.len(),
        ibge.len()
    );
}

#[test]
fn ibge_every_entry_exists_in_hardcoded() {
    let ibge = ibge_source();
    let hardcoded: HashMap<u32, &municipio::Municipio> =
        municipio::ALL.iter().map(|m| (m.ibge_code, m)).collect();

    let mut missing = Vec::new();
    for (id, nome, uf_sigla) in &ibge {
        if !hardcoded.contains_key(id) {
            missing.push(format!("{id} ({nome}/{uf_sigla})"));
        }
    }
    assert!(
        missing.is_empty(),
        "in IBGE but missing from hardcoded:\n{}",
        missing.join("\n")
    );
}

#[test]
fn ibge_every_hardcoded_exists_in_source() {
    let ibge = ibge_source();
    let ibge_ids: HashMap<u32, _> = ibge.iter().map(|(id, n, u)| (*id, (n, u))).collect();

    let mut extra = Vec::new();
    for m in municipio::ALL {
        if !ibge_ids.contains_key(&m.ibge_code) {
            extra.push(format!("{} ({}/{})", m.ibge_code, m.name, m.state));
        }
    }
    assert!(
        extra.is_empty(),
        "in hardcoded but missing from IBGE:\n{}",
        extra.join("\n")
    );
}

#[test]
fn ibge_names_match() {
    let ibge = ibge_source();
    let ibge_map: HashMap<u32, (&str, &str)> = ibge
        .iter()
        .map(|(id, nome, uf_sigla)| (*id, (nome.as_str(), uf_sigla.as_str())))
        .collect();

    let mut mismatches = Vec::new();
    for m in municipio::ALL {
        if let Some((nome, _)) = ibge_map.get(&m.ibge_code) {
            if m.name != *nome {
                mismatches.push(format!(
                    "  {}: hardcoded=\"{}\" vs IBGE=\"{nome}\"",
                    m.ibge_code, m.name
                ));
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "name mismatches:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn ibge_states_match() {
    let ibge = ibge_source();
    let ibge_map: HashMap<u32, (&str, &str)> = ibge
        .iter()
        .map(|(id, nome, uf_sigla)| (*id, (nome.as_str(), uf_sigla.as_str())))
        .collect();

    let mut mismatches = Vec::new();
    for m in municipio::ALL {
        if let Some((_, uf_sigla)) = ibge_map.get(&m.ibge_code) {
            if m.state.abbreviation() != *uf_sigla {
                mismatches.push(format!(
                    "  {} ({}): hardcoded={} vs IBGE={uf_sigla}",
                    m.ibge_code, m.name, m.state
                ));
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "state mismatches:\n{}",
        mismatches.join("\n")
    );
}
