use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use clap::Parser;
use synthestitch;
use shlex;

#[pyfunction]
fn ping() -> PyResult<&'static str> { Ok("ready") }

#[pyfunction]
fn run_top_down_cli(args: String, py: Python<'_>) -> PyResult<String> {
    let toks = shlex::split(&args).ok_or_else(|| pyo3::exceptions::PyValueError::new_err("failed to parse args"))?;
    let mut argv = vec!["top_down_py".to_string()];
    argv.extend(toks);
    let cfg = synthestitch::Args::try_parse_from(&argv)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let json = py.allow_threads(|| synthestitch::dispatch_domain(&cfg));
    Ok(json)
}

#[pymodule]
fn stitch_py(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_function(wrap_pyfunction!(run_top_down_cli, m)?)?;
    Ok(())
}
