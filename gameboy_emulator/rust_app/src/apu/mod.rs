pub struct APU {
    nr10: u8, // 0xFF10 - NR10: Channel 1 Sweep Register
    nr11: u8, // 0xFF11 - NR11: Channel 1 Sound Length/Wave Pattern Duty
    nr12: u8, // 0xFF12 - NR12: Channel 1 Volume Envelope
    nr13: u8, // 0xFF13 - NR13: Channel 1 Frequency Low
    nr14: u8, // 0xFF14 - NR14: Channel 1 Frequency High

    nr21: u8, // 0xFF16 - NR21: Channel 2 Sound Length/Wave Pattern Duty
    nr22: u8, // 0xFF17 - NR22: Channel 2 Volume Envelope
    nr23: u8, // 0xFF18 - NR23: Channel 2 Frequency Low
    nr24: u8, // 0xFF19 - NR24: Channel 2 Frequency High

    nr30: u8, // 0xFF1A - NR30: Channel 3 Sound On/Off
    nr31: u8, // 0xFF1B - NR31: Channel 3 Sound Length
    nr32: u8, // 0xFF1C - NR32: Channel 3 Output Level
    nr33: u8, // 0xFF1D - NR33: Channel 3 Frequency Low
    nr34: u8, // 0xFF1E - NR34: Channel 3 Frequency High

    nr41: u8, // 0xFF20 - NR41: Channel 4 Sound Length
    nr42: u8, // 0xFF21 - NR42: Channel 4 Volume Envelope
    nr43: u8, // 0xFF22 - NR43: Channel 4 Polynomial Counter
    nr44: u8, // 0xFF23 - NR44: Channel 4 Counter/Consecutive; Initial

    nr50: u8, // 0xFF24 - NR50: Channel Control/Volume Control
    nr51: u8, // 0xFF25 - NR51: Selection of Sound Output Terminal
    nr52: u8, // 0xFF26 - NR52: Sound On/Off

    wave_pattern_ram: [u8; 16],
}

impl APU {
    pub fn new() -> Self {
        APU {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            nr50: 0,
            nr51: 0,
            nr52: 0,
            wave_pattern_ram: [0; 16],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF10 => self.nr10,
            0xFF11 => self.nr11,
            0xFF12 => self.nr12,
            0xFF13 => self.nr13,
            0xFF14 => self.nr14,
            0xFF16 => self.nr21,
            0xFF17 => self.nr22,
            0xFF18 => self.nr23,
            0xFF19 => self.nr24,
            0xFF1A => self.nr30,
            0xFF1B => self.nr31,
            0xFF1C => self.nr32,
            0xFF1D => self.nr33,
            0xFF1E => self.nr34,
            0xFF20 => self.nr41,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44,
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.nr52,
            0xFF30..=0xFF3F => {
                self.wave_pattern_ram[address as usize - 0xFF30]
            }
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.nr10 = value,
            0xFF11 => self.nr11 = value,
            0xFF12 => self.nr12 = value,
            0xFF13 => self.nr13 = value,
            0xFF14 => self.nr14 = value,
            0xFF16 => self.nr21 = value,
            0xFF17 => self.nr22 = value,
            0xFF18 => self.nr23 = value,
            0xFF19 => self.nr24 = value,
            0xFF1A => self.nr30 = value,
            0xFF1B => self.nr31 = value,
            0xFF1C => self.nr32 = value,
            0xFF1D => self.nr33 = value,
            0xFF1E => self.nr34 = value,
            0xFF20 => self.nr41 = value,
            0xFF21 => self.nr42 = value,
            0xFF22 => self.nr43 = value,
            0xFF23 => self.nr44 = value,
            0xFF24 => self.nr50 = value,
            0xFF25 => self.nr51 = value,
            0xFF26 => self.nr52 = value,
            0xFF30..=0xFF3F => {
                self.wave_pattern_ram[address as usize - 0xFF30] = value;
            }
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}
