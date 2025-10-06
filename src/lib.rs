// use lambdas::python_bridge::set_current_symbol;
// use pyo3::prelude::*;
// use pyo3::types::{PyAny, PyDict, PyTuple};
// use std::collections::HashMap;
// use pyo3::wrap_pyfunction;
// use std::sync::Mutex;
// use once_cell::sync::OnceCell;
// use clap::Parser;
// use synthestitch;
// use shlex;

// #[pyfunction]
// fn ping() -> PyResult<&'static str> { Ok("ready") }

// #[pyfunction]
// fn register_callable(symbol: String, func: Bound<PyAny>) -> PyResult<()> {
//     lambdas::python_bridge::register_callable(symbol, &func)
// }

// #[pyfunction]
// fn register_primitive(symbol: String, tp: String, registry_name: String) -> PyResult<()> {
//     set_current_symbol(&symbol);
//     lambdas::python_bridge::register_primitive(symbol, tp, registry_name)
// }


// #[pyfunction]
// fn run_top_down_cli(args: String, py: Python<'_>) -> PyResult<String> {
//     let toks = shlex::split(&args).ok_or_else(|| pyo3::exceptions::PyValueError::new_err("failed to parse args"))?;
//     let mut argv = vec!["top_down_py".to_string()];
//     argv.extend(toks);
//     let cfg = synthestitch::Args::try_parse_from(&argv)
//         .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

//     let json = py.allow_threads(|| synthestitch::dispatch_domain(&cfg));
//     Ok(json)
// }


// stitch_py/src/lib.rs

// use lambdas::python_bridge;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use std::collections::HashMap;
use pyo3::wrap_pyfunction;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use clap::Parser;
use synthestitch;
use shlex;
use lambdas::{DSL, Symbol, SlowType};
use lambdas::domains::simple::SimpleVal;
use lambdas::Domain;


// This is a handle, it owns its own registry of primitives
#[pyclass]
pub struct StitchHandle {
    dsl: DSL<SimpleVal>,   // per-instance DSL (no globals)
}

#[pymethods]
impl StitchHandle {
        #[new]
        fn new() -> Self {
            Self { dsl: SimpleVal::new_dsl() }   // build the native DSL for Simple domain
        }

        /// Early-capture registration:
        /// Python: h.register("double", "(int)->int", py_callable)
        fn register(&mut self, symbol: &str, tp_str: &str, func: PyObject) -> PyResult<()> {
            // Parse the type string to SlowType (same parser you use in DSL constructors)
            let tp: SlowType = tp_str.parse()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("type parse error: {e}")))?;

            // Own the callable across GIL boundaries
            let owned: Py<PyAny> = func; //Python::with_gil(|py| func.into_py(py));

            // Install directly into the DSL (early-capture)
            self.dsl.add_python_primitive(Symbol::from(symbol), tp, None, owned);
            Ok(())
        }

        // (Optional) Keep a getter if you want to pass &self.dsl to a runner later
        // #[allow(dead_code)]
        // pub fn _dsl_ptr(&self) -> *const DSL<SimpleVal> { &self.dsl }

        fn run_cli(&self, args: String, py: Python<'_>) -> PyResult<String> {
        let toks = shlex::split(&args)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("failed to parse args"))?;
        let mut argv = vec!["top_down_py".to_string()];
        argv.extend(toks);
        let cfg = synthestitch::Args::try_parse_from(&argv)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let json = py.allow_threads(|| synthestitch::dispatch_domain_simple(&cfg, &self.dsl));
        Ok(json)
        }
    }

// #[pyfunction]
// fn run_top_down_cli(args: String, py: Python<'_>) -> PyResult<String> {
//     let toks = shlex::split(&args).ok_or_else(|| pyo3::exceptions::PyValueError::new_err("failed to parse args"))?;
//     let mut argv = vec!["top_down_py".to_string()];
//     argv.extend(toks);
//     let cfg = synthestitch::Args::try_parse_from(&argv)
//         .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

//     let json = py.allow_threads(|| synthestitch::dispatch_domain(&cfg));
//     Ok(json)
// }

#[pymodule]
fn stitch_py(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(ping, m)?)?;
    // m.add_function(wrap_pyfunction!(register_callable, m)?)?;
    // m.add_function(wrap_pyfunction!(register_primitive, m)?)?;
    m.add_class::<StitchHandle>()?;
    //m.add_function(wrap_pyfunction!(run_top_down_cli, m)?)?;
    //m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}
