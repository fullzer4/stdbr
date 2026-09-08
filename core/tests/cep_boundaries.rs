use stdbr_core::cep::Cep;
use stdbr_core::uf::State;

const OFFICIAL_RANGES: &str = include_str!("../../tests/fixtures/cep_ranges.csv");

fn state(cep: &str) -> Option<State> {
    cep.parse::<Cep>().unwrap().state()
}

#[test]
fn state_boundaries_follow_the_official_postal_blocks() {
    let cases = [
        ("00999999", None),
        ("01000000", Some(State::SP)),
        ("19999999", Some(State::SP)),
        ("20000000", Some(State::RJ)),
        ("28999999", Some(State::RJ)),
        ("29000000", Some(State::ES)),
        ("29999999", Some(State::ES)),
        ("30000000", Some(State::MG)),
        ("39999999", Some(State::MG)),
        ("40000000", Some(State::BA)),
        ("48999999", Some(State::BA)),
        ("49000000", Some(State::SE)),
        ("49999999", Some(State::SE)),
        ("50000000", Some(State::PE)),
        ("56999999", Some(State::PE)),
        ("57000000", Some(State::AL)),
        ("57999999", Some(State::AL)),
        ("58000000", Some(State::PB)),
        ("58999999", Some(State::PB)),
        ("59000000", Some(State::RN)),
        ("59999999", Some(State::RN)),
        ("60000000", Some(State::CE)),
        ("63999999", Some(State::CE)),
        ("64000000", Some(State::PI)),
        ("64999999", Some(State::PI)),
        ("65000000", Some(State::MA)),
        ("65999999", Some(State::MA)),
        ("66000000", Some(State::PA)),
        ("68899999", Some(State::PA)),
        ("68900000", Some(State::AP)),
        ("68999999", Some(State::AP)),
        ("69000000", Some(State::AM)),
        ("69299999", Some(State::AM)),
        ("69300000", Some(State::RR)),
        ("69399999", Some(State::RR)),
        ("69400000", Some(State::AM)),
        ("69899999", Some(State::AM)),
        ("69900000", Some(State::AC)),
        ("69999999", Some(State::AC)),
        ("70000000", Some(State::DF)),
        ("72799999", Some(State::DF)),
        ("72800000", Some(State::GO)),
        ("72999999", Some(State::GO)),
        ("73000000", Some(State::DF)),
        ("73699999", Some(State::DF)),
        ("73700000", Some(State::GO)),
        ("76799999", Some(State::GO)),
        ("76800000", Some(State::RO)),
        ("76999999", Some(State::RO)),
        ("77000000", Some(State::TO)),
        ("77999999", Some(State::TO)),
        ("78000000", Some(State::MT)),
        ("78899999", Some(State::MT)),
        ("78900000", None),
        ("78999999", None),
        ("79000000", Some(State::MS)),
        ("79999999", Some(State::MS)),
        ("80000000", Some(State::PR)),
        ("87999999", Some(State::PR)),
        ("88000000", Some(State::SC)),
        ("89999999", Some(State::SC)),
        ("90000000", Some(State::RS)),
        ("99999999", Some(State::RS)),
    ];

    for (cep, expected) in cases {
        assert_eq!(state(cep), expected, "state for CEP {cep}");
    }
}

#[test]
fn seeded_state_generation_reaches_each_disjoint_segment() {
    for target in [State::AM, State::DF, State::GO] {
        let mut prefixes = [false; 2];
        for seed in 0..10_000 {
            let cep = stdbr_core::cep::generate_for_state_with_seed(seed, target);
            assert_eq!(cep.state(), Some(target), "generated CEP {cep}");

            let value: u32 = cep.as_str().parse().unwrap();
            prefixes[0] |= match target {
                State::AM => value <= 69_299_999,
                State::DF => value <= 72_799_999,
                State::GO => value <= 72_999_999,
                _ => unreachable!(),
            };
            prefixes[1] |= match target {
                State::AM => value >= 69_400_000,
                State::DF => value >= 73_000_000,
                State::GO => value >= 73_700_000,
                _ => unreachable!(),
            };
        }
        assert_eq!(prefixes, [true, true], "both segments for {target:?}");
    }
}

#[test]
fn checked_in_correios_fixture_matches_state_lookup_boundaries() {
    assert!(OFFICIAL_RANGES.contains("source_url=https://buscacepinter.correios.com.br/"));

    for line in OFFICIAL_RANGES.lines() {
        if line.is_empty() || line.starts_with('#') || line.starts_with("uf,") {
            continue;
        }
        let mut fields = line.split(',');
        let abbreviation = fields.next().unwrap();
        let start = fields.next().unwrap();
        let end = fields.next().unwrap();
        assert!(fields.next().is_none(), "unexpected fixture row: {line}");

        let expected = State::from_abbreviation(abbreviation).unwrap();
        assert_eq!(state(start), Some(expected), "fixture start {line}");
        assert_eq!(state(end), Some(expected), "fixture end {line}");
    }
}
