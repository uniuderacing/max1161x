
use core::marker::PhantomData;
use embedded_hal::i2c::I2c; 
use crate::variants::Variant;

 
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

    pub fn new(mut i2c: I2C, setup_byte: u8, config_byte: u8)-> Self{

        let init_bytes = [setup_byte, config_byte];
        let addr= V::ADDR;

        match i2c.write(addr, &init_bytes) {
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

    

}
