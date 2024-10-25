mod instructions;

use crate::mmu::MMU;
use instructions::*;

use std::sync::{Arc, Mutex};

pub enum INTERRUPT_ADDRESS {
    VBlank = 0x0040,   // Address for Vertical Blank interrupt
    LCDStat = 0x0048,  // Address for LCD Status interrupt
    Timer = 0x0050,    // Address for Timer interrupt
    Serial = 0x0058,   // Address for Serial I/O interrupt
    Joypad = 0x0060,   // Address for Joypad interrupt
}

impl INTERRUPT_ADDRESS {
    pub fn address(self) -> u16 {
        self as u16
    }
}

pub enum INTERRUPT_FLAG {
    VBlank = 0b00000001,   // Flag bit for Vertical Blank interrupt
    LCDStat = 0b00000010,  // Flag bit for LCD Status interrupt
    Timer = 0b00000100,    // Flag bit for Timer interrupt
    Serial = 0b00001000,   // Flag bit for Serial I/O interrupt
    Joypad = 0b00010000,   // Flag bit for Joypad interrupt
}

impl INTERRUPT_FLAG {
    pub fn value(self) -> u8 {
        self as u8
    }
}

pub enum RST {
    Rst00H = 0x0000,
    Rst08H = 0x0008,
    Rst10H = 0x0010,
    Rst18H = 0x0018,
    Rst20H = 0x0020,
    Rst28H = 0x0028,
    Rst30H = 0x0030,
    Rst38H = 0x0038,
}

impl RST {
    pub fn address(self) -> u16 {
        self as u16
    }
}

pub struct CPU {
    pub a: u8,                     // Accumulator register. Used in arithmetic and logic operations.
    pub b: u8,                     // General-purpose register B. Often paired with C as BC.
    pub c: u8,                     // General-purpose register C. Often paired with B as BC.
    pub d: u8,                     // General-purpose register D. Often paired with E as DE.
    pub e: u8,                     // General-purpose register E. Often paired with D as DE.
    pub f: u8,                     // Flag register. Contains status flags (Zero, Subtract, Half Carry, Carry).
    pub h: u8,                     // General-purpose register H. Often paired with L as HL.
    pub l: u8,                     // General-purpose register L. Often paired with H as HL.
    pub pc: u16,                   // Program Counter. Points to the next instruction to be executed in memory.
    pub sp: u16,                   // Stack Pointer. Points to the top of the stack.
    pub ime: bool,                 // Interrupt Master Enable flag. Controls the global interrupt enable/disable state.
    pub halted: bool,              // Halt flag. Indicates if the CPU is in a halted state, waiting for an interrupt.
    pub cycles: u16,               // Cycles number. Stores the number of cycles executed by the last instruction.
    pub mmu: Arc<Mutex<MMU>>,      // Memory Management Unit. Manages access to different memory regions.
}

impl CPU {
    pub fn new(mmu: Arc<Mutex<MMU>>) -> Self {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            ime: false,
            halted: false,
            cycles: 0,
            mmu,
        }
    }

    pub fn tick(&mut self) {
        self.handle_interrupts();
        let opcode = self.mmu.lock().unwrap().fetch_instruction(self.pc);
        self.execute(opcode);
    }

    pub fn execute(&mut self, opcode: u8) {
        if opcode != 0xCB {
            self.execute_not_prefixed_instruction(opcode);
        } else {
            self.pc += 1;
            let new_opcode = self.mmu.lock().unwrap().fetch_instruction(self.pc);
            self.execute_prefixed_instruction(new_opcode);
        }
    }

    pub fn set_cycles(&mut self, cycles: u16) {
        self.cycles = cycles;
    }

    pub fn get_cycles(&self) -> u16 {
        self.cycles
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        let mut mmu = self.mmu.lock().unwrap();
        let current_interrupt_flag = mmu.read_byte(0xFF0F);
        let new_interrupt_flag = current_interrupt_flag | interrupt;
        mmu.write_byte(0xFF0F, new_interrupt_flag);
    }

    fn handle_interrupts(&mut self) -> bool {
        if !self.ime {
            return false;
        } 

        let mut mmu = self.mmu.lock().unwrap();
        let interrupt_flag = mmu.read_byte(0xFF0F);
        let interrupt_enable = mmu.read_byte(0xFFFF);
        let is_enabled_interrupts = interrupt_flag & interrupt_enable & 0b00011111;

        if is_enabled_interrupts == 0 {
            return false;
        }

        self.sp -= 2;
        mmu.write_byte(self.sp + 1, (self.pc >> 8) as u8);
        mmu.write_byte(self.sp, (self.pc & 0xFF) as u8);

        if interrupt_flag & INTERRUPT_FLAG::VBlank.value() != 0 {
            mmu.write_byte(0xFF0F, interrupt_flag & !INTERRUPT_FLAG::VBlank.value());
            self.pc = INTERRUPT_ADDRESS::VBlank.address();
            self.ime = false;
        } else if interrupt_flag & INTERRUPT_FLAG::LCDStat.value() != 0 {
            mmu.write_byte(0xFF0F, interrupt_flag & !INTERRUPT_FLAG::LCDStat.value());
            self.pc = INTERRUPT_ADDRESS::LCDStat.address();
            self.ime = false;
        } else if interrupt_flag & INTERRUPT_FLAG::Timer.value() != 0 {
            mmu.write_byte(0xFF0F, interrupt_flag & !INTERRUPT_FLAG::Timer.value());
            self.pc = INTERRUPT_ADDRESS::Timer.address();
            self.ime = false;
        } else if interrupt_flag & INTERRUPT_FLAG::Serial.value() != 0 {
            mmu.write_byte(0xFF0F, interrupt_flag & !INTERRUPT_FLAG::Serial.value());
            self.pc = INTERRUPT_ADDRESS::Serial.address();
            self.ime = false;
        } else if interrupt_flag & INTERRUPT_FLAG::Joypad.value() != 0 {
            mmu.write_byte(0xFF0F, interrupt_flag & !INTERRUPT_FLAG::Joypad.value());
            self.pc = INTERRUPT_ADDRESS::Joypad.address();
            self.ime = false;
        }

        true
    }


    fn update_flags(&mut self, zero: Option<bool>, carry: Option<bool>, negative: Option<bool>, half_carry: Option<bool>) {
        if let Some(z) = zero {
            if z {
                self.f |= 0x80;
            } else {
                self.f &= !0x80;
            }
        }

        if let Some(n) = negative {
            if n {
                self.f |= 0x40;
            } else {
                self.f &= !0x40;
            }
        }

        if let Some(h) = half_carry {
            if h {
                self.f |= 0x20;
            } else {
                self.f &= !0x20;
            }
        }

        if let Some(c) = carry {
            if c {
                self.f |= 0x10;
            } else {
                self.f &= !0x10;
            }
        }
    }

    fn execute_not_prefixed_instruction(&mut self, opcode: u8) {
        let bits_7_6 = opcode >> 6;
        match bits_7_6 {
            0 => self.execute_not_prefixed_instruction_starting_with_00(opcode),
            1 => self.execute_not_prefixed_instruction_starting_with_01(opcode),
            2 => self.execute_not_prefixed_instruction_starting_with_10(opcode),
            3 => self.execute_not_prefixed_instruction_starting_with_11(opcode),
            _ => unreachable!(),
        }
    }

    fn execute_prefixed_instruction(&mut self, opcode: u8) {
        let bits_7_6 = opcode >> 6;
        match bits_7_6 {
            0 => self.execute_prefixed_instruction_starting_with_00(opcode),
            1 => self.execute_prefixed_instruction_starting_with_01(opcode),
            2 => self.execute_prefixed_instruction_starting_with_10(opcode),
            3 => self.execute_prefixed_instruction_starting_with_11(opcode),
            _ => unreachable!(),
        }
    }
    
    fn execute_not_prefixed_instruction_starting_with_00(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                match bits_5_4_3 {
                    0 => {// NOP
                        nop(self);
                    },
                    1 => {// LD (nn), SP
                        ld_m_u16_sp(self);
                    },
                    2 => {// STOP
                        stop(self);
                    },
                    3 => {// JR d
                        jr_i8(self);
                    },
                    4 => {// JR NZ, d
                        jr_nz_i8(self);
                    },
                    5 => {// JR Z, d
                        jr_z_i8(self);
                    },
                    6 => {// JR NC, d
                        jr_nc_i8(self);
                    },
                    7 => {// JR C, d
                        jr_c_i8(self);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => {// LD BC, nn
                        ld_bc_u16(self);
                    },
                    1 => {// ADD HL, BC
                        let register_b = self.b;
                        let register_c = self.c;
                        add_hl_r_u16(self, register_b, register_c);
                    },
                    2 => {// LD DE, nn
                        ld_de_u16(self);
                    },
                    3 => {// ADD HL, DE
                        let register_d = self.d;
                        let register_e = self.e;
                        add_hl_r_u16(self, register_d, register_e);
                    },
                    4 => {// LD HL, nn
                        ld_hl_u16(self);
                    },
                    5 => {// ADD HL, HL
                        let register_h = self.h;
                        let register_l = self.l;
                        add_hl_r_u16(self, register_h, register_l);
                    },
                    6 => {// LD SP, nn
                        ld_sp_u16(self);
                    },
                    7 => {// ADD HL, SP
                        add_hl_sp(self);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => {// LD (BC), A
                        let register_b = self.b;
                        let register_c = self.c;
                        ld_m_r_u16_a(self, register_b, register_c);
                    },
                    1 => {// LD A, (BC)
                        let register_b = self.b;
                        let register_c = self.c;
                        ld_a_m_r_u16(self, register_b, register_c);
                    },
                    2 => {// LD (DE), A
                        let register_d = self.d;
                        let register_e = self.e;
                        ld_m_r_u16_a(self, register_d, register_e);
                    },
                    3 => {// LD A, (DE)
                        let register_d = self.d;
                        let register_e = self.e;
                        ld_a_m_r_u16(self, register_d, register_e);
                    },
                    4 => {// LD (HL+), A
                        ld_hl_inc_a(self);
                    },
                    5 => {// LD A, (HL+)
                        ld_a_hl_inc(self);
                    },
                    6 => {// LD (HL-), A
                        ld_hl_dec_a(self);
                    },
                    7 => {// LD A, (HL-)
                        ld_a_hl_dec(self);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => {// INC BC
                        let register_b = self.b;
                        let register_c = self.c;
                        (self.b, self.c) = inc_r_u16(self, register_b, register_c);
                    },
                    1 => {// DEC BC
                        let register_b = self.b;
                        let register_c = self.c;
                        (self.b, self.c) = dec_r_u16(self, register_b, register_c);
                    },
                    2 => {// INC DE
                        let register_d = self.d;
                        let register_e = self.e;
                        (self.d, self.e) = inc_r_u16(self, register_d, register_e);
                    },
                    3 => {// DEC DE
                        let register_d = self.d;
                        let register_e = self.e;
                        (self.d, self.e) = dec_r_u16(self, register_d, register_e);
                    },
                    4 => {// INC HL
                        let register_h = self.h;
                        let register_l = self.l;
                        (self.h, self.l) = inc_r_u16(self, register_h, register_l);
                    },
                    5 => {// DEC HL
                        let register_h = self.h;
                        let register_l = self.l;
                        (self.h, self.l) = dec_r_u16(self, register_h, register_l);
                    },
                    6 => {// INC SP
                        inc_sp(self);
                    },
                    7 => {// DEC SP
                        dec_sp(self);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => {// INC B
                        let register_b = self.b;
                        self.b = inc_r_u8(self, register_b);
                    },
                    1 => {// INC C
                        let register_c = self.c;
                        self.c = inc_r_u8(self, register_c);
                    },
                    2 => {// INC D
                        let register_d = self.d;
                        self.d = inc_r_u8(self, register_d);
                    },
                    3 => {// INC E
                        let register_e = self.e;
                        self.e = inc_r_u8(self, register_e);
                    },
                    4 => {// INC H
                        let register_h = self.h;
                        self.h = inc_r_u8(self, register_h);
                    },
                    5 => {// INC L
                        let register_l = self.l;
                        self.l = inc_r_u8(self, register_l);
                    },
                    6 => {// INC (HL)
                        inc_m_hl(self);
                    },
                    7 => {// INC A
                        let register_a = self.a;
                        self.a = inc_r_u8(self, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => {// DEC B
                        let register_b = self.b;
                        self.b = dec_r_u8(self, register_b);
                    },
                    1 => {// DEC C
                        let register_c = self.c;
                        self.c = dec_r_u8(self, register_c);
                    },
                    2 => {// DEC D
                        let register_d = self.d;
                        self.d = dec_r_u8(self, register_d);
                    },
                    3 => {// DEC E
                        let register_e = self.e;
                        self.e = dec_r_u8(self, register_e);
                    },
                    4 => {// DEC H
                        let register_h = self.h;
                        self.h = dec_r_u8(self, register_h);
                    },
                    5 => {// DEC L
                        let register_l = self.l;
                        self.l = dec_r_u8(self, register_l);
                    },
                    6 => {// DEC (HL)
                        dec_m_hl(self);
                    },
                    7 => {// DEC A
                        let register_a = self.a;
                        self.a = dec_r_u8(self, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// LD B, n
                        ld_b_u8(self);
                    },
                    1 => {// LD C, n
                        ld_c_u8(self);
                    },
                    2 => {// LD D, n
                        ld_d_u8(self);
                    },
                    3 => {// LD E, n
                        ld_e_u8(self);
                    },
                    4 => {// LD H, n
                        ld_h_u8(self);
                    },
                    5 => {// LD L, n
                        ld_l_u8(self);
                    },
                    6 => {// LD (HL), n
                        ld_m_hl_u8(self);
                    },
                    7 => {// LD A, n
                        ld_a_u8(self);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => {// RLCA
                        rlca(self);
                    },
                    1 => {// RRCA
                        rrca(self);
                    },
                    2 => {// RLA
                        rla(self);
                    },
                    3 => {// RRA
                        rra(self);
                    },
                    4 => {// DAA
                        daa(self);
                    },
                    5 => {// CPL
                        cpl(self);
                    },
                    6 => {// SCF
                        scf(self);
                    },
                    7 => {// CCF
                        ccf(self);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_01(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// LD B, B
                        ld_b_r_u8(self, register_b);
                    },
                    1 => {// LD C, B
                        ld_c_r_u8(self, register_b);
                    },
                    2 => {// LD D, B
                        ld_d_r_u8(self, register_b);
                    },
                    3 => {// LD E, B
                        ld_e_r_u8(self, register_b);
                    },
                    4 => {// LD H, B
                        ld_h_r_u8(self, register_b);
                    },
                    5 => {// LD L, B
                        ld_l_r_u8(self, register_b);
                    },
                    6 => {// LD (HL), B
                        ld_m_hl_r_u8(self, register_b);
                    },
                    7 => {// LD A, B
                        ld_a_r_u8(self, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// LD B, C
                        ld_b_r_u8(self, register_c);
                    },
                    1 => {// LD C, C
                        ld_c_r_u8(self, register_c);
                    },
                    2 => {// LD D, C
                        ld_d_r_u8(self, register_c);
                    },
                    3 => {// LD E, C
                        ld_e_r_u8(self, register_c);
                    },
                    4 => {// LD H, C
                        ld_h_r_u8(self, register_c);
                    },
                    5 => {// LD L, C
                        ld_l_r_u8(self, register_c);
                    },
                    6 => {// LD (HL), C
                        ld_m_hl_r_u8(self, register_c);
                    },
                    7 => {// LD A, C
                        ld_a_r_u8(self, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// LD B, D
                        ld_b_r_u8(self, register_d);
                    },
                    1 => {// LD C, D
                        ld_c_r_u8(self, register_d);
                    },
                    2 => {// LD D, D
                        ld_d_r_u8(self, register_d);
                    },
                    3 => {// LD E, D
                        ld_e_r_u8(self, register_d);
                    },
                    4 => {// LD H, D
                        ld_h_r_u8(self, register_d);
                    },
                    5 => {// LD L, D
                        ld_l_r_u8(self, register_d);
                    },
                    6 => {// LD (HL), D
                        ld_m_hl_r_u8(self, register_d);
                    },
                    7 => {// LD A, D
                        ld_a_r_u8(self, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// LD B, E
                        ld_b_r_u8(self, register_e);
                    },
                    1 => {// LD C, E
                        ld_c_r_u8(self, register_e);
                    },
                    2 => {// LD D, E
                        ld_d_r_u8(self, register_e);
                    },
                    3 => {// LD E, E
                        ld_e_r_u8(self, register_e);
                    },
                    4 => {// LD H, E
                        ld_h_r_u8(self, register_e);
                    },
                    5 => {// LD L, E
                        ld_l_r_u8(self, register_e);
                    },
                    6 => {// LD (HL), E
                        ld_m_hl_r_u8(self, register_e);
                    },
                    7 => {// LD A, E
                        ld_a_r_u8(self, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// LD B, H
                        ld_b_r_u8(self, register_h);
                    },
                    1 => {// LD C, H
                        ld_c_r_u8(self, register_h);
                    },
                    2 => {// LD D, H
                        ld_d_r_u8(self, register_h);
                    },
                    3 => {// LD E, H
                        ld_e_r_u8(self, register_h);
                    },
                    4 => {// LD H, H
                        ld_h_r_u8(self, register_h);
                    },
                    5 => {// LD L, H
                        ld_l_r_u8(self, register_h);
                    },
                    6 => {// LD (HL), H
                        ld_m_hl_r_u8(self, register_h);
                    },
                    7 => {// LD A, H
                        ld_a_r_u8(self, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// LD B, L
                        ld_b_r_u8(self, register_l);
                    },
                    1 => {// LD C, L
                        ld_c_r_u8(self, register_l);
                    },
                    2 => {// LD D, L
                        ld_d_r_u8(self, register_l);
                    },
                    3 => {// LD E, L
                        ld_e_r_u8(self, register_l);
                    },
                    4 => {// LD H, L
                        ld_h_r_u8(self, register_l);
                    },
                    5 => {// LD L, L
                        ld_l_r_u8(self, register_l);
                    },
                    6 => {// LD (HL), L
                        ld_m_hl_r_u8(self, register_l);
                    },
                    7 => {// LD A, L
                        ld_a_r_u8(self, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// LD B, (HL)
                        self.b = ld_r_u8_m_hl(self);
                    },
                    1 => {// LD C, (HL)
                        self.c = ld_r_u8_m_hl(self);
                    },
                    2 => {// LD D, (HL)
                        self.d = ld_r_u8_m_hl(self);
                    },
                    3 => {// LD E, (HL)
                        self.e = ld_r_u8_m_hl(self);
                    },
                    4 => {// LD H, (HL)
                        self.h = ld_r_u8_m_hl(self);
                    },
                    5 => {// LD L, (HL)
                        self.l = ld_r_u8_m_hl(self);
                    },
                    6 => {// HALT
                        halt(self);
                    },
                    7 => {// LD A, (HL)
                        self.a = ld_r_u8_m_hl(self);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// LD B, A
                        ld_b_r_u8(self, register_a);
                    },
                    1 => {// LD C, A
                        ld_c_r_u8(self, register_a);
                    },
                    2 => {// LD D, A
                        ld_d_r_u8(self, register_a);
                    },
                    3 => {// LD E, A
                        ld_e_r_u8(self, register_a);
                    },
                    4 => {// LD H, A
                        ld_h_r_u8(self, register_a);
                    },
                    5 => {// LD L, A
                        ld_l_r_u8(self, register_a);
                    },
                    6 => {// LD (HL), A
                        ld_m_hl_r_u8(self, register_a);
                    },
                    7 => {// LD A, A
                        ld_a_r_u8(self, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_10(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// ADD A, B
                        add_a_r_u8(self, register_b);
                    },
                    1 => {// ADC A, B
                        adc_a_r_u8(self, register_b);
                    },
                    2 => {// SUB A, B
                        sub_a_r_u8(self, register_b);
                    },
                    3 => {// SBC A, B
                        sbc_a_r_u8(self, register_b);
                    },
                    4 => {// AND A, B
                        and_a_r_u8(self, register_b);
                    },
                    5 => {// XOR A, B
                        xor_a_r_u8(self, register_b);
                    },
                    6 => {// OR A, B
                        or_a_r_u8(self, register_b);
                    },
                    7 => {// CP A, B
                        cp_a_r_u8(self, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// ADD A, C
                        add_a_r_u8(self, register_c);
                    },
                    1 => {// ADC A, C                       
                        adc_a_r_u8(self, register_c);
                    },
                    2 => {// SUB A, C                      
                        sub_a_r_u8(self, register_c);
                    },
                    3 => {// SBC A, C                        
                        sbc_a_r_u8(self, register_c);
                    },
                    4 => {// AND A, C                  
                        and_a_r_u8(self, register_c);
                    },
                    5 => {// XOR A, C                        
                        xor_a_r_u8(self, register_c);
                    },
                    6 => {// OR A, C                      
                        or_a_r_u8(self, register_c);
                    },
                    7 => {// CP A, C                       
                        cp_a_r_u8(self, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// ADD A, D
                        add_a_r_u8(self, register_d);
                    },
                    1 => {// ADC A, D
                        adc_a_r_u8(self, register_d);
                    },
                    2 => {// SUB A, D
                        sub_a_r_u8(self, register_d);
                    },
                    3 => {// SBC A, D
                        sbc_a_r_u8(self, register_d);
                    },
                    4 => {// AND A, D
                        and_a_r_u8(self, register_d);
                    },
                    5 => {// XOR A, D
                        xor_a_r_u8(self, register_d);
                    },
                    6 => {// OR A, D
                        or_a_r_u8(self, register_d);
                    },
                    7 => {// CP A, D
                        cp_a_r_u8(self, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// ADD A, E
                        add_a_r_u8(self, register_e);
                    },
                    1 => {// ADC A, E
                        adc_a_r_u8(self, register_e);
                    },
                    2 => {// SUB A, E
                        sub_a_r_u8(self, register_e);
                    },
                    3 => {// SBC A, E
                        sbc_a_r_u8(self, register_e);
                    },
                    4 => {// AND A, E
                        and_a_r_u8(self, register_e);
                    },
                    5 => {// XOR A, E
                        xor_a_r_u8(self, register_e);
                    },
                    6 => {// OR A, E
                        or_a_r_u8(self, register_e);
                    },
                    7 => {// CP A, E
                        cp_a_r_u8(self, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// ADD A, H
                        add_a_r_u8(self, register_h);
                    },
                    1 => {// ADC A, H
                        adc_a_r_u8(self, register_h);
                    },
                    2 => {// SUB A, H
                        sub_a_r_u8(self, register_h);
                    },
                    3 => {// SBC A, H
                        sbc_a_r_u8(self, register_h);
                    },
                    4 => {// AND A, H
                        and_a_r_u8(self, register_h);
                    },
                    5 => {// XOR A, H
                        xor_a_r_u8(self, register_h);
                    },
                    6 => {// OR A, H
                        or_a_r_u8(self, register_h);
                    },
                    7 => {// CP A, H
                        cp_a_r_u8(self, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// ADD A, L
                        add_a_r_u8(self, register_l);
                    },
                    1 => {// ADC A, L
                        adc_a_r_u8(self, register_l);
                    },
                    2 => {// SUB A, L
                        sub_a_r_u8(self, register_l);
                    },
                    3 => {// SBC A, L
                        sbc_a_r_u8(self, register_l);
                    },
                    4 => {// AND A, L
                        and_a_r_u8(self, register_l);
                    },
                    5 => {// XOR A, L
                        xor_a_r_u8(self, register_l);
                    },
                    6 => {// OR A, L
                        or_a_r_u8(self, register_l);
                    },
                    7 => {// CP A, L
                        cp_a_r_u8(self, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// ADD A, (HL)
                        add_a_m_hl(self);
                    },
                    1 => {// ADC A, (HL)
                        adc_a_m_hl(self);
                    },
                    2 => {// SUB A, (HL)
                        sub_a_m_hl(self);
                    },
                    3 => {// SBC A, (HL)
                        sbc_a_m_hl(self);
                    },
                    4 => {// AND A, (HL)
                        and_a_m_hl(self);
                    },
                    5 => {// XOR A, (HL)
                        xor_a_m_hl(self);
                    },
                    6 => {// OR A, (HL)
                        or_a_m_hl(self);
                    },
                    7 => {// CP A, (HL)
                        cp_a_m_hl(self);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// ADD A, A
                        add_a_r_u8(self, register_a);
                    },
                    1 => {// ADC A, A
                        adc_a_r_u8(self, register_a);
                    },
                    2 => {// SUB A, A
                        sub_a_r_u8(self, register_a);
                    },
                    3 => {// SBC A, A
                        sbc_a_r_u8(self, register_a);
                    },
                    4 => {// AND A, A
                        and_a_r_u8(self, register_a);
                    },
                    5 => {// XOR A, A
                        xor_a_r_u8(self, register_a);
                    },
                    6 => {// OR A, A
                        or_a_r_u8(self, register_a);
                    },
                    7 => {// CP A, A
                        cp_a_r_u8(self, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_11(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                match bits_5_4_3 {
                    0 => {// RET NZ
                        ret_nz(self);
                    },
                    1 => {// RET Z
                        ret_z(self);
                    },
                    2 => {// RET NC
                        ret_nc(self);
                    },
                    3 => {// RET C
                        ret_c(self);
                    },
                    4 => {// LD (0xFF00+n), A
                        ld_ff00_plus_u8_a(self);
                    },
                    5 => {// ADD SP, d
                        add_sp_i8(self);
                    },
                    6 => {// LD A, (0xFF00+n)
                        ld_a_ff00_plus_u8(self);
                    },
                    7 => {// LD HL, SP+d
                        ld_hl_sp_plus_i8(self);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => {// POP BC
                        pop_bc(self);
                    },
                    1 => {// RET
                        ret(self);
                    },
                    2 => {// POP DE
                        pop_de(self);
                    },
                    3 => {// RETI
                        reti(self);
                    },
                    4 => {// POP HL
                        pop_hl(self);
                    },
                    5 => {// JP HL
                        jp_hl(self);
                    },
                    6 => {// POP AF
                        pop_af(self);
                    },
                    7 => {// LD SP, HL
                        ld_sp_hl(self);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => {// JP NZ, nn
                        jp_nz_u16(self);
                    },
                    1 => {// JP Z, nn
                        jp_z_u16(self);
                    },
                    2 => {// JP NC, nn
                        jp_nc_u16(self);
                    },
                    3 => {// JP C, nn
                        jp_c_u16(self);
                    },
                    4 => {// LD (0xFF00+C), A
                        ld_ff00_plus_c_a(self);
                    },
                    5 => {// LD (nn), A
                        ld_u16_a(self);
                    },
                    6 => {// LD A, (0xFF00+C)
                        ld_a_ff00_plus_c(self);
                    },
                    7 => {// LD A, (nn)
                        ld_a_u16(self);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => {// JP nn
                        jp_u16(self);
                    },
                    6 => {// DI
                        di(self);
                    },
                    7 => {// EI
                        ei(self);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => {// CALL NZ, nn
                        call_nz_u16(self);
                    },
                    1 => {// CALL Z, nn
                        call_z_u16(self);
                    },
                    2 => {// CALL NC, nn
                        call_nc_u16(self);
                    },
                    3 => {// CALL C, nn
                        call_c_u16(self);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => {// PUSH BC
                        push_bc(self);
                    },
                    1 => {// CALL nn
                        call_u16(self);
                    },
                    2 => {// PUSH DE
                        push_de(self);
                    },
                    4 => {// PUSH HL
                        push_hl(self);
                    },
                    6 => {// PUSH AF
                        push_af(self);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// ADD A, n
                        add_a_u8(self);
                    },
                    1 => {// ADC A, n
                        adc_a_u8(self);
                    },
                    2 => {// SUB A, n
                        sub_a_u8(self);
                    },
                    3 => {// SBC A, n
                        sbc_a_u8(self);
                    },
                    4 => {// AND A, n
                        and_a_u8(self);
                    },
                    5 => {// XOR A, n
                        xor_a_u8(self);
                    },
                    6 => {// OR A, n
                        or_a_u8(self);
                    },
                    7 => {// CP A, n
                        cp_a_u8(self);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => {// RST 00h
                        rst(self, RST::Rst00H.address());
                    },
                    1 => {// RST 08h
                        rst(self, RST::Rst08H.address());
                    },
                    2 => {// RST 10h
                        rst(self, RST::Rst10H.address());
                    },
                    3 => {// RST 18h
                        rst(self, RST::Rst18H.address());
                    },
                    4 => {// RST 20h
                        rst(self, RST::Rst20H.address());
                    },
                    5 => {// RST 28h
                        rst(self, RST::Rst28H.address());
                    },
                    6 => {// RST 30h
                        rst(self, RST::Rst30H.address());
                    },
                    7 => {// RST 38h
                        rst(self, RST::Rst38H.address());
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_prefixed_instruction_starting_with_00(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// RLC B
                        self.b = rlc(self, register_b);
                    },
                    1 => {// RRC B
                        self.b = rrc(self, register_b);
                    },
                    2 => {// RL B
                        self.b = rl(self, register_b);
                    },
                    3 => {// RR B
                        self.b = rr(self, register_b);
                    },
                    4 => {// SLA B
                        self.b = sla(self, register_b);
                    },
                    5 => {// SRA B
                        self.b = sra(self, register_b);
                    },
                    6 => {// SWAP B
                        self.b = swap(self, register_b);
                    },
                    7 => {// SRL B
                        self.b = srl(self, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// RLC C
                        self.c = rlc(self, register_c);
                    },
                    1 => {// RRC C
                        self.c = rrc(self, register_c);
                    },
                    2 => {// RL C
                        self.c = rl(self, register_c);
                    },
                    3 => {// RR C
                        self.c = rr(self, register_c);
                    },
                    4 => {// SLA C
                        self.c = sla(self, register_c);
                    },
                    5 => {// SRA C
                        self.c = sra(self, register_c);
                    },
                    6 => {// SWAP C
                        self.c = swap(self, register_c);
                    },
                    7 => {// SRL C
                        self.c = srl(self, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// RLC D
                        self.d = rlc(self, register_d);
                    },
                    1 => {// RRC D
                        self.d = rrc(self, register_d);
                    },
                    2 => {// RL D
                        self.d = rl(self, register_d);
                    },
                    3 => {// RR D
                        self.d = rr(self, register_d);
                    },
                    4 => {// SLA D
                        self.d = sla(self, register_d);
                    },
                    5 => {// SRA D
                        self.d = sra(self, register_d);
                    },
                    6 => {// SWAP D
                        self.d = swap(self, register_d);
                    },
                    7 => {// SRL D
                        self.d = srl(self, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// RLC E
                        self.e = rlc(self, register_e);
                    },
                    1 => {// RRC E
                        self.e = rrc(self, register_e);
                    },
                    2 => {// RL E
                        self.e = rl(self, register_e);
                    },
                    3 => {// RR E
                        self.e = rr(self, register_e);
                    },
                    4 => {// SLA E
                        self.e = sla(self, register_e);
                    },
                    5 => {// SRA E
                        self.e = sra(self, register_e);
                    },
                    6 => {// SWAP E
                        self.e = swap(self, register_e);
                    },
                    7 => {// SRL E
                        self.e = srl(self, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// RLC H
                        self.h = rlc(self, register_h);
                    },
                    1 => {// RRC H
                        self.h = rrc(self, register_h);
                    },
                    2 => {// RL H
                        self.h = rl(self, register_h);
                    },
                    3 => {// RR H
                        self.h = rr(self, register_h);
                    },
                    4 => {// SLA H
                        self.h = sla(self, register_h);
                    },
                    5 => {// SRA H
                        self.h = sra(self, register_h);
                    },
                    6 => {// SWAP H
                        self.h = swap(self, register_h);
                    },
                    7 => {// SRL H
                        self.h = srl(self, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// RLC L
                        self.l = rlc(self, register_l);
                    },
                    1 => {// RRC L
                        self.l = rrc(self, register_l);
                    },
                    2 => {// RL L
                        self.l = rl(self, register_l);
                    },
                    3 => {// RR L
                        self.l = rr(self, register_l);
                    },
                    4 => {// SLA L
                        self.l = sla(self, register_l);
                    },
                    5 => {// SRA L
                        self.l = sra(self, register_l);
                    },
                    6 => {// SWAP L
                        self.l = swap(self, register_l);
                    },
                    7 => {// SRL L
                        self.l = srl(self, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// RLC (HL)
                        rlc_m_hl(self);
                    },
                    1 => {// RRC (HL)
                        rrc_m_hl(self);
                    },
                    2 => {// RL (HL)
                        rl_m_hl(self);
                    },
                    3 => {// RR (HL)
                        rr_m_hl(self);
                    },
                    4 => {// SLA (HL)
                        sla_m_hl(self);
                    },
                    5 => {// SRA (HL)
                        sra_m_hl(self);
                    },
                    6 => {// SWAP (HL)
                        swap_m_hl(self);
                    },
                    7 => {// SRL (HL)
                        srl_m_hl(self);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// RLC A
                        self.a = rlc(self, register_a);
                    },
                    1 => {// RRC A
                        self.a = rrc(self, register_a);
                    },
                    2 => {// RL A
                        self.a = rl(self, register_a);
                    },
                    3 => {// RR A
                        self.a = rr(self, register_a);
                    },
                    4 => {// SLA A
                        self.a = sla(self, register_a);
                    },
                    5 => {// SRA A
                        self.a = sra(self, register_a);
                    },
                    6 => {// SWAP A
                        self.a = swap(self, register_a);
                    },
                    7 => {// SRL A
                        self.a = srl(self, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_prefixed_instruction_starting_with_01(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// BIT 0, B
                        bit_n_r_u8(self, 0, register_b);
                    },
                    1 => {// BIT 1, B
                        bit_n_r_u8(self, 1, register_b);
                    },
                    2 => {// BIT 2, B
                        bit_n_r_u8(self, 2, register_b);
                    },
                    3 => {// BIT 3, B
                        bit_n_r_u8(self, 3, register_b);
                    },
                    4 => {// BIT 4, B
                        bit_n_r_u8(self, 4, register_b);
                    },
                    5 => {// BIT 5, B
                        bit_n_r_u8(self, 5, register_b);
                    },
                    6 => {// BIT 6, B
                        bit_n_r_u8(self, 6, register_b);
                    },
                    7 => {// BIT 7, B
                        bit_n_r_u8(self, 7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// BIT 0, C
                        bit_n_r_u8(self, 0, register_c);
                    },
                    1 => {// BIT 1, C
                        bit_n_r_u8(self, 1, register_c);
                    },
                    2 => {// BIT 2, C
                        bit_n_r_u8(self, 2, register_c);
                    },
                    3 => {// BIT 3, C
                        bit_n_r_u8(self, 3, register_c);
                    },
                    4 => {// BIT 4, C
                        bit_n_r_u8(self, 4, register_c);
                    },
                    5 => {// BIT 5, C
                        bit_n_r_u8(self, 5, register_c);
                    },
                    6 => {// BIT 6, C
                        bit_n_r_u8(self, 6, register_c);
                    },
                    7 => {// BIT 7, C
                        bit_n_r_u8(self, 7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// BIT 0, D
                        bit_n_r_u8(self, 0, register_d);
                    },
                    1 => {// BIT 1, D
                        bit_n_r_u8(self, 1, register_d);
                    },
                    2 => {// BIT 2, D
                        bit_n_r_u8(self, 2, register_d);
                    },
                    3 => {// BIT 3, D
                        bit_n_r_u8(self, 3, register_d);
                    },
                    4 => {// BIT 4, D
                        bit_n_r_u8(self, 4, register_d);
                    },
                    5 => {// BIT 5, D
                        bit_n_r_u8(self, 5, register_d);
                    },
                    6 => {// BIT 6, D
                        bit_n_r_u8(self, 6, register_d);
                    },
                    7 => {// BIT 7, D
                        bit_n_r_u8(self, 7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// BIT 0, E
                        bit_n_r_u8(self, 0, register_e);
                    },
                    1 => {// BIT 1, E
                        bit_n_r_u8(self, 1, register_e);
                    },
                    2 => {// BIT 2, E
                        bit_n_r_u8(self, 2, register_e);
                    },
                    3 => {// BIT 3, E
                        bit_n_r_u8(self, 3, register_e);
                    },
                    4 => {// BIT 4, E
                        bit_n_r_u8(self, 4, register_e);
                    },
                    5 => {// BIT 5, E
                        bit_n_r_u8(self, 5, register_e);
                    },
                    6 => {// BIT 6, E
                        bit_n_r_u8(self, 6, register_e);
                    },
                    7 => {// BIT 7, E
                        bit_n_r_u8(self, 7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// BIT 0, H
                        bit_n_r_u8(self, 0, register_h);
                    },
                    1 => {// BIT 1, H
                        bit_n_r_u8(self, 1, register_h);
                    },
                    2 => {// BIT 2, H
                        bit_n_r_u8(self, 2, register_h);
                    },
                    3 => {// BIT 3, H
                        bit_n_r_u8(self, 3, register_h);
                    },
                    4 => {// BIT 4, H
                        bit_n_r_u8(self, 4, register_h);
                    },
                    5 => {// BIT 5, H
                        bit_n_r_u8(self, 5, register_h);
                    },
                    6 => {// BIT 6, H
                        bit_n_r_u8(self, 6, register_h);
                    },
                    7 => {// BIT 7, H
                        bit_n_r_u8(self, 7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// BIT 0, L
                        bit_n_r_u8(self, 0, register_l);
                    },
                    1 => {// BIT 1, L
                        bit_n_r_u8(self, 1, register_l);
                    },
                    2 => {// BIT 2, L
                        bit_n_r_u8(self, 2, register_l);
                    },
                    3 => {// BIT 3, L
                        bit_n_r_u8(self, 3, register_l);
                    },
                    4 => {// BIT 4, L
                        bit_n_r_u8(self, 4, register_l);
                    },
                    5 => {// BIT 5, L
                        bit_n_r_u8(self, 5, register_l);
                    },
                    6 => {// BIT 6, L
                        bit_n_r_u8(self, 6, register_l);
                    },
                    7 => {// BIT 7, L
                        bit_n_r_u8(self, 7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// BIT 0, (HL)
                        bit_n_m_hl(self, 0);
                    },
                    1 => {// BIT 1, (HL)
                        bit_n_m_hl(self, 1);
                    },
                    2 => {// BIT 2, (HL)
                        bit_n_m_hl(self, 2);
                    },
                    3 => {// BIT 3, (HL)
                        bit_n_m_hl(self, 3);
                    },
                    4 => {// BIT 4, (HL)
                        bit_n_m_hl(self, 4);
                    },
                    5 => {// BIT 5, (HL)
                        bit_n_m_hl(self, 5);
                    },
                    6 => {// BIT 6, (HL)
                        bit_n_m_hl(self, 6);
                    },
                    7 => {// BIT 7, (HL)
                        bit_n_m_hl(self, 7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// BIT 0, A
                        bit_n_r_u8(self, 0, register_a);
                    },
                    1 => {// BIT 1, A
                        bit_n_r_u8(self, 1, register_a);
                    },
                    2 => {// BIT 2, A
                        bit_n_r_u8(self, 2, register_a);
                    },
                    3 => {// BIT 3, A
                        bit_n_r_u8(self, 3, register_a);
                    },
                    4 => {// BIT 4, A
                        bit_n_r_u8(self, 4, register_a);
                    },
                    5 => {// BIT 5, A
                        bit_n_r_u8(self, 5, register_a);
                    },
                    6 => {// BIT 6, A
                        bit_n_r_u8(self, 6, register_a);
                    },
                    7 => {// BIT 7, A
                        bit_n_r_u8(self, 7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    fn execute_prefixed_instruction_starting_with_10(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// RES 0, B
                        self.b = res_n_r_u8(self, 0, register_b);
                    },
                    1 => {// RES 1, B
                        self.b = res_n_r_u8(self, 1, register_b);
                    },
                    2 => {// RES 2, B
                        self.b = res_n_r_u8(self, 2, register_b);
                    },
                    3 => {// RES 3, B
                        self.b = res_n_r_u8(self, 3, register_b);
                    },
                    4 => {// RES 4, B
                        self.b = res_n_r_u8(self, 4, register_b);
                    },
                    5 => {// RES 5, B
                        self.b = res_n_r_u8(self, 5, register_b);
                    },
                    6 => {// RES 6, B
                        self.b = res_n_r_u8(self, 6, register_b);
                    },
                    7 => {// RES 7, B
                        self.b = res_n_r_u8(self, 7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// RES 0, C
                        self.c = res_n_r_u8(self, 0, register_c);
                    },
                    1 => {// RES 1, C
                        self.c = res_n_r_u8(self, 1, register_c);
                    },
                    2 => {// RES 2, C
                        self.c = res_n_r_u8(self, 2, register_c);
                    },
                    3 => {// RES 3, C
                        self.c = res_n_r_u8(self, 3, register_c);
                    },
                    4 => {// RES 4, C
                        self.c = res_n_r_u8(self, 4, register_c);
                    },
                    5 => {// RES 5, C
                        self.c = res_n_r_u8(self, 5, register_c);
                    },
                    6 => {// RES 6, C
                        self.c = res_n_r_u8(self, 6, register_c);
                    },
                    7 => {// RES 7, C
                        self.c = res_n_r_u8(self, 7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// RES 0, D
                        self.d = res_n_r_u8(self, 0, register_d);
                    },
                    1 => {// RES 1, D
                        self.d = res_n_r_u8(self, 1, register_d);
                    },
                    2 => {// RES 2, D
                        self.d = res_n_r_u8(self, 2, register_d);
                    },
                    3 => {// RES 3, D
                        self.d = res_n_r_u8(self, 3, register_d);
                    },
                    4 => {// RES 4, D
                        self.d = res_n_r_u8(self, 4, register_d);
                    },
                    5 => {// RES 5, D
                        self.d = res_n_r_u8(self, 5, register_d);
                    },
                    6 => {// RES 6, D
                        self.d = res_n_r_u8(self, 6, register_d);
                    },
                    7 => {// RES 7, D
                        self.d = res_n_r_u8(self, 7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// RES 0, E
                        self.e = res_n_r_u8(self, 0, register_e);
                    },
                    1 => {// RES 1, E
                        self.e = res_n_r_u8(self, 1, register_e);
                    },
                    2 => {// RES 2, E
                        self.e = res_n_r_u8(self, 2, register_e);
                    },
                    3 => {// RES 3, E
                        self.e = res_n_r_u8(self, 3, register_e);
                    },
                    4 => {// RES 4, E
                        self.e = res_n_r_u8(self, 4, register_e);
                    },
                    5 => {// RES 5, E
                        self.e = res_n_r_u8(self, 5, register_e);
                    },
                    6 => {// RES 6, E
                        self.e = res_n_r_u8(self, 6, register_e);
                    },
                    7 => {// RES 7, E
                        self.e = res_n_r_u8(self, 7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// RES 0, H
                        self.h = res_n_r_u8(self, 0, register_h);
                    },
                    1 => {// RES 1, H
                        self.h = res_n_r_u8(self, 1, register_h);
                    },
                    2 => {// RES 2, H
                        self.h = res_n_r_u8(self, 2, register_h);
                    },
                    3 => {// RES 3, H
                        self.h = res_n_r_u8(self, 3, register_h);
                    },
                    4 => {// RES 4, H
                        self.h = res_n_r_u8(self, 4, register_h);
                    },
                    5 => {// RES 5, H
                        self.h = res_n_r_u8(self, 5, register_h);
                    },
                    6 => {// RES 6, H
                        self.h = res_n_r_u8(self, 6, register_h);
                    },
                    7 => {// RES 7, H
                        self.h = res_n_r_u8(self, 7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// RES 0, L
                        self.l = res_n_r_u8(self, 0, register_l);
                    },
                    1 => {// RES 1, L
                        self.l = res_n_r_u8(self, 1, register_l);
                    },
                    2 => {// RES 2, L
                        self.l = res_n_r_u8(self, 2, register_l);
                    },
                    3 => {// RES 3, L
                        self.l = res_n_r_u8(self, 3, register_l);
                    },
                    4 => {// RES 4, L
                        self.l = res_n_r_u8(self, 4, register_l);
                    },
                    5 => {// RES 5, L
                        self.l = res_n_r_u8(self, 5, register_l);
                    },
                    6 => {// RES 6, L
                        self.l = res_n_r_u8(self, 6, register_l);
                    },
                    7 => {// RES 7, L
                        self.l = res_n_r_u8(self, 7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// RES 0, (HL)
                        res_n_m_hl(self, 0);
                    },
                    1 => {// RES 1, (HL)
                        res_n_m_hl(self, 1);
                    },
                    2 => {// RES 2, (HL)
                        res_n_m_hl(self, 2);
                    },
                    3 => {// RES 3, (HL)
                        res_n_m_hl(self, 3);
                    },
                    4 => {// RES 4, (HL)
                        res_n_m_hl(self, 4);
                    },
                    5 => {// RES 5, (HL)
                        res_n_m_hl(self, 5);
                    },
                    6 => {// RES 6, (HL)
                        res_n_m_hl(self, 6);
                    },
                    7 => {// RES 7, (HL)
                        res_n_m_hl(self, 7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// RES 0, A
                        self.a = res_n_r_u8(self, 0, register_a);
                    },
                    1 => {// RES 1, A
                        self.a = res_n_r_u8(self, 1, register_a);
                    },
                    2 => {// RES 2, A
                        self.a = res_n_r_u8(self, 2, register_a);
                    },
                    3 => {// RES 3, A
                        self.a = res_n_r_u8(self, 3, register_a);
                    },
                    4 => {// RES 4, A
                        self.a = res_n_r_u8(self, 4, register_a);
                    },
                    5 => {// RES 5, A
                        self.a = res_n_r_u8(self, 5, register_a);
                    },
                    6 => {// RES 6, A
                        self.a = res_n_r_u8(self, 6, register_a);
                    },
                    7 => {// RES 7, A
                        self.a = res_n_r_u8(self, 7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    fn execute_prefixed_instruction_starting_with_11(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        let bits_2_1_0 = opcode & 0b111;
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {// SET 0, B
                        self.b = set_n_r_u8(self, 0, register_b);
                    },
                    1 => {// SET 1, B
                        self.b = set_n_r_u8(self, 1, register_b);
                    },
                    2 => {// SET 2, B
                        self.b = set_n_r_u8(self, 2, register_b);
                    },
                    3 => {// SET 3, B
                        self.b = set_n_r_u8(self, 3, register_b);
                    },
                    4 => {// SET 4, B
                        self.b = set_n_r_u8(self, 4, register_b);
                    },
                    5 => {// SET 5, B
                        self.b = set_n_r_u8(self, 5, register_b);
                    },
                    6 => {// SET 6, B
                        self.b = set_n_r_u8(self, 6, register_b);
                    },
                    7 => {// SET 7, B
                        self.b = set_n_r_u8(self, 7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// SET 0, C
                        self.c = set_n_r_u8(self, 0, register_c);
                    },
                    1 => {// SET 1, C
                        self.c = set_n_r_u8(self, 1, register_c);
                    },
                    2 => {// SET 2, C
                        self.c = set_n_r_u8(self, 2, register_c);
                    },
                    3 => {// SET 3, C
                        self.c = set_n_r_u8(self, 3, register_c);
                    },
                    4 => {// SET 4, C
                        self.c = set_n_r_u8(self, 4, register_c);
                    },
                    5 => {// SET 5, C
                        self.c = set_n_r_u8(self, 5, register_c);
                    },
                    6 => {// SET 6, C
                        self.c = set_n_r_u8(self, 6, register_c);
                    },
                    7 => {// SET 7, C
                        self.c = set_n_r_u8(self, 7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// SET 0, D
                        self.d = set_n_r_u8(self, 0, register_d);
                    },
                    1 => {// SET 1, D
                        self.d = set_n_r_u8(self, 1, register_d);
                    },
                    2 => {// SET 2, D
                        self.d = set_n_r_u8(self, 2, register_d);
                    },
                    3 => {// SET 3, D
                        self.d = set_n_r_u8(self, 3, register_d);
                    },
                    4 => {// SET 4, D
                        self.d = set_n_r_u8(self, 4, register_d);
                    },
                    5 => {// SET 5, D
                        self.d = set_n_r_u8(self, 5, register_d);
                    },
                    6 => {// SET 6, D
                        self.d = set_n_r_u8(self, 6, register_d);
                    },
                    7 => {// SET 7, D
                        self.d = set_n_r_u8(self, 7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// SET 0, E
                        self.e = set_n_r_u8(self, 0, register_e);
                    },
                    1 => {// SET 1, E
                        self.e = set_n_r_u8(self, 1, register_e);
                    },
                    2 => {// SET 2, E
                        self.e = set_n_r_u8(self, 2, register_e);
                    },
                    3 => {// SET 3, E
                        self.e = set_n_r_u8(self, 3, register_e);
                    },
                    4 => {// SET 4, E
                        self.e = set_n_r_u8(self, 4, register_e);
                    },
                    5 => {// SET 5, E
                        self.e = set_n_r_u8(self, 5, register_e);
                    },
                    6 => {// SET 6, E
                        self.e = set_n_r_u8(self, 6, register_e);
                    },
                    7 => {// SET 7, E
                        self.e = set_n_r_u8(self, 7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// SET 0, H
                        self.h = set_n_r_u8(self, 0, register_h);
                    },
                    1 => {// SET 1, H
                        self.h = set_n_r_u8(self, 1, register_h);
                    },
                    2 => {// SET 2, H
                        self.h = set_n_r_u8(self, 2, register_h);
                    },
                    3 => {// SET 3, H
                        self.h = set_n_r_u8(self, 3, register_h);
                    },
                    4 => {// SET 4, H
                        self.h = set_n_r_u8(self, 4, register_h);
                    },
                    5 => {// SET 5, H
                        self.h = set_n_r_u8(self, 5, register_h);
                    },
                    6 => {// SET 6, H
                        self.h = set_n_r_u8(self, 6, register_h);
                    },
                    7 => {// SET 7, H
                        self.h = set_n_r_u8(self, 7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// SET 0, L
                        self.l = set_n_r_u8(self, 0, register_l);
                    },
                    1 => {// SET 1, L
                        self.l = set_n_r_u8(self, 1, register_l);
                    },
                    2 => {// SET 2, L
                        self.l = set_n_r_u8(self, 2, register_l);
                    },
                    3 => {// SET 3, L
                        self.l = set_n_r_u8(self, 3, register_l);
                    },
                    4 => {// SET 4, L
                        self.l = set_n_r_u8(self, 4, register_l);
                    },
                    5 => {// SET 5, L
                        self.l = set_n_r_u8(self, 5, register_l);
                    },
                    6 => {// SET 6, L
                        self.l = set_n_r_u8(self, 6, register_l);
                    },
                    7 => {// SET 7, L
                        self.l = set_n_r_u8(self, 7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// SET 0, (HL)
                        set_n_m_hl(self, 0);
                    },
                    1 => {// SET 1, (HL)
                        set_n_m_hl(self, 1);
                    },
                    2 => {// SET 2, (HL)
                        set_n_m_hl(self, 2);
                    },
                    3 => {// SET 3, (HL)
                        set_n_m_hl(self, 3);
                    },
                    4 => {// SET 4, (HL)
                        set_n_m_hl(self, 4);
                    },
                    5 => {// SET 5, (HL)
                        set_n_m_hl(self, 5);
                    },
                    6 => {// SET 6, (HL)
                        set_n_m_hl(self, 6);
                    },
                    7 => {// SET 7, (HL)
                        set_n_m_hl(self, 7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// SET 0, A
                        self.a = set_n_r_u8(self, 0, register_a);
                    },
                    1 => {// SET 1, A
                        self.a = set_n_r_u8(self, 1, register_a);
                    },
                    2 => {// SET 2, A
                        self.a = set_n_r_u8(self, 2, register_a);
                    },
                    3 => {// SET 3, A
                        self.a = set_n_r_u8(self, 3, register_a);
                    },
                    4 => {// SET 4, A
                        self.a = set_n_r_u8(self, 4, register_a);
                    },
                    5 => {// SET 5, A
                        self.a = set_n_r_u8(self, 5, register_a);
                    },
                    6 => {// SET 6, A
                        self.a = set_n_r_u8(self, 6, register_a);
                    },
                    7 => {// SET 7, A
                        self.a = set_n_r_u8(self, 7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
}
