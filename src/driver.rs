
use core::marker::PhantomData;
use embedded_hal::i2c::I2c; 

use crate::{devices::variants::Variant, messages::{config::Config, instructionmessage::InstructionMessage}};

 
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

    pub fn new(i2c: I2C, setupbyte: Config)-> Self{
        Self{
            i2c,
            _variant:PhantomData
        }

    }

    pub fn read(&mut self, msg: InstructionMessage) -> Result<u16, I2C::Error> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(V::ADDR, &[msg.to_byte()], &mut buf)?;
        let raw = u16::from_be_bytes(buf) & 0x0FFF;
        Ok(raw)
    }

}
