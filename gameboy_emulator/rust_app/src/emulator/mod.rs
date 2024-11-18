use std::sync::{Arc, Mutex};

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::mmu::MMU;
use crate::timer::TIMER;
use crate::joypad::JOYPAD;
use crate::cartridge::CARTRIDGE;

pub struct EMULATOR {
    joypad: Arc<Mutex<JOYPAD>>,
    timer: Arc<Mutex<TIMER>>,
    ppu: Arc<Mutex<PPU>>,
    apu: Arc<Mutex<APU>>,
    mmu: Arc<Mutex<MMU>>,
    cpu: Arc<Mutex<CPU>>,
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
        }
    }

    pub fn save_ram(&self) -> Option<Vec<u8>> {
        self.mmu.lock().unwrap().save_ram()
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

    pub fn render_frame(&mut self) -> Vec<u32> {
        loop {
            let mut cpu = self.cpu.lock().unwrap();
            cpu.tick();
            let cycles = cpu.get_cycles() as u16;
            drop(cpu);
    
            let mut ppu = self.ppu.lock().unwrap();
            ppu.dma_transfer();
            ppu.tick(cycles);
            drop(ppu);
    
            let mut timer = self.timer.lock().unwrap();
            timer.tick(cycles);
            drop(timer);
    
            let mut joypad = self.joypad.lock().unwrap();
            joypad.check_for_interrupt();
            drop(joypad);

            let mut ppu = self.ppu.lock().unwrap();
            let frame_ready = ppu.is_frame_ready();
            if frame_ready {
                ppu.reset_frame_ready();
                drop(ppu);
                break;
            }
            drop(ppu);
        }
    
        let ppu = self.ppu.lock().unwrap();
        ppu.get_screen_buffer()
    }
}