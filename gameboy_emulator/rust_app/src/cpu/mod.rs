use crate::mmu::MMU;

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub mmu: MMU,
}

impl CPU {
    pub fn new(mmu: MMU) -> Self {
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
            mmu,
        }
    }

    pub fn execute(&mut self, opcode: u8) {
        self.execute_not_prefixed_instruction(opcode);
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

    fn inc_r(&mut self, register: u8) -> u8 {
        let half_carry = (register & 0x0F) == 0x0F;
        let result = register.wrapping_add(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(false), Some(half_carry));
        result
    }

    fn dec_r(&mut self, register: u8) -> u8 {
        let half_carry = (register & 0x0F) == 0x00;
        let result = register.wrapping_sub(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(true), Some(half_carry));
        result
    }

    fn add_a_r(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_add(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) + (register & 0xF) > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.a = result;
    }

    fn adc_a_r(&mut self, register: u8) {
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_add(register);
        let (result2, carry2) = result1.overflowing_add(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) + (register & 0xF) + carry_flag > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.a = result2;
    }

    fn sub_a_r(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_sub(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (register & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.a = result;
    }

    fn sbc_a_r(&mut self, register: u8) {
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_sub(register);
        let (result2, carry2) = result1.overflowing_sub(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) < ((register & 0xF) + carry_flag);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.a = result2;
    }

    fn and_a_r(&mut self, register: u8) {
        self.a &= register;
        let zero = self.a == 0;
        let half_carry = true;
        self.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
    }

    fn xor_a_r(&mut self, register: u8) {
        self.a ^= register;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
    }

    fn or_a_r(&mut self, register: u8) {
        self.a |= register;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
    }

    fn cp_a_r(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_sub(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (register & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    }

    #[inline(always)]
    fn ld_b_r(&mut self, register: u8) {
        self.b = register;
    }

    #[inline(always)]
    fn ld_c_r(&mut self, register: u8) {
        self.c = register;
    }

    #[inline(always)]
    fn ld_d_r(&mut self, register: u8) {
        self.d = register;
    }

    #[inline(always)]
    fn ld_e_r(&mut self, register: u8) {
        self.e = register;
    }

    #[inline(always)]
    fn ld_h_r(&mut self, register: u8) {
        self.h = register;
    }

    #[inline(always)]
    fn ld_l_r(&mut self, register: u8) {
        self.l = register;
    }

    #[inline(always)]
    fn ld_a_r(&mut self, register: u8) {
        self.a = register;
    }

    fn rlc(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = (register << 1) | carry_flag;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn rrc(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = (register >> 1) | (carry_flag << 7);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn rl(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = (register << 1) | ((self.f & 0x10) >> 4);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }
    
    fn rr(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = (register >> 1) | ((self.f & 0x10) << 3);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn sla(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = register << 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn sra(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = register >> 1 | register & 0x80;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn swap(&mut self, register: u8) -> u8 {
        let result = register >> 4 | register << 4;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        result
    }

    fn srl(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = register >> 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        result
    }

    fn bit_n_r(&mut self, bit: u8, register: u8) {
        let zero = (register & (1 << bit)) != 0;
        self.update_flags(Some(zero), None, Some(false), Some(true));
    }

    fn res_n_r(&mut self, bit: u8, register: u8) -> u8 {
        register & !(1 << bit)
    }

    fn set_n_r(&mut self, bit: u8, register: u8) -> u8 {
        register | 1 << bit
    }

    fn execute_not_prefixed_instruction(&mut self, opcode: u8) {
        let bits_7_6 = opcode >> 6;
    
        println!("Opcode: {:08b}", opcode);
        println!("Bits 7, 6: {:02b}", bits_7_6);
    
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
    
        println!("Opcode: {:08b}", opcode);
        println!("Bits 7, 6: {:02b}", bits_7_6);
    
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
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                match bits_5_4_3 {
                    0 => println!("NOP"),
                    1 => println!("LD nn, SP"),
                    2 => println!("STOP"),
                    3 => println!("JR d"),
                    4 => println!("JR NZ, d"),
                    5 => println!("JR Z, d"),
                    6 => println!("JR NC, d"),
                    7 => println!("JR C, d"),
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => println!("LD BC, nn"),
                    1 => println!("ADD HL, BC"),
                    2 => println!("LD DE, nn"),
                    3 => println!("ADD HL, DE"),
                    4 => println!("LD HL, nn"),
                    5 => println!("ADD HL, HL"),
                    6 => println!("LD SP, nn"),
                    7 => println!("ADD HL, SP"),
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => println!("LD (BC), A"),
                    1 => println!("LD A, (BC)"),
                    2 => println!("LD (DE), A"),
                    3 => println!("LD A, (DE)"),
                    4 => println!("LD (HL+), A"),
                    5 => println!("LD A, (HL+)"),
                    6 => println!("LD (HL-), A"),
                    7 => println!("LD A, (HL-)"),
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => println!("INC BC"),
                    1 => println!("DEC BC"),
                    2 => println!("INC DE"),
                    3 => println!("DEC DE"),
                    4 => println!("INC HL"),
                    5 => println!("DEC HL"),
                    6 => println!("INC SP"),
                    7 => println!("DEC SP"),
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => {
                        println!("INC B");
                        let register_b = self.b;
                        self.b = self.inc_r(register_b);
                    },
                    1 => {
                        println!("INC C");
                        let register_c = self.c;
                        self.c = self.inc_r(register_c);
                    },
                    2 => {
                        println!("INC D");
                        let register_d = self.d;
                        self.d = self.inc_r(register_d);
                    },
                    3 => {
                        println!("INC E");
                        let register_e = self.e;
                        self.e = self.inc_r(register_e);
                    },
                    4 => {
                        println!("INC H");
                        let register_h = self.h;
                        self.h = self.inc_r(register_h);
                    },
                    5 => {
                        println!("INC L");
                        let register_l = self.l;
                        self.l = self.inc_r(register_l);
                    },
                    6 => {
                        println!("INC (HL)");
                    },
                    7 => {
                        println!("INC A");
                        let register_a = self.a;
                        self.a = self.inc_r(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => {
                        println!("DEC B");
                        let register_b = self.b;
                        self.b = self.dec_r(register_b);
                    },
                    1 => {
                        println!("DEC C");
                        let register_c = self.c;
                        self.c = self.dec_r(register_c);
                    },
                    2 => {
                        println!("DEC D");
                        let register_d = self.d;
                        self.d = self.dec_r(register_d);
                    },
                    3 => {
                        println!("DEC E");
                        let register_e = self.e;
                        self.e = self.dec_r(register_e);
                    },
                    4 => {
                        println!("DEC H");
                        let register_h = self.h;
                        self.h = self.dec_r(register_h);
                    },
                    5 => {
                        println!("DEC L");
                        let register_l = self.l;
                        self.l = self.dec_r(register_l);
                    },
                    6 => {
                        println!("DEC (HL)");
                    },
                    7 => {
                        println!("DEC A");
                        let register_a = self.a;
                        self.a = self.dec_r(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => println!("LD B, n"),
                    1 => println!("LD C, n"),
                    2 => println!("LD D, n"),
                    3 => println!("LD E, n"),
                    4 => println!("LD H, n"),
                    5 => println!("LD L, n"),
                    6 => println!("LD (HL), n"),
                    7 => println!("LD A, n"),
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => println!("RLCA"),
                    1 => println!("RRCA"),
                    2 => println!("RLA"),
                    3 => println!("RRA"),
                    4 => println!("DAA"),
                    5 => println!("CPL"),
                    6 => println!("SCF"),
                    7 => println!("CCF"),
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_01(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, B");
                        self.ld_b_r(register_b);
                    },
                    1 => {
                        println!("LD C, B");
                        self.ld_c_r(register_b);
                    },
                    2 => {
                        println!("LD D, B");
                        self.ld_d_r(register_b);
                    },
                    3 => {
                        println!("LD E, B");
                        self.ld_e_r(register_b);
                    },
                    4 => {
                        println!("LD H, B");
                        self.ld_h_r(register_b);
                    },
                    5 => {
                        println!("LD L, B");
                        self.ld_l_r(register_b);
                    },
                    6 => {
                        println!("LD (HL), B");
                    },
                    7 => {
                        println!("LD A, B");
                        self.ld_a_r(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, C");
                        self.ld_b_r(register_c);
                    },
                    1 => {
                        println!("LD C, C");
                        self.ld_c_r(register_c);
                    },
                    2 => {
                        println!("LD D, C");
                        self.ld_d_r(register_c);
                    },
                    3 => {
                        println!("LD E, C");
                        self.ld_e_r(register_c);
                    },
                    4 => {
                        println!("LD H, C");
                        self.ld_h_r(register_c);
                    },
                    5 => {
                        println!("LD L, C");
                        self.ld_l_r(register_c);
                    },
                    6 => {
                        println!("LD (HL), C");
                    },
                    7 => {
                        println!("LD A, C");
                        self.ld_a_r(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, D");
                        self.ld_b_r(register_d);
                    },
                    1 => {
                        println!("LD C, D");
                        self.ld_c_r(register_d);
                    },
                    2 => {
                        println!("LD D, D");
                        self.ld_d_r(register_d);
                    },
                    3 => {
                        println!("LD E, D");
                        self.ld_e_r(register_d);
                    },
                    4 => {
                        println!("LD H, D");
                        self.ld_h_r(register_d);
                    },
                    5 => {
                        println!("LD L, D");
                        self.ld_l_r(register_d);
                    },
                    6 => {
                        println!("LD (HL), D");
                    },
                    7 => {
                        println!("LD A, D");
                        self.ld_a_r(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, E");
                        self.ld_b_r(register_e);
                    },
                    1 => {
                        println!("LD C, E");
                        self.ld_c_r(register_e);
                    },
                    2 => {
                        println!("LD D, E");
                        self.ld_d_r(register_e);
                    },
                    3 => {
                        println!("LD E, E");
                        self.ld_e_r(register_e);
                    },
                    4 => {
                        println!("LD H, E");
                        self.ld_h_r(register_e);
                    },
                    5 => {
                        println!("LD L, E");
                        self.ld_l_r(register_e);
                    },
                    6 => {
                        println!("LD (HL), E");
                    },
                    7 => {
                        println!("LD A, E");
                        self.ld_a_r(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, H");
                        self.ld_b_r(register_h);
                    },
                    1 => {
                        println!("LD C, H");
                        self.ld_c_r(register_h);
                    },
                    2 => {
                        println!("LD D, H");
                        self.ld_d_r(register_h);
                    },
                    3 => {
                        println!("LD E, H");
                        self.ld_e_r(register_h);
                    },
                    4 => {
                        println!("LD H, H");
                        self.ld_h_r(register_h);
                    },
                    5 => {
                        println!("LD L, H");
                        self.ld_l_r(register_h);
                    },
                    6 => {
                        println!("LD (HL), H");
                    },
                    7 => {
                        println!("LD A, H");
                        self.ld_a_r(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, L");
                        self.ld_b_r(register_l);
                    },
                    1 => {
                        println!("LD C, L");
                        self.ld_c_r(register_l);
                    },
                    2 => {
                        println!("LD D, L");
                        self.ld_d_r(register_l);
                    },
                    3 => {
                        println!("LD E, L");
                        self.ld_e_r(register_l);
                    },
                    4 => {
                        println!("LD H, L");
                        self.ld_h_r(register_l);
                    },
                    5 => {
                        println!("LD L, L");
                        self.ld_l_r(register_l);
                    },
                    6 => {
                        println!("LD (HL), L");
                    },
                    7 => {
                        println!("LD A, L");
                        self.ld_a_r(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, (HL)");
                    },
                    1 => {
                        println!("LD C, (HL)");
                    },
                    2 => {
                        println!("LD D, (HL)");
                    },
                    3 => {
                        println!("LD E, (HL)");
                    },
                    4 => {
                        println!("LD H, (HL)");
                    },
                    5 => {
                        println!("LD L, (HL)");
                    },
                    6 => {
                        println!("HALT");
                    },
                    7 => {
                        println!("LD A, (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("LD B, A");
                        self.ld_b_r(register_a);
                    },
                    1 => {
                        println!("LD C, A");
                        self.ld_c_r(register_a);
                    },
                    2 => {
                        println!("LD D, A");
                        self.ld_d_r(register_a);
                    },
                    3 => {
                        println!("LD E, A");
                        self.ld_e_r(register_a);
                    },
                    4 => {
                        println!("LD H, A");
                        self.ld_h_r(register_a);
                    },
                    5 => {
                        println!("LD L, A");
                        self.ld_l_r(register_a);
                    },
                    6 => {
                        println!("LD (HL), A");
                    },
                    7 => {
                        println!("LD A, A");
                        self.ld_a_r(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_10(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, B");
                        self.add_a_r(register_b);
                    },
                    1 => {
                        println!("ADC A, B");
                        self.adc_a_r(register_b);
                    },
                    2 => {
                        println!("SUB A, B");
                        self.sub_a_r(register_b);
                    },
                    3 => {
                        println!("SBC A, B");
                        self.sbc_a_r(register_b);
                    },
                    4 => {
                        println!("AND A, B");
                        self.and_a_r(register_b);
                    },
                    5 => {
                        println!("XOR A, B");
                        self.xor_a_r(register_b);
                    },
                    6 => {
                        println!("OR A, B");
                        self.or_a_r(register_b);
                    },
                    7 => {
                        println!("CP A, B");
                        self.cp_a_r(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, C");
                        self.add_a_r(register_c);
                    },
                    1 => {
                        println!("ADC A, C");                        
                        self.adc_a_r(register_c);
                    },
                    2 => {
                        println!("SUB A, C");                        
                        self.sub_a_r(register_c);
                    },
                    3 => {
                        println!("SBC A, C");                        
                        self.sbc_a_r(register_c);
                    },
                    4 => {
                        println!("AND A, C");                        
                        self.and_a_r(register_c);
                    },
                    5 => {
                        println!("XOR A, C");                        
                        self.xor_a_r(register_c);
                    },
                    6 => {
                        println!("OR A, C");                        
                        self.or_a_r(register_c);
                    },
                    7 => {
                        println!("CP A, C");                        
                        self.cp_a_r(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, D");
                        self.add_a_r(register_d);
                    },
                    1 => {
                        println!("ADC A, D");
                        self.adc_a_r(register_d);
                    },
                    2 => {
                        println!("SUB A, D");
                        self.sub_a_r(register_d);
                    },
                    3 => {
                        println!("SBC A, D");
                        self.sbc_a_r(register_d);
                    },
                    4 => {
                        println!("AND A, D");
                        self.and_a_r(register_d);
                    },
                    5 => {
                        println!("XOR A, D");
                        self.xor_a_r(register_d);
                    },
                    6 => {
                        println!("OR A, D");
                        self.or_a_r(register_d);
                    },
                    7 => {
                        println!("CP A, D");
                        self.cp_a_r(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, E");
                        self.add_a_r(register_e);
                    },
                    1 => {
                        println!("ADC A, E");
                        self.adc_a_r(register_e);
                    },
                    2 => {
                        println!("SUB A, E");
                        self.sub_a_r(register_e);
                    },
                    3 => {
                        println!("SBC A, E");
                        self.sbc_a_r(register_e);
                    },
                    4 => {
                        println!("AND A, E");
                        self.and_a_r(register_e);
                    },
                    5 => {
                        println!("XOR A, E");
                        self.xor_a_r(register_e);
                    },
                    6 => {
                        println!("OR A, E");
                        self.or_a_r(register_e);
                    },
                    7 => {
                        println!("CP A, E");
                        self.cp_a_r(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, H");
                        self.add_a_r(register_h);
                    },
                    1 => {
                        println!("ADC A, H");
                        self.adc_a_r(register_h);
                    },
                    2 => {
                        println!("SUB A, H");
                        self.sub_a_r(register_h);
                    },
                    3 => {
                        println!("SBC A, H");
                        self.sbc_a_r(register_h);
                    },
                    4 => {
                        println!("AND A, H");
                        self.and_a_r(register_h);
                    },
                    5 => {
                        println!("XOR A, H");
                        self.xor_a_r(register_h);
                    },
                    6 => {
                        println!("OR A, H");
                        self.or_a_r(register_h);
                    },
                    7 => {
                        println!("CP A, H");
                        self.cp_a_r(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, L");
                        self.add_a_r(register_l);
                    },
                    1 => {
                        println!("ADC A, L");
                        self.adc_a_r(register_l);
                    },
                    2 => {
                        println!("SUB A, L");
                        self.sub_a_r(register_l);
                    },
                    3 => {
                        println!("SBC A, L");
                        self.sbc_a_r(register_l);
                    },
                    4 => {
                        println!("AND A, L");
                        self.and_a_r(register_l);
                    },
                    5 => {
                        println!("XOR A, L");
                        self.xor_a_r(register_l);
                    },
                    6 => {
                        println!("OR A, L");
                        self.or_a_r(register_l);
                    },
                    7 => {
                        println!("CP A, L");
                        self.cp_a_r(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, (HL)");
                    },
                    1 => {
                        println!("ADC A, (HL)");
                    },
                    2 => {
                        println!("SUB A, (HL)");
                    },
                    3 => {
                        println!("SBC A, (HL)");
                    },
                    4 => {
                        println!("AND A, (HL)");
                    },
                    5 => {
                        println!("XOR A, (HL)");
                    },
                    6 => {
                        println!("OR A, (HL)");
                    },
                    7 => {
                        println!("CP A, (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("ADD A, A");
                        self.add_a_r(register_a);
                    },
                    1 => {
                        println!("ADC A, A");
                        self.adc_a_r(register_a);
                    },
                    2 => {
                        println!("SUB A, A");
                        self.sub_a_r(register_a);
                    },
                    3 => {
                        println!("SBC A, A");
                        self.sbc_a_r(register_a);
                    },
                    4 => {
                        println!("AND A, A");
                        self.and_a_r(register_a);
                    },
                    5 => {
                        println!("XOR A, A");
                        self.xor_a_r(register_a);
                    },
                    6 => {
                        println!("OR A, A");
                        self.or_a_r(register_a);
                    },
                    7 => {
                        println!("CP A, A");
                        self.cp_a_r(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_not_prefixed_instruction_starting_with_11(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                match bits_5_4_3 {
                    0 => println!("RET NZ"),
                    1 => println!("RET Z"),
                    2 => println!("RET NC"),
                    3 => println!("RET C"),
                    4 => println!("LD (0xFF00+n), A"),
                    5 => println!("ADD SP, d"),
                    6 => println!("LD A, (0xFF00+n)"),
                    7 => println!("LD HL, SP+d"),
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => println!("POP BC"),
                    1 => println!("RET"),
                    2 => println!("POP DE"),
                    3 => println!("RETI"),
                    4 => println!("POP HL"),
                    5 => println!("JP HL"),
                    6 => println!("POP AF"),
                    7 => println!("LD SP, HL"),
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => println!("JP NZ, nn"),
                    1 => println!("JP Z, nn"),
                    2 => println!("JP NC, nn"),
                    3 => println!("JP C, nn"),
                    4 => println!("LD (0xFF00+C), A"),
                    5 => println!("LD (nn), A"),
                    6 => println!("LD A, (0xFF00+C)"),
                    7 => println!("LD A, (nn)"),
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => println!("JP nn"),
                    1 => println!("PREFIX CB"),
                    6 => println!("DI"),
                    7 => println!("EI"),
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => println!("CALL NZ, nn"),
                    1 => println!("CALL Z, nn"),
                    2 => println!("CALL NC, nn"),
                    3 => println!("CALL C, nn"),
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => println!("PUSH BC"),
                    1 => println!("CALL nn"),
                    2 => println!("PUSH DE"),
                    4 => println!("PUSH HL"),
                    6 => println!("PUSH AF"),
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => println!("ADD A, n"),
                    1 => println!("ADC A, n"),
                    2 => println!("SUB A, n"),
                    3 => println!("SBC A, n"),
                    4 => println!("AND A, n"),
                    5 => println!("XOR A, n"),
                    6 => println!("OR A, n"),
                    7 => println!("CP A, n"),
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => println!("RST 00h"),
                    1 => println!("RST 08h"),
                    2 => println!("RST 10h"),
                    3 => println!("RST 18h"),
                    4 => println!("RST 20h"),
                    5 => println!("RST 28h"),
                    6 => println!("RST 30h"),
                    7 => println!("RST 38h"),
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_prefixed_instruction_starting_with_00(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC B");
                        self.b = self.rlc(register_b);
                    },
                    1 => {
                        println!("RRC B");
                        self.b = self.rrc(register_b);
                    },
                    2 => {
                        println!("RL B");
                        self.b = self.rl(register_b);
                    },
                    3 => {
                        println!("RR B");
                        self.b = self.rr(register_b);
                    },
                    4 => {
                        println!("SLA B");
                        self.b = self.sla(register_b);
                    },
                    5 => {
                        println!("SRA B");
                        self.b = self.sra(register_b);
                    },
                    6 => {
                        println!("SWAP B");
                        self.b = self.swap(register_b);
                    },
                    7 => {
                        println!("SRL B");
                        self.b = self.srl(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC C");
                        self.c = self.rlc(register_c);
                    },
                    1 => {
                        println!("RRC C");
                        self.c = self.rrc(register_c);
                    },
                    2 => {
                        println!("RL C");
                        self.c = self.rl(register_c);
                    },
                    3 => {
                        println!("RR C");
                        self.c = self.rr(register_c);
                    },
                    4 => {
                        println!("SLA C");
                        self.c = self.sla(register_c);
                    },
                    5 => {
                        println!("SRA C");
                        self.c = self.sra(register_c);
                    },
                    6 => {
                        println!("SWAP C");
                        self.c = self.swap(register_c);
                    },
                    7 => {
                        println!("SRL C");
                        self.c = self.srl(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC D");
                        self.d = self.rlc(register_d);
                    },
                    1 => {
                        println!("RRC D");
                        self.d = self.rrc(register_d);
                    },
                    2 => {
                        println!("RL D");
                        self.d = self.rl(register_d);
                    },
                    3 => {
                        println!("RR D");
                        self.d = self.rr(register_d);
                    },
                    4 => {
                        println!("SLA D");
                        self.d = self.sla(register_d);
                    },
                    5 => {
                        println!("SRA D");
                        self.d = self.sra(register_d);
                    },
                    6 => {
                        println!("SWAP D");
                        self.d = self.swap(register_d);
                    },
                    7 => {
                        println!("SRL D");
                        self.d = self.srl(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC E");
                        self.e = self.rlc(register_e);
                    },
                    1 => {
                        println!("RRC E");
                        self.e = self.rrc(register_e);
                    },
                    2 => {
                        println!("RL E");
                        self.e = self.rl(register_e);
                    },
                    3 => {
                        println!("RR E");
                        self.e = self.rr(register_e);
                    },
                    4 => {
                        println!("SLA E");
                        self.e = self.sla(register_e);
                    },
                    5 => {
                        println!("SRA E");
                        self.e = self.sra(register_e);
                    },
                    6 => {
                        println!("SWAP E");
                        self.e = self.swap(register_e);
                    },
                    7 => {
                        println!("SRL E");
                        self.e = self.srl(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC H");
                        self.h = self.rlc(register_h);
                    },
                    1 => {
                        println!("RRC H");
                        self.h = self.rrc(register_h);
                    },
                    2 => {
                        println!("RL H");
                        self.h = self.rl(register_h);
                    },
                    3 => {
                        println!("RR H");
                        self.h = self.rr(register_h);
                    },
                    4 => {
                        println!("SLA H");
                        self.h = self.sla(register_h);
                    },
                    5 => {
                        println!("SRA H");
                        self.h = self.sra(register_h);
                    },
                    6 => {
                        println!("SWAP H");
                        self.h = self.swap(register_h);
                    },
                    7 => {
                        println!("SRL H");
                        self.h = self.srl(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC L");
                        self.l = self.rlc(register_l);
                    },
                    1 => {
                        println!("RRC L");
                        self.l = self.rrc(register_l);
                    },
                    2 => {
                        println!("RL L");
                        self.l = self.rl(register_l);
                    },
                    3 => {
                        println!("RR L");
                        self.l = self.rr(register_l);
                    },
                    4 => {
                        println!("SLA L");
                        self.l = self.sla(register_l);
                    },
                    5 => {
                        println!("SRA L");
                        self.l = self.sra(register_l);
                    },
                    6 => {
                        println!("SWAP L");
                        self.l = self.swap(register_l);
                    },
                    7 => {
                        println!("SRL L");
                        self.l = self.srl(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("RLC (HL)");
                    },
                    1 => {
                        println!("RRC (HL)");
                    },
                    2 => {
                        println!("RL (HL)");
                    },
                    3 => {
                        println!("RR (HL)");
                    },
                    4 => {
                        println!("SLA (HL)");
                    },
                    5 => {
                        println!("SRA (HL)");
                    },
                    6 => {
                        println!("SWAP (HL)");
                    },
                    7 => {
                        println!("SRL (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("RLC A");
                        self.a = self.rlc(register_a);
                    },
                    1 => {
                        println!("RRC A");
                        self.a = self.rrc(register_a);
                    },
                    2 => {
                        println!("RL A");
                        self.a = self.rl(register_a);
                    },
                    3 => {
                        println!("RR A");
                        self.a = self.rr(register_a);
                    },
                    4 => {
                        println!("SLA A");
                        self.a = self.sla(register_a);
                    },
                    5 => {
                        println!("SRA A");
                        self.a = self.sra(register_a);
                    },
                    6 => {
                        println!("SWAP A");
                        self.a = self.swap(register_a);
                    },
                    7 => {
                        println!("SRL A");
                        self.a = self.srl(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    fn execute_prefixed_instruction_starting_with_01(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, B");
                        self.bit_n_r(0, register_b);
                    },
                    1 => {
                        println!("BIT 1, B");
                        self.bit_n_r(1, register_b);
                    },
                    2 => {
                        println!("BIT 2, B");
                        self.bit_n_r(2, register_b);
                    },
                    3 => {
                        println!("BIT 3, B");
                        self.bit_n_r(3, register_b);
                    },
                    4 => {
                        println!("BIT 4, B");
                        self.bit_n_r(4, register_b);
                    },
                    5 => {
                        println!("BIT 5, B");
                        self.bit_n_r(5, register_b);
                    },
                    6 => {
                        println!("BIT 6, B");
                        self.bit_n_r(6, register_b);
                    },
                    7 => {
                        println!("BIT 7, B");
                        self.bit_n_r(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, C");
                        self.bit_n_r(0, register_c);
                    },
                    1 => {
                        println!("BIT 1, C");
                        self.bit_n_r(1, register_c);
                    },
                    2 => {
                        println!("BIT 2, C");
                        self.bit_n_r(2, register_c);
                    },
                    3 => {
                        println!("BIT 3, C");
                        self.bit_n_r(3, register_c);
                    },
                    4 => {
                        println!("BIT 4, C");
                        self.bit_n_r(4, register_c);
                    },
                    5 => {
                        println!("BIT 5, C");
                        self.bit_n_r(5, register_c);
                    },
                    6 => {
                        println!("BIT 6, C");
                        self.bit_n_r(6, register_c);
                    },
                    7 => {
                        println!("BIT 7, C");
                        self.bit_n_r(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, D");
                        self.bit_n_r(0, register_d);
                    },
                    1 => {
                        println!("BIT 1, D");
                        self.bit_n_r(1, register_d);
                    },
                    2 => {
                        println!("BIT 2, D");
                        self.bit_n_r(2, register_d);
                    },
                    3 => {
                        println!("BIT 3, D");
                        self.bit_n_r(3, register_d);
                    },
                    4 => {
                        println!("BIT 4, D");
                        self.bit_n_r(4, register_d);
                    },
                    5 => {
                        println!("BIT 5, D");
                        self.bit_n_r(5, register_d);
                    },
                    6 => {
                        println!("BIT 6, D");
                        self.bit_n_r(6, register_d);
                    },
                    7 => {
                        println!("BIT 7, D");
                        self.bit_n_r(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, E");
                        self.bit_n_r(0, register_e);
                    },
                    1 => {
                        println!("BIT 1, E");
                        self.bit_n_r(1, register_e);
                    },
                    2 => {
                        println!("BIT 2, E");
                        self.bit_n_r(2, register_e);
                    },
                    3 => {
                        println!("BIT 3, E");
                        self.bit_n_r(3, register_e);
                    },
                    4 => {
                        println!("BIT 4, E");
                        self.bit_n_r(4, register_e);
                    },
                    5 => {
                        println!("BIT 5, E");
                        self.bit_n_r(5, register_e);
                    },
                    6 => {
                        println!("BIT 6, E");
                        self.bit_n_r(6, register_e);
                    },
                    7 => {
                        println!("BIT 7, E");
                        self.bit_n_r(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, H");
                        self.bit_n_r(0, register_h);
                    },
                    1 => {
                        println!("BIT 1, H");
                        self.bit_n_r(1, register_h);
                    },
                    2 => {
                        println!("BIT 2, H");
                        self.bit_n_r(2, register_h);
                    },
                    3 => {
                        println!("BIT 3, H");
                        self.bit_n_r(3, register_h);
                    },
                    4 => {
                        println!("BIT 4, H");
                        self.bit_n_r(4, register_h);
                    },
                    5 => {
                        println!("BIT 5, H");
                        self.bit_n_r(5, register_h);
                    },
                    6 => {
                        println!("BIT 6, H");
                        self.bit_n_r(6, register_h);
                    },
                    7 => {
                        println!("BIT 7, H");
                        self.bit_n_r(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, L");
                        self.bit_n_r(0, register_l);
                    },
                    1 => {
                        println!("BIT 1, L");
                        self.bit_n_r(1, register_l);
                    },
                    2 => {
                        println!("BIT 2, L");
                        self.bit_n_r(2, register_l);
                    },
                    3 => {
                        println!("BIT 3, L");
                        self.bit_n_r(3, register_l);
                    },
                    4 => {
                        println!("BIT 4, L");
                        self.bit_n_r(4, register_l);
                    },
                    5 => {
                        println!("BIT 5, L");
                        self.bit_n_r(5, register_l);
                    },
                    6 => {
                        println!("BIT 6, L");
                        self.bit_n_r(6, register_l);
                    },
                    7 => {
                        println!("BIT 7, L");
                        self.bit_n_r(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, (HL)");
                    },
                    1 => {
                        println!("BIT 1, (HL)");
                    },
                    2 => {
                        println!("BIT 2, (HL)");
                    },
                    3 => {
                        println!("BIT 3, (HL)");
                    },
                    4 => {
                        println!("BIT 4, (HL)");
                    },
                    5 => {
                        println!("BIT 5, (HL)");
                    },
                    6 => {
                        println!("BIT 6, (HL)");
                    },
                    7 => {
                        println!("BIT 7, (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("BIT 0, A");
                        self.bit_n_r(0, register_a);
                    },
                    1 => {
                        println!("BIT 1, A");
                        self.bit_n_r(1, register_a);
                    },
                    2 => {
                        println!("BIT 2, A");
                        self.bit_n_r(2, register_a);
                    },
                    3 => {
                        println!("BIT 3, A");
                        self.bit_n_r(3, register_a);
                    },
                    4 => {
                        println!("BIT 4, A");
                        self.bit_n_r(4, register_a);
                    },
                    5 => {
                        println!("BIT 5, A");
                        self.bit_n_r(5, register_a);
                    },
                    6 => {
                        println!("BIT 6, A");
                        self.bit_n_r(6, register_a);
                    },
                    7 => {
                        println!("BIT 7, A");
                        self.bit_n_r(7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    fn execute_prefixed_instruction_starting_with_10(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, B");
                        self.b = self.res_n_r(0, register_b);
                    },
                    1 => {
                        println!("RES 1, B");
                        self.b = self.res_n_r(1, register_b);
                    },
                    2 => {
                        println!("RES 2, B");
                        self.b = self.res_n_r(2, register_b);
                    },
                    3 => {
                        println!("RES 3, B");
                        self.b = self.res_n_r(3, register_b);
                    },
                    4 => {
                        println!("RES 4, B");
                        self.b = self.res_n_r(4, register_b);
                    },
                    5 => {
                        println!("RES 5, B");
                        self.b = self.res_n_r(5, register_b);
                    },
                    6 => {
                        println!("RES 6, B");
                        self.b = self.res_n_r(6, register_b);
                    },
                    7 => {
                        println!("RES 7, B");
                        self.b = self.res_n_r(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, C");
                        self.c = self.res_n_r(0, register_c);
                    },
                    1 => {
                        println!("RES 1, C");
                        self.c = self.res_n_r(1, register_c);
                    },
                    2 => {
                        println!("RES 2, C");
                        self.c = self.res_n_r(2, register_c);
                    },
                    3 => {
                        println!("RES 3, C");
                        self.c = self.res_n_r(3, register_c);
                    },
                    4 => {
                        println!("RES 4, C");
                        self.c = self.res_n_r(4, register_c);
                    },
                    5 => {
                        println!("RES 5, C");
                        self.c = self.res_n_r(5, register_c);
                    },
                    6 => {
                        println!("RES 6, C");
                        self.c = self.res_n_r(6, register_c);
                    },
                    7 => {
                        println!("RES 7, C");
                        self.c = self.res_n_r(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, D");
                        self.d = self.res_n_r(0, register_d);
                    },
                    1 => {
                        println!("RES 1, D");
                        self.d = self.res_n_r(1, register_d);
                    },
                    2 => {
                        println!("RES 2, D");
                        self.d = self.res_n_r(2, register_d);
                    },
                    3 => {
                        println!("RES 3, D");
                        self.d = self.res_n_r(3, register_d);
                    },
                    4 => {
                        println!("RES 4, D");
                        self.d = self.res_n_r(4, register_d);
                    },
                    5 => {
                        println!("RES 5, D");
                        self.d = self.res_n_r(5, register_d);
                    },
                    6 => {
                        println!("RES 6, D");
                        self.d = self.res_n_r(6, register_d);
                    },
                    7 => {
                        println!("RES 7, D");
                        self.d = self.res_n_r(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, E");
                        self.e = self.res_n_r(0, register_e);
                    },
                    1 => {
                        println!("RES 1, E");
                        self.e = self.res_n_r(1, register_e);
                    },
                    2 => {
                        println!("RES 2, E");
                        self.e = self.res_n_r(2, register_e);
                    },
                    3 => {
                        println!("RES 3, E");
                        self.e = self.res_n_r(3, register_e);
                    },
                    4 => {
                        println!("RES 4, E");
                        self.e = self.res_n_r(4, register_e);
                    },
                    5 => {
                        println!("RES 5, E");
                        self.e = self.res_n_r(5, register_e);
                    },
                    6 => {
                        println!("RES 6, E");
                        self.e = self.res_n_r(6, register_e);
                    },
                    7 => {
                        println!("RES 7, E");
                        self.e = self.res_n_r(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, H");
                        self.h = self.res_n_r(0, register_h);
                    },
                    1 => {
                        println!("RES 1, H");
                        self.h = self.res_n_r(1, register_h);
                    },
                    2 => {
                        println!("RES 2, H");
                        self.h = self.res_n_r(2, register_h);
                    },
                    3 => {
                        println!("RES 3, H");
                        self.h = self.res_n_r(3, register_h);
                    },
                    4 => {
                        println!("RES 4, H");
                        self.h = self.res_n_r(4, register_h);
                    },
                    5 => {
                        println!("RES 5, H");
                        self.h = self.res_n_r(5, register_h);
                    },
                    6 => {
                        println!("RES 6, H");
                        self.h = self.res_n_r(6, register_h);
                    },
                    7 => {
                        println!("RES 7, H");
                        self.h = self.res_n_r(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, L");
                        self.l = self.res_n_r(0, register_l);
                    },
                    1 => {
                        println!("RES 1, L");
                        self.l = self.res_n_r(1, register_l);
                    },
                    2 => {
                        println!("RES 2, L");
                        self.l = self.res_n_r(2, register_l);
                    },
                    3 => {
                        println!("RES 3, L");
                        self.l = self.res_n_r(3, register_l);
                    },
                    4 => {
                        println!("RES 4, L");
                        self.l = self.res_n_r(4, register_l);
                    },
                    5 => {
                        println!("RES 5, L");
                        self.l = self.res_n_r(5, register_l);
                    },
                    6 => {
                        println!("RES 6, L");
                        self.l = self.res_n_r(6, register_l);
                    },
                    7 => {
                        println!("RES 7, L");
                        self.l = self.res_n_r(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, (HL)");
                    },
                    1 => {
                        println!("RES 1, (HL)");
                    },
                    2 => {
                        println!("RES 2, (HL)");
                    },
                    3 => {
                        println!("RES 3, (HL)");
                    },
                    4 => {
                        println!("RES 4, (HL)");
                    },
                    5 => {
                        println!("RES 5, (HL)");
                    },
                    6 => {
                        println!("RES 6, (HL)");
                    },
                    7 => {
                        println!("RES 7, (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("RES 0, A");
                        self.a = self.res_n_r(0, register_a);
                    },
                    1 => {
                        println!("RES 1, A");
                        self.a = self.res_n_r(1, register_a);
                    },
                    2 => {
                        println!("RES 2, A");
                        self.a = self.res_n_r(2, register_a);
                    },
                    3 => {
                        println!("RES 3, A");
                        self.a = self.res_n_r(3, register_a);
                    },
                    4 => {
                        println!("RES 4, A");
                        self.a = self.res_n_r(4, register_a);
                    },
                    5 => {
                        println!("RES 5, A");
                        self.a = self.res_n_r(5, register_a);
                    },
                    6 => {
                        println!("RES 6, A");
                        self.a = self.res_n_r(6, register_a);
                    },
                    7 => {
                        println!("RES 7, A");
                        self.a = self.res_n_r(7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    fn execute_prefixed_instruction_starting_with_11(&mut self, opcode: u8) {
        let bits_5_4_3 = (opcode >> 3) & 0b111;
        println!("Bits 5, 4, 3: {:03b}", bits_5_4_3);
        let bits_2_1_0 = opcode & 0b111;
        println!("Bits 2, 1, 0: {:03b}", bits_2_1_0);
        match bits_2_1_0 {
            0 => {
                let register_b = self.b;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, B");
                        self.b = self.set_n_r(0, register_b);
                    },
                    1 => {
                        println!("SET 1, B");
                        self.b = self.set_n_r(1, register_b);
                    },
                    2 => {
                        println!("SET 2, B");
                        self.b = self.set_n_r(2, register_b);
                    },
                    3 => {
                        println!("SET 3, B");
                        self.b = self.set_n_r(3, register_b);
                    },
                    4 => {
                        println!("SET 4, B");
                        self.b = self.set_n_r(4, register_b);
                    },
                    5 => {
                        println!("SET 5, B");
                        self.b = self.set_n_r(5, register_b);
                    },
                    6 => {
                        println!("SET 6, B");
                        self.b = self.set_n_r(6, register_b);
                    },
                    7 => {
                        println!("SET 7, B");
                        self.b = self.set_n_r(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, C");
                        self.c = self.set_n_r(0, register_c);
                    },
                    1 => {
                        println!("SET 1, C");
                        self.c = self.set_n_r(1, register_c);
                    },
                    2 => {
                        println!("SET 2, C");
                        self.c = self.set_n_r(2, register_c);
                    },
                    3 => {
                        println!("SET 3, C");
                        self.c = self.set_n_r(3, register_c);
                    },
                    4 => {
                        println!("SET 4, C");
                        self.c = self.set_n_r(4, register_c);
                    },
                    5 => {
                        println!("SET 5, C");
                        self.c = self.set_n_r(5, register_c);
                    },
                    6 => {
                        println!("SET 6, C");
                        self.c = self.set_n_r(6, register_c);
                    },
                    7 => {
                        println!("SET 7, C");
                        self.c = self.set_n_r(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, D");
                        self.d = self.set_n_r(0, register_d);
                    },
                    1 => {
                        println!("SET 1, D");
                        self.d = self.set_n_r(1, register_d);
                    },
                    2 => {
                        println!("SET 2, D");
                        self.d = self.set_n_r(2, register_d);
                    },
                    3 => {
                        println!("SET 3, D");
                        self.d = self.set_n_r(3, register_d);
                    },
                    4 => {
                        println!("SET 4, D");
                        self.d = self.set_n_r(4, register_d);
                    },
                    5 => {
                        println!("SET 5, D");
                        self.d = self.set_n_r(5, register_d);
                    },
                    6 => {
                        println!("SET 6, D");
                        self.d = self.set_n_r(6, register_d);
                    },
                    7 => {
                        println!("SET 7, D");
                        self.d = self.set_n_r(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, E");
                        self.e = self.set_n_r(0, register_e);
                    },
                    1 => {
                        println!("SET 1, E");
                        self.e = self.set_n_r(1, register_e);
                    },
                    2 => {
                        println!("SET 2, E");
                        self.e = self.set_n_r(2, register_e);
                    },
                    3 => {
                        println!("SET 3, E");
                        self.e = self.set_n_r(3, register_e);
                    },
                    4 => {
                        println!("SET 4, E");
                        self.e = self.set_n_r(4, register_e);
                    },
                    5 => {
                        println!("SET 5, E");
                        self.e = self.set_n_r(5, register_e);
                    },
                    6 => {
                        println!("SET 6, E");
                        self.e = self.set_n_r(6, register_e);
                    },
                    7 => {
                        println!("SET 7, E");
                        self.e = self.set_n_r(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, H");
                        self.h = self.set_n_r(0, register_h);
                    },
                    1 => {
                        println!("SET 1, H");
                        self.h = self.set_n_r(1, register_h);
                    },
                    2 => {
                        println!("SET 2, H");
                        self.h = self.set_n_r(2, register_h);
                    },
                    3 => {
                        println!("SET 3, H");
                        self.h = self.set_n_r(3, register_h);
                    },
                    4 => {
                        println!("SET 4, H");
                        self.h = self.set_n_r(4, register_h);
                    },
                    5 => {
                        println!("SET 5, H");
                        self.h = self.set_n_r(5, register_h);
                    },
                    6 => {
                        println!("SET 6, H");
                        self.h = self.set_n_r(6, register_h);
                    },
                    7 => {
                        println!("SET 7, H");
                        self.h = self.set_n_r(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, L");
                        self.l = self.set_n_r(0, register_l);
                    },
                    1 => {
                        println!("SET 1, L");
                        self.l = self.set_n_r(1, register_l);
                    },
                    2 => {
                        println!("SET 2, L");
                        self.l = self.set_n_r(2, register_l);
                    },
                    3 => {
                        println!("SET 3, L");
                        self.l = self.set_n_r(3, register_l);
                    },
                    4 => {
                        println!("SET 4, L");
                        self.l = self.set_n_r(4, register_l);
                    },
                    5 => {
                        println!("SET 5, L");
                        self.l = self.set_n_r(5, register_l);
                    },
                    6 => {
                        println!("SET 6, L");
                        self.l = self.set_n_r(6, register_l);
                    },
                    7 => {
                        println!("SET 7, L");
                        self.l = self.set_n_r(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, (HL)");
                    },
                    1 => {
                        println!("SET 1, (HL)");
                    },
                    2 => {
                        println!("SET 2, (HL)");
                    },
                    3 => {
                        println!("SET 3, (HL)");
                    },
                    4 => {
                        println!("SET 4, (HL)");
                    },
                    5 => {
                        println!("SET 5, (HL)");
                    },
                    6 => {
                        println!("SET 6, (HL)");
                    },
                    7 => {
                        println!("SET 7, (HL)");
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {
                        println!("SET 0, A");
                        self.a = self.set_n_r(0, register_a);
                    },
                    1 => {
                        println!("SET 1, A");
                        self.a = self.set_n_r(1, register_a);
                    },
                    2 => {
                        println!("SET 2, A");
                        self.a = self.set_n_r(2, register_a);
                    },
                    3 => {
                        println!("SET 3, A");
                        self.a = self.set_n_r(3, register_a);
                    },
                    4 => {
                        println!("SET 4, A");
                        self.a = self.set_n_r(4, register_a);
                    },
                    5 => {
                        println!("SET 5, A");
                        self.a = self.set_n_r(5, register_a);
                    },
                    6 => {
                        println!("SET 6, A");
                        self.a = self.set_n_r(6, register_a);
                    },
                    7 => {
                        println!("SET 7, A");
                        self.a = self.set_n_r(7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
}
