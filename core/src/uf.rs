//! Brazilian states (Unidades Federativas).
//!
//! The 26 states plus the Federal District, with their two-letter abbreviations
//! and region classification.

use core::fmt;

/// The 27 Brazilian federative units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum State {
    AC = 0,
    AL = 1,
    AM = 2,
    AP = 3,
    BA = 4,
    CE = 5,
    DF = 6,
    ES = 7,
    GO = 8,
    MA = 9,
    MG = 10,
    MS = 11,
    MT = 12,
    PA = 13,
    PB = 14,
    PE = 15,
    PI = 16,
    PR = 17,
    RJ = 18,
    RN = 19,
    RO = 20,
    RR = 21,
    RS = 22,
    SC = 23,
    SE = 24,
    SP = 25,
    TO = 26,
}

/// All 27 states in alphabetical order by abbreviation.
pub const ALL: [State; 27] = [
    State::AC,
    State::AL,
    State::AM,
    State::AP,
    State::BA,
    State::CE,
    State::DF,
    State::ES,
    State::GO,
    State::MA,
    State::MG,
    State::MS,
    State::MT,
    State::PA,
    State::PB,
    State::PE,
    State::PI,
    State::PR,
    State::RJ,
    State::RN,
    State::RO,
    State::RR,
    State::RS,
    State::SC,
    State::SE,
    State::SP,
    State::TO,
];

/// Geographic region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Region {
    Norte = 0,
    Nordeste = 1,
    CentroOeste = 2,
    Sudeste = 3,
    Sul = 4,
}

impl State {
    /// Two-letter abbreviation (e.g. `"SP"`, `"RJ"`).
    pub fn abbreviation(self) -> &'static str {
        match self {
            State::AC => "AC",
            State::AL => "AL",
            State::AM => "AM",
            State::AP => "AP",
            State::BA => "BA",
            State::CE => "CE",
            State::DF => "DF",
            State::ES => "ES",
            State::GO => "GO",
            State::MA => "MA",
            State::MG => "MG",
            State::MS => "MS",
            State::MT => "MT",
            State::PA => "PA",
            State::PB => "PB",
            State::PE => "PE",
            State::PI => "PI",
            State::PR => "PR",
            State::RJ => "RJ",
            State::RN => "RN",
            State::RO => "RO",
            State::RR => "RR",
            State::RS => "RS",
            State::SC => "SC",
            State::SE => "SE",
            State::SP => "SP",
            State::TO => "TO",
        }
    }

    /// Full name of the state.
    pub fn name(self) -> &'static str {
        match self {
            State::AC => "Acre",
            State::AL => "Alagoas",
            State::AM => "Amazonas",
            State::AP => "Amapá",
            State::BA => "Bahia",
            State::CE => "Ceará",
            State::DF => "Distrito Federal",
            State::ES => "Espírito Santo",
            State::GO => "Goiás",
            State::MA => "Maranhão",
            State::MG => "Minas Gerais",
            State::MS => "Mato Grosso do Sul",
            State::MT => "Mato Grosso",
            State::PA => "Pará",
            State::PB => "Paraíba",
            State::PE => "Pernambuco",
            State::PI => "Piauí",
            State::PR => "Paraná",
            State::RJ => "Rio de Janeiro",
            State::RN => "Rio Grande do Norte",
            State::RO => "Rondônia",
            State::RR => "Roraima",
            State::RS => "Rio Grande do Sul",
            State::SC => "Santa Catarina",
            State::SE => "Sergipe",
            State::SP => "São Paulo",
            State::TO => "Tocantins",
        }
    }

    /// Geographic region this state belongs to.
    pub fn region(self) -> Region {
        match self {
            State::AC | State::AM | State::AP | State::PA | State::RO | State::RR | State::TO => {
                Region::Norte
            }
            State::AL
            | State::BA
            | State::CE
            | State::MA
            | State::PB
            | State::PE
            | State::PI
            | State::RN
            | State::SE => Region::Nordeste,
            State::DF | State::GO | State::MS | State::MT => Region::CentroOeste,
            State::ES | State::MG | State::RJ | State::SP => Region::Sudeste,
            State::PR | State::RS | State::SC => Region::Sul,
        }
    }

    /// Parse from a two-letter abbreviation (case-insensitive).
    pub fn from_abbreviation(s: &str) -> Option<Self> {
        match s.as_bytes() {
            b"AC" | b"ac" | b"Ac" => Some(State::AC),
            b"AL" | b"al" | b"Al" => Some(State::AL),
            b"AM" | b"am" | b"Am" => Some(State::AM),
            b"AP" | b"ap" | b"Ap" => Some(State::AP),
            b"BA" | b"ba" | b"Ba" => Some(State::BA),
            b"CE" | b"ce" | b"Ce" => Some(State::CE),
            b"DF" | b"df" | b"Df" => Some(State::DF),
            b"ES" | b"es" | b"Es" => Some(State::ES),
            b"GO" | b"go" | b"Go" => Some(State::GO),
            b"MA" | b"ma" | b"Ma" => Some(State::MA),
            b"MG" | b"mg" | b"Mg" => Some(State::MG),
            b"MS" | b"ms" | b"Ms" => Some(State::MS),
            b"MT" | b"mt" | b"Mt" => Some(State::MT),
            b"PA" | b"pa" | b"Pa" => Some(State::PA),
            b"PB" | b"pb" | b"Pb" => Some(State::PB),
            b"PE" | b"pe" | b"Pe" => Some(State::PE),
            b"PI" | b"pi" | b"Pi" => Some(State::PI),
            b"PR" | b"pr" | b"Pr" => Some(State::PR),
            b"RJ" | b"rj" | b"Rj" => Some(State::RJ),
            b"RN" | b"rn" | b"Rn" => Some(State::RN),
            b"RO" | b"ro" | b"Ro" => Some(State::RO),
            b"RR" | b"rr" | b"Rr" => Some(State::RR),
            b"RS" | b"rs" | b"Rs" => Some(State::RS),
            b"SC" | b"sc" | b"Sc" => Some(State::SC),
            b"SE" | b"se" | b"Se" => Some(State::SE),
            b"SP" | b"sp" | b"Sp" => Some(State::SP),
            b"TO" | b"to" | b"To" => Some(State::TO),
            _ => None,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.abbreviation())
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Region::Norte => "Norte",
            Region::Nordeste => "Nordeste",
            Region::CentroOeste => "Centro-Oeste",
            Region::Sudeste => "Sudeste",
            Region::Sul => "Sul",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_states_count() {
        assert_eq!(ALL.len(), 27);
    }

    #[test]
    fn abbreviation_roundtrip() {
        for state in ALL {
            let abbr = state.abbreviation();
            assert_eq!(abbr.len(), 2);
            assert_eq!(State::from_abbreviation(abbr), Some(state));
        }
    }

    #[test]
    fn from_abbreviation_case_insensitive() {
        assert_eq!(State::from_abbreviation("sp"), Some(State::SP));
        assert_eq!(State::from_abbreviation("SP"), Some(State::SP));
        assert_eq!(State::from_abbreviation("Sp"), Some(State::SP));
        assert_eq!(State::from_abbreviation("XX"), None);
        assert_eq!(State::from_abbreviation(""), None);
    }

    #[test]
    fn display_shows_abbreviation() {
        use alloc::string::ToString;
        assert_eq!(State::SP.to_string(), "SP");
        assert_eq!(State::RJ.to_string(), "RJ");
    }

    #[test]
    fn region_classification() {
        assert_eq!(State::SP.region(), Region::Sudeste);
        assert_eq!(State::AM.region(), Region::Norte);
        assert_eq!(State::BA.region(), Region::Nordeste);
        assert_eq!(State::DF.region(), Region::CentroOeste);
        assert_eq!(State::RS.region(), Region::Sul);
    }

    #[test]
    fn name_not_empty() {
        for state in ALL {
            assert!(!state.name().is_empty());
        }
    }
}
