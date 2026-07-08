use linux_embedded_hal::I2cdev;
use max1161x::devices::variants::{Variant, MAX11615};
use max1161x::driver::MAX116xx;
use max1161x::messages::config::Config;
// sudo chmod 666 /dev/i2c-1
fn main() {
    // apre il bus I2C reale
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    // crea il driver
    let mut adc: MAX116xx<_, MAX11615> = MAX116xx::new(i2c, Config::default());

    // legge il canale 0
    //let raw = adc.read_channel(0).unwrap();
    //println!("AIN0 raw: {}", raw);
//
    //// legge in millivolt
    //let mv = adc.raw_to_mv(raw);
    //println!("AIN0: {} mV", mv);
}