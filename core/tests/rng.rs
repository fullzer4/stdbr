use stdbr_core::rand::{RandomSource, SeededRng};
use stdbr_core::{cep, cnpj, cpf, rg, uf};

#[derive(Clone)]
struct TestRng(u64);

impl RandomSource for TestRng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }
}

#[test]
fn seeded_rng_makes_all_generators_deterministic() {
    let mut first = SeededRng::new(42);
    let mut second = SeededRng::new(42);

    assert_eq!(
        cpf::generate_with_rng(&mut first),
        cpf::generate_with_rng(&mut second)
    );
    assert_eq!(
        cnpj::generate_with_rng(&mut first, cnpj::CnpjKind::Alphanumeric),
        cnpj::generate_with_rng(&mut second, cnpj::CnpjKind::Alphanumeric)
    );
    assert_eq!(
        cep::generate_cep_with_rng(&mut first),
        cep::generate_cep_with_rng(&mut second)
    );
    assert_eq!(
        rg::generate_with_rng(&mut first, uf::State::SP),
        rg::generate_with_rng(&mut second, uf::State::SP)
    );
}

#[test]
fn generators_accept_an_application_random_source() {
    let mut first = TestRng(7);
    let mut second = first.clone();

    let cpf = cpf::generate_for_region_with_rng(&mut first, cpf::FiscalRegion::Sp);
    assert_eq!(
        cpf,
        cpf::generate_for_region_with_rng(&mut second, cpf::FiscalRegion::Sp)
    );
    assert_eq!(cpf.fiscal_region(), cpf::FiscalRegion::Sp);

    let cep = cep::generate_for_state_with_rng(&mut first, uf::State::GO);
    assert_eq!(
        cep,
        cep::generate_for_state_with_rng(&mut second, uf::State::GO)
    );
    assert_eq!(cep.state(), Some(uf::State::GO));
}

#[test]
fn public_errors_implement_core_error() {
    fn assert_error<E: core::error::Error>() {}

    assert_error::<cpf::CpfError>();
    assert_error::<cnpj::CnpjError>();
    assert_error::<cep::CepError>();
    assert_error::<rg::RgError>();
}

#[test]
fn explicit_lenient_names_preserve_compatibility_aliases() {
    assert_eq!(cpf::normalize("529.982.247-25"), "52998224725");
    assert_eq!(
        cpf::is_valid_lenient("529$982#247!25"),
        cpf::is_valid("529$982#247!25")
    );

    assert_eq!(cnpj::normalize("11.222.333/0001-81"), "11222333000181");
    assert_eq!(
        cnpj::is_valid_lenient("11$222#333!0001@81"),
        cnpj::is_valid("11$222#333!0001@81")
    );

    assert_eq!(cep::normalize("01310-100"), "01310100");
    assert_eq!(
        cep::is_valid_lenient("01310$100"),
        cep::is_valid("01310$100")
    );

    assert_eq!(rg::normalize("29.465.327-2", uf::State::SP), "294653272");
    assert_eq!(
        rg::is_valid_lenient("29$465#327!2", uf::State::SP),
        rg::is_valid("29$465#327!2", uf::State::SP)
    );
}
