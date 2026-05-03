//! CEP (Código de Endereçamento Postal) - validation, formatting and generation.
//!
//! Brazilian postal codes are 8 digits with no check digit. Validation is
//! structural (8 numeric characters, optionally formatted as `XXXXX-XXX`)
//! plus optional region/state lookup by range.

use alloc::string::String;
use core::fmt;

use crate::rand::{simple_seed, xorshift64};
use crate::uf::State;
use crate::util::impl_document_traits;

const CEP_LEN: usize = 8;
const FORMATTED_DIGIT_POS: [usize; 8] = [0, 1, 2, 3, 4, 6, 7, 8];

/// Postal region mapped by the first digit of a CEP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PostalRegion {
    GranSaoPaulo = 0,
    InteriorSaoPaulo = 1,
    RjEs = 2,
    Mg = 3,
    BaSe = 4,
    PeAlPbRn = 5,
    CePiMaPaAmAcApRr = 6,
    DfGoToMtMsRo = 7,
    PrSc = 8,
    Rs = 9,
}

impl PostalRegion {
    fn from_digit(d: u8) -> Self {
        assert!(d <= 9, "digit must be 0..=9");
        // SAFETY: repr(u8) with discriminants 0..=9, d validated above.
        unsafe { core::mem::transmute(d) }
    }
}

/// CEP range `(start, end)` inclusive for a given state.
fn cep_range(state: State) -> (u32, u32) {
    match state {
        State::SP => (1_000_000, 19_999_999),
        State::RJ => (20_000_000, 28_999_999),
        State::ES => (29_000_000, 29_999_999),
        State::MG => (30_000_000, 39_999_999),
        State::BA => (40_000_000, 48_999_999),
        State::SE => (49_000_000, 49_999_999),
        State::PE => (50_000_000, 56_999_999),
        State::AL => (57_000_000, 57_999_999),
        State::PB => (58_000_000, 58_999_999),
        State::RN => (59_000_000, 59_999_999),
        State::CE => (60_000_000, 63_999_999),
        State::PI => (64_000_000, 64_999_999),
        State::MA => (65_000_000, 65_999_999),
        State::PA => (66_000_000, 68_899_999),
        State::AM => (69_000_000, 69_299_999),
        State::AC => (69_900_000, 69_999_999),
        State::AP => (68_900_000, 68_999_999),
        State::RR => (69_300_000, 69_399_999),
        State::DF => (70_000_000, 72_799_999),
        State::GO => (72_800_000, 76_799_999),
        State::TO => (77_000_000, 77_999_999),
        State::MT => (78_000_000, 78_899_999),
        State::MS => (79_000_000, 79_999_999),
        State::RO => (76_800_000, 76_999_999),
        State::PR => (80_000_000, 87_999_999),
        State::SC => (88_000_000, 89_999_999),
        State::RS => (90_000_000, 99_999_999),
    }
}

/// Determines the state from a CEP numeric value by range lookup.
fn state_from_cep_value(value: u32) -> Option<State> {
    for &state in &crate::uf::ALL {
        let (start, end) = cep_range(state);
        if value >= start && value <= end {
            return Some(state);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CepError {
    InvalidLength,
    InvalidCharacter,
    InvalidFormat,
}

impl fmt::Display for CepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLength => "CEP must contain exactly 8 digits",
            Self::InvalidCharacter => "CEP contains invalid characters",
            Self::InvalidFormat => "CEP format must be #####-### or 8 digits",
        })
    }
}

/// A validated CEP stored as 8 ASCII bytes.
///
/// ```
/// use stdbr_core::cep::{Cep, generate_cep};
///
/// let cep = generate_cep();
/// assert_eq!(cep.as_str().len(), 8);
/// assert_eq!(cep.to_string().len(), 9); // #####-###
///
/// let parsed: Cep = cep.to_string().parse().unwrap();
/// assert_eq!(cep, parsed);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cep {
    bytes: [u8; CEP_LEN],
}

impl Cep {
    /// Unformatted 8-digit `&str`.
    pub fn as_str(&self) -> &str {
        // SAFETY: constructors guarantee ASCII digits only.
        unsafe { core::str::from_utf8_unchecked(&self.bytes) }
    }

    /// The 8 numeric digits (0–9).
    pub fn digits(&self) -> [u8; CEP_LEN] {
        self.bytes.map(|b| b - b'0')
    }

    /// Postal region derived from the 1st digit.
    pub fn postal_region(&self) -> PostalRegion {
        PostalRegion::from_digit(self.bytes[0] - b'0')
    }

    /// State lookup by CEP range.
    pub fn state(&self) -> Option<State> {
        state_from_cep_value(self.as_u32())
    }

    /// Formatted as `XXXXX-XXX`.
    pub fn formatted(&self) -> String {
        let s = self.as_str();
        alloc::format!("{}-{}", &s[0..5], &s[5..8])
    }

    /// Masked: `XXXXX-***`.
    pub fn masked(&self) -> String {
        let s = self.as_str();
        alloc::format!("{}-***", &s[0..5])
    }

    /// Numeric value of the CEP.
    fn as_u32(self) -> u32 {
        let d = self.digits();
        u32::from(d[0]) * 10_000_000
            + u32::from(d[1]) * 1_000_000
            + u32::from(d[2]) * 100_000
            + u32::from(d[3]) * 10_000
            + u32::from(d[4]) * 1_000
            + u32::from(d[5]) * 100
            + u32::from(d[6]) * 10
            + u32::from(d[7])
    }

    fn from_numeric(digits: [u8; CEP_LEN]) -> Self {
        Self {
            bytes: digits.map(|d| d + b'0'),
        }
    }
}

impl fmt::Display for Cep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{}-{}", &s[0..5], &s[5..8])
    }
}

impl_document_traits!(Cep, CepError);

pub fn remove_symbols(cep: &str) -> String {
    cep.chars().filter(char::is_ascii_digit).collect()
}

pub fn is_valid(cep: &str) -> bool {
    let raw = remove_symbols(cep);
    raw.len() == CEP_LEN
}

/// Strict validation - accepts `#####-###` or `########` only.
pub fn is_valid_strict(cep: &str) -> Result<(), CepError> {
    parse_strict(cep).map(|_| ())
}

/// Formats as `#####-###`, or `None` if not 8 digits.
pub fn format_cep(cep: &str) -> Option<String> {
    let d = remove_symbols(cep);
    (d.len() == CEP_LEN).then(|| alloc::format!("{}-{}", &d[0..5], &d[5..8]))
}

/// Generates a random valid CEP as an 8-digit string.
pub fn generate() -> String {
    generate_cep().as_str().into()
}

/// Generates a random valid [`Cep`].
pub fn generate_cep() -> Cep {
    let mut seed = simple_seed();
    let mut digits = [0u8; CEP_LEN];
    for d in &mut digits {
        seed = xorshift64(seed);
        *d = (seed % 10) as u8;
    }
    Cep::from_numeric(digits)
}

/// Generates a random [`Cep`] for a given postal region (1st digit fixed).
pub fn generate_for_region(region: PostalRegion) -> Cep {
    let mut seed = simple_seed();
    let mut digits = [0u8; CEP_LEN];
    digits[0] = region as u8;
    for d in &mut digits[1..] {
        seed = xorshift64(seed);
        *d = (seed % 10) as u8;
    }
    Cep::from_numeric(digits)
}

/// Generates a random [`Cep`] within the range of a given state.
pub fn generate_for_state(state: State) -> Cep {
    let (start, end) = cep_range(state);
    let mut seed = simple_seed();
    seed = xorshift64(seed);
    let range = end - start + 1;
    #[allow(clippy::cast_possible_truncation)]
    let value = start + (seed as u32 % range);

    let mut digits = [0u8; CEP_LEN];
    let mut v = value;
    for i in (0..CEP_LEN).rev() {
        digits[i] = (v % 10) as u8;
        v /= 10;
    }
    Cep::from_numeric(digits)
}

fn parse_strict(s: &str) -> Result<Cep, CepError> {
    let raw = s.as_bytes();

    match raw.len() {
        8 => {
            if !raw.iter().all(u8::is_ascii_digit) {
                return Err(CepError::InvalidCharacter);
            }
        }
        9 => {
            if raw[5] != b'-' {
                return Err(CepError::InvalidFormat);
            }
            for &i in &FORMATTED_DIGIT_POS {
                if !raw[i].is_ascii_digit() {
                    return Err(CepError::InvalidCharacter);
                }
            }
        }
        _ => return Err(CepError::InvalidLength),
    }

    let mut digits = [0u8; CEP_LEN];
    for (idx, &pos) in FORMATTED_DIGIT_POS.iter().enumerate() {
        if pos < raw.len() {
            digits[idx] = raw[pos] - b'0';
        }
    }

    // For the 8-char case, positions map 1:1
    if raw.len() == 8 {
        for (i, &b) in raw.iter().enumerate() {
            digits[i] = b - b'0';
        }
    }

    Ok(Cep::from_numeric(digits))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    fn cep_sp() -> Cep {
        // São Paulo: 01310-100
        Cep::from_numeric([0, 1, 3, 1, 0, 1, 0, 0])
    }

    fn cep_rj() -> Cep {
        // Rio de Janeiro: 20040-020
        Cep::from_numeric([2, 0, 0, 4, 0, 0, 2, 0])
    }

    fn cep_rs() -> Cep {
        // Porto Alegre: 90010-000
        Cep::from_numeric([9, 0, 0, 1, 0, 0, 0, 0])
    }

    #[test]
    fn is_valid_accepts_valid_unformatted() {
        assert!(is_valid(cep_sp().as_str()));
        assert!(is_valid(cep_rj().as_str()));
        assert!(is_valid(cep_rs().as_str()));
    }

    #[test]
    fn is_valid_accepts_valid_formatted() {
        assert!(is_valid(cep_sp().as_ref()));
        assert!(is_valid(cep_rj().as_ref()));
    }

    #[test]
    fn is_valid_lenient_strips_garbage() {
        let cep = cep_sp();
        let s = cep.as_str();
        let garbage = alloc::format!("{}${}", &s[0..5], &s[5..8]);
        assert!(is_valid(&garbage));

        let padded = alloc::format!("  {}  ", cep_sp());
        assert!(is_valid(&padded));
    }

    #[test]
    fn is_valid_rejects_wrong_length() {
        assert!(!is_valid(""));
        assert!(!is_valid("1234567"));
        assert!(!is_valid("123456789"));
    }

    #[test]
    fn is_valid_rejects_no_digits() {
        assert!(!is_valid("abcdefgh"));
        assert!(!is_valid("...---"));
    }

    #[test]
    fn strict_accepts_valid_unformatted() {
        assert!(is_valid_strict(cep_sp().as_str()).is_ok());
        assert!(is_valid_strict(cep_rj().as_str()).is_ok());
    }

    #[test]
    fn strict_accepts_valid_formatted() {
        assert!(is_valid_strict(cep_sp().as_ref()).is_ok());
        assert!(is_valid_strict(cep_rj().as_ref()).is_ok());
    }

    #[test]
    fn strict_rejects_garbage_between_digits() {
        let cep = cep_sp();
        let s = cep.as_str();
        let garbage = alloc::format!("{}${}", &s[0..5], &s[5..8]);
        assert!(is_valid_strict(&garbage).is_err());
    }

    #[test]
    fn strict_rejects_whitespace() {
        let padded = alloc::format!("  {}  ", cep_sp().as_str());
        assert_eq!(is_valid_strict(&padded), Err(CepError::InvalidLength));
    }

    #[test]
    fn strict_rejects_misplaced_separators() {
        assert_eq!(is_valid_strict("0131-0100"), Err(CepError::InvalidFormat));
    }

    #[test]
    fn strict_rejects_letters() {
        assert_eq!(is_valid_strict("abcdefgh"), Err(CepError::InvalidCharacter));
    }

    #[test]
    fn parse_roundtrip() {
        let cep = cep_sp();
        let parsed: Cep = cep.to_string().parse().unwrap();
        assert_eq!(cep, parsed);
        assert_eq!(parsed.as_str(), cep.as_str());
    }

    #[test]
    fn parse_unformatted() {
        let cep = cep_sp();
        let parsed: Cep = cep.as_str().parse().unwrap();
        assert_eq!(cep, parsed);
    }

    #[test]
    fn parse_equality_across_formats() {
        let from_fmt: Cep = cep_sp().to_string().parse().unwrap();
        let from_raw: Cep = cep_sp().as_str().parse().unwrap();
        assert_eq!(from_fmt, from_raw);
    }

    #[test]
    fn cep_is_copy() {
        let a = cep_sp();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn cep_as_ref_str() {
        let cep = cep_sp();
        let r: &str = cep.as_ref();
        assert_eq!(r, cep.as_str());
    }

    #[test]
    fn debug_format() {
        let cep = cep_sp();
        let dbg = alloc::format!("{cep:?}");
        assert!(dbg.starts_with("Cep("));
        assert!(dbg.ends_with(')'));
        assert!(dbg.contains('-'));
    }

    #[test]
    fn postal_region() {
        assert_eq!(cep_sp().postal_region(), PostalRegion::GranSaoPaulo);
        assert_eq!(cep_rj().postal_region(), PostalRegion::RjEs);
        assert_eq!(cep_rs().postal_region(), PostalRegion::Rs);
    }

    #[test]
    fn state_lookup() {
        assert_eq!(cep_sp().state(), Some(State::SP));
        assert_eq!(cep_rj().state(), Some(State::RJ));
        assert_eq!(cep_rs().state(), Some(State::RS));
    }

    #[test]
    fn state_abbreviation() {
        assert_eq!(State::SP.abbreviation(), "SP");
        assert_eq!(State::RJ.abbreviation(), "RJ");
        assert_eq!(State::RS.abbreviation(), "RS");
        assert_eq!(State::DF.abbreviation(), "DF");
    }

    #[test]
    fn formatted() {
        assert_eq!(cep_sp().formatted(), "01310-100");
        assert_eq!(cep_rj().formatted(), "20040-020");
    }

    #[test]
    fn masked() {
        assert_eq!(cep_sp().masked(), "01310-***");
        assert_eq!(cep_rj().masked(), "20040-***");
    }

    #[test]
    fn remove_symbols_strips_formatting() {
        let cep = cep_sp();
        let formatted = cep.to_string();
        assert_eq!(remove_symbols(&formatted), cep.as_str());
        assert_eq!(remove_symbols(cep.as_str()), cep.as_str());
        assert_eq!(remove_symbols(""), "");
    }

    #[test]
    fn format_cep_produces_formatted_output() {
        let cep = cep_sp();
        let formatted = cep.to_string();
        assert_eq!(format_cep(cep.as_str()), Some(formatted.clone()));
        assert_eq!(format_cep(&formatted), Some(formatted));
    }

    #[test]
    fn format_cep_returns_none_on_bad_length() {
        assert_eq!(format_cep("1234"), None);
        assert_eq!(format_cep(""), None);
    }

    #[test]
    fn format_cep_preserves_leading_zeros() {
        let cep = cep_sp();
        let formatted = format_cep(cep.as_str()).unwrap();
        assert!(formatted.starts_with("01"));
    }

    #[test]
    fn generate_produces_valid_ceps() {
        for _ in 0..100 {
            let cep = generate();
            assert_eq!(cep.len(), 8);
            assert!(is_valid(&cep), "generated invalid CEP: {cep}");
        }
    }

    #[test]
    fn generate_cep_roundtrips() {
        for _ in 0..100 {
            let cep = generate_cep();
            assert!(is_valid(cep.as_str()));
            let parsed: Cep = cep.as_str().parse().unwrap();
            assert_eq!(cep, parsed);
        }
    }

    #[test]
    fn generate_for_region_respects_first_digit() {
        let regions = [
            PostalRegion::GranSaoPaulo,
            PostalRegion::InteriorSaoPaulo,
            PostalRegion::RjEs,
            PostalRegion::Mg,
            PostalRegion::BaSe,
            PostalRegion::PeAlPbRn,
            PostalRegion::CePiMaPaAmAcApRr,
            PostalRegion::DfGoToMtMsRo,
            PostalRegion::PrSc,
            PostalRegion::Rs,
        ];
        for region in regions {
            let cep = generate_for_region(region);
            assert_eq!(cep.postal_region(), region);
            assert!(is_valid(cep.as_str()));
        }
    }

    #[test]
    fn generate_for_state_within_range() {
        let states = [
            State::SP,
            State::RJ,
            State::MG,
            State::RS,
            State::DF,
            State::AM,
            State::AC,
        ];
        for state in states {
            for _ in 0..10 {
                let cep = generate_for_state(state);
                assert_eq!(
                    cep.state(),
                    Some(state),
                    "CEP {cep} should map to {state:?}"
                );
            }
        }
    }

    #[test]
    fn leading_zero_cep() {
        let cep = cep_sp();
        assert!(cep.as_str().starts_with('0'));
        assert!(is_valid(cep.as_str()));

        let parsed: Cep = cep.as_str().parse().unwrap();
        assert_eq!(parsed.digits()[0], 0);
    }
}
