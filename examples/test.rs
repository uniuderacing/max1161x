use core::result::Result::{Err, Ok};

use max1161x::{max_driver::MAX116xx, max_variants::MAX11617};

fn main() {
    vcu_rt::hal::init(true).unwrap();
    log::info!("External MAX11617 example");

    let peripherals = Peripherals::take().unwrap();

    // Configurazione I2C a 400kHz (Fast Mode supportata dal MAX11616)
    let config = I2cConfig::new().baudrate(400.kHz().into());

    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio8, // SDA
        peripherals.pins.gpio9, // SCL
        &config,
    )
    .unwrap();

    let setup_byte: u8 = 0b11010010;
    let config_byte: u8 = 0b01100001;

    let mut sensor: MAX116xx<_, MAX11617> = MAX116xx::new(i2c, setup_byte, config_byte);
}
