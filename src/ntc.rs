#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorFault {
    RailLow,  // divider shorted low
    RailHigh, // sensor open / shorted high
    Bus,      // I2C/mux failure
    NotRead,
}
pub type Reading = Result<f32, SensorFault>;

// Tune to your divider. Anything this close to a rail is a wiring fault, not a temperature.
const RAW_MIN: u16 = 16;
const RAW_MAX: u16 = 4079;

pub fn raw_to_millivolts(raw: u16, vdd: f32) -> Reading {
    if raw < RAW_MIN { return Err(SensorFault::RailLow); }
    if raw > RAW_MAX { return Err(SensorFault::RailHigh); }
    let v = raw as f32 * vdd / 4096.0;
    return Ok(v);
}

pub fn raw_to_celsius(raw: u16, vdd: f32) -> Reading {
    match raw_to_millivolts(raw, vdd){
        // Cubic fit, calibrated 15..75 °C. Horner form in f32 (no soft-f64 on the ESP32).
        Ok(v) => Ok(((1.796_156 * v - 9.372_486) * v + 38.077_925) * v - 2.533_325),
        Err(e) => Err(e),
    }
}