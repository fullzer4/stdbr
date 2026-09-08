use stdbr_core::rand::SeededRng;
use stdbr_core::{cep, cnpj, cpf, municipio, rg, uf};

const SEEDS: core::ops::Range<u64> = 0..512;

fn mutate_digit(value: &str, index: usize) -> String {
    let mut bytes = value.as_bytes().to_vec();
    bytes[index] = b'0' + (bytes[index] - b'0' + 1) % 10;
    String::from_utf8(bytes).unwrap()
}

#[test]
fn seeded_and_injected_generation_roundtrip() {
    for seed in SEEDS {
        let cpf = cpf::generate_with_seed(seed);
        let mut rng = SeededRng::new(seed);
        assert_eq!(cpf, cpf::generate_with_rng(&mut rng), "CPF seed {seed}");
        assert!(cpf::is_valid_strict(cpf.as_str()).is_ok());
        assert_eq!(cpf.as_str().parse::<cpf::Cpf>().unwrap(), cpf);
        assert_eq!(cpf.to_string().parse::<cpf::Cpf>().unwrap(), cpf);

        for kind in [cnpj::CnpjKind::Numeric, cnpj::CnpjKind::Alphanumeric] {
            let cnpj = cnpj::generate_with_seed(seed, kind);
            let mut rng = SeededRng::new(seed);
            assert_eq!(
                cnpj,
                cnpj::generate_with_rng(&mut rng, kind),
                "CNPJ seed {seed}, kind {kind:?}"
            );
            assert!(cnpj::is_valid_strict(cnpj.as_str()).is_ok());
            assert_eq!(cnpj.as_str().parse::<cnpj::Cnpj>().unwrap(), cnpj);
            assert_eq!(cnpj.to_string().parse::<cnpj::Cnpj>().unwrap(), cnpj);
        }

        let cep = cep::generate_cep_with_seed(seed);
        let mut rng = SeededRng::new(seed);
        assert_eq!(cep, cep::generate_cep_with_rng(&mut rng), "CEP seed {seed}");
        assert!(cep::is_valid_strict(cep.as_str()).is_ok());
        assert_eq!(cep.as_str().parse::<cep::Cep>().unwrap(), cep);
        assert_eq!(cep.to_string().parse::<cep::Cep>().unwrap(), cep);

        let rg = rg::generate_with_seed(seed, uf::State::SP).unwrap();
        let mut rng = SeededRng::new(seed);
        assert_eq!(
            rg,
            rg::generate_with_rng(&mut rng, uf::State::SP).unwrap(),
            "RG seed {seed}"
        );
        assert!(rg::is_valid_strict(rg.as_str(), uf::State::SP).is_ok());
        assert_eq!(rg::parse_strict(rg.as_str(), uf::State::SP).unwrap(), rg);
    }
}

#[test]
fn seeded_specialized_generation_respects_requested_domain() {
    let fiscal_regions = [
        cpf::FiscalRegion::Rs,
        cpf::FiscalRegion::DfGoMsMtTo,
        cpf::FiscalRegion::AcAmApPaRoRr,
        cpf::FiscalRegion::CeMaPi,
        cpf::FiscalRegion::AlPbPeRn,
        cpf::FiscalRegion::BaSe,
        cpf::FiscalRegion::Mg,
        cpf::FiscalRegion::EsRj,
        cpf::FiscalRegion::Sp,
        cpf::FiscalRegion::PrSc,
    ];

    for seed in 0..128 {
        for region in fiscal_regions {
            let value = cpf::generate_for_region_with_seed(seed, region);
            let mut rng = SeededRng::new(seed);
            assert_eq!(value, cpf::generate_for_region_with_rng(&mut rng, region));
            assert_eq!(value.fiscal_region(), region);
        }

        for state in uf::ALL {
            let value = cep::generate_for_state_with_seed(seed, state);
            let mut rng = SeededRng::new(seed);
            assert_eq!(value, cep::generate_for_state_with_rng(&mut rng, state));
            assert_eq!(value.state(), Some(state), "CEP {value}, seed {seed}");
        }
    }
}

#[test]
fn changing_any_check_digit_rejects_generated_documents() {
    for seed in SEEDS {
        let cpf = cpf::generate_with_seed(seed);
        for index in 9..11 {
            let mutated = mutate_digit(cpf.as_str(), index);
            assert!(
                !cpf::is_valid(&mutated),
                "CPF mutation {mutated}, seed {seed}"
            );
            assert!(mutated.parse::<cpf::Cpf>().is_err());
        }

        for kind in [cnpj::CnpjKind::Numeric, cnpj::CnpjKind::Alphanumeric] {
            let cnpj = cnpj::generate_with_seed(seed, kind);
            for index in 12..14 {
                let mutated = mutate_digit(cnpj.as_str(), index);
                assert!(
                    !cnpj::is_valid(&mutated),
                    "CNPJ mutation {mutated}, seed {seed}"
                );
                assert!(mutated.parse::<cnpj::Cnpj>().is_err());
            }
        }

        let rg = rg::generate_with_seed(seed, uf::State::SP).unwrap();
        let mut mutated = rg.as_str().as_bytes().to_vec();
        mutated[8] = if mutated[8] == b'X' {
            b'0'
        } else {
            b'0' + (mutated[8] - b'0' + 1) % 10
        };
        let mutated = String::from_utf8(mutated).unwrap();
        assert!(!rg::is_valid(&mutated, uf::State::SP));
        assert!(rg::parse_strict(&mutated, uf::State::SP).is_err());
    }
}

fn exercise_public_parsers(input: &str) {
    let _ = input.parse::<cpf::Cpf>();
    let _ = cpf::is_valid(input);
    let _ = cpf::is_valid_strict(input);
    let _ = cpf::format_cpf(input);
    let _ = cpf::compute_check_digits(input);

    let _ = input.parse::<cnpj::Cnpj>();
    let _ = cnpj::is_valid(input);
    let _ = cnpj::is_valid_strict(input);
    let _ = cnpj::format_cnpj(input);
    let _ = cnpj::compute_check_digits(input);

    let _ = input.parse::<cep::Cep>();
    let _ = cep::is_valid(input);
    let _ = cep::is_valid_strict(input);
    let _ = cep::format_cep(input);

    for state in uf::ALL {
        let _ = rg::is_valid(input, state);
        let _ = rg::is_valid_strict(input, state);
        let _ = rg::parse_strict(input, state);
        let _ = rg::format_rg(input, state);
        let _ = rg::compute_check_digit(input, state);
    }

    let _ = municipio::Municipio::search_by_name(input).take(2).count();
}

#[test]
fn unicode_and_size_corpus_never_panics() {
    let corpus = [
        "",
        "\0",
        "ASCII",
        "012345678901234567",
        "acentuação",
        "A\u{0303}",
        "１２３４５６７８９０",
        "١٢٣٤٥٦٧٨٩٠",
        "中文",
        "🦀🇧🇷",
        "\u{200d}\u{fe0f}\u{00a0}",
    ];
    for input in corpus {
        exercise_public_parsers(input);
    }

    let atoms = ['0', 'X', '.', '-', 'á', '\u{0301}', '界', '🦀', '\0'];
    for size in 0..=128 {
        let input: String = (0..size)
            .map(|index| atoms[(index * 17 + size * 31) % atoms.len()])
            .collect();
        exercise_public_parsers(&input);
    }
}
