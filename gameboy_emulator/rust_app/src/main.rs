mod cpu;
mod mmu;

use cpu::CPU;
use mmu::MMU;

fn main() {
    let mut mmu = MMU::new();
    mmu.rom[0] = 0xF8;
    mmu.rom[1] = 255;
    //mmu.rom[2] = 0xFF;
    //mmu.write_byte(0xFFEE, 69);
    let mut cpu = CPU::new(mmu);
    cpu.sp = 4444;

    loop {
        let opcode = cpu.mmu.fetch_instruction(cpu.pc);
        cpu.execute(opcode);

        if cpu.pc == 2 {
            break;
        }
    }
    
    println!("{:X}", cpu.h);
    println!("{:X}", cpu.l);
    //println!("{}", cpu.mmu.read_byte(0xFFEE));
    //println!("{:X}", cpu.mmu.read_byte(0xC035));
}
