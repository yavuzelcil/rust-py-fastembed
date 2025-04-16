use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;

#[pyfunction]
fn process_int_array(input: Vec<i32>) -> PyResult<Vec<i32>> {
    let mut result = Vec::new();
    for &num in &input {
        if num < 0 {
            return Err(PyValueError::new_err("Negative number found"));
        }
        result.push(num * 2);
    }
    Ok(result)
}

#[pyfunction]
fn process_string_array(input: Vec<String>) -> PyResult<Vec<String>> {
    let mut result = Vec::new();
    for s in &input {
        if s.is_empty() {
            return Err(PyValueError::new_err("Empty string found"));
        }
        result.push(s.to_owned() + " - manupilated");
    }
    Ok(result)
}

#[pyfunction]
fn process_dict(input: &PyDict) -> PyResult<HashMap<i32, String>> {
    let mut result: HashMap<i32, String> = HashMap::new();
    for (key, value) in input.iter() {
        let key: i32 = key.extract()?;
        let value: String = value.extract()?;
        if key < 0 {
            return Err(PyValueError::new_err("Negative key found"));
        }
        result.insert(key + 1, format!("{} - processed", value));
    }
    Ok(result)
}

#[pymodule]
fn rust_py_fastembed(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_int_array, m)?)?;
    m.add_function(wrap_pyfunction!(process_string_array, m)?)?;
    m.add_function(wrap_pyfunction!(process_dict, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::IntoPyDict;

    #[test]
    fn test_process_int_array() {
        Python::with_gil(|py| {
            let result: Vec<i32> = process_int_array(vec![1, 2, 3]).unwrap();
            assert_eq!(result, vec![2, 4, 6]);
        });
    }

    #[test]
    fn test_process_string_array() {
        Python::with_gil(|py| {
            let result: Vec<String> = process_string_array(vec!["hello".to_string(), "world".to_string()]).unwrap();
            assert_eq!(result, vec!["hello - manupilated", "world - manupilated"]);
        });
    }

    #[test]
    fn test_process_dict() {
        Python::with_gil(|py| {
            let dict = [("1", "one"), ("2", "two")].into_py_dict(py);
            let result: HashMap<i32, String> = process_dict(dict).unwrap();
            assert_eq!(result.get(&2), Some(&"one - processed".to_string()));
            assert_eq!(result.get(&3), Some(&"two - processed".to_string()));
        });
    }
}
