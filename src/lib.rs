use pyo3::prelude::*;

#[pyclass]
pub struct Tester {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub age: i64
}
#[pymethods]
impl Tester {
    #[new]
    pub fn new(data: String, age: i64) -> Tester {
        Tester {
            name: data,
            age: age,
        }
    }
    pub fn printer(&self) {
        println!("{}", &self.name);
        println!("{}", &self.age);
    }
}


#[pyfunction]
fn test(a: i64) -> PyResult<i64> {
    Ok(a + 1)
}

#[pymodule]
fn rstemplate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<Tester>()?;
    Ok(())
}
