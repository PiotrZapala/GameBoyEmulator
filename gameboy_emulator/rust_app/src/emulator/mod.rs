use std::sync::{Arc, Mutex};

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::mmu::MMU;
use crate::timer::TIMER;
use crate::joypad::JOYPAD;
use crate::cartridge::CARTRIDGE;

pub struct EMULATOR {
    pub joypad: Arc<Mutex<JOYPAD>>,
    timer: Arc<Mutex<TIMER>>,
    ppu: Arc<Mutex<PPU>>,
    apu: Arc<Mutex<APU>>,
    mmu: Arc<Mutex<MMU>>,
    cpu: Arc<Mutex<CPU>>,
    is_vblank: bool,
}

impl EMULATOR {
    pub fn new(cartridge: CARTRIDGE) -> Self {
        let timer = Arc::new(Mutex::new(TIMER::new()));
        let ppu = Arc::new(Mutex::new(PPU::new()));
        let apu = Arc::new(Mutex::new(APU::new()));
        let joypad = Arc::new(Mutex::new(JOYPAD::new()));
        let mmu = Arc::new(Mutex::new(MMU::new(
            Arc::clone(&joypad),
            Arc::clone(&timer),
            Arc::clone(&apu),
            Arc::clone(&ppu),
            cartridge,
        )));
        ppu.lock().unwrap().set_mmu(Arc::clone(&mmu));
        let cpu = Arc::new(Mutex::new(CPU::new(Arc::clone(&mmu))));
        timer.lock().unwrap().set_cpu(Arc::clone(&cpu));
        joypad.lock().unwrap().set_cpu(Arc::clone(&cpu));
        ppu.lock().unwrap().set_cpu(Arc::clone(&cpu));

        EMULATOR {
            joypad,
            timer,
            ppu,
            apu,
            mmu,
            cpu,
            is_vblank: false,
        }
    }

    pub fn save_ram(&self) -> Option<Vec<u8>> {
        self.mmu.lock().unwrap().save_ram()
    }

    pub fn run_cycles(&mut self, mut cycles_to_run: u32) {
        while cycles_to_run > 0 {
            let mut cpu = self.cpu.lock().unwrap();
            cpu.tick();
            let cycles_executed = cpu.get_cycles() as u16;
            drop(cpu);
            let mut ppu = self.ppu.lock().unwrap();
            ppu.dma_transfer();
            drop(ppu);
            let mut timer = self.timer.lock().unwrap();
            timer.tick(cycles_executed);       
            drop(timer);
            self.check_buttons();
            cycles_to_run = cycles_to_run.saturating_sub(cycles_executed as u32);
        }
    } 

    pub fn set_button_states(&mut self, up: u8, down: u8, left: u8, right: u8, a: u8, b: u8, start: u8, select: u8) {
        let mut joypad = self.joypad.lock().unwrap();
        joypad.set_button_state(0b00000100, up == 0);      // Up
        joypad.set_button_state(0b00001000, down == 0);    // Down
        joypad.set_button_state(0b00000010, left == 0);    // Left
        joypad.set_button_state(0b00000001, right == 0);   // Right
        joypad.set_button_state(0b00010000, a == 0);       // A
        joypad.set_button_state(0b00100000, b == 0);       // B
        joypad.set_button_state(0b10000000, start == 0);   // Start
        joypad.set_button_state(0b01000000, select == 0);  // Select
    }

    pub fn check_buttons(&mut self) {
        self.is_vblank = true;
        if self.is_vblank {
            let mut joypad = self.joypad.lock().unwrap();
            joypad.check_for_interrupt();
        }
        self.is_vblank = false;
    }

    pub fn handle_vblank(&mut self) {
        for ly in 144..154 {
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.set_stat_mode(1);
                ppu.set_ly(ly);
            }
            self.run_cycles(456);
        }
    }

    pub fn render_frame(&mut self) -> Vec<u32> {
        let display_enabled;
        {
            let ppu = self.ppu.lock().unwrap();
            display_enabled = ppu.is_display_enabled();
        }
    
        if !display_enabled {
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.set_stat_mode(0);
                ppu.set_ly(0);
            }
    
            for _ in 0..154 {
                self.run_cycles(456);
            }
    
            return vec![0x00FFFFFF; 144 * 160];
        }

        for ly in 0..144 {

            // Mode 2: OAM Search
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.set_ly(ly);
                ppu.set_stat_mode(2);
            }
            self.run_cycles(80);
    
            // Mode 3: Data transfer to LCD
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.set_stat_mode(3);
            }
            self.run_cycles(170);
    
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.render_scanline();
            }
    
            // Mode 0: HBLANK
            {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.set_stat_mode(0);
            }
            self.run_cycles(206);
        }
    
        // Mode 1: VBLANK
        {
            let mut cpu = self.cpu.lock().unwrap();
            cpu.request_interrupt(0b00000001);
        }
    
        let ppu = self.ppu.lock().unwrap();
        ppu.get_screen_buffer()
    }
    
}