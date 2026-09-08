#![no_main]

use libfuzzer_sys::fuzz_target;
use stdbr_core::{rg, uf};

fn exercise(input: &str) {
    for state in uf::ALL {
        let _ = rg::normalize(input, state);
        let _ = rg::is_valid(input, state);
        let _ = rg::is_valid_strict(input, state);
        let _ = rg::format_rg(input, state);
        let _ = rg::compute_check_digit(input, state);

        if let Ok(document) = rg::parse_strict(input, state) {
            let _ = document.as_str();
            let _ = document.to_string();
            let _ = document.formatted();
            let _ = document.masked();
            let _ = document.body();
            let _ = document.check_digit();
            let _ = document.uf();
        }
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        exercise(input);
    }
    exercise(&String::from_utf8_lossy(data));
});
