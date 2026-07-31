
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
    
    /*
        Converte un valore voltaggio in temperatura secondo una regressione polinomiale di terzo grado
        Polinomio tarato sperimetalmente con un range di temperature da 15 a 75C con punti ogni 5C
        Tarato con max11617

        f(x)= a*x^3 + b*x^2 +c*x + d 
    */
    fn convert(v: f32) -> f64{
        let a = 1.79615598298204;
        let b=  -9.37248626037254;
        let c= 38.0779254777707;
        let d = -2.53332540256446;

        return a * (v as f64).powi(3)
                + b * (v as f64).powi(2)
                + c * (v as f64)
                + d
    }

    

}
