use pyo3::prelude::*;
use pyo3::types::{PyModule, PyAny};

pub fn fit_prophet_model(py: Python, df: Py<PyAny>) -> PyResult<Py<PyAny>>{
    let code = r#"
from prophet import Prophet

def fit_prophet_model(df):
    model = Prophet()
    model.fit(df)
    return model
    "#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;

    let res_function = res_module.getattr("fit_prophet_model")?;

    let result = res_function.call1((df,))?;

    let model: Py<PyAny> = result.extract()?;

    Ok(model)
}

pub fn forecast_prophet_model(py: Python, df: Py<PyAny>, periods: i32) -> PyResult<Py<PyAny>>{
    let code = r#"
from prophet import Prophet

def forecast_prophet_model(model, periods):
    future = model.make_future_dataframe(periods=periods)
    forecast = model.predict(future)
    return forecast
    "#;

    let res_module = PyModule::from_code_bound(py, code, "", "")?;

    let res_function = res_module.getattr("forecast_prophet_model")?;

    let result = res_function.call1((df, periods))?;

    let forecast: Py<PyAny> = result.extract()?;

    Ok(forecast)
}