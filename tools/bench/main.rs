use std::hint::black_box;
use std::time::Instant;
use stdbr_core::{cnpj, cpf, municipio};

fn argument(index: usize, default: usize) -> usize {
    std::env::args()
        .nth(index)
        .map(|value| {
            value
                .parse()
                .expect("benchmark arguments must be positive integers")
        })
        .unwrap_or(default)
}

fn main() {
    let validation_items = argument(1, 100_000);
    let search_iterations = argument(2, 1_000);

    let cpfs: Vec<_> = (0..validation_items)
        .map(|seed| cpf::generate_with_seed(seed as u64).as_str().to_owned())
        .collect();
    let cnpjs: Vec<_> = (0..validation_items)
        .map(|seed| {
            cnpj::generate_with_seed(seed as u64, cnpj::CnpjKind::Alphanumeric)
                .as_str()
                .to_owned()
        })
        .collect();

    let started = Instant::now();
    let valid = cpfs
        .iter()
        .filter(|value| cpf::is_valid(black_box(value)))
        .count()
        + cnpjs
            .iter()
            .filter(|value| cnpj::is_valid(black_box(value)))
            .count();
    let elapsed = started.elapsed();
    black_box(valid);
    println!(
        "batch validation: {} items in {:?} ({:.0} items/s)",
        validation_items * 2,
        elapsed,
        validation_items as f64 * 2.0 / elapsed.as_secs_f64()
    );

    let queries = ["sao", "porto", "santa", "rio", "belo", "ac"];
    let started = Instant::now();
    let mut matches = 0usize;
    for index in 0..search_iterations {
        matches +=
            municipio::Municipio::search_by_name(black_box(queries[index % queries.len()])).count();
    }
    let elapsed = started.elapsed();
    black_box(matches);
    println!(
        "municipal search: {search_iterations} queries in {:?} ({:.0} queries/s)",
        elapsed,
        search_iterations as f64 / elapsed.as_secs_f64()
    );
}
