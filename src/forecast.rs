use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ForecastResult {
    pub ds: String,
    pub yhat: f64,
    pub yhat_lower: f64,
    pub yhat_upper: f64,
}

pub fn forecast(data: Vec<(String, f64)>, periods: i32) -> PyResult<Vec<ForecastResult>> {
    Python::with_gil(|py| {
        let data_dict: Vec<&PyDict> = data
            .iter()
            .map(|(ds, y)| {
                let dict = PyDict::new_bound(py);
                dict.set_item("ds", ds).unwrap();
                dict.set_item("y", y).unwrap();
                dict
            })
            .collect();

        let forecast_code = r#"
import pandas as pd
from prophet import Prophet

def forecast(data, periods):
    df = pd.DataFrame(data)
    m = Prophet()
    m.fit(df)
    future = m.make_future_dataframe(periods=periods)
    forecast = m.predict(future)
    return forecast[['ds', 'yhat', 'yhat_lower', 'yhat_upper']].to_dict('records')
        "#;

        let forecast_module = PyModule::from_code_bound(py, forecast_code, "", "")?;
        let forecast_func = forecast_module.getattr("forecast")?;
        let result = forecast_func.call1((data_dict, periods))?;

        let forecast: Vec<ForecastResult> = result.extract()?;
        // let forecast = result.extract()?;
        Ok(forecast)
    })
}
