use core::result::Result::{Err, Ok};
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    units::FromValueType,
};

use max1161x::{
    max_messages::setup_byte::SetupByte, 
    max_variants::MAX11617, 
    mux_governor::MuxGovernor, 
    ntc::{Reading, SensorFault}, 
    tca9543a::Tca9543a
};

fn main() {
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

    let setup = SetupByte::default();
    let mux = Tca9543a::new(false, false);
    let mut governor = MuxGovernor::<_,MAX11617>::new(i2c, mux, setup)
    .expect("unsupported ADC setup");
    governor.init().expect("ADC/mux init failed");

    let mut readings = [Err(SensorFault::NotRead);24];

    loop {
        match governor.read_all(&mut readings, 3.3) {
            Ok(0) => {}  // all sensors have been correctly red
            Ok(n) => {
                log::warn!("{n} ADC(s) failed, re-initialising");
                gov.invalidate();
                gov.init().expect("ADC/mux init failed");
            }
            Err(e) => {
                log::error!("Error during read_all operation: {e:?}");
                gov.invalidate();
                gov.init().expect("ADC/mux init failed");
            }
        }
        for (i, r) in out.iter().enumerate() {
            let (block, ain) = (i / 12, i % 12);     // block 0 = mux ch0, block 1 = mux ch1
            match r {
                Ok(t) => log::info!("block {block} AIN{ain}: {t:.2} °C"),
                Err(f) => log::warn!("block {block} AIN{ain}: {f:?}"),
            }
        }
        FreeRtos::delay_ms(1000);
    }
}
