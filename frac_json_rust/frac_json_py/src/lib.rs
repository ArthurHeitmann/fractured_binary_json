use std::borrow::Cow;

use pyo3::{
    create_exception,
    prelude::*,
    types::{PyDict, PyFloat, PyList, PyString},
};
use serde_json::{Map, Number, Value};

use ::frac_json as fj;

create_exception!(frac_json, FracJsonError, pyo3::exceptions::PyException);

#[pyfunction]
pub fn encode(
    py: Python,
    object: PyObject,
    global_keys_table_bytes: Option<Vec<u8>>,
    compression_level: Option<i32>,
    zstd_dict: Option<Vec<u8>>,
) -> PyResult<Cow<[u8]>> {
    let value = py_to_json(py, &object).map_err(|err| FracJsonError::new_err(err))?;
    fj::encode(&value, global_keys_table_bytes.as_ref(), compression_level, zstd_dict.as_ref())
        .map(|vec| Cow::from(vec))
        .map_err(|err| FracJsonError::new_err(err))
}

#[pyfunction]
pub fn decode(
    py: Python,
    frac_json_bytes: Vec<u8>,
    global_keys_table_bytes: Option<Vec<u8>>,
    zstd_dict: Option<Vec<u8>>,
) -> PyResult<PyObject> {
    let value = fj::decode(frac_json_bytes.as_ref(), global_keys_table_bytes.as_ref(), zstd_dict.as_ref())
        .map_err(|err| FracJsonError::new_err(err))?;
    Ok(json_to_py(py, &value).map_err(|err| FracJsonError::new_err(err))?)
}

#[pyfunction]
pub fn keys_table_from_keys(_py: Python, keys: Vec<String>) -> PyResult<Cow<[u8]>> {
    fj::global_table_from_keys(keys)
        .map(|vec| Cow::from(vec))
        .map_err(|err| FracJsonError::new_err(err))
}

#[pyfunction]
pub fn keys_table_from_json(
    py: Python,
    object: PyObject,
    max_count: Option<i64>,
    occurrence_cutoff: Option<i64>,
) -> PyResult<Cow<[u8]>> {
    let value = py_to_json(py, &object).map_err(|err| FracJsonError::new_err(err))?;
    fj::global_table_from_json_limited(
        &value,
        max_count.map(|v| v as usize),
        occurrence_cutoff.map(|v| v as usize),
    )
    .map(|vec| Cow::from(vec))
    .map_err(|err| FracJsonError::new_err(err))
}

#[pymodule]
fn frac_json(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(keys_table_from_keys, m)?)?;
    m.add_function(wrap_pyfunction!(keys_table_from_json, m)?)?;
    Ok(())
}

fn py_to_json(py: Python, obj: &PyObject) -> Result<serde_json::Value, String> {
    macro_rules! return_cast {
        ($t:ty, $f:expr) => {
            if let Ok(val) = obj.downcast::<$t>(py) {
                return $f(val);
            }
        };
    }

    macro_rules! return_to_value {
        ($t:ty) => {
            if let Ok(val) = obj.extract::<$t>(py) {
                return serde_json::value::to_value(val).map_err(|error| format!("{}", error));
            }
        };
    }

    // primitives
    if obj.is_none(py) {
        return Ok(Value::Null);
    }
    return_to_value!(String);
    return_to_value!(bool);
    return_to_value!(u64);
    return_to_value!(i64);
    return_cast!(PyFloat, |x: &PyFloat| {
        match Number::from_f64(x.value()) {
            Some(n) => Ok(Value::Number(n)),
            None => Err("Failed to convert float to JSON number".to_string()),
        }
    });

    // dict
    return_cast!(PyDict, |x: &PyDict| {
        let mut map = Map::new();
        for (key_obj, value) in x.iter() {
            let key = if let Ok(val) = key_obj.downcast::<PyString>() {
                Ok(val.to_str().map_err(|e| e.to_string())?)
            } else {
                match key_obj.str() {
                    Ok(val) => Err(format!(
                        "Failed to convert key {} to string",
                        val.to_string()
                    )),
                    Err(_) => Err("Failed to convert key to string".to_string()),
                }
            };
            map.insert(key?.to_string(), py_to_json(py, &value.to_object(py))?);
        }
        Ok(Value::Object(map))
    });

    // list
    return_cast!(PyList, |x: &PyList| Ok(Value::Array(
        x.iter()
            .map(|x| py_to_json(py, &x.to_object(py)))
            .collect::<Result<Vec<_>, _>>()?
    )));

    // At this point we can't cast it, set up the error object
    Err(obj
        .to_object(py)
        .as_ref(py)
        .get_type()
        .name()
        .and_then(|name| Ok(format!("Failed to convert {} to JSON", name)))
        .unwrap_or("Failed to convert object to JSON".to_string()))
}

fn json_to_py(py: Python, value: &serde_json::Value) -> Result<PyObject, String> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(n) = n.as_u64() {
                Ok(n.to_object(py))
            } else if let Some(n) = n.as_i64() {
                Ok(n.to_object(py))
            } else if let Some(n) = n.as_f64() {
                Ok(n.to_object(py))
            } else {
                Err("Failed to convert JSON number to Python number".to_string())
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(a) => {
            let list = PyList::empty(py);
            for item in a {
                list.append(json_to_py(py, item)?)
                    .map_err(|err| err.to_string())?;
            }
            Ok(list.to_object(py))
        }
        Value::Object(o) => {
            let dict = PyDict::new(py);
            for (key, value) in o {
                dict.set_item(key, json_to_py(py, value)?)
                    .map_err(|err| err.to_string())?;
            }
            Ok(dict.to_object(py))
        }
    }
}
