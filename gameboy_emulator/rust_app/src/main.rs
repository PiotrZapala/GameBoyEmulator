mod cpu;
mod ppu;
mod apu;
mod mmu;
mod timer;
mod utils;
mod cartridge;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::mmu::MMU;
use crate::timer::TIMER;
use crate::cartridge::CARTRIDGE;

pub struct EMULATOR {
    timer: Rc<RefCell<TIMER>>,
    ppu: Rc<RefCell<PPU>>,
    apu: Rc<RefCell<APU>>,
    mmu: Rc<RefCell<MMU>>,
    cpu: CPU,
}

impl EMULATOR {
    pub fn new(cartridge: CARTRIDGE) -> Self {
        let timer = Rc::new(RefCell::new(TIMER::new()));
        let ppu = Rc::new(RefCell::new(PPU::new()));
        let apu = Rc::new(RefCell::new(APU::new()));
        let mmu = Rc::new(RefCell::new(MMU::new(Rc::clone(&timer), Rc::clone(&apu), Rc::clone(&ppu), cartridge)));
        let cpu = CPU::new(Rc::clone(&mmu), Rc::clone(&timer));

        EMULATOR {
            timer,
            ppu,
            apu,
            mmu,
            cpu, 
        }
    }

    pub fn run(&mut self) {
        self.cpu.a = 10;
        self.cpu.execute(0x3C);
        println!("{}", self.cpu.a);
    }
}

fn main() {
    let data = match utils::load_cartridge_from_file("src/Tetris.gb") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load cartridge: {}", e);
            return;
        }
    };
    let cartridge = CARTRIDGE::new(data);
    println!("{}", cartridge.decode_title());
    let mut emulator = EMULATOR::new(cartridge);
    emulator.run();
}
