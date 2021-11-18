// use pyo3::types::PyDict;
// mod submodule;
// use submodule::*;
// mod ahoc;
// use ahoc::*;
// use std::time::Duration;
// use tokio;


use pyo3::{prelude::*};
// , wrap_pyfunction, wrap_pymodule};


static mut COUNTER: usize = 0;


#[pyfunction]
#[no_mangle]
pub unsafe extern "C" fn add_to_count(inc: usize) -> usize {

    COUNTER += inc;
    COUNTER

}


#[pyclass]
struct UService {
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl UService {
    #[new]
    pub fn new(value: i32) -> Self {
        UService { value }
    }

    pub fn increment(&mut self) -> PyResult<()> {
        self.value+=1;
        Ok(())
    }

    pub fn greetme(&self) -> &'static str {
        "Hello, world!"
    }

    pub fn personalgreet(&self, name: &str)-> PyResult<String> {
        Ok(format!("Hello there {}", name))
    }
}


#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UService>()?;
    m.add_function(wrap_pyfunction!(add_to_count, m)?)?;

    Ok(())
}
