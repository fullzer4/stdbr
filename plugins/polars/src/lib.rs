use pyo3_polars::PolarsAllocator;

mod expressions;

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();
