#[cfg(feature = "municipio")]
#[test]
fn default_features_expose_municipio_dataset() {
    assert!(!stdbr_core::municipio::ALL.is_empty());
}

#[cfg(not(feature = "municipio"))]
#[test]
fn core_apis_compile_without_municipio_dataset() {
    assert_eq!(stdbr_core::uf::ALL.len(), 27);
}
