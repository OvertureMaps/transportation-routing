pub fn decode_lat_lon(encoded_lat: u32, encoded_lon: u32) -> (f64, f64) {
    let latitude = (encoded_lat as f64 / 10f64.powi(7)) - 90.0;
    let longitude = (encoded_lon as f64 / 10f64.powi(7)) - 180.0;
    (latitude, longitude)
}

pub fn encode_lat_lon(decoded_lat: f64, decoded_lon: f64) -> (u32, u32) {
    let encoded_lat = ((decoded_lat + 90.0) * 10f64.powi(7)) as u32;
    let encoded_lon = ((decoded_lon + 180.0) * 10f64.powi(7)) as u32;
    (encoded_lat, encoded_lon)
}
