
fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn covert_temperature(celsius: f64) -> Vec<f64> {
    let mut out = vec![0f64; 2];
    out[0] = celsius + 273.15f64;
    out[1] += 32f64 + celsius * 1.8f64;
    out
}
