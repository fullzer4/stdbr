//! Integration test: validates that hardcoded municipality data matches the IBGE source.
//!
//! The JSON file was fetched from:
//! <https://servicodados.ibge.gov.br/api/v1/localidades/municipios>
//!
//! To update: `curl -o tests/data/ibge_municipios_YYYY-MM-DD.json \
//!   "https://servicodados.ibge.gov.br/api/v1/localidades/municipios?orderBy=nome"`

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use stdbr_core::municipio;
use stdbr_core::uf::State;

/// Finds the most recent IBGE JSON file in tests/data/.
fn find_ibge_json() -> PathBuf {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut files: Vec<_> = fs::read_dir(&data_dir)
        .expect("tests/data/ directory must exist")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .is_some_and(|n| n.starts_with("ibge_municipios_") && n.ends_with(".json"))
        })
        .collect();
    files.sort_by_key(|e| e.file_name());
    files
        .last()
        .unwrap_or_else(|| panic!("No ibge_municipios_*.json found in {}", data_dir.display()))
        .path()
}

fn state_from_sigla(sigla: &str) -> State {
    State::from_abbreviation(sigla)
        .unwrap_or_else(|| panic!("Unknown state abbreviation: {sigla}"))
}

#[derive(Debug)]
struct IbgeMunicipio {
    id: u32,
    nome: String,
    uf_sigla: String,
}

fn parse_ibge_json(path: &PathBuf) -> Vec<IbgeMunicipio> {
    let content = fs::read_to_string(path).expect("Failed to read IBGE JSON");
    let data: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");
    let arr = data.as_array().expect("JSON root must be an array");

    arr.iter()
        .map(|m| {
            let id = m["id"].as_u64().expect("id must be u64") as u32;
            let nome = m["nome"].as_str().expect("nome must be string").to_string();

            // Try microrregiao path first, fallback to regiao-imediata
            let uf_sigla = m["microrregiao"]["mesorregiao"]["UF"]["sigla"]
                .as_str()
                .or_else(|| m["regiao-imediata"]["regiao-intermediaria"]["UF"]["sigla"].as_str())
                .unwrap_or_else(|| panic!("Cannot determine UF for {nome} (id={id})"))
                .to_string();

            IbgeMunicipio { id, nome, uf_sigla }
        })
        .collect()
}

#[test]
fn hardcoded_count_matches_ibge() {
    let path = find_ibge_json();
    let ibge = parse_ibge_json(&path);
    assert_eq!(
        municipio::ALL.len(),
        ibge.len(),
        "Municipality count mismatch: hardcoded={} vs IBGE={}",
        municipio::ALL.len(),
        ibge.len()
    );
}

#[test]
fn every_ibge_entry_exists_in_hardcoded() {
    let path = find_ibge_json();
    let ibge = parse_ibge_json(&path);

    let hardcoded: HashMap<u32, &municipio::Municipio> =
        municipio::ALL.iter().map(|m| (m.ibge_code, m)).collect();

    let mut missing = Vec::new();
    for entry in &ibge {
        if !hardcoded.contains_key(&entry.id) {
            missing.push(format!("{} ({}/{})", entry.id, entry.nome, entry.uf_sigla));
        }
    }

    assert!(
        missing.is_empty(),
        "Municipalities in IBGE but missing from hardcoded data:\n{}",
        missing.join("\n")
    );
}

#[test]
fn every_hardcoded_entry_exists_in_ibge() {
    let path = find_ibge_json();
    let ibge = parse_ibge_json(&path);

    let ibge_codes: HashMap<u32, &IbgeMunicipio> = ibge.iter().map(|m| (m.id, m)).collect();

    let mut extra = Vec::new();
    for m in municipio::ALL {
        if !ibge_codes.contains_key(&m.ibge_code) {
            extra.push(format!("{} ({}/{})", m.ibge_code, m.name, m.state));
        }
    }

    assert!(
        extra.is_empty(),
        "Municipalities in hardcoded data but missing from IBGE:\n{}",
        extra.join("\n")
    );
}

#[test]
fn names_match_ibge() {
    let path = find_ibge_json();
    let ibge = parse_ibge_json(&path);

    let ibge_map: HashMap<u32, &IbgeMunicipio> = ibge.iter().map(|m| (m.id, m)).collect();

    let mut mismatches = Vec::new();
    for m in municipio::ALL {
        if let Some(ibge_entry) = ibge_map.get(&m.ibge_code) {
            if m.name != ibge_entry.nome {
                mismatches.push(format!(
                    "  {}: hardcoded=\"{}\" vs IBGE=\"{}\"",
                    m.ibge_code, m.name, ibge_entry.nome
                ));
            }
        }
    }

    assert!(
        mismatches.is_empty(),
        "Name mismatches:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn states_match_ibge() {
    let path = find_ibge_json();
    let ibge = parse_ibge_json(&path);

    let ibge_map: HashMap<u32, &IbgeMunicipio> = ibge.iter().map(|m| (m.id, m)).collect();

    let mut mismatches = Vec::new();
    for m in municipio::ALL {
        if let Some(ibge_entry) = ibge_map.get(&m.ibge_code) {
            let expected_state = state_from_sigla(&ibge_entry.uf_sigla);
            if m.state != expected_state {
                mismatches.push(format!(
                    "  {} ({}): hardcoded={} vs IBGE={}",
                    m.ibge_code, m.name, m.state, ibge_entry.uf_sigla
                ));
            }
        }
    }

    assert!(
        mismatches.is_empty(),
        "State mismatches:\n{}",
        mismatches.join("\n")
    );
}
