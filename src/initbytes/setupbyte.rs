// Setup byte

#[derive(Clone, Copy)]
pub enum RefSelect {
    /// SEL=000:  VDD come Vref, AIN_/REF come input analogico
    VddRef                    = 0b000,
    /// SEL=010:  Vref esterno su REF, reference interno spento
    ExternalRef               = 0b010,
    /// SEL=100:  Vref interno, AIN_/REF come input, reference sempre spento
    InternalRefOff            = 0b100,
    /// SEL=101:  Vref interno, AIN_/REF come input, reference sempre acceso
    InternalRefOn             = 0b101,
    /// SEL=110:  Vref interno, REF come output, reference sempre spento
    InternalRefOutputOff      = 0b110,
    /// SEL=111:  Vref interno, REF come output, reference sempre acceso
    InternalRefOutputOn       = 0b111,
}

#[derive(Clone, Copy)]
pub enum Clock {
    Internal = 1,
    External = 0,
}

#[derive(Clone, Copy)]
pub enum Mode {
    Unipolar = 0,
    Bipolar = 1,
}

pub struct SetupByte {
    pub ref_select: RefSelect,
    pub clock:      Clock,
    pub mode:       Mode,
    pub reset:      bool,   // false = resetta config register, true = no action
}

impl SetupByte {
    pub fn new(ref_select: RefSelect, clock: Clock, mode: Mode, reset: bool) -> Self {
        Self { ref_select, clock, mode, reset }
    }

    // Costruttore con i default del chip al power-up
    pub fn default() -> Self {
        Self {
            ref_select: RefSelect::VddRef,
            clock:      Clock::Internal,
            mode:       Mode::Unipolar,
            reset:      true,
        }
    }

    pub fn to_byte(&self)-> u8{

        let reg:u8 = 1 << 7;
        let sel = (self.ref_select as u8) << 4;
        let clk= (self.clock as u8) << 3;
        let bip= (self.mode as u8) << 2;
        let rst= (self.reset as u8) << 1;

        let setupbyte= reg | sel | clk | bip | rst;

        return setupbyte;
    }
}