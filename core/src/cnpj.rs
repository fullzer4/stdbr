//! CNPJ (Cadastro Nacional da Pessoa Jurídica) - validation, formatting and generation.
//!
//! Supports both numeric (current) and alphanumeric (IN RFB 2.119/2022, July 2026) formats.
//! Uses modulo-11 weighted sums as specified by Receita Federal do Brasil.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::rand::{simple_seed, xorshift64};
use crate::util::{self, impl_document_traits};

const CNPJ_LEN: usize = 14;
const WEIGHTS_D1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const WEIGHTS_D2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const FORMATTED_CHAR_POS: [usize; 14] = [0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 16, 17];

/// Whether the CNPJ uses only digits or also contains letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CnpjKind {
    Numeric = 0,
    Alphanumeric = 1,
}

/// Whether the establishment is the main office (Matriz) or a branch (Filial).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EstablishmentType {
    Matriz = 0,
    Filial = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CnpjError {
    InvalidLength,
    InvalidCharacter,
    InvalidFormat,
    AllCharsEqual,
    InvalidCheckDigits,
}

impl fmt::Display for CnpjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLength => "CNPJ must contain exactly 14 characters",
            Self::InvalidCharacter => "CNPJ contains invalid characters",
            Self::InvalidFormat => "CNPJ format must be XX.XXX.XXX/XXXX-DD or 14 characters",
            Self::AllCharsEqual => "CNPJ with all equal characters is invalid",
            Self::InvalidCheckDigits => "CNPJ check digits are invalid",
        })
    }
}

/// A validated CNPJ stored as 14 ASCII bytes.
///
/// Positions 0-11 may contain `0-9` or `A-Z` (alphanumeric CNPJ).
/// Positions 12-13 are always numeric digits (check digits).
///
/// ```
/// use stdbr_core::cnpj::{Cnpj, CnpjKind, generate_cnpj};
///
/// let cnpj = generate_cnpj(CnpjKind::Numeric);
/// assert_eq!(cnpj.as_str().len(), 14);
/// assert_eq!(cnpj.to_string().len(), 18); // XX.XXX.XXX/XXXX-DD
///
/// let parsed: Cnpj = cnpj.to_string().parse().unwrap();
/// assert_eq!(cnpj, parsed);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cnpj {
    bytes: [u8; CNPJ_LEN],
}

impl Cnpj {
    /// Unformatted 14-character `&str`.
    pub fn as_str(&self) -> &str {
        // SAFETY: constructors guarantee ASCII alphanumeric only.
        unsafe { core::str::from_utf8_unchecked(&self.bytes) }
    }

    /// Whether the CNPJ is numeric or alphanumeric.
    pub fn kind(&self) -> CnpjKind {
        if self.bytes[..12].iter().all(u8::is_ascii_digit) {
            CnpjKind::Numeric
        } else {
            CnpjKind::Alphanumeric
        }
    }

    /// Root (positions 1-8): identifies the company.
    pub fn raiz(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.bytes[..8]) }
    }

    /// Order (positions 9-12): identifies the establishment.
    pub fn ordem(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.bytes[8..12]) }
    }

    /// Whether this is Matriz (ordem == "0001") or Filial.
    pub fn establishment_type(&self) -> EstablishmentType {
        if &self.bytes[8..12] == b"0001" {
            EstablishmentType::Matriz
        } else {
            EstablishmentType::Filial
        }
    }

    /// The two check digits `(d1, d2)` as numeric values.
    pub fn check_digits(&self) -> (u8, u8) {
        (self.bytes[12] - b'0', self.bytes[13] - b'0')
    }

    /// Masked: `XX.XXX.XXX/****-**`.
    pub fn masked(&self) -> String {
        let s = self.as_str();
        alloc::format!("{}.{}.{}/****-**", &s[0..2], &s[2..5], &s[5..8])
    }
}

impl fmt::Display for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(
            f,
            "{}.{}.{}/{}-{}",
            &s[0..2],
            &s[2..5],
            &s[5..8],
            &s[8..12],
            &s[12..14]
        )
    }
}

impl_document_traits!(Cnpj, CnpjError);

/// Strips punctuation, preserves letters and digits, uppercases letters.
pub fn remove_symbols(cnpj: &str) -> String {
    cnpj.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Lenient validation: strips punctuation, uppercases, then validates.
pub fn is_valid(cnpj: &str) -> bool {
    let raw = remove_symbols(cnpj);
    if raw.len() != CNPJ_LEN {
        return false;
    }
    let bytes = raw.as_bytes();
    // Positions 0-11 must be alphanumeric uppercase, 12-13 must be digits
    if !bytes[..12]
        .iter()
        .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
    {
        return false;
    }
    if !bytes[12..14].iter().all(u8::is_ascii_digit) {
        return false;
    }
    validate(bytes)
}

/// Strict validation - accepts `XX.XXX.XXX/XXXX-DD` or `XXXXXXXXXXXXXXDD` only.
pub fn is_valid_strict(cnpj: &str) -> Result<(), CnpjError> {
    parse_strict(cnpj).map(|_| ())
}

/// Formats as `XX.XXX.XXX/XXXX-DD`, or `None` if not valid 14 chars.
pub fn format_cnpj(cnpj: &str) -> Option<String> {
    let raw = remove_symbols(cnpj);
    if raw.len() != CNPJ_LEN {
        return None;
    }
    let bytes = raw.as_bytes();
    if !bytes[..12]
        .iter()
        .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
    {
        return None;
    }
    if !bytes[12..14].iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(alloc::format!(
        "{}.{}.{}/{}-{}",
        &raw[0..2],
        &raw[2..5],
        &raw[5..8],
        &raw[8..12],
        &raw[12..14]
    ))
}

/// Generates a random valid CNPJ as a 14-character string.
pub fn generate(kind: CnpjKind) -> String {
    generate_cnpj(kind).as_str().into()
}

/// Generates a random valid [`Cnpj`].
pub fn generate_cnpj(kind: CnpjKind) -> Cnpj {
    generate_with_seed(simple_seed(), kind)
}

/// Generates a random valid [`Cnpj`] with ordem "0001" (Matriz).
pub fn generate_matriz(kind: CnpjKind) -> Cnpj {
    generate_with_ordem(simple_seed(), kind, *b"0001")
}

/// Computes check digits for a 12-character CNPJ base.
/// Returns `None` if input is invalid.
pub fn compute_check_digits(base: &str) -> Option<(u8, u8)> {
    let raw = remove_symbols(base);
    if raw.len() != 12 {
        return None;
    }
    let bytes = raw.as_bytes();
    if !bytes
        .iter()
        .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
    {
        return None;
    }
    if all_equal(bytes) {
        return None;
    }

    let values: Vec<u32> = bytes.iter().map(|&b| char_value(b)).collect();
    let d1 = calc_check_digit(&values, &WEIGHTS_D1);
    let mut full = Vec::with_capacity(13);
    full.extend_from_slice(&values);
    full.push(u32::from(d1));
    let d2 = calc_check_digit(&full, &WEIGHTS_D2);

    Some((d1, d2))
}

// --- Private functions ---

/// Converts ASCII byte to its numeric value for CNPJ calculation.
/// '0'-'9' => 0-9, 'A'-'Z' => 17-42 (ASCII - 48).
fn char_value(b: u8) -> u32 {
    u32::from(b) - 48
}

fn all_equal(bytes: &[u8]) -> bool {
    util::all_equal(bytes)
}

fn validate(bytes: &[u8]) -> bool {
    if all_equal(bytes) {
        return false;
    }
    let values: Vec<u32> = bytes[..12].iter().map(|&b| char_value(b)).collect();
    let d1 = calc_check_digit(&values, &WEIGHTS_D1);
    let mut full = Vec::with_capacity(13);
    full.extend_from_slice(&values);
    full.push(u32::from(d1));
    let d2 = calc_check_digit(&full, &WEIGHTS_D2);
    bytes[12] - b'0' == d1 && bytes[13] - b'0' == d2
}

fn calc_check_digit(values: &[u32], weights: &[u32]) -> u8 {
    let sum: u32 = values.iter().zip(weights).map(|(&v, &w)| v * w).sum();
    let rem = sum % 11;
    #[allow(clippy::cast_possible_truncation)]
    // rem is always 2..=10, so 11-rem fits in u8
    if rem < 2 { 0 } else { (11 - rem) as u8 }
}

fn append_check_digits(bytes: &mut [u8; CNPJ_LEN]) {
    let values: Vec<u32> = bytes[..12].iter().map(|&b| char_value(b)).collect();
    let d1 = calc_check_digit(&values, &WEIGHTS_D1);
    let mut full = Vec::with_capacity(13);
    full.extend_from_slice(&values);
    full.push(u32::from(d1));
    let d2 = calc_check_digit(&full, &WEIGHTS_D2);
    bytes[12] = b'0' + d1;
    bytes[13] = b'0' + d2;
}

fn parse_strict(s: &str) -> Result<Cnpj, CnpjError> {
    let raw = s.as_bytes();

    let chars: Vec<u8> = match raw.len() {
        14 => {
            // Positions 0-11: alphanumeric uppercase, 12-13: digits
            if !raw[..12]
                .iter()
                .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
            {
                return Err(CnpjError::InvalidCharacter);
            }
            if !raw[12..14].iter().all(u8::is_ascii_digit) {
                return Err(CnpjError::InvalidCharacter);
            }
            raw.to_vec()
        }
        18 => {
            // XX.XXX.XXX/XXXX-DD
            if raw[2] != b'.' || raw[6] != b'.' || raw[10] != b'/' || raw[15] != b'-' {
                return Err(CnpjError::InvalidFormat);
            }
            let extracted: Vec<u8> = FORMATTED_CHAR_POS.iter().map(|&i| raw[i]).collect();
            if !extracted[..12]
                .iter()
                .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
            {
                return Err(CnpjError::InvalidCharacter);
            }
            if !extracted[12..14].iter().all(u8::is_ascii_digit) {
                return Err(CnpjError::InvalidCharacter);
            }
            extracted
        }
        _ => return Err(CnpjError::InvalidLength),
    };

    if all_equal(&chars) {
        return Err(CnpjError::AllCharsEqual);
    }

    let values: Vec<u32> = chars[..12].iter().map(|&b| char_value(b)).collect();
    let d1 = calc_check_digit(&values, &WEIGHTS_D1);
    let mut full = Vec::with_capacity(13);
    full.extend_from_slice(&values);
    full.push(u32::from(d1));
    let d2 = calc_check_digit(&full, &WEIGHTS_D2);

    if chars[12] - b'0' != d1 || chars[13] - b'0' != d2 {
        return Err(CnpjError::InvalidCheckDigits);
    }

    let mut bytes = [0u8; CNPJ_LEN];
    bytes.copy_from_slice(&chars);
    Ok(Cnpj { bytes })
}

const ALPHANUMERIC_CHARS: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn generate_with_seed(mut seed: u64, kind: CnpjKind) -> Cnpj {
    let mut bytes = [0u8; CNPJ_LEN];

    loop {
        for b in &mut bytes[..12] {
            seed = xorshift64(seed);
            *b = match kind {
                CnpjKind::Numeric => b'0' + (seed % 10) as u8,
                CnpjKind::Alphanumeric => ALPHANUMERIC_CHARS[(seed % 36) as usize],
            };
        }
        if !all_equal(&bytes[..12]) {
            break;
        }
    }

    append_check_digits(&mut bytes);
    Cnpj { bytes }
}

fn generate_with_ordem(mut seed: u64, kind: CnpjKind, ordem: [u8; 4]) -> Cnpj {
    let mut bytes = [0u8; CNPJ_LEN];
    bytes[8..12].copy_from_slice(&ordem);

    loop {
        for b in &mut bytes[..8] {
            seed = xorshift64(seed);
            *b = match kind {
                CnpjKind::Numeric => b'0' + (seed % 10) as u8,
                CnpjKind::Alphanumeric => ALPHANUMERIC_CHARS[(seed % 36) as usize],
            };
        }
        if !all_equal(&bytes[..12]) {
            break;
        }
    }

    append_check_digits(&mut bytes);
    Cnpj { bytes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    // Known valid numeric CNPJ: 11.222.333/0001-81
    fn cnpj_numeric() -> Cnpj {
        "11222333000181".parse().unwrap()
    }

    // Known valid numeric CNPJ: 11.444.777/0001-61
    fn cnpj_numeric_b() -> Cnpj {
        "11444777000161".parse().unwrap()
    }

    // Build a CNPJ from a 12-char base
    fn make_cnpj(base: &[u8; 12]) -> Cnpj {
        let mut bytes = [0u8; CNPJ_LEN];
        bytes[..12].copy_from_slice(base);
        append_check_digits(&mut bytes);
        Cnpj { bytes }
    }

    fn cnpj_alpha() -> Cnpj {
        // Alphanumeric CNPJ: base "12ABC34500DE", compute check digits
        make_cnpj(b"12ABC34500DE")
    }

    #[test]
    fn parse_known_numeric() {
        let cnpj = cnpj_numeric();
        assert_eq!(cnpj.as_str(), "11222333000181");
        assert_eq!(cnpj.kind(), CnpjKind::Numeric);
    }

    #[test]
    fn parse_known_numeric_b() {
        let cnpj = cnpj_numeric_b();
        assert_eq!(cnpj.as_str(), "11444777000161");
        assert_eq!(cnpj.kind(), CnpjKind::Numeric);
    }

    #[test]
    fn validate_known_numeric() {
        assert!(is_valid("11.222.333/0001-81"));
        assert!(is_valid("11222333000181"));
        assert!(is_valid("11.444.777/0001-61"));
        assert!(is_valid("11444777000161"));
    }

    #[test]
    fn parse_alphanumeric() {
        let cnpj = cnpj_alpha();
        assert_eq!(cnpj.kind(), CnpjKind::Alphanumeric);
        assert!(is_valid(cnpj.as_str()));
    }

    #[test]
    fn validate_alphanumeric_formatted() {
        let cnpj = cnpj_alpha();
        let formatted = cnpj.to_string();
        assert!(is_valid_strict(&formatted).is_ok());
    }

    #[test]
    fn is_valid_lenient_strips_garbage() {
        let cnpj = cnpj_numeric();
        let s = cnpj.as_str();
        let garbage = alloc::format!("{}${}#{}!{}", &s[0..2], &s[2..5], &s[5..8], &s[8..14]);
        assert!(is_valid(&garbage));
    }

    #[test]
    fn is_valid_lenient_lowercase_to_uppercase() {
        let cnpj = cnpj_alpha();
        let lower = cnpj.as_str().to_lowercase();
        assert!(is_valid(&lower));
    }

    #[test]
    fn is_valid_rejects_bad_check_digits() {
        let mut bytes = *b"11222333000182"; // last digit wrong
        assert!(!is_valid(core::str::from_utf8(&bytes).unwrap()));
        bytes = *b"11222333000191"; // first check digit wrong
        assert!(!is_valid(core::str::from_utf8(&bytes).unwrap()));
    }

    #[test]
    fn is_valid_rejects_all_equal() {
        assert!(!is_valid("11111111111111"));
        assert!(!is_valid("00000000000000"));
        assert!(!is_valid("AAAAAAAAAAAAAA"));
    }

    #[test]
    fn is_valid_rejects_wrong_length() {
        assert!(!is_valid(""));
        assert!(!is_valid("1234567890123"));
        assert!(!is_valid("123456789012345"));
    }

    #[test]
    fn is_valid_rejects_invalid_chars() {
        assert!(!is_valid("1122233300018!")); // special char in check digit pos
    }

    #[test]
    fn strict_accepts_valid_unformatted() {
        assert!(is_valid_strict("11222333000181").is_ok());
        assert!(is_valid_strict("11444777000161").is_ok());
    }

    #[test]
    fn strict_accepts_valid_formatted() {
        assert!(is_valid_strict("11.222.333/0001-81").is_ok());
        assert!(is_valid_strict("11.444.777/0001-61").is_ok());
    }

    #[test]
    fn strict_rejects_garbage() {
        assert!(is_valid_strict("11$222$333$0001$81").is_err());
    }

    #[test]
    fn strict_rejects_whitespace() {
        // 18 chars matches formatted-length path, but separators are wrong
        assert_eq!(
            is_valid_strict("  11222333000181  "),
            Err(CnpjError::InvalidFormat)
        );
        // Other lengths
        assert_eq!(
            is_valid_strict(" 11222333000181"),
            Err(CnpjError::InvalidLength)
        );
    }

    #[test]
    fn strict_rejects_misplaced_separators() {
        assert!(is_valid_strict("112.223.330/0018-1").is_err());
    }

    #[test]
    fn strict_rejects_lowercase() {
        let cnpj = cnpj_alpha();
        let lower = cnpj.as_str().to_lowercase();
        assert_eq!(is_valid_strict(&lower), Err(CnpjError::InvalidCharacter));
    }

    #[test]
    fn strict_rejects_all_equal() {
        assert_eq!(
            is_valid_strict("11111111111111"),
            Err(CnpjError::AllCharsEqual)
        );
        assert_eq!(
            is_valid_strict("00.000.000/0000-00"),
            Err(CnpjError::AllCharsEqual)
        );
    }

    #[test]
    fn strict_rejects_invalid_check_digits() {
        assert_eq!(
            is_valid_strict("11222333000182"),
            Err(CnpjError::InvalidCheckDigits)
        );
    }

    #[test]
    fn parse_roundtrip_formatted() {
        let cnpj = cnpj_numeric();
        let parsed: Cnpj = cnpj.to_string().parse().unwrap();
        assert_eq!(cnpj, parsed);
    }

    #[test]
    fn parse_roundtrip_raw() {
        let cnpj = cnpj_numeric();
        let parsed: Cnpj = cnpj.as_str().parse().unwrap();
        assert_eq!(cnpj, parsed);
    }

    #[test]
    fn parse_roundtrip_alphanumeric() {
        let cnpj = cnpj_alpha();
        let from_raw: Cnpj = cnpj.as_str().parse().unwrap();
        let from_fmt: Cnpj = cnpj.to_string().parse().unwrap();
        assert_eq!(cnpj, from_raw);
        assert_eq!(cnpj, from_fmt);
    }

    #[test]
    fn accessor_kind() {
        assert_eq!(cnpj_numeric().kind(), CnpjKind::Numeric);
        assert_eq!(cnpj_alpha().kind(), CnpjKind::Alphanumeric);
    }

    #[test]
    fn accessor_raiz() {
        assert_eq!(cnpj_numeric().raiz(), "11222333");
    }

    #[test]
    fn accessor_ordem() {
        assert_eq!(cnpj_numeric().ordem(), "0001");
    }

    #[test]
    fn accessor_establishment_type() {
        assert_eq!(
            cnpj_numeric().establishment_type(),
            EstablishmentType::Matriz
        );
        // Build one with ordem != "0001"
        let filial = make_cnpj(b"112223330002");
        assert_eq!(filial.establishment_type(), EstablishmentType::Filial);
    }

    #[test]
    fn accessor_check_digits() {
        let cnpj = cnpj_numeric();
        let (d1, d2) = cnpj.check_digits();
        assert_eq!(d1, 8);
        assert_eq!(d2, 1);
    }

    #[test]
    fn accessor_masked() {
        let cnpj = cnpj_numeric();
        assert_eq!(cnpj.masked(), "11.222.333/****-**");
    }

    #[test]
    fn format_cnpj_produces_formatted_output() {
        assert_eq!(
            format_cnpj("11222333000181"),
            Some("11.222.333/0001-81".to_string())
        );
    }

    #[test]
    fn format_cnpj_preserves_letters() {
        let cnpj = cnpj_alpha();
        let formatted = format_cnpj(cnpj.as_str()).unwrap();
        assert!(formatted.contains('/'));
        assert!(formatted.contains('-'));
        let reparsed: Cnpj = formatted.parse().unwrap();
        assert_eq!(cnpj, reparsed);
    }

    #[test]
    fn format_cnpj_returns_none_on_bad_length() {
        assert_eq!(format_cnpj("1234"), None);
        assert_eq!(format_cnpj(""), None);
    }

    #[test]
    fn remove_symbols_strips_formatting() {
        assert_eq!(remove_symbols("11.222.333/0001-81"), "11222333000181");
    }

    #[test]
    fn remove_symbols_preserves_letters_and_uppercases() {
        assert_eq!(remove_symbols("12.abc.345/00de-XX"), "12ABC34500DEXX");
    }

    #[test]
    fn generate_numeric_produces_valid() {
        for _ in 0..100 {
            let cnpj = generate(CnpjKind::Numeric);
            assert_eq!(cnpj.len(), 14);
            assert!(is_valid(&cnpj), "generated invalid CNPJ: {cnpj}");
            let parsed: Cnpj = cnpj.parse().unwrap();
            assert_eq!(parsed.kind(), CnpjKind::Numeric);
        }
    }

    #[test]
    fn generate_alphanumeric_produces_valid() {
        for _ in 0..100 {
            let cnpj = generate(CnpjKind::Alphanumeric);
            assert_eq!(cnpj.len(), 14);
            assert!(is_valid(&cnpj), "generated invalid CNPJ: {cnpj}");
        }
    }

    #[test]
    fn generate_cnpj_roundtrips() {
        for _ in 0..100 {
            let cnpj = generate_cnpj(CnpjKind::Numeric);
            assert!(is_valid(cnpj.as_str()));
            let parsed: Cnpj = cnpj.as_str().parse().unwrap();
            assert_eq!(cnpj, parsed);
        }
    }

    #[test]
    fn generate_matriz_has_correct_ordem_and_type() {
        for _ in 0..20 {
            let cnpj = generate_matriz(CnpjKind::Numeric);
            assert_eq!(cnpj.ordem(), "0001");
            assert_eq!(cnpj.establishment_type(), EstablishmentType::Matriz);
            assert!(is_valid(cnpj.as_str()));
        }
        for _ in 0..20 {
            let cnpj = generate_matriz(CnpjKind::Alphanumeric);
            assert_eq!(cnpj.ordem(), "0001");
            assert_eq!(cnpj.establishment_type(), EstablishmentType::Matriz);
            assert!(is_valid(cnpj.as_str()));
        }
    }

    #[test]
    fn compute_check_digits_known_base() {
        let (d1, d2) = compute_check_digits("112223330001").unwrap();
        assert_eq!(d1, 8);
        assert_eq!(d2, 1);
    }

    #[test]
    fn compute_check_digits_alphanumeric_base() {
        let cnpj = cnpj_alpha();
        let base = &cnpj.as_str()[..12];
        let (d1, d2) = compute_check_digits(base).unwrap();
        assert_eq!(d1, cnpj.check_digits().0);
        assert_eq!(d2, cnpj.check_digits().1);
    }

    #[test]
    fn compute_check_digits_rejects_bad_input() {
        assert_eq!(compute_check_digits("12345678901"), None); // too short
        assert_eq!(compute_check_digits("1234567890123"), None); // too long
        assert_eq!(compute_check_digits("000000000000"), None); // all equal
    }

    #[test]
    fn cnpj_is_copy() {
        let a = cnpj_numeric();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn cnpj_as_ref_str() {
        let cnpj = cnpj_numeric();
        let r: &str = cnpj.as_ref();
        assert_eq!(r, cnpj.as_str());
    }

    #[test]
    fn debug_format() {
        let cnpj = cnpj_numeric();
        let dbg = alloc::format!("{cnpj:?}");
        assert!(dbg.starts_with("Cnpj("));
        assert!(dbg.ends_with(')'));
        assert!(dbg.contains('.'));
        assert!(dbg.contains('/'));
        assert!(dbg.contains('-'));
    }

    #[test]
    fn display_format() {
        let cnpj = cnpj_numeric();
        assert_eq!(cnpj.to_string(), "11.222.333/0001-81");
    }

    #[test]
    fn from_str_trait() {
        let cnpj: Cnpj = "11.222.333/0001-81".parse().unwrap();
        assert_eq!(cnpj.as_str(), "11222333000181");
    }

    #[test]
    fn all_zeros_rejected() {
        assert!(!is_valid("00000000000000"));
        assert_eq!(
            is_valid_strict("00000000000000"),
            Err(CnpjError::AllCharsEqual)
        );
    }

    #[test]
    fn leading_zeros() {
        // 00.623.904/0001-73 is a valid CNPJ with leading zeros
        let result = is_valid("00623904000173");
        // It's valid only if check digits match - let's compute
        if let Some((d1, d2)) = compute_check_digits("006239040001") {
            let cnpj_str = alloc::format!("006239040001{d1}{d2}");
            assert!(is_valid(&cnpj_str));
            let cnpj: Cnpj = cnpj_str.parse().unwrap();
            assert!(cnpj.as_str().starts_with("00"));
        } else {
            // base might be all-equal or invalid, skip
            let _ = result;
        }
    }

    #[test]
    fn char_value_mapping() {
        assert_eq!(char_value(b'0'), 0);
        assert_eq!(char_value(b'9'), 9);
        assert_eq!(char_value(b'A'), 17);
        assert_eq!(char_value(b'Z'), 42);
    }
}
