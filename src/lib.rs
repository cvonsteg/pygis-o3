use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64;

#[pyfunction]
/// Calculate haversine distance between to sets of lat/lon coords
fn haversine(lat_from: f64, lon_from: f64, lat_to: f64, lon_to: f64) -> PyResult<f64> {
    const R:f64 = 6371f64;

    // convert latitudes to radians
    let lat_rad_from = lat_from.to_radians();
    let lat_rad_to = lat_to.to_radians();
    let lat_delta = (lat_to - lat_from).to_radians();
    let lon_delta = (lon_to - lon_from).to_radians();

    let a = (lat_delta / 2.0).sin().powi(2) + lat_rad_from.cos() * lat_rad_to.cos() * (lon_delta / 2.0).sin().powi(2);

    let central_angle = 2.0 * a.sqrt().asin();

    Ok(R * central_angle)
}


#[pymodule]
fn pygis_o3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(haversine))?;

    Ok(())
}



