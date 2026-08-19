use std::io::Error;

use crate::{max_driver::MAX116xx, max_variants::Variant};
use embedded_hal::i2c::I2c;

pub struct MuxGovernor<I2C, V: Variant>
where
    I2C: I2c,
{
    pub adc_list: Vec<(MAX116xx<I2C, V>, u8)>, // (adc, physical address)
}

///
/// Oggetto utilizzato per semplificare l'uso di più adc collegati ad un unico multiplexer
/// Astrae la gestione dei singoli adc 
/// 
impl<I2C, V> MuxGovernor<I2C, V>
where
    I2C: I2c,
    V: Variant,
{
    pub fn new() -> Self {
        Self {
            adc_list: Vec::new(),
        }
    }

    /// aggiunge un adc da controllare
    pub fn add_adc(&mut self, max116xx: MAX116xx<I2C, V>, address: u8) {
        self.adc_list.push((max116xx, address))
    }

    /// richiede la lettura di tutti gli adc
    pub fn update_all(&mut self) -> Result<Vec<f32>, I2C::Error> {
        let mut vec_adc_values: Vec<f32> = Vec::new();

        for (i, adc) in self.adc_list.iter_mut().enumerate(){
            let mut buff = [0.0; 12];

            adc.0.read_all_sensors(&mut buff); // aggiungi gestione errori match{...}

            for j in 0..buff.len(){
                vec_adc_values.push(buff[j]);
            }
        }

        return Ok(vec_adc_values);
    }

    pub fn update_single_adc(&mut self, address: u8) -> i32{
        1
    }

}
