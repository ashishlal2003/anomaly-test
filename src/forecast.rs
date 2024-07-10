use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFloat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ForecastResult {
    pub ds: f64,
    pub yhat: f64,
    pub yhat_lower: f64,
    pub yhat_upper: f64,
}

/// implemented FromPyObject for ForecastResult
/// the exact_bound() function needs some work with error handling (too many unwraps)
/// but this implementation works
/// 
/// Switched python datetime to epoch timestamp to get integer values, can switch timestamp
/// to a string and keep `ds` a string if you want (but the conversion needs to happen in Python)
impl FromPyObject<'_> for ForecastResult {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let ds = ob.get_item("ds").unwrap().downcast_into::<PyFloat>().unwrap().to_string().parse::<f64>().unwrap();
        let yhat = ob.get_item("yhat").unwrap().downcast_into::<PyFloat>().unwrap().to_string().parse::<f64>().unwrap();
        let yhat_lower = ob.get_item("yhat_lower").unwrap().downcast_into::<PyFloat>().unwrap().to_string().parse::<f64>().unwrap();
        let yhat_upper = ob.get_item("yhat_upper").unwrap().downcast_into::<PyFloat>().unwrap().to_string().parse::<f64>().unwrap();

        let res = ForecastResult {
            ds,
            yhat,
            yhat_lower,
            yhat_upper
        };
        Ok(res)
    }
}

pub fn forecast(data: Vec<(String, f64)>, periods: i32) -> PyResult<Vec<ForecastResult>> {
    Python::with_gil(|py| {
        let data_dict: Vec<_> = data
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

    # conversion to unix epoch timestamp
    # https://stackoverflow.com/a/54313505/21082374
    forecast['ds'] = pd.to_datetime(forecast['ds'],utc=True).astype(int) / 10**9
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
