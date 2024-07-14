mod forecast;

use forecast::forecast;
use pyo3::prelude::*;
use std::error::Error;
use chrono::NaiveDateTime;
use std::any::type_name;

fn read_csv(file_path: &str) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let date_str = &record[0];
        let value: f64 = record[8].parse()?;

        let date = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ")?.date().to_string();

        if !date.is_empty() && !value.is_nan() {
            data.push((date, value));
        }
        else{
            eprintln!("Invalid data: Date: {}, Value: {}", date, value);
        }
        
    }

    Ok(data)
}

fn main() -> PyResult<()> {
    let file_path = "C:\\Users\\ashis\\Downloads\\frontend-logs.csv";

    match read_csv(file_path) {
        Ok(data) => {
            match forecast(data, 10) {
                Ok(result) => println!("{:?}", result),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Error reading CSV file: {:?}", e),
    }

    Ok(())
}
