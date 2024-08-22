pub struct TIMER {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
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
}