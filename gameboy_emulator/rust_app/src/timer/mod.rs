use std::sync::{Arc, Mutex};

use crate::cpu::CPU;

pub struct TIMER {
    div: u16,          // DIV (Divider Register)
    tima: u8,          // TIMA (Timer Counter)
    tma: u8,           // TMA (Timer Modulo)
    tac: u8,           // TAC (Timer Control)
    div_counter: u16,  // Counter for DIV
    tima_counter: u16, // Counter for TIMA
    cpu: Option<Arc<Mutex<CPU>>>,
}

impl TIMER {
    pub fn new() -> Self {
        TIMER {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_counter: 0,
            tima_counter: 0,
            cpu: None,
        }
    }

    pub fn set_cpu(&mut self, cpu: Arc<Mutex<CPU>>) {
        self.cpu = Some(cpu);
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

    pub fn tick(&mut self, cycles: u16) {
        self.div_counter += cycles;
        if self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }

        if self.tac & 0b100 != 0 {
            let clock_select = match self.tac & 0b11 {
                0b00 => 1024,  // 4096 Hz (1024 cycles CPU)
                0b01 => 16,    // 262144 Hz (16 cycles CPU)
                0b10 => 64,    // 65536 Hz (64 cycles CPU)
                0b11 => 256,   // 16384 Hz (256 cycles CPU)
                _ => unreachable!(),
            };

            self.tima_counter += cycles;

            if self.tima_counter >= clock_select {
                self.tima_counter -= clock_select;
                self.tima = self.tima.wrapping_add(1);

                if self.tima == 0 {
                    self.tima = self.tma;
                    if let Some(ref cpu) = self.cpu {
                        cpu.lock().unwrap().request_interrupt(0b00000100);
                    }
                }
            }
        }
    }
}
