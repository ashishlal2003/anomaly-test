mod data;
mod prophet_model;
mod anomaly_detection;

use data::load_and_preprocess_data;
use prophet_model::{fit_prophet_model, forecast_prophet_model};
use anomaly_detection::{calculated_residuals, calculate_z_scores, detect_anomalies};
use pyo3::prelude::*;
use pyo3::types::{PyList};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let file_path = "assets\\frontend-logs.csv";

        let df = load_and_preprocess_data(py, file_path, 404, "2s")?;

        let model = fit_prophet_model(py, df.clone_ref(py))?;

        let forecast = forecast_prophet_model(py, model, 30)?;

        let df_with_residuals = calculated_residuals(py, df, forecast)?;

        let residuals = df_with_residuals.call_method1(py, "get", ("residual",))?;
        let ds_col = df_with_residuals.call_method1(py, "get", ("ds",))?;

        let residuals_list: Vec<f64> = residuals.call_method0(py, "tolist")?.extract(py)?;
        let residuals_py = PyList::new_bound(py, &residuals_list).into();

        let ds_list: Vec<String> = ds_col.call_method0(py, "tolist")?.extract(py)?;

        let z_scores = calculate_z_scores(py, residuals_py)?;

        let anomalies = detect_anomalies(py, z_scores, 3.0)?;

        let anomalies_list: Vec<usize> = anomalies.extract(py)?;
        let anomaly_dates: Vec<String> = anomalies_list
            .iter()
            .filter_map(|&index| ds_list.get(index).cloned())
            .collect();

        println!("Anomalies detected at: {:?}", anomaly_dates);

        Ok(())
    })
}
