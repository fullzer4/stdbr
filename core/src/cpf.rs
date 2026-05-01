//! CPF (Cadastro de Pessoas Físicas) - validation, formatting and generation.
//!
//! Uses modulo-11 weighted sums as specified by Receita Federal do Brasil.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

const CPF_LEN: usize = 11;
const WEIGHTS_D1: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
const WEIGHTS_D2: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
const FORMATTED_DIGIT_POS: [usize; 11] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13];

/// Fiscal region mapped by the 9th digit of a CPF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FiscalRegion {
    Rs = 0,
    DfGoMsMtTo = 1,
    AcAmApPaRoRr = 2,
    CeMaPi = 3,
    AlPbPeRn = 4,
    BaSe = 5,
    Mg = 6,
    EsRj = 7,
    Sp = 8,
    PrSc = 9,
}

impl FiscalRegion {
    fn from_digit(d: u8) -> Self {
        assert!(d <= 9, "digit must be 0..=9");
        // SAFETY: repr(u8) with discriminants 0..=9, d validated above.
        unsafe { core::mem::transmute(d) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpfError {
    InvalidLength,
    InvalidCharacter,
    InvalidFormat,
    AllDigitsEqual,
    InvalidCheckDigits,
}

impl fmt::Display for CpfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLength => "CPF must contain exactly 11 digits",
            Self::InvalidCharacter => "CPF contains invalid characters",
            Self::InvalidFormat => "CPF format must be ###.###.###-## or 11 digits",
            Self::AllDigitsEqual => "CPF with all equal digits is invalid",
            Self::InvalidCheckDigits => "CPF check digits are invalid",
        })
    }
}

/// A validated CPF stored as 11 ASCII bytes.
///
/// ```
/// use stdbr_core::cpf::{Cpf, generate_cpf};
///
/// let cpf = generate_cpf();
/// assert_eq!(cpf.as_str().len(), 11);
/// assert_eq!(cpf.to_string().len(), 14); // ###.###.###-##
///
/// let parsed: Cpf = cpf.to_string().parse().unwrap();
/// assert_eq!(cpf, parsed);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cpf {
    bytes: [u8; CPF_LEN],
}

impl Cpf {
    /// Unformatted 11-digit `&str`.
    pub fn as_str(&self) -> &str {
        // SAFETY: constructors guarantee ASCII digits only.
        unsafe { core::str::from_utf8_unchecked(&self.bytes) }
    }

    /// The 11 numeric digits (0–9).
    pub fn digits(&self) -> [u8; CPF_LEN] {
        self.bytes.map(|b| b - b'0')
    }

    /// Fiscal region derived from the 9th digit.
    pub fn fiscal_region(&self) -> FiscalRegion {
        FiscalRegion::from_digit(self.bytes[8] - b'0')
    }

    /// Masked: `XXX.***.***-XX`.
    pub fn masked(&self) -> String {
        let s = self.as_str();
        alloc::format!("{}.***.***-{}", &s[0..3], &s[9..11])
    }

    /// The two check digits `(d1, d2)`.
    pub fn check_digits(&self) -> (u8, u8) {
        (self.bytes[9] - b'0', self.bytes[10] - b'0')
    }

    fn from_numeric(digits: [u8; CPF_LEN]) -> Self {
        Self {
            bytes: digits.map(|d| d + b'0'),
        }
    }
}

impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{}.{}.{}-{}", &s[0..3], &s[3..6], &s[6..9], &s[9..11])
    }
}

impl fmt::Debug for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cpf({self})")
    }
}

impl AsRef<str> for Cpf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::str::FromStr for Cpf {
    type Err = CpfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_strict(s)
    }
}

pub fn remove_symbols(cpf: &str) -> String {
    cpf.chars().filter(char::is_ascii_digit).collect()
}

pub fn is_valid(cpf: &str) -> bool {
    let raw = remove_symbols(cpf);
    if raw.len() != CPF_LEN {
        return false;
    }
    let d: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    validate_digits(&d)
}

/// Strict validation - accepts `###.###.###-##` or `###########` only.
pub fn is_valid_strict(cpf: &str) -> Result<(), CpfError> {
    parse_strict(cpf).map(|_| ())
}

/// Formats as `###.###.###-##`, or `None` if not 11 digits.
pub fn format_cpf(cpf: &str) -> Option<String> {
    let d = remove_symbols(cpf);
    (d.len() == CPF_LEN)
        .then(|| alloc::format!("{}.{}.{}-{}", &d[0..3], &d[3..6], &d[6..9], &d[9..11]))
}

/// Generates a random valid CPF as an 11-digit string.
pub fn generate() -> String {
    generate_cpf().as_str().into()
}

/// Generates a random valid [`Cpf`].
pub fn generate_cpf() -> Cpf {
    generate_with_seed(simple_seed())
}

/// Generates a random valid [`Cpf`] for a given fiscal region.
pub fn generate_for_region(region: FiscalRegion) -> Cpf {
    let mut seed = simple_seed();
    let mut digits = [0u8; CPF_LEN];

    loop {
        for d in &mut digits[..8] {
            seed = xorshift64(seed);
            *d = (seed % 10) as u8;
        }
        digits[8] = region as u8;
        if !all_equal(&digits[..9]) {
            break;
        }
    }

    append_check_digits(&mut digits);
    Cpf::from_numeric(digits)
}

pub fn compute_check_digits(base: &str) -> Option<(u8, u8)> {
    let raw = remove_symbols(base);
    if raw.len() != 9 {
        return None;
    }

    let d: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    if all_equal(&d) {
        return None;
    }

    let d1 = check_digit(&d, &WEIGHTS_D1);
    let mut full = [0u8; 10];
    full[..9].copy_from_slice(&d);
    full[9] = d1;
    let d2 = check_digit(&full, &WEIGHTS_D2);

    Some((d1, d2))
}

fn all_equal(digits: &[u8]) -> bool {
    digits.iter().all(|&v| v == digits[0])
}

fn validate_digits(d: &[u8]) -> bool {
    !all_equal(d)
        && d[9] == check_digit(&d[..9], &WEIGHTS_D1)
        && d[10] == check_digit(&d[..10], &WEIGHTS_D2)
}

fn check_digit(digits: &[u8], weights: &[u32]) -> u8 {
    let sum: u32 = digits
        .iter()
        .zip(weights)
        .map(|(&d, &w)| u32::from(d) * w)
        .sum();
    let rem = (sum * 10) % 11;
    if rem == 10 { 0 } else { rem as u8 }
}

fn append_check_digits(digits: &mut [u8; CPF_LEN]) {
    digits[9] = check_digit(&digits[..9], &WEIGHTS_D1);
    digits[10] = check_digit(&digits[..10], &WEIGHTS_D2);
}

fn parse_strict(s: &str) -> Result<Cpf, CpfError> {
    let raw = s.as_bytes();

    let numeric: Vec<u8> = match raw.len() {
        11 => {
            if !raw.iter().all(u8::is_ascii_digit) {
                return Err(CpfError::InvalidCharacter);
            }
            raw.iter().map(|b| b - b'0').collect()
        }
        14 => {
            if raw[3] != b'.' || raw[7] != b'.' || raw[11] != b'-' {
                return Err(CpfError::InvalidFormat);
            }
            for &i in &FORMATTED_DIGIT_POS {
                if !raw[i].is_ascii_digit() {
                    return Err(CpfError::InvalidCharacter);
                }
            }
            FORMATTED_DIGIT_POS.iter().map(|&i| raw[i] - b'0').collect()
        }
        _ => return Err(CpfError::InvalidLength),
    };

    if all_equal(&numeric) {
        return Err(CpfError::AllDigitsEqual);
    }

    let d1 = check_digit(&numeric[..9], &WEIGHTS_D1);
    let d2 = check_digit(&numeric[..10], &WEIGHTS_D2);
    if numeric[9] != d1 || numeric[10] != d2 {
        return Err(CpfError::InvalidCheckDigits);
    }

    let mut digits = [0u8; CPF_LEN];
    digits.copy_from_slice(&numeric);
    Ok(Cpf::from_numeric(digits))
}

fn generate_with_seed(mut seed: u64) -> Cpf {
    let mut digits = [0u8; CPF_LEN];

    loop {
        for d in &mut digits[..9] {
            seed = xorshift64(seed);
            *d = (seed % 10) as u8;
        }
        if !all_equal(&digits[..9]) {
            break;
        }
    }

    append_check_digits(&mut digits);
    Cpf::from_numeric(digits)
}

#[cfg(feature = "std")]
fn simple_seed() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

#[cfg(not(feature = "std"))]
fn simple_seed() -> u64 {
    let stack_var: u8 = 0;
    let addr = &stack_var as *const u8 as u64;
    addr.wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

#[cfg(test)]
fn make_cpf(base: [u8; 9]) -> Cpf {
    let mut digits = [0u8; CPF_LEN];
    digits[..9].copy_from_slice(&base);
    append_check_digits(&mut digits);
    Cpf::from_numeric(digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    fn cpf_a() -> Cpf {
        make_cpf([5, 2, 9, 9, 8, 2, 2, 4, 7])
    }
    fn cpf_b() -> Cpf {
        make_cpf([3, 4, 7, 0, 6, 6, 1, 2, 0])
    }
    fn cpf_c() -> Cpf {
        make_cpf([0, 0, 1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn is_valid_accepts_valid_unformatted() {
        assert!(is_valid(cpf_a().as_str()));
        assert!(is_valid(cpf_b().as_str()));
        assert!(is_valid(cpf_c().as_str()));
    }

    #[test]
    fn is_valid_accepts_valid_formatted() {
        let a = cpf_a().to_string();
        let b = cpf_b().to_string();
        assert!(is_valid(&a));
        assert!(is_valid(&b));
    }

    #[test]
    fn is_valid_accepts_leading_zero_cpf() {
        let cpf = cpf_c();
        assert!(cpf.as_str().starts_with("00"));
        assert!(is_valid(cpf.as_str()));
    }

    #[test]
    fn is_valid_lenient_strips_garbage() {
        let cpf = cpf_a();
        let s = cpf.as_str();
        let garbage = alloc::format!("{}${}#{}!{}", &s[0..3], &s[3..6], &s[6..9], &s[9..11]);
        assert!(is_valid(&garbage));

        let padded = alloc::format!("  {}  ", cpf_a());
        assert!(is_valid(&padded));

        let mut spaced = String::new();
        for c in s.bytes() {
            use core::fmt::Write;
            write!(spaced, "{} ", c as char).unwrap();
        }
        assert!(is_valid(spaced.trim()));
    }

    #[test]
    fn is_valid_rejects_wrong_check_digits() {
        let mut bad = cpf_a().digits();
        bad[10] = (bad[10] + 1) % 10;
        let s: String = bad.iter().map(|&d| (b'0' + d) as char).collect();
        assert!(!is_valid(&s));
    }

    #[test]
    fn is_valid_rejects_all_same_digits() {
        for d in 0..=9u8 {
            let cpf: String = core::iter::repeat_n(char::from(b'0' + d), 11).collect();
            assert!(!is_valid(&cpf), "should reject {cpf}");
        }
    }

    #[test]
    fn is_valid_rejects_wrong_length() {
        assert!(!is_valid(""));
        assert!(!is_valid("1234567890"));
        assert!(!is_valid("123456789012"));
    }

    #[test]
    fn is_valid_rejects_no_digits() {
        assert!(!is_valid("abc.def.ghi-jk"));
        assert!(!is_valid("...---"));
    }

    #[test]
    fn is_valid_rejects_embedded_digits_in_long_string() {
        assert!(!is_valid("abc1234567890123def"));
    }

    #[test]
    fn strict_accepts_valid_unformatted() {
        assert!(is_valid_strict(cpf_a().as_str()).is_ok());
        assert!(is_valid_strict(cpf_b().as_str()).is_ok());
    }

    #[test]
    fn strict_accepts_valid_formatted() {
        let a = cpf_a().to_string();
        let b = cpf_b().to_string();
        assert!(is_valid_strict(&a).is_ok());
        assert!(is_valid_strict(&b).is_ok());
    }

    #[test]
    fn strict_rejects_garbage_between_digits() {
        let cpf = cpf_a();
        let s = cpf.as_str();
        let garbage = alloc::format!("{}${}#{}!{}", &s[0..3], &s[3..6], &s[6..9], &s[9..11]);
        assert!(is_valid_strict(&garbage).is_err());
    }

    #[test]
    fn strict_rejects_whitespace() {
        let padded = alloc::format!("  {}  ", cpf_a().as_str());
        assert_eq!(is_valid_strict(&padded), Err(CpfError::InvalidLength));

        let padded_fmt = alloc::format!(" {} ", cpf_a());
        assert_eq!(is_valid_strict(&padded_fmt), Err(CpfError::InvalidLength));
    }

    #[test]
    fn strict_rejects_misplaced_separators() {
        let cpf = cpf_a();
        let s = cpf.as_str();
        let bad_fmt = alloc::format!("{}.{}.{}.{}", &s[0..4], &s[4..6], &s[6..9], &s[9..11]);
        assert!(is_valid_strict(&bad_fmt).is_err());
    }

    #[test]
    fn strict_rejects_letters() {
        assert_eq!(
            is_valid_strict("abcdefghijk"),
            Err(CpfError::InvalidCharacter)
        );
    }

    #[test]
    fn strict_rejects_all_same_digits() {
        assert_eq!(
            is_valid_strict("11111111111"),
            Err(CpfError::AllDigitsEqual)
        );
        assert_eq!(
            is_valid_strict("000.000.000-00"),
            Err(CpfError::AllDigitsEqual)
        );
    }

    #[test]
    fn strict_rejects_invalid_check_digits() {
        let mut bad = cpf_a().digits();
        bad[10] = (bad[10] + 1) % 10;
        let s: String = bad.iter().map(|&d| (b'0' + d) as char).collect();
        assert_eq!(is_valid_strict(&s), Err(CpfError::InvalidCheckDigits));
    }

    #[test]
    fn parse_roundtrip() {
        let cpf = cpf_a();
        let parsed: Cpf = cpf.to_string().parse().unwrap();
        assert_eq!(cpf, parsed);
        assert_eq!(parsed.as_str(), cpf.as_str());
    }

    #[test]
    fn parse_unformatted() {
        let cpf = cpf_a();
        let parsed: Cpf = cpf.as_str().parse().unwrap();
        assert_eq!(cpf, parsed);
    }

    #[test]
    fn parse_equality_across_formats() {
        let from_fmt: Cpf = cpf_a().to_string().parse().unwrap();
        let from_raw: Cpf = cpf_a().as_str().parse().unwrap();
        assert_eq!(from_fmt, from_raw);
    }

    #[test]
    fn cpf_is_copy() {
        let a = cpf_a();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn cpf_as_ref_str() {
        let cpf = cpf_a();
        let r: &str = cpf.as_ref();
        assert_eq!(r, cpf.as_str());
    }

    #[test]
    fn debug_format() {
        let cpf = cpf_a();
        let dbg = alloc::format!("{cpf:?}");
        assert!(dbg.starts_with("Cpf("));
        assert!(dbg.ends_with(')'));
        assert!(dbg.contains('.'));
        assert!(dbg.contains('-'));
    }

    #[test]
    fn fiscal_region() {
        let cpf = cpf_a();
        let d = cpf.digits();
        assert_eq!(cpf.fiscal_region(), FiscalRegion::from_digit(d[8]));

        let cpf = cpf_b();
        assert_eq!(cpf.digits()[8], 0);
        assert_eq!(cpf.fiscal_region(), FiscalRegion::Rs);
    }

    #[test]
    fn digits_array() {
        let cpf = cpf_a();
        assert_eq!(cpf.digits()[..9], [5, 2, 9, 9, 8, 2, 2, 4, 7]);
    }

    #[test]
    fn parse_rejects_invalid() {
        let mut bad = cpf_a().digits();
        bad[10] = (bad[10] + 1) % 10;
        let s: String = bad.iter().map(|&d| (b'0' + d) as char).collect();
        assert!(s.parse::<Cpf>().is_err());
        assert!("abc".parse::<Cpf>().is_err());
        assert!("".parse::<Cpf>().is_err());
    }

    #[test]
    fn masked() {
        let cpf = cpf_a();
        let s = cpf.as_str();
        let expected = alloc::format!("{}.***.***-{}", &s[0..3], &s[9..11]);
        assert_eq!(cpf.masked(), expected);
        assert_eq!(cpf.masked().len(), 14);
    }

    #[test]
    fn check_digits() {
        let cpf = cpf_a();
        let (d1, d2) = cpf.check_digits();
        assert_eq!(d1, cpf.digits()[9]);
        assert_eq!(d2, cpf.digits()[10]);
    }

    #[test]
    fn remove_symbols_strips_formatting() {
        let cpf = cpf_a();
        let formatted = cpf.to_string();
        assert_eq!(remove_symbols(&formatted), cpf.as_str());
        assert_eq!(remove_symbols(cpf.as_str()), cpf.as_str());
        assert_eq!(remove_symbols(""), "");
    }

    #[test]
    fn remove_symbols_strips_arbitrary_chars() {
        assert_eq!(remove_symbols("abc123def456ghi78901"), "12345678901");
    }

    #[test]
    fn format_cpf_produces_formatted_output() {
        let cpf = cpf_a();
        let formatted = cpf.to_string();
        assert_eq!(format_cpf(cpf.as_str()), Some(formatted.clone()));
        assert_eq!(format_cpf(&formatted), Some(formatted));
    }

    #[test]
    fn format_cpf_returns_none_on_bad_length() {
        assert_eq!(format_cpf("1234"), None);
        assert_eq!(format_cpf(""), None);
    }

    #[test]
    fn format_cpf_preserves_leading_zeros() {
        let cpf = cpf_c();
        let formatted = format_cpf(cpf.as_str()).unwrap();
        assert!(formatted.starts_with("001."));
    }

    #[test]
    fn generate_produces_valid_cpfs() {
        for _ in 0..100 {
            let cpf = generate();
            assert_eq!(cpf.len(), 11);
            assert!(is_valid(&cpf), "generated invalid CPF: {cpf}");
        }
    }

    #[test]
    fn generate_cpf_roundtrips() {
        for _ in 0..100 {
            let cpf = generate_cpf();
            assert!(is_valid(cpf.as_str()));
            let parsed: Cpf = cpf.as_str().parse().unwrap();
            assert_eq!(cpf, parsed);
        }
    }

    #[test]
    fn generate_for_region_respects_region_digit() {
        let regions = [
            FiscalRegion::Rs,
            FiscalRegion::DfGoMsMtTo,
            FiscalRegion::AcAmApPaRoRr,
            FiscalRegion::CeMaPi,
            FiscalRegion::AlPbPeRn,
            FiscalRegion::BaSe,
            FiscalRegion::Mg,
            FiscalRegion::EsRj,
            FiscalRegion::Sp,
            FiscalRegion::PrSc,
        ];
        for region in regions {
            let cpf = generate_for_region(region);
            assert_eq!(cpf.fiscal_region(), region);
            assert!(is_valid(cpf.as_str()));
        }
    }

    #[test]
    fn compute_check_digits_known_base() {
        let cpf = cpf_a();
        let base = &cpf.as_str()[..9];
        let (d1, d2) = compute_check_digits(base).unwrap();
        assert_eq!(d1, cpf.digits()[9]);
        assert_eq!(d2, cpf.digits()[10]);
    }

    #[test]
    fn compute_check_digits_rejects_bad_input() {
        assert_eq!(compute_check_digits("12345678"), None);
        assert_eq!(compute_check_digits("1234567890"), None);
        assert_eq!(compute_check_digits("000000000"), None);
    }

    #[test]
    fn leading_zero_cpf() {
        let cpf = cpf_c();
        assert!(cpf.as_str().starts_with("00"));
        assert!(is_valid(cpf.as_str()));

        let parsed: Cpf = cpf.as_str().parse().unwrap();
        assert_eq!(parsed.digits()[0], 0);
        assert_eq!(parsed.digits()[1], 0);
    }
}
