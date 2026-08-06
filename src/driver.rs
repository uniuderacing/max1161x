
use core::{marker::PhantomData, result::Result::{Err, Ok}};
use embedded_hal::i2c::I2c; 
use crate::{initbytes::setupbyte, variants::Variant};

 
/// Driver per la famiglia MAX11xx
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

    pub fn new(mut i2c: I2C, setup_byte: setupbyte::SetupByte, config_byte: u8)-> Self{

        let init_byte = [setup_byte.to_byte()];
        let addr= V::ADDR;

        match i2c.write(addr, &init_byte) {
        Ok(_)=>{
            log::info!("Trovato");

        }
        Err(e) => {
            log::info!("Nessun dispositivo I2C trovato (0x{:02X}): {:?}", addr, e);
        }
        }

        Self{
            i2c,
            _variant:PhantomData
        }

    }
    
    /*
        Converte un valore voltaggio in temperatura secondo una regressione polinomiale di terzo grado
        Polinomio tarato sperimetalmente con un range di temperature da 15 a 75C con punti ogni 5C
        Tarato con max11617

        f(x)= a*x^3 + b*x^2 +c*x + d 
    */
    fn voltage_to_temperature(&mut self, v: f32) -> f32{
        let a = 1.79615598298204;
        let b=  -9.37248626037254;
        let c= 38.0779254777707;
        let d = -2.53332540256446;

        return (a * (v as f64).powi(3)
                + b * (v as f64).powi(2)
                + c * (v as f64)
                + d) as f32
    }

    pub fn read_all_sensors(& mut self, output: &mut [f32]) -> Result<(), I2C::Error> { // dovra tornare un array di temperature
        let addr= V::ADDR;
        let mut rx_buffer= [0u8; 24];
        let config_byte:[u8; 1]  = [0b01100000];
        // rx_buffer contains 2 bytes for each channel of the v ariant
        let rx_size = 2*V::CHANNELS;
        // each channel reads two bytes, 0 (high=MSB) and 1 (low=LSB) so 0000 D11 D10 D9 D8 in the high and D7 ... D0 in the low 
        // and since it's a 12-bit ADC the 4 most significant bits of the high byte need to be masked to 0 using the & 0x0F operation
        match self.i2c.write_read(addr, &config_byte, &mut rx_buffer[..rx_size]){
            Ok(_)=>{
                for (channel_index, pair) in rx_buffer[..rx_size].chunks_exact(2).enumerate(){
                    // pair[0] is High Byte, pair[1] is Low Byte
                    // 1. Mask the high byte (keep only the lower 4 bits)
                    let high_part = (pair[0] & 0x0F) as u16;
                    // 2. Shift high part left by 8 and OR with low byte
                    let raw_value = (high_part << 8) | (pair[1] as u16);
                    
                    // 3. Convert to Voltage (Example: 2.048V Internal Ref)
                    // Voltage = (Raw / 4095) * VREF
                    let voltage = (raw_value as f32 * 3.3) / 4096.0;


                    let temperature = self.voltage_to_temperature(voltage);
                    log::info!("Channel {}: Voltage={}V | Temperature {} °C", channel_index, voltage, temperature);
                    output[channel_index] = temperature;
                }
                
            }
            Err(e)=>{
                log::error!("ERR");
                return Err(e);
            }
        }
        return Ok(());
    }

    

}
