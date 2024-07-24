use pyo3::prelude::*;
use pyo3::types::{PyModule, PyAny};

pub fn calculated_residuals(py: Python, df: Py<PyAny>, forecast: Py<PyAny>) -> PyResult<Py<PyAny>> {
    let code = r#"
import numpy as np

def calculated_residuals(df, forecast):
    df_merged = df.merge(forecast[['ds', 'yhat']],on='ds')
    df_merged['residual'] = df_merged['y'] - df_merged['yhat']
    return df_merged
    "#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;

    let res_function = res_module.getattr("calculated_residuals")?;

    let result = res_function.call1((df, forecast))?;

    let residuals: Py<PyAny> = result.extract()?;

    Ok(residuals)
}

pub fn calculate_z_scores(py: Python, residuals: Py<PyAny>) -> PyResult<Py<PyAny>> {
    let code = r#"
import numpy as np

def calculate_z_scores(residuals):
    mean_residual = np.mean(residuals)
    std_residual = np.std(residuals)
    z_scores = (residuals - mean_residual) / std_residual
    return z_scores
    "#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;
    let res_function = res_module.getattr("calculate_z_scores")?;
    let result = res_function.call1((residuals,))?;
    
    let z_scores: Py<PyAny> = result.extract()?;
    Ok(z_scores)
}

pub fn detect_anomalies(py: Python, z_scores: Py<PyAny>, threshold: f64) -> PyResult<Py<PyAny>> {
    let code = r#"
import numpy as np

def detect_anomalies(z_scores, threshold=3):
    return np.where(np.abs(z_scores) > threshold)[0]
    "#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;
    let res_function = res_module.getattr("detect_anomalies")?;
    let result = res_function.call1((z_scores, threshold))?;
    
    let anomalies: Py<PyAny> = result.extract()?;
    Ok(anomalies)
}