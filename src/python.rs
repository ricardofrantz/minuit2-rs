#![cfg(feature = "python")]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::collections::{HashMap, HashSet};

use crate::{
    FCN, FunctionMinimum, MnContours, MnHesse, MnMigrad, MnMinos, MnScan, MnSimplex,
};

// ============================================================================
// FCN Wrapper
// ============================================================================

struct PythonFCN {
    fcn: PyObject,
}

impl FCN for PythonFCN {
    fn value(&self, par: &[f64]) -> f64 {
        // Acquire GIL
        Python::with_gil(|py| {
            // Convert parameters to a Python tuple
            let args = PyTuple::new(py, par);
            
            // Call the Python callable
            match self.fcn.call(py, args, None) {
                Ok(val) => {
                    // Extract f64 result
                    if let Ok(f) = val.extract::<f64>(py) {
                        f
                    } else {
                        // If return value is not float (e.g. None), return infinity
                        f64::INFINITY
                    }
                }
                Err(e) => {
                    // If Python function raises exception, print it and return infinity
                    // to avoid crashing the Rust process.
                    e.print(py);
                    f64::INFINITY
                }
            }
        })
    }
}

// ============================================================================
// Minuit Class
// ============================================================================

#[pyclass(name = "Minuit")]
struct Minuit {
    fcn: PyObject,
    names: Vec<String>,
    values: HashMap<String, f64>,
    errors: HashMap<String, f64>,
    limits: HashMap<String, (f64, f64)>, // (lower, upper)
    fixed: HashSet<String>,
    
    // Internal state
    last_minimum: Option<FunctionMinimum>,
    strategy: u32,
    tolerance: f64,
    max_calls: Option<usize>,
}

#[pymethods]
impl Minuit {
    #[new]
    #[pyo3(signature = (fcn, **params))]
    fn new(fcn: PyObject, params: Option<&PyDict>) -> PyResult<Self> {
        let mut names = Vec::new();
        let mut values = HashMap::new();
        let mut errors = HashMap::new();
        let limits = HashMap::new();
        let fixed = HashSet::new();

        if let Some(p) = params {
            for (name, value) in p {
                let name_str = name.extract::<String>()?;
                let val = value.extract::<f64>()?;
                names.push(name_str.clone());
                values.insert(name_str.clone(), val);
                // Default error: 0.1
                errors.insert(name_str, 0.1);
            }
        }

        Ok(Minuit {
            fcn,
            names,
            values,
            errors,
            limits,
            fixed,
            last_minimum: None,
            strategy: 1,
            tolerance: 0.1,
            max_calls: None,
        })
    }

    // --- Properties ---

    #[getter]
    fn get_values(&self) -> HashMap<String, f64> {
        self.values.clone()
    }

    #[setter]
    fn set_values(&mut self, values: HashMap<String, f64>) {
        for (k, v) in values {
            if self.values.contains_key(&k) {
                self.values.insert(k, v);
            }
        }
    }

    #[getter]
    fn get_errors(&self) -> HashMap<String, f64> {
        self.errors.clone()
    }

    #[setter]
    fn set_errors(&mut self, errors: HashMap<String, f64>) {
        for (k, v) in errors {
            if self.errors.contains_key(&k) {
                self.errors.insert(k, v);
            }
        }
    }

    #[getter]
    fn get_limits(&self) -> HashMap<String, (Option<f64>, Option<f64>)> {
        let mut res = HashMap::new();
        for name in &self.names {
            if let Some((l, u)) = self.limits.get(name) {
                res.insert(name.clone(), (Some(*l), Some(*u)));
            } else {
                res.insert(name.clone(), (None, None));
            }
        }
        res
    }

    #[setter]
    fn set_limits(&mut self, limits: &PyDict) -> PyResult<()> {
        for (key, value) in limits {
            let name = key.extract::<String>()?;
            if !self.names.contains(&name) {
                continue;
            }

            if value.is_none() {
                self.limits.remove(&name);
            } else {
                if let Ok(tuple) = value.downcast::<PyTuple>() {
                    if tuple.len() == 2 {
                        let l = tuple.get_item(0)?.extract::<Option<f64>>()?;
                        let u = tuple.get_item(1)?.extract::<Option<f64>>()?;
                        
                        match (l, u) {
                            (Some(low), Some(up)) => { self.limits.insert(name, (low, up)); },
                            _ => { self.limits.remove(&name); }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[getter]
    fn get_fixed(&self) -> Vec<String> {
        self.fixed.iter().cloned().collect()
    }

    #[setter]
    fn set_fixed(&mut self, fixed: Vec<String>) {
        self.fixed.clear();
        for name in fixed {
            if self.names.contains(&name) {
                self.fixed.insert(name);
            }
        }
    }
    
    #[getter]
    fn get_fval(&self) -> Option<f64> {
        self.last_minimum.as_ref().map(|m| m.fval())
    }

    #[getter]
    fn get_valid(&self) -> Option<bool> {
        self.last_minimum.as_ref().map(|m| m.is_valid())
    }
    
    #[getter]
    fn get_covariance(&self, py: Python) -> PyResult<Option<PyObject>> {
        if let Some(min) = &self.last_minimum {
            if let Some(cov) = min.user_state().covariance() {
                let n = cov.nrow();
                let mut matrix = Vec::with_capacity(n);
                for r in 0..n {
                    let mut row = Vec::with_capacity(n);
                    for c in 0..n {
                        row.push(cov.get(r, c));
                    }
                    matrix.push(row);
                }
                return Ok(Some(matrix.into_py(py)));
            }
        }
        Ok(None)
    }

    #[getter]
    fn get_global_cc(&self) -> Option<Vec<f64>> {
        self.last_minimum.as_ref()
            .and_then(|m| m.user_state().global_cc())
            .map(|s| s.to_vec())
    }

    // --- Minimizers ---

    fn migrad(&mut self) -> PyResult<()> {
        Python::with_gil(|py| {
            let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
            let mut minimizer = MnMigrad::new()
                .with_strategy(self.strategy)
                .tolerance(self.tolerance);

            if let Some(max) = self.max_calls {
                minimizer = minimizer.max_fcn(max);
            }
            
            for name in &self.names {
                let val = *self.values.get(name).unwrap_or(&0.0);
                let err = *self.errors.get(name).unwrap_or(&0.1);
                
                if self.fixed.contains(name) {
                    minimizer = minimizer.add_const(name, val);
                } else if let Some((l, u)) = self.limits.get(name) {
                    minimizer = minimizer.add_limited(name, val, err, *l, *u);
                } else {
                    minimizer = minimizer.add(name, val, err);
                }
            }

            let result = minimizer.minimize(&fcn);
            self.update_state_from_result(&result);
            self.last_minimum = Some(result);

            Ok(())
        })
    }

    fn simplex(&mut self) -> PyResult<()> {
        Python::with_gil(|py| {
            let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
            let mut minimizer = MnSimplex::new()
                .with_strategy(self.strategy)
                .tolerance(self.tolerance);
                
            if let Some(max) = self.max_calls {
                minimizer = minimizer.max_fcn(max);
            }

            for name in &self.names {
                let val = *self.values.get(name).unwrap_or(&0.0);
                let err = *self.errors.get(name).unwrap_or(&0.1);
                
                if self.fixed.contains(name) {
                    minimizer = minimizer.add_const(name, val);
                } else if let Some((l, u)) = self.limits.get(name) {
                    minimizer = minimizer.add_limited(name, val, err, *l, *u);
                } else {
                    minimizer = minimizer.add(name, val, err);
                }
            }

            let result = minimizer.minimize(&fcn);
            self.update_state_from_result(&result);
            self.last_minimum = Some(result);

            Ok(())
        })
    }

    fn hesse(&mut self) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(min) = &self.last_minimum {
                let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
                let mut hesse = MnHesse::new()
                    .with_strategy(self.strategy);
                
                if let Some(max) = self.max_calls {
                    hesse = hesse.with_max_calls(max);
                }

                let result = hesse.calculate(&fcn, min);
                self.update_state_from_result(&result);
                self.last_minimum = Some(result);
                
                Ok(())
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err("Run migrad/simplex first"))
            }
        })
    }
    
    fn minos(&mut self, py: Python) -> PyResult<PyObject> {
        if let Some(min) = &self.last_minimum {
            let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
            let mut minos = MnMinos::new(&fcn, min)
                .with_strategy(self.strategy)
                .with_tolerance(self.tolerance);
                
            if let Some(max) = self.max_calls {
                minos = minos.with_max_calls(max);
            }

            let mut results = HashMap::new();
            
            for (i, name) in self.names.iter().enumerate() {
                if !self.fixed.contains(name) {
                    let err = minos.minos_error(i);
                    let dict = PyDict::new(py);
                    dict.set_item("lower", err.lower_error())?;
                    dict.set_item("upper", err.upper_error())?;
                    dict.set_item("is_valid", err.is_valid())?;
                    dict.set_item("lower_valid", err.lower_valid())?;
                    dict.set_item("upper_valid", err.upper_valid())?;
                    results.insert(name.clone(), dict);
                }
            }
            
            Ok(results.into_py(py))
        } else {
            Err(pyo3::exceptions::PyRuntimeError::new_err("Run migrad/simplex first"))
        }
    }

    fn scan(&self, param: String, nsteps: usize, low: f64, high: f64) -> PyResult<Vec<(f64, f64)>> {
        Python::with_gil(|py| {
            if let Some(min) = &self.last_minimum {
                let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
                let scan = MnScan::new(&fcn, min);
                
                if let Some(idx) = self.names.iter().position(|n| *n == param) {
                    Ok(scan.scan(idx, nsteps, low, high))
                } else {
                    Err(pyo3::exceptions::PyValueError::new_err("Parameter not found"))
                }
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err("Run migrad/simplex first"))
            }
        })
    }

    fn contour(&self, par_x: String, par_y: String, npoints: usize) -> PyResult<Vec<(f64, f64)>> {
        Python::with_gil(|py| {
            if let Some(min) = &self.last_minimum {
                let fcn = PythonFCN { fcn: self.fcn.clone_ref(py) };
                let contours = MnContours::new(&fcn, min)
                    .with_strategy(self.strategy);
                
                let idx_x = self.names.iter().position(|n| *n == par_x)
                    .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Parameter X not found"))?;
                let idx_y = self.names.iter().position(|n| *n == par_y)
                    .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Parameter Y not found"))?;
                    
                Ok(contours.points(idx_x, idx_y, npoints))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err("Run migrad/simplex first"))
            }
        })
    }
}

impl Minuit {
    fn update_state_from_result(&mut self, result: &FunctionMinimum) {
        let user_state = result.user_state();
        let params = user_state.params();
        
        // Update values from result
        for i in 0..params.len() {
            let p = params.trafo().parameter(i);
            if let Some(name) = self.names.get(i) {
                self.values.insert(name.clone(), p.value());
                self.errors.insert(name.clone(), p.error());
            }
        }
    }
}

// ============================================================================
// PyModule: _minuit2
// ============================================================================

#[pymodule]
fn _minuit2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Minuit>()?;
    m.add("__version__", "0.3.0")?;
    
    Ok(())
}
