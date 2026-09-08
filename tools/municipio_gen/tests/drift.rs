use municipio_gen_lib::{has_drift, parse_and_normalize, render_source};

#[test]
fn checked_in_municipality_data_has_no_drift() {
    let source = include_str!("../ibge_municipios.json");
    let target = include_str!("../../../core/src/municipio/generated.rs");
    let normalized = parse_and_normalize(source).expect("snapshot must be valid");
    assert_eq!(
        render_source(&normalized).unwrap(),
        source,
        "IBGE snapshot is not canonical"
    );
    assert!(
        !has_drift(source, target).expect("snapshot and generated Rust must be valid"),
        "municipality data has drift; run `bazel run //tools/municipio_gen -- generate`"
    );
}

#[test]
fn api_fixture_is_normalized_and_sorted_deterministically() {
    let fixture = include_str!("ibge_api_fixture.json");
    let normalized = parse_and_normalize(fixture).expect("fixture must match the IBGE API schema");
    assert_eq!(normalized.len(), 2);
    assert_eq!(normalized[0].id, 1_200_013);
    assert_eq!(normalized[0].uf, "AC");
    assert_eq!(normalized[1].id, 3_550_308);
    assert_eq!(normalized[1].uf, "SP");

    let expected = concat!(
        "[\n",
        "  {\n",
        "    \"id\": 1200013,\n",
        "    \"nome\": \"Acrelândia\",\n",
        "    \"uf\": \"AC\"\n",
        "  },\n",
        "  {\n",
        "    \"id\": 3550308,\n",
        "    \"nome\": \"São Paulo\",\n",
        "    \"uf\": \"SP\"\n",
        "  }\n",
        "]\n",
    );
    assert_eq!(render_source(&normalized).unwrap(), expected);
}
