#![no_main]

use libfuzzer_sys::fuzz_target;
use stdbr_core::cnpj;

fn exercise(input: &str) {
    let _ = cnpj::normalize(input);
    let _ = cnpj::is_valid(input);
    let _ = cnpj::is_valid_strict(input);
    let _ = cnpj::format_cnpj(input);
    let _ = cnpj::compute_check_digits(input);

    if let Ok(document) = input.parse::<cnpj::Cnpj>() {
        let _ = document.as_str();
        let _ = document.to_string();
        let _ = document.masked();
        let _ = document.kind();
        let _ = document.raiz();
        let _ = document.ordem();
        let _ = document.establishment_type();
        let _ = document.check_digits();
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        exercise(input);
    }
    exercise(&String::from_utf8_lossy(data));
});
