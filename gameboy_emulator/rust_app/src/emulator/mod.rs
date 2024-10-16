use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::mmu::MMU;
use crate::timer::TIMER;
use crate::joypad::JOYPAD;
use crate::cartridge::CARTRIDGE;

pub struct EMULATOR {
    joypad: Rc<RefCell<JOYPAD>>,
    timer: Rc<RefCell<TIMER>>,
    ppu: Rc<RefCell<PPU>>,
    apu: Rc<RefCell<APU>>,
    mmu: Rc<RefCell<MMU>>,
    cpu: Rc<RefCell<CPU>>,
}

impl EMULATOR {
    pub fn new(cartridge: CARTRIDGE) -> Self {
        let timer = Rc::new(RefCell::new(TIMER::new()));
        let ppu = Rc::new(RefCell::new(PPU::new()));
        let apu = Rc::new(RefCell::new(APU::new()));
        let joypad = Rc::new(RefCell::new(JOYPAD::new()));
        let mmu = Rc::new(RefCell::new(MMU::new(Rc::clone(&joypad), Rc::clone(&timer), Rc::clone(&apu), Rc::clone(&ppu), cartridge)));
        ppu.borrow_mut().set_mmu(Rc::clone(&mmu));
        let cpu = Rc::new(RefCell::new(CPU::new(Rc::clone(&mmu))));
        timer.borrow_mut().set_cpu(Rc::clone(&cpu));
        joypad.borrow_mut().set_cpu(Rc::clone(&cpu));
        ppu.borrow_mut().set_cpu(Rc::clone(&cpu));

        EMULATOR {
            joypad,
            timer,
            ppu,
            apu,
            mmu,
            cpu, 
        }
    }

    pub fn run_cycles(&mut self, mut cycles_to_run: u32) {
        while cycles_to_run > 0 {
            self.cpu.borrow_mut().tick();
            let cycles_executed = self.cpu.borrow().get_cycles() as u16;
            self.timer.borrow_mut().tick(cycles_executed);
            cycles_to_run = cycles_to_run.saturating_sub(cycles_executed as u32);
        }
    }

    pub fn render_frame(&mut self) -> Vec<u32> {
        for ly in 0..144 {
            self.ppu.borrow_mut().set_ly(ly);
            
            // Mode 2: OAM Search
            self.ppu.borrow_mut().set_stat_mode(2);
            self.run_cycles(80);

            // Mode 3: Data transfer to LCD
            self.ppu.borrow_mut().set_stat_mode(3);
            self.run_cycles(170);

            self.ppu.borrow_mut().render_scanline();

            // Mode 0: HBLANK
            self.ppu.borrow_mut().set_stat_mode(0);
            self.run_cycles(206);
        }

        // Mode 1: VBLANK
        self.cpu.borrow_mut().request_interrupt(0b00000001);
        self.ppu.borrow_mut().set_stat_mode(1);

        for ly in 144..154 {
            self.ppu.borrow_mut().set_ly(ly);
            self.run_cycles(456);
        }

        self.ppu.borrow().get_screen_buffer()
    }
}