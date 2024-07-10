mod forecast;

use forecast::forecast;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let data = vec![
        ("2023-01-01".to_string(), 10.0),
        ("2023-01-02".to_string(), 15.0),
        ("2023-01-03".to_string(), 13.0),
        ("2023-01-04".to_string(), 17.0),
    ];

    match forecast(data, 10) {
        Ok(result) => println!("{:?}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
