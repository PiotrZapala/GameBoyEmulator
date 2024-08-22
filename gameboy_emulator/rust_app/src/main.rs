mod cpu;
mod mmu;
mod timer;
mod utils;
mod cartridge;

use cpu::CPU;
use mmu::MMU;
use timer::TIMER;
use cartridge::CARTRIDGE;

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
    let mut mmu = MMU::new(cartridge);
    let mut timer = TIMER::new();
    let mut cpu = CPU::new(mmu, timer);
}
