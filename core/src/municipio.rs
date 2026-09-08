//! Municípios brasileiros.
//!
//! Dados extraídos da API de Localidades do IBGE:
//! <https://servicodados.ibge.gov.br/api/v1/localidades/municipios>
//!
//! Referência oficial dos códigos:
//! <https://www.ibge.gov.br/explica/codigos-dos-municipios.php>

use crate::uf::State;
use core::fmt;

mod generated;
pub use generated::ALL;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Municipio {
    /// IBGE 7-digit code.
    pub ibge_code: u32,
    /// Municipality name.
    pub name: &'static str,
    /// State this municipality belongs to.
    pub state: State,
}

/// IBGE codes of the 27 state capitals, ordered to match [`crate::uf::ALL`].
#[allow(clippy::unreadable_literal)]
pub const CAPITAL_CODES: [u32; 27] = [
    1200401, // Rio Branco - AC
    2704302, // Maceió - AL
    1302603, // Manaus - AM
    1600303, // Macapá - AP
    2927408, // Salvador - BA
    2304400, // Fortaleza - CE
    5300108, // Brasília - DF
    3205309, // Vitória - ES
    5208707, // Goiânia - GO
    2111300, // São Luís - MA
    3106200, // Belo Horizonte - MG
    5002704, // Campo Grande - MS
    5103403, // Cuiabá - MT
    1501402, // Belém - PA
    2507507, // João Pessoa - PB
    2611606, // Recife - PE
    2211001, // Teresina - PI
    4106902, // Curitiba - PR
    3304557, // Rio de Janeiro - RJ
    2408102, // Natal - RN
    1100205, // Porto Velho - RO
    1400100, // Boa Vista - RR
    4314902, // Porto Alegre - RS
    4205407, // Florianópolis - SC
    2800308, // Aracaju - SE
    3550308, // São Paulo - SP
    1721000, // Palmas - TO
];

impl Municipio {
    /// Find a municipality by its IBGE code.
    pub fn from_ibge_code(code: u32) -> Option<&'static Municipio> {
        ALL.iter().find(|m| m.ibge_code == code)
    }

    /// Get the capital of a given state.
    pub fn capital_of(state: State) -> &'static Municipio {
        let code = CAPITAL_CODES
            .iter()
            .zip(crate::uf::ALL.iter())
            .find(|(_, s)| **s == state)
            .map(|(c, _)| *c)
            .unwrap();
        Self::from_ibge_code(code).unwrap()
    }

    /// Returns all municipalities in a given state.
    ///
    /// Efficient: exploits the fact that `ALL` is sorted by state.
    pub fn by_state(state: State) -> &'static [Municipio] {
        let start = ALL.iter().position(|m| m.state == state);
        match start {
            Some(s) => {
                let end = ALL[s..]
                    .iter()
                    .position(|m| m.state != state)
                    .map_or(ALL.len(), |e| s + e);
                &ALL[s..end]
            }
            None => &[],
        }
    }

    /// Find municipalities whose name contains the given substring (case-insensitive).
    pub fn search_by_name(query: &str) -> impl Iterator<Item = &'static Municipio> {
        let query_lower: alloc::string::String =
            query.chars().flat_map(char::to_lowercase).collect();
        ALL.iter().filter(move |m| {
            let name_lower: alloc::string::String =
                m.name.chars().flat_map(char::to_lowercase).collect();
            name_lower.contains(query_lower.as_str())
        })
    }

    pub fn is_capital(&self) -> bool {
        CAPITAL_CODES.contains(&self.ibge_code)
    }
}

impl fmt::Display for Municipio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.name, self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn total_count() {
        assert_eq!(ALL.len(), 5571);
    }

    #[test]
    fn sorted_by_state_then_name() {
        for pair in ALL.windows(2) {
            let a = &pair[0];
            let b = &pair[1];
            assert!(
                (a.state as u8) < (b.state as u8)
                    || ((a.state as u8) == (b.state as u8) && a.name <= b.name),
                "{a} should come before {b}"
            );
        }
    }

    #[test]
    fn each_state_has_municipalities() {
        for &state in &crate::uf::ALL {
            let munis = Municipio::by_state(state);
            assert!(!munis.is_empty(), "{state:?} has no municipalities");
        }
    }

    #[test]
    fn capitals_exist_in_all() {
        for &code in &CAPITAL_CODES {
            assert!(
                Municipio::from_ibge_code(code).is_some(),
                "Capital code {code} not found"
            );
        }
    }

    #[test]
    fn capital_of_each_state() {
        for &state in &crate::uf::ALL {
            let cap = Municipio::capital_of(state);
            assert_eq!(cap.state, state);
            assert!(cap.is_capital());
        }
    }

    #[test]
    fn from_ibge_code_sao_paulo() {
        let sp = Municipio::from_ibge_code(3_550_308).unwrap();
        assert_eq!(sp.name, "São Paulo");
        assert_eq!(sp.state, State::SP);
    }

    #[test]
    fn from_ibge_code_returns_none() {
        assert!(Municipio::from_ibge_code(0).is_none());
        assert!(Municipio::from_ibge_code(9_999_999).is_none());
    }

    #[test]
    fn by_state_returns_correct_state() {
        let sp_munis = Municipio::by_state(State::SP);
        for m in sp_munis {
            assert_eq!(m.state, State::SP);
        }
        // SP has 645 municipalities
        assert!(sp_munis.len() > 600);
    }

    #[test]
    fn search_by_name_works() {
        let results: Vec<_> = Municipio::search_by_name("porto").collect();
        assert!(results.len() >= 2);
    }

    #[test]
    fn is_capital_correct() {
        let sp = Municipio::from_ibge_code(3_550_308).unwrap();
        assert!(sp.is_capital());

        // Campinas is not a capital
        let campinas = Municipio::from_ibge_code(3_509_502).unwrap();
        assert!(!campinas.is_capital());
    }

    #[test]
    fn display_format() {
        use alloc::string::ToString;
        let sp = Municipio::from_ibge_code(3_550_308).unwrap();
        assert_eq!(sp.to_string(), "São Paulo/SP");
    }

    #[test]
    fn ibge_codes_are_7_digits() {
        for m in ALL {
            assert!(
                m.ibge_code >= 1_000_000 && m.ibge_code <= 9_999_999,
                "{} has invalid IBGE code: {}",
                m.name,
                m.ibge_code
            );
        }
    }
}
