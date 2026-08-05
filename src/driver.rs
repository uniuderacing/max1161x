
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
    fn convert(&mut self, v: f32) -> f32{
        let a = 1.79615598298204;
        let b=  -9.37248626037254;
        let c= 38.0779254777707;
        let d = -2.53332540256446;

        return (a * (v as f64).powi(3)
                + b * (v as f64).powi(2)
                + c * (v as f64)
                + d) as f32
    }

    pub fn read(& mut self) -> f32 { // dovra tornare un array di temperature
        let mut res =0;
        let addr= V::ADDR;

        let config_byte:[u8; 1]  = [0b01100000];

        let mut rx_buffer= [0u8; 8];
        match self.i2c.write_read(addr, &config_byte, &mut rx_buffer){
            Ok(_)=>{
                log::info!("Noerr");
                // gestisci per buffer piu grandi
                let msb = (rx_buffer[0] & 0x0F) as u16;
                let lsb = rx_buffer[1] as u16;
                let raw_value = (msb << 8) | lsb;
                //3.3 TODO: gestisci il setup byte 
                let voltage = (raw_value as f32 * 3.3) / 4096.0;

                converted_voltage = self.convert(voltage);

                log::info!("RX: {:02X}; {:02X}", rx_buffer[0], rx_buffer[1]);
            }
            Err(e)=>{
                log::info!("ERR");
            }
        }


        return res as f32;
    }

    

}
