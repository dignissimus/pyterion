use criterion::Criterion;
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyfunction(name = "benchmark")]
fn pyterion_decorator(function: &PyAny) -> PyResult<()> {
    /* The following will become parameters for the decorator */
    let benchmark_name = "Pyterion Benchmark";
    let output_directory = "pyterion";
    let significance_level = 0.05;
    let confidence_level = 0.95;

    let mut benchmark = Criterion::default()
        .output_directory(&PathBuf::from(output_directory))
        .confidence_level(confidence_level)
        .significance_level(significance_level);

    benchmark.bench_function(benchmark_name, |bench| {
        bench.iter(|| function.call0().expect("Error"))
    });

    Ok(())
}

#[pymodule]
fn pyterion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyterion_decorator, m)?)?;
    Ok(())
}
