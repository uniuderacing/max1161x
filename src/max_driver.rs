use crate::{
    max_messages::{config_byte, setup_byte},
    max_variants::Variant,
};
use core::{
    marker::PhantomData,
    result::Result::{Err, Ok},
};
use embedded_hal::i2c::I2c;
use std::{thread, time::Duration};

/// Driver per la famiglia MAX1161x
///

pub struct MAX116xx<I2C, V: Variant> {
    i2c: I2C,
    _variant: PhantomData<V>,
}

impl<I2C, V> MAX116xx<I2C, V>
where
    I2C: I2c,
    V: Variant,
{
    pub fn new(mut i2c: I2C, setup_byte: setup_byte::SetupByte, config_byte: u8) -> Self {
        let init_byte = [setup_byte.to_byte()];
        let addr = V::ADDR;

        match i2c.write(addr, &init_byte) {
            Ok(_) => {
                log::info!("Trovato");
            }
            Err(e) => {
                log::info!("Nessun dispositivo I2C trovato (0x{:02X}): {:?}", addr, e);
            }
        }

        Self {
            i2c,
            _variant: PhantomData,
        }
    }

    /*
        Converte un valore voltaggio in temperatura secondo una regressione polinomiale di terzo grado
        Polinomio tarato sperimetalmente con un range di temperature da 15 a 75C con punti ogni 5C
        Tarato con max11617

        f(x)= a*x^3 + b*x^2 +c*x + d
    */
    fn voltage_to_temperature(&mut self, v: f32) -> f32 {
        let a = 1.79615598298204;
        let b = -9.37248626037254;
        let c = 38.0779254777707;
        let d = -2.53332540256446;

        return (a * (v as f64).powi(3) + b * (v as f64).powi(2) + c * (v as f64) + d) as f32;
    }

    pub fn read_all_sensors(&mut self, output: &mut [f32]) -> Result<(), I2C::Error> {
        let addr = V::ADDR;
        let mut rx_buffer = [0u8; 24];
        let config_byte: [u8; 1] = [config_byte::ConfigByte::default().to_byte()];

        let rx_size = 2 * V::CHANNELS; // 2 bytes for each channel of the variant

        match self
            .i2c
            .write_read(addr, &config_byte, &mut rx_buffer[..rx_size])
        {
            Ok(_) => {
                for (channel_index, pair) in rx_buffer[..rx_size].chunks_exact(2).enumerate() {
                    let high_part = (pair[0] & 0x0F) as u16; // keeps only the lower 4 bits: 12 bit adc: 4 + 8 bits

                    let raw_value = (high_part << 8) | (pair[1] as u16);

                    let voltage = (raw_value as f32 * 3.3) / 4096.0;

                    let temperature = self.voltage_to_temperature(voltage);

                    // test
                    //if channel_index == 0 {
                    //    log::info!("Channel {}: Voltage={:.4}V | Raw Value {} | Est. temperature= {temperature} C", channel_index, voltage, raw_value);
                    //}

                    output[channel_index] = temperature;
                }
            }
            Err(e) => {
                log::error!("max116xx read all sensors error");
                return Err(e);
            }
        }
        return Ok(());
    }

    // for testing purposes only
    pub fn calibrate_me(&mut self) {
        const BUFFSIZE: usize = 12;
        const ATTEMPTS: usize = 5;
        let addr = V::ADDR;

        let mut raw_arr = [0.0; BUFFSIZE];
        let mut volt_arr = [0.0; BUFFSIZE];

        for _ in 0..ATTEMPTS {
            let mut rx_buffer = [0u8; 24];

            let config_byte: [u8; 1] = [config_byte::ConfigByte::default().to_byte()];

            let rx_size = 2 * V::CHANNELS;

            match self
                .i2c
                .write_read(addr, &config_byte, &mut rx_buffer[..rx_size])
            {
                Ok(_) => {
                    for (channel_index, pair) in rx_buffer[..rx_size].chunks_exact(2).enumerate() {
                        let high_part = (pair[0] & 0x0F) as u16;
                        let raw_value = (high_part << 8) | (pair[1] as u16);
                        let voltage = (raw_value as f32 * 3.3) / 4096.0;

                        raw_arr[channel_index] += raw_value as f32;
                        volt_arr[channel_index] += voltage;
                    }
                }

                Err(_) => {
                    log::error!("ERR");
                }
            }

            thread::sleep(Duration::from_millis(500));
        }

        for i in 0..BUFFSIZE {
            raw_arr[i] /= ATTEMPTS as f32;
            volt_arr[i] /= ATTEMPTS as f32;
        }

        log::info!("Raw Value {} | Voltage={:.4}V ", raw_arr[0], volt_arr[0]);
    }
}
