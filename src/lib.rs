use std::path::PathBuf;

use criterion::{Bencher, Criterion};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyCFunction, PyDict, PyTuple};

#[derive(Clone)]
struct PyterionConfig {
    benchmark_name: String,
    output_directory: String,
    significance_level: f64,
    confidence_level: f64,
}

impl Default for PyterionConfig {
    fn default() -> PyterionConfig {
        PyterionConfig {
            benchmark_name: "Pyterion Benchmark".to_string(),
            output_directory: "pyterion-output".to_string(),
            significance_level: 0.05,
            confidence_level: 0.95,
        }
    }
}

impl PyterionConfig {
    fn _options(
        mut self,
        name: Option<String>,
        directory: Option<String>,
        significance_level: Option<f64>,
        confidence_level: Option<f64>,
    ) -> PyterionConfig {
        if let Some(name) = name {
            self.benchmark_name = name;
        }
        if let Some(directory) = directory {
            self.output_directory = directory;
        }
        if let Some(significance_level) = significance_level {
            self.significance_level = significance_level;
        }
        if let Some(confidence_level) = confidence_level {
            self.confidence_level = confidence_level;
        }

        self
    }
}

fn run_benchmark(function: &PyAny) -> impl FnMut(&mut Bencher) + '_ {
    |bench: &mut Bencher| {
        bench.iter(|| {
            function
                .call0()
                .expect("An error occured while executing the function to be benchmarked.")
        });
    }
}

fn perform_benchmark(function: &PyAny, config: &PyterionConfig) {
    let mut benchmark = Criterion::default()
        .output_directory(&PathBuf::from(config.output_directory.to_string()))
        .confidence_level(config.confidence_level)
        .significance_level(config.significance_level);

    benchmark.bench_function(&config.benchmark_name, run_benchmark(function));
}

#[allow(unused_must_use)]
fn _inner_decorator() -> PyResult<Py<PyAny>> {
    let closure = |args: &PyTuple, _kwargs: Option<&PyDict>| {
        Python::with_gil(|py| -> PyResult<()> {
            let function = args.extract::<(PyObject,)>()?.0;
            let iconfig = PyterionConfig::default(); // &config.clone()
            perform_benchmark(function.as_ref(py), &iconfig);
            Ok(())
        });
    };

    Python::with_gil(|py| PyCFunction::new_closure(closure, py).map(|function| function.into()))
}

#[pyfunction(name = "benchmark")]
fn pyterion_decorator(
    function: Option<&PyAny>,
    name: Option<String>,
    directory: Option<String>,
    significance_level: Option<f64>,
    confidence_level: Option<f64>,
) -> PyResult<Option<&PyAny>> {
    if name.is_some()
        || directory.is_some()
        || significance_level.is_some()
        || confidence_level.is_some()
    {
        Ok(None)
    } else if let Some(function) = function {
        if function.is_callable() {
            perform_benchmark(function, &PyterionConfig::default());
            Ok(None)
        } else {
            Err(PyTypeError::new_err(""))
        }
    } else {
        Err(PyTypeError::new_err(""))
    }
}

#[pymodule]
fn pyterion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyterion_decorator, m)?)?;
    Ok(())
}
