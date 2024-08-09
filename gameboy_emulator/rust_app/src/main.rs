mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

fn main() {
    let mmu = MMU::new();
    let mut cpu = CPU::new(mmu);
    cpu.a = 20;
    cpu.b = 30;
    let opcode = 0x80;
    cpu.execute(opcode);
    println!("{}", cpu.a);
}
