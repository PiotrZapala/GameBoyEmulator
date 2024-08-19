mod cpu;
mod mmu;
mod timer;

use cpu::CPU;
use mmu::MMU;
use timer::TIMER;

fn main() {
    let mut mmu = MMU::new();
    let mut timer = TIMER::new();
    mmu.rom[0] = 0x27;
    let mut cpu = CPU::new(mmu, timer);
    cpu.a = 0x5D;

    loop {
        let opcode = cpu.mmu.fetch_instruction(cpu.pc);
        cpu.tick(opcode);

        if cpu.pc == 1 {
            break;
        }
    }
    
    println!("{:X}", cpu.a);
}
