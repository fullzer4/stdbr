//! RG (Registro Geral) - per-UF identity card validation, formatting and generation.
//!
//! RG is issued independently by each Brazilian state and there is no single
//! national algorithm. Only **SP** has a widely-adopted, documented mod-11
//! check digit algorithm — implemented here in full. For every other UF this
//! module performs **structural validation only** (length range + digit
//! charset) and treats the input as opaque digits.
//!
//! # SP algorithm
//!
//! 8-digit body `d1..d8`. Weights `2,3,4,5,6,7,8,9` applied left-to-right.
//! `sum = Σ d_i * (i+1)` for `i=0..8`. Check digit = `sum mod 11`; remainder
//! `10` is rendered as the ASCII character `'X'`. Canonical formatted form is
//! `XX.XXX.XXX-X`.
//!
//! # Other UFs
//!
//! `is_valid`/`is_valid_strict` only enforce length (5..=14 digits). Generation
//! returns `RgError::UnsupportedUfForGeneration`. Promote a UF from structural
//! to full validation by extending `uf_spec` once an authoritative algorithm
//! is verified.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::rand::{simple_seed, xorshift64};
use crate::uf::State;

const RG_MAX_LEN: usize = 14;
const SP_BODY_LEN: u8 = 9;
const SP_FORMATTED_LEN: u8 = 12;
const SP_BASE_LEN: usize = 8;
const SP_WEIGHTS: [u32; SP_BASE_LEN] = [2, 3, 4, 5, 6, 7, 8, 9];
const SP_FORMATTED_DIGIT_POS: [usize; 9] = [0, 1, 3, 4, 5, 7, 8, 9, 11];

const STRUCTURAL_MIN_LEN: u8 = 5;
const STRUCTURAL_MAX_LEN: u8 = 14;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RgError {
    InvalidLength,
    InvalidCharacter,
    InvalidFormat,
    InvalidCheckDigit,
    UnsupportedUfForGeneration,
}

impl fmt::Display for RgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLength => "RG length is outside the accepted range for this UF",
            Self::InvalidCharacter => "RG contains invalid characters",
            Self::InvalidFormat => "RG format does not match the canonical mask for this UF",
            Self::InvalidCheckDigit => "RG check digit is invalid",
            Self::UnsupportedUfForGeneration => {
                "RG generation is not supported for this UF (no verified algorithm)"
            }
        })
    }
}

/// Per-UF formatting and validation spec.
#[derive(Clone, Copy)]
struct UfSpec {
    body_len: Option<u8>,
    has_check_digit: bool,
    allow_x_terminator: bool,
    separators: &'static [(u8, char)],
    formatted_len: Option<u8>,
}

const STRUCTURAL_DEFAULT: UfSpec = UfSpec {
    body_len: None,
    has_check_digit: false,
    allow_x_terminator: false,
    separators: &[],
    formatted_len: None,
};

const SP_SPEC: UfSpec = UfSpec {
    body_len: Some(SP_BODY_LEN),
    has_check_digit: true,
    allow_x_terminator: true,
    separators: &[(2, '.'), (5, '.'), (8, '-')],
    formatted_len: Some(SP_FORMATTED_LEN),
};

const fn uf_spec(uf: State) -> UfSpec {
    match uf {
        State::SP => SP_SPEC,
        _ => STRUCTURAL_DEFAULT,
    }
}

/// A validated RG stored as ASCII bytes (digits, plus optional trailing `'X'`
/// for SP), tagged with its issuing UF.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rg {
    bytes: [u8; RG_MAX_LEN],
    len: u8,
    uf: State,
}

impl Rg {
    /// Unformatted body as `&str` (digits, optionally trailing `'X'`).
    pub fn as_str(&self) -> &str {
        // SAFETY: constructors guarantee ASCII digits/`X` only.
        unsafe { core::str::from_utf8_unchecked(&self.bytes[..self.len as usize]) }
    }

    /// Issuing state.
    pub fn uf(&self) -> State {
        self.uf
    }

    /// Formatted per the UF mask. For UFs without a known mask, returns the
    /// unformatted body.
    pub fn formatted(&self) -> String {
        format_with_spec(self.as_str(), uf_spec(self.uf))
            .unwrap_or_else(|| self.as_str().into())
    }

    /// Check digit when the UF has a verified algorithm. `Some(0..=9)` for
    /// digits, `Some(10)` for the SP `'X'` terminator, `None` otherwise.
    pub fn check_digit(&self) -> Option<u8> {
        let spec = uf_spec(self.uf);
        if !spec.has_check_digit {
            return None;
        }
        let last = self.bytes[self.len as usize - 1];
        if last == b'X' {
            Some(10)
        } else {
            Some(last - b'0')
        }
    }
}

impl AsRef<str> for Rg {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Rg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.formatted())
    }
}

impl fmt::Debug for Rg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rg({}, {})", self.uf.abbreviation(), self.formatted())
    }
}

/// Strip dots, dashes, slashes and whitespace. For SP, preserve a trailing
/// `'X'` (case-insensitive, normalized to uppercase). For other UFs, drop
/// non-digits.
pub fn remove_symbols(rg: &str, uf: State) -> String {
    let spec = uf_spec(uf);
    let mut out = String::with_capacity(rg.len());
    for c in rg.chars() {
        if c.is_ascii_digit() {
            out.push(c);
        } else if spec.allow_x_terminator && (c == 'X' || c == 'x') {
            out.push('X');
        }
    }
    out
}

/// Lenient validation - strips symbols, then checks length and (for SP) the
/// check digit.
pub fn is_valid(rg: &str, uf: State) -> bool {
    let spec = uf_spec(uf);
    let raw = remove_symbols(rg, uf);
    if !validate_body_length(&raw, spec) {
        return false;
    }
    if !validate_charset(&raw, spec) {
        return false;
    }
    if spec.has_check_digit {
        return sp_check_digit_ok(&raw);
    }
    true
}

/// Strict validation - input must be either the canonical formatted mask or
/// the unformatted body. No leading/trailing whitespace, no extra symbols.
pub fn is_valid_strict(rg: &str, uf: State) -> Result<(), RgError> {
    parse_strict(rg, uf).map(|_| ())
}

/// Insert per-UF separators. Returns `None` if the input length doesn't match
/// the UF body length (or, for variable-length UFs, the structural range).
pub fn format_rg(rg: &str, uf: State) -> Option<String> {
    let spec = uf_spec(uf);
    let raw = remove_symbols(rg, uf);
    if !validate_body_length(&raw, spec) {
        return None;
    }
    Some(format_with_spec(&raw, spec).unwrap_or(raw))
}

/// SP-only: compute the check digit for an 8-digit base. Returns `Some(0..=9)`
/// or `Some(10)` (caller renders as `'X'`); `None` for non-SP or wrong length.
pub fn compute_check_digit(base: &str, uf: State) -> Option<u8> {
    if !matches!(uf, State::SP) {
        return None;
    }
    let raw: Vec<u8> = base.bytes().filter(u8::is_ascii_digit).collect();
    if raw.len() != SP_BASE_LEN {
        return None;
    }
    Some(sp_check_digit(&raw))
}

/// Parse a raw RG string into a validated [`Rg`].
pub fn parse_strict(raw: &str, uf: State) -> Result<Rg, RgError> {
    let spec = uf_spec(uf);
    let bytes = raw.as_bytes();

    let body = if let Some(len) = spec.formatted_len.filter(|&l| bytes.len() == l as usize) {
        let _ = len;
        parse_formatted_sp(bytes)?
    } else if let Some(body_len) = spec.body_len {
        if bytes.len() != body_len as usize {
            return Err(RgError::InvalidLength);
        }
        parse_unformatted(bytes, spec)?
    } else {
        if bytes.len() < STRUCTURAL_MIN_LEN as usize
            || bytes.len() > STRUCTURAL_MAX_LEN as usize
        {
            return Err(RgError::InvalidLength);
        }
        parse_unformatted(bytes, spec)?
    };

    if spec.has_check_digit && !sp_check_digit_ok_bytes(&body) {
        return Err(RgError::InvalidCheckDigit);
    }

    Ok(Rg::from_body(&body, uf))
}

/// Generate a random valid RG. SP only; other UFs return
/// `RgError::UnsupportedUfForGeneration`.
pub fn generate_for_uf(uf: State) -> Result<Rg, RgError> {
    if !matches!(uf, State::SP) {
        return Err(RgError::UnsupportedUfForGeneration);
    }
    Ok(generate_sp())
}

/// Convenience: generate a random valid SP RG.
pub fn generate_sp() -> Rg {
    let mut seed = simple_seed();
    let mut digits = [0u8; SP_BASE_LEN];
    for d in &mut digits {
        seed = xorshift64(seed);
        *d = (seed % 10) as u8;
    }
    let mut body = [0u8; 9];
    for (i, &d) in digits.iter().enumerate() {
        body[i] = d + b'0';
    }
    let dv = sp_check_digit(&digits);
    body[8] = if dv == 10 { b'X' } else { b'0' + dv };
    Rg::from_body(&body, State::SP)
}

// ─────────────────────────── helpers ───────────────────────────

impl Rg {
    fn from_body(body: &[u8], uf: State) -> Self {
        let mut bytes = [0u8; RG_MAX_LEN];
        bytes[..body.len()].copy_from_slice(body);
        Self {
            bytes,
            len: body.len() as u8,
            uf,
        }
    }
}

fn validate_body_length(raw: &str, spec: UfSpec) -> bool {
    match spec.body_len {
        Some(n) => raw.len() == n as usize,
        None => {
            let n = raw.len();
            n >= STRUCTURAL_MIN_LEN as usize && n <= STRUCTURAL_MAX_LEN as usize
        }
    }
}

fn validate_charset(raw: &str, spec: UfSpec) -> bool {
    let bytes = raw.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if spec.allow_x_terminator {
        let (last_idx, rest) = (bytes.len() - 1, &bytes[..bytes.len() - 1]);
        if !rest.iter().all(u8::is_ascii_digit) {
            return false;
        }
        let last = bytes[last_idx];
        last.is_ascii_digit() || last == b'X'
    } else {
        bytes.iter().all(u8::is_ascii_digit)
    }
}

fn parse_unformatted(bytes: &[u8], spec: UfSpec) -> Result<Vec<u8>, RgError> {
    if !validate_charset(
        // SAFETY: caller already verified ASCII boundaries via spec body_len/range checks.
        unsafe { core::str::from_utf8_unchecked(bytes) },
        spec,
    ) {
        return Err(RgError::InvalidCharacter);
    }
    Ok(bytes.to_vec())
}

fn parse_formatted_sp(bytes: &[u8]) -> Result<Vec<u8>, RgError> {
    // Format mask: `XX.XXX.XXX-X` — separators at offsets 2, 6, 10.
    if bytes[2] != b'.' || bytes[6] != b'.' || bytes[10] != b'-' {
        return Err(RgError::InvalidFormat);
    }
    let mut out = Vec::with_capacity(SP_BODY_LEN as usize);
    for (i, &idx) in SP_FORMATTED_DIGIT_POS.iter().enumerate() {
        let b = bytes[idx];
        let last = i == SP_FORMATTED_DIGIT_POS.len() - 1;
        if b.is_ascii_digit() || (last && b == b'X') {
            out.push(b);
        } else if last && b == b'x' {
            out.push(b'X');
        } else {
            return Err(RgError::InvalidCharacter);
        }
    }
    Ok(out)
}

fn format_with_spec(body: &str, spec: UfSpec) -> Option<String> {
    if spec.separators.is_empty() {
        return None;
    }
    let body_len = spec.body_len? as usize;
    if body.len() != body_len {
        return None;
    }
    let total = body_len + spec.separators.len();
    let mut out = String::with_capacity(total);
    let mut sep_iter = spec.separators.iter().peekable();
    for (i, ch) in body.chars().enumerate() {
        while let Some(&&(pos, sep_ch)) = sep_iter.peek() {
            if pos as usize == i && i != 0 {
                out.push(sep_ch);
                sep_iter.next();
            } else {
                break;
            }
        }
        out.push(ch);
    }
    Some(out)
}

fn sp_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .zip(SP_WEIGHTS.iter())
        .map(|(&d, &w)| u32::from(d) * w)
        .sum();
    (sum % 11) as u8
}

fn sp_check_digit_ok(body: &str) -> bool {
    sp_check_digit_ok_bytes(body.as_bytes())
}

fn sp_check_digit_ok_bytes(bytes: &[u8]) -> bool {
    if bytes.len() != SP_BODY_LEN as usize {
        return false;
    }
    let mut digits = [0u8; SP_BASE_LEN];
    for (i, &b) in bytes[..SP_BASE_LEN].iter().enumerate() {
        if !b.is_ascii_digit() {
            return false;
        }
        digits[i] = b - b'0';
    }
    let expected = sp_check_digit(&digits);
    let last = bytes[SP_BASE_LEN];
    let actual = if last == b'X' {
        10
    } else if last.is_ascii_digit() {
        last - b'0'
    } else {
        return false;
    };
    actual == expected
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    fn sp_rg(base: [u8; SP_BASE_LEN]) -> Rg {
        let dv = sp_check_digit(&base);
        let mut body = [0u8; 9];
        for (i, &d) in base.iter().enumerate() {
            body[i] = d + b'0';
        }
        body[8] = if dv == 10 { b'X' } else { b'0' + dv };
        Rg::from_body(&body, State::SP)
    }

    #[test]
    fn sp_check_digit_known_values() {
        // 12345678: sum = 2+6+12+20+30+42+56+72 = 240; 240 mod 11 = 9.
        assert_eq!(sp_check_digit(&[1, 2, 3, 4, 5, 6, 7, 8]), 9);
        // 44444444: sum = 4*44 = 176; 176 mod 11 = 0.
        assert_eq!(sp_check_digit(&[4, 4, 4, 4, 4, 4, 4, 4]), 0);
        // 11111111: sum = 1*44 = 44; 44 mod 11 = 0.
        assert_eq!(sp_check_digit(&[1, 1, 1, 1, 1, 1, 1, 1]), 0);
        // Find an example whose DV = 10 (rendered 'X').
        // 00000005: sum = 9*5 = 45; 45 mod 11 = 1. Not 10.
        // Search a small example: 0,0,0,0,0,0,0,X → need sum mod 11 = 10.
        // 9*8 = 72; 72 mod 11 = 6. 9*9 = 81; 81 mod 11 = 4.
        // Try 1,0,0,0,0,0,0,1: 2 + 9 = 11; mod 11 = 0.
        // Try 1,0,0,0,0,0,0,0: 2; mod 11 = 2.
        // Try 0,0,0,0,0,0,0,9: 81 mod 11 = 4.
        // Try 5,0,0,0,0,0,0,0: 10; mod 11 = 10. ✓
        assert_eq!(sp_check_digit(&[5, 0, 0, 0, 0, 0, 0, 0]), 10);
    }

    #[test]
    fn is_valid_sp_accepts_valid() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(is_valid(rg.as_str(), State::SP));
        assert!(is_valid(&rg.to_string(), State::SP));
    }

    #[test]
    fn is_valid_sp_rejects_wrong_dv() {
        // 12345678 should give DV=9. Use 8 instead.
        assert!(!is_valid("123456788", State::SP));
        assert!(!is_valid("12.345.678-8", State::SP));
    }

    #[test]
    fn is_valid_sp_x_terminator() {
        let rg = sp_rg([5, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(rg.as_str(), "50000000X");
        assert!(is_valid(rg.as_str(), State::SP));
        assert!(is_valid("50.000.000-X", State::SP));
        assert!(is_valid("50.000.000-x", State::SP));
    }

    #[test]
    fn is_valid_sp_lenient_strips_garbage() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        let s = rg.as_str();
        let garbage = alloc::format!("{}!{}@{}#{}", &s[0..2], &s[2..5], &s[5..8], &s[8..9]);
        assert!(is_valid(&garbage, State::SP));
    }

    #[test]
    fn is_valid_sp_rejects_wrong_length() {
        assert!(!is_valid("", State::SP));
        assert!(!is_valid("12345", State::SP));
        assert!(!is_valid("1234567890", State::SP));
    }

    #[test]
    fn structural_other_uf_accepts_any_digits_in_range() {
        assert!(is_valid("12345", State::RJ));
        assert!(is_valid("1234567890", State::MG));
        assert!(is_valid("12345678901234", State::PR));
    }

    #[test]
    fn structural_other_uf_rejects_too_short_or_long() {
        assert!(!is_valid("1234", State::RJ));
        assert!(!is_valid("123456789012345", State::RJ));
    }

    #[test]
    fn structural_other_uf_rejects_letters() {
        // Lenient strips non-digits (X is not allowed for non-SP UFs); a letter-only
        // input collapses to an empty string and fails the length check.
        assert!(!is_valid("abcdef", State::RJ));
        assert!(!is_valid("", State::RJ));
    }

    #[test]
    fn strict_sp_accepts_unformatted() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(is_valid_strict(rg.as_str(), State::SP).is_ok());
    }

    #[test]
    fn strict_sp_accepts_formatted() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(is_valid_strict(&rg.to_string(), State::SP).is_ok());
    }

    #[test]
    fn strict_sp_rejects_misplaced_separators() {
        assert_eq!(
            is_valid_strict("123.45.678-9", State::SP),
            Err(RgError::InvalidFormat)
        );
        assert_eq!(
            is_valid_strict("12.345.6789-", State::SP),
            Err(RgError::InvalidFormat)
        );
    }

    #[test]
    fn strict_sp_rejects_garbage() {
        assert_eq!(
            is_valid_strict("12.345.678!9", State::SP),
            Err(RgError::InvalidFormat)
        );
    }

    #[test]
    fn strict_sp_rejects_bad_dv() {
        assert_eq!(
            is_valid_strict("123456788", State::SP),
            Err(RgError::InvalidCheckDigit)
        );
    }

    #[test]
    fn strict_sp_rejects_x_in_middle() {
        assert_eq!(
            is_valid_strict("1234X6789", State::SP),
            Err(RgError::InvalidCharacter)
        );
    }

    #[test]
    fn strict_other_uf_accepts_digits_only() {
        assert!(is_valid_strict("1234567", State::RJ).is_ok());
    }

    #[test]
    fn strict_other_uf_rejects_separators() {
        assert!(is_valid_strict("12.345.67", State::RJ).is_err());
    }

    #[test]
    fn parse_sp_roundtrip() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        let parsed = parse_strict(&rg.to_string(), State::SP).unwrap();
        assert_eq!(parsed, rg);
        let parsed_raw = parse_strict(rg.as_str(), State::SP).unwrap();
        assert_eq!(parsed_raw, rg);
    }

    #[test]
    fn parse_sp_x_terminator() {
        let parsed = parse_strict("50.000.000-X", State::SP).unwrap();
        assert_eq!(parsed.as_str(), "50000000X");
        assert_eq!(parsed.check_digit(), Some(10));
    }

    #[test]
    fn parse_other_uf_returns_digits() {
        let parsed = parse_strict("1234567", State::RJ).unwrap();
        assert_eq!(parsed.as_str(), "1234567");
        assert_eq!(parsed.uf(), State::RJ);
        assert_eq!(parsed.check_digit(), None);
    }

    #[test]
    fn format_sp_inserts_separators() {
        assert_eq!(
            format_rg("123456789", State::SP),
            Some("12.345.678-9".into())
        );
        assert_eq!(
            format_rg("50000000X", State::SP),
            Some("50.000.000-X".into())
        );
    }

    #[test]
    fn format_sp_passes_through_already_formatted() {
        assert_eq!(
            format_rg("12.345.678-9", State::SP),
            Some("12.345.678-9".into())
        );
    }

    #[test]
    fn format_other_uf_returns_digits_unchanged() {
        assert_eq!(format_rg("1234567", State::RJ), Some("1234567".into()));
    }

    #[test]
    fn format_returns_none_on_bad_length() {
        assert_eq!(format_rg("12", State::SP), None);
        assert_eq!(format_rg("12", State::RJ), None);
    }

    #[test]
    fn remove_symbols_sp_keeps_x() {
        assert_eq!(remove_symbols("50.000.000-X", State::SP), "50000000X");
        assert_eq!(remove_symbols("50.000.000-x", State::SP), "50000000X");
    }

    #[test]
    fn remove_symbols_other_uf_drops_letters() {
        assert_eq!(remove_symbols("12.345-67", State::RJ), "1234567");
        assert_eq!(remove_symbols("X1234567", State::RJ), "1234567");
    }

    #[test]
    fn compute_check_digit_sp() {
        assert_eq!(compute_check_digit("12345678", State::SP), Some(9));
        assert_eq!(compute_check_digit("44444444", State::SP), Some(0));
        assert_eq!(compute_check_digit("50000000", State::SP), Some(10));
    }

    #[test]
    fn compute_check_digit_rejects_other_uf() {
        assert_eq!(compute_check_digit("1234567", State::RJ), None);
    }

    #[test]
    fn compute_check_digit_rejects_bad_length() {
        assert_eq!(compute_check_digit("1234567", State::SP), None);
        assert_eq!(compute_check_digit("123456789", State::SP), None);
    }

    #[test]
    fn generate_sp_produces_valid() {
        for _ in 0..100 {
            let rg = generate_sp();
            assert!(is_valid(rg.as_str(), State::SP));
            let parsed = parse_strict(rg.as_str(), State::SP).unwrap();
            assert_eq!(parsed, rg);
        }
    }

    #[test]
    fn generate_for_uf_sp_ok_others_err() {
        assert!(generate_for_uf(State::SP).is_ok());
        assert_eq!(
            generate_for_uf(State::RJ),
            Err(RgError::UnsupportedUfForGeneration)
        );
    }

    #[test]
    fn rg_is_copy() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        let copy = rg;
        assert_eq!(rg, copy);
    }

    #[test]
    fn rg_as_ref_str() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        let r: &str = rg.as_ref();
        assert_eq!(r, rg.as_str());
    }

    #[test]
    fn debug_format_includes_uf() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        let dbg = alloc::format!("{rg:?}");
        assert!(dbg.starts_with("Rg(SP, "));
        assert!(dbg.ends_with(')'));
    }

    #[test]
    fn display_uses_formatted() {
        let rg = sp_rg([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(rg.to_string(), "12.345.678-9");
    }

    #[test]
    fn other_uf_display_passes_through() {
        let rg = parse_strict("1234567", State::RJ).unwrap();
        assert_eq!(rg.to_string(), "1234567");
    }
}
