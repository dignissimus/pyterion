use criterion::Criterion;
use pyo3::prelude::*;

#[pyfunction(name = "benchmark")]
fn pyterion_decorator(function: &PyAny) -> PyResult<()> {
    let mut benchmark = Criterion::default();
    benchmark.bench_function("Pyterion Benchmark", |bench| {
        bench.iter(|| function.call0().expect("Error"))
    });
    Ok(())
}

#[pymodule]
fn pyterion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyterion_decorator, m)?)?;
    Ok(())
}
