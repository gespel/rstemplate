use pyo3::prelude::*;

#[pyfunction]
fn test(a: i64) -> PyResult<i64> {
    Ok(a + 1)
}

#[pymodule]
fn rstemplate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
