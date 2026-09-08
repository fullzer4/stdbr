#![no_main]

use libfuzzer_sys::fuzz_target;
use stdbr_core::cep;

fn exercise(input: &str) {
    let _ = cep::normalize(input);
    let _ = cep::is_valid(input);
    let _ = cep::is_valid_strict(input);
    let _ = cep::format_cep(input);

    if let Ok(document) = input.parse::<cep::Cep>() {
        let _ = document.as_str();
        let _ = document.to_string();
        let _ = document.formatted();
        let _ = document.masked();
        let _ = document.digits();
        let _ = document.postal_region();
        let _ = document.state();
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        exercise(input);
    }
    exercise(&String::from_utf8_lossy(data));
});
