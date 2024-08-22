pub struct TIMER {
    div: u16,   // DIV (Divider Register)
    tima: u8,   // TIMA (Timer Counter)
    tma: u8,    // TMA (Timer Modulo)
    tac: u8,    // TAC (Timer Control)
    cycles_counter: u16,
}

impl TIMER {
    pub fn new() -> Self {
        TIMER {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            cycles_counter: 0,
        }
    }

    pub fn add_cycles(&mut self, cycles: u16) {
        self.cycles_counter += cycles;
        if self.cycles_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.cycles_counter -= 256;
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}
