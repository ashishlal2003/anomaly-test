use pyo3::prelude::*;
use pyo3::types::{PyModule, PyAny};

pub fn load_and_preprocess_data(py: Python, file_path: &str, status_code: i32, freq: &str) -> PyResult<Py<PyAny>> {

    let code = r#"
import pandas as pd

def load_and_preprocess_data(filepath, status_code=404, freq='2s'):
    df = pd.read_csv(filepath, parse_dates=['datetime'])

    df_filtered = df[df['status'] == status_code]
    df_filtered['datetime'] = df_filtered['datetime'].dt.tz_localize(None)
    df_filtered = df_filtered.set_index('datetime').resample(freq).size().reset_index(name='y')
    df_filtered.rename(columns={'datetime': 'ds'}, inplace=True)
    print(df_filtered)
    return df_filtered
"#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;

    let res_function = res_module.getattr("load_and_preprocess_data")?;

    let result= res_function.call1((file_path, status_code, freq))?;

    let df: Py<PyAny> = result.extract()?;

    Ok(df)
}
