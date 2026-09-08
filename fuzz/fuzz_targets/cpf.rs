#![no_main]

use libfuzzer_sys::fuzz_target;
use stdbr_core::cpf;

fn exercise(input: &str) {
    let _ = cpf::normalize(input);
    let _ = cpf::is_valid(input);
    let _ = cpf::is_valid_strict(input);
    let _ = cpf::format_cpf(input);
    let _ = cpf::compute_check_digits(input);

    if let Ok(document) = input.parse::<cpf::Cpf>() {
        let _ = document.as_str();
        let _ = document.to_string();
        let _ = document.masked();
        let _ = document.check_digits();
        let _ = document.fiscal_region();
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        exercise(input);
    }
    exercise(&String::from_utf8_lossy(data));
});
