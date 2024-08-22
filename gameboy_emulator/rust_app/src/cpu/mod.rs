use crate::mmu::MMU;
use crate::timer::TIMER;

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
    pub timer: TIMER,
}

impl CPU {
    pub fn new(mmu: MMU, timer: TIMER) -> Self {
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
            timer,
        }
    }

    pub fn tick(&mut self, opcode: u8) {
        if opcode != 0xCB {
            self.execute_not_prefixed_instruction(opcode);
        } else {
            self.pc += 1;
            let new_opcode = self.mmu.fetch_instruction(self.pc);
            self.execute_prefixed_instruction(new_opcode);
        }
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

    fn inc_r_u8(&mut self, register: u8) -> u8 {
        let half_carry = (register & 0x0F) == 0x0F;
        let result = register.wrapping_add(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(false), Some(half_carry));
        self.timer.add_cycles(4);
        self.pc += 1;
        result
    }

    fn dec_r_u8(&mut self, register: u8) -> u8 {
        let half_carry = (register & 0x0F) == 0x00;
        let result = register.wrapping_sub(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(true), Some(half_carry));
        self.timer.add_cycles(4);
        self.pc += 1;
        result
    }

    fn inc_r_u16(&mut self, register1: u8, register2: u8) -> (u8, u8) {
        let combined = ((register1 as u16) << 8) | (register2 as u16);
        let result = combined.wrapping_add(1);
        let new_register1 = (result >> 8) as u8;
        let new_register2 = (result & 0xFF) as u8;
        self.pc += 1;
        self.timer.add_cycles(8);
        (new_register1, new_register2)
    }

    fn dec_r_u16(&mut self, register1: u8, register2: u8) -> (u8, u8) {
        let combined = ((register1 as u16) << 8) | (register2 as u16);
        let result = combined.wrapping_sub(1);
        let new_register1 = (result >> 8) as u8;
        let new_register2 = (result & 0xFF) as u8;
        self.pc += 1;
        self.timer.add_cycles(8);
        (new_register1, new_register2)
    }

    fn inc_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn dec_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn inc_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let half_carry = (value & 0x0F) == 0x0F;
        let result = value.wrapping_add(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(false), Some(half_carry));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn dec_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let half_carry = (value & 0x0F) == 0x00;
        let result = value.wrapping_sub(1);
        let zero = result == 0;
        self.update_flags(Some(zero), None, Some(true), Some(half_carry));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(12);
        self.pc += 1;
    }
    
    fn add_a_r_u8(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_add(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) + (register & 0xF) > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(4);
        self.a = result;
        self.pc += 1;
    }

    fn add_hl_r_u16(&mut self, register1: u8, register2: u8) {
        let combined1 = ((self.h as u16) << 8) | (self.l as u16);
        let combined2 = ((register1 as u16) << 8) | (register2 as u16);
        let (result, carry) = combined1.overflowing_add(combined2);
        let half_carry = (combined1 & 0x0FFF) + (combined2 & 0x0FFF) > 0x0FFF;
        self.update_flags(None, Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.h = (result >> 8) as u8;
        self.l = (result & 0xFF) as u8;
        self.pc += 1;
    }

    fn add_hl_sp(&mut self) {
        let combined = ((self.h as u16) << 8) | (self.l as u16);
        let (result, carry) = combined.overflowing_add(self.sp);
        let half_carry = (combined & 0x0FFF) + (self.sp & 0x0FFF) > 0x0FFF;
        self.update_flags(None, Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.h = (result >> 8) as u8;
        self.l = (result & 0xFF) as u8;
        self.pc += 1;
    }

    fn add_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let (result, carry) = self.a.overflowing_add(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) + (value & 0xF) > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result;
        self.pc += 1;
    }

    fn add_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let (result, carry) = self.a.overflowing_add(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) + (value & 0xF) > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result;
        self.pc += 2;
    }

    fn add_sp_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        let (result, carry) = self.sp.overflowing_add(signed_value);
        let half_carry = (self.sp & 0x0F) + (signed_value & 0x0F) > 0x0F;
        self.update_flags(Some(false), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(16);
        self.sp = result;
        self.pc += 2;
    }

    fn adc_a_r_u8(&mut self, register: u8) {
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_add(register);
        let (result2, carry2) = result1.overflowing_add(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) + (register & 0xF) + carry_flag > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(4);
        self.a = result2;
        self.pc += 1;
    }

    fn adc_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_add(value);
        let (result2, carry2) = result1.overflowing_add(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) + (value & 0xF) + carry_flag > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result2;
        self.pc += 1;
    }

    fn adc_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_add(value);
        let (result2, carry2) = result1.overflowing_add(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) + (value & 0xF) + carry_flag > 0xF;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result2;
        self.pc += 2;
    }

    fn sub_a_r_u8(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_sub(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (register & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(4);
        self.a = result;
        self.pc += 1;
    }

    fn sub_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let (result, carry) = self.a.overflowing_sub(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (value & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result;
        self.pc += 1;
    }

    fn sub_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let (result, carry) = self.a.overflowing_sub(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (value & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result;
        self.pc += 2;
    }

    fn sbc_a_r_u8(&mut self, register: u8) {
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_sub(register);
        let (result2, carry2) = result1.overflowing_sub(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) < ((register & 0xF) + carry_flag);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(4);
        self.a = result2;
        self.pc += 1;
    }

    fn sbc_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_sub(value);
        let (result2, carry2) = result1.overflowing_sub(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) < ((value & 0xF) + carry_flag);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result2;
        self.pc += 1;
    }

    fn sbc_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let carry_flag = if self.f & 0x10 != 0 { 1 } else { 0 };
        let (result1, carry1) = self.a.overflowing_sub(value);
        let (result2, carry2) = result1.overflowing_sub(carry_flag);
        let carry = carry1 || carry2;
        let zero = result2 == 0;
        let half_carry = (self.a & 0xF) < ((value & 0xF) + carry_flag);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.a = result2;
        self.pc += 2;
    }

    fn and_a_r_u8(&mut self, register: u8) {
        self.a &= register;
        let zero = self.a == 0;
        let half_carry = true;
        self.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn and_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        self.a &= value;
        let zero = self.a == 0;
        let half_carry = true;
        self.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn and_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        self.a &= value;
        let zero = self.a == 0;
        let half_carry = true;
        self.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn xor_a_r_u8(&mut self, register: u8) {
        self.a ^= register;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn xor_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        self.a ^= value;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn xor_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        self.a ^= value;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn or_a_r_u8(&mut self, register: u8) {
        self.a |= register;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn or_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        self.a |= value;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn or_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        self.a |= value;
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn cp_a_r_u8(&mut self, register: u8) {
        let (result, carry) = self.a.overflowing_sub(register);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (register & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn cp_a_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let (result, carry) = self.a.overflowing_sub(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (value & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn cp_a_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let (result, carry) = self.a.overflowing_sub(value);
        let zero = result == 0;
        let half_carry = (self.a & 0xF) < (value & 0xF);
        self.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_b_r_u8(&mut self, register: u8) {
        self.b = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_c_r_u8(&mut self, register: u8) {
        self.c = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_d_r_u8(&mut self, register: u8) {
        self.d = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_e_r_u8(&mut self, register: u8) {
        self.e = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_h_r_u8(&mut self, register: u8) {
        self.h = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_l_r_u8(&mut self, register: u8) {
        self.l = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_a_r_u8(&mut self, register: u8) {
        self.a = register;
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ld_b_u8(&mut self) {
        self.b = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_c_u8(&mut self) {
        self.c = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_d_u8(&mut self) {
        self.d = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_e_u8(&mut self) {
        self.e = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_h_u8(&mut self) {
        self.h = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_l_u8(&mut self) {
        self.l = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_a_u8(&mut self) {
        self.a = self.mmu.fetch_u8(self.pc + 1);
        self.timer.add_cycles(8);
        self.pc += 2;
    }

    fn ld_m_hl_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1);
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.mmu.write_byte(address, value);
        self.timer.add_cycles(12);
        self.pc += 2;
    }

    fn ld_bc_u16(&mut self) {
        self.c = self.mmu.fetch_u8(self.pc + 1);
        self.b = self.mmu.fetch_u8(self.pc + 2);
        self.timer.add_cycles(12);
        self.pc += 3;
    }

    fn ld_de_u16(&mut self) {
        self.e = self.mmu.fetch_u8(self.pc + 1);
        self.d = self.mmu.fetch_u8(self.pc + 2);
        self.timer.add_cycles(12);
        self.pc += 3;
    }

    fn ld_hl_u16(&mut self) {
        self.l = self.mmu.fetch_u8(self.pc + 1);
        self.h = self.mmu.fetch_u8(self.pc + 2);
        self.timer.add_cycles(12);
        self.pc += 3;
    }

    fn ld_sp_u16(&mut self) {
        self.sp = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        self.timer.add_cycles(12);
        self.pc += 3;
    }

    fn ld_m_r_u16_a(&mut self, register1: u8, register2: u8) {
        let address = ((register1 as u16) << 8) | (register2 as u16);
        self.mmu.write_byte(address, self.a);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_a_m_r_u16(&mut self, register1: u8, register2: u8) {
        let address = ((register1 as u16) << 8) | (register2 as u16);
        self.a = self.mmu.read_byte(address);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_a_hl_inc(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.a = self.mmu.read_byte(address);
        let incremented_address = address.wrapping_add(1);
        self.h = (incremented_address >> 8) as u8;
        self.l = (incremented_address & 0xFF) as u8;
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_hl_inc_a(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.mmu.write_byte(address, self.a);
        let incremented_address = address.wrapping_add(1);
        self.h = (incremented_address >> 8) as u8;
        self.l = (incremented_address & 0xFF) as u8;
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_a_hl_dec(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.a = self.mmu.read_byte(address);
        let decremented_address = address.wrapping_sub(1);
        self.h = (decremented_address >> 8) as u8;
        self.l = (decremented_address & 0xFF) as u8;
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_hl_dec_a(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.mmu.write_byte(address, self.a);
        let decremented_address = address.wrapping_sub(1);
        self.h = (decremented_address >> 8) as u8;
        self.l = (decremented_address & 0xFF) as u8;
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_m_hl_r_u8(&mut self, register: u8) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        self.mmu.write_byte(address, register);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_r_u8_m_hl(&mut self) -> u8 {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        self.timer.add_cycles(8);
        self.pc += 1;  
        value
    }

    fn ld_m_u16_sp(&mut self) {
        let address = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        let lower_byte = (self.sp & 0xFF) as u8;
        let upper_byte = (self.sp >> 8) as u8;
        self.mmu.write_byte(address, lower_byte);
        self.mmu.write_byte(address + 1, upper_byte);
        self.timer.add_cycles(20);
        self.pc += 3;
    }

    fn ld_sp_hl(&mut self) {
        self.sp = ((self.h as u16) << 8) | (self.l as u16);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_ff00_plus_u8_a(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1) as u16;
        let address = 0xFF00 + value;
        self.mmu.write_byte(address, self.a);
        self.timer.add_cycles(12);
        self.pc += 2;
    }

    fn ld_a_ff00_plus_u8(&mut self) {
        let value = self.mmu.fetch_u8(self.pc + 1) as u16;
        let address = 0xFF00 + value;
        self.a = self.mmu.read_byte(address);
        self.timer.add_cycles(12);
        self.pc += 2;
    }

    fn ld_ff00_plus_c_a(&mut self) {
        let value = self.c as u16;
        let address = 0xFF00 + value;
        self.mmu.write_byte(address, self.a);
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn ld_a_ff00_plus_c(&mut self) {
        let value = self.c as u16;
        let address = 0xFF00 + value;
        self.a = self.mmu.read_byte(address);
        self.timer.add_cycles(8);
        self.pc += 1;
    }    

    fn ld_a_u16(&mut self) {
        let address = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        self.a = self.mmu.read_byte(address);
        self.timer.add_cycles(16);
        self.pc += 3;
    }

    fn ld_u16_a(&mut self) {
        let address = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        self.mmu.write_byte(address, self.a);
        self.timer.add_cycles(16);
        self.pc += 3;
    }

    fn ld_hl_sp_plus_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        let (result, carry) = self.sp.overflowing_add(signed_value);
        let half_carry = (self.sp & 0x0F) + (signed_value & 0x0F) > 0x0F;
        self.update_flags(Some(false), Some(carry), Some(false), Some(half_carry));
        self.h = (result >> 8) as u8;
        self.l = (result & 0xFF) as u8;
        self.timer.add_cycles(12);
        self.pc += 2;      
    }

    fn jr_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        let (result, _) = self.pc.overflowing_add(signed_value);
        self.timer.add_cycles(12);
        self.pc = result;
    }

    fn jr_nz_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        if (self.f & 0x80) >> 7 != 1 {
            let (result, _) = self.pc.overflowing_add(signed_value);
            self.timer.add_cycles(12);
            self.pc = result;
        } else {
            self.timer.add_cycles(8);
            self.pc += 2;
        }
    }

    fn jr_z_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        if (self.f & 0x80) >> 7 == 1 {
            let (result, _) = self.pc.overflowing_add(signed_value);
            self.timer.add_cycles(12);
            self.pc = result;
        } else {
            self.timer.add_cycles(8);
            self.pc += 2;
        }
    }

    fn jr_nc_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;      
        if (self.f & 0x10) >> 4 != 1 {
            let (result, _) = self.pc.overflowing_add(signed_value);
            self.timer.add_cycles(12);
            self.pc = result;
        } else {
            self.timer.add_cycles(8);
            self.pc += 2;
        }
    }

    fn jr_c_i8(&mut self) {
        let offset = self.mmu.fetch_i8(self.pc + 1);
        let signed_value = offset as i16 as u16;
        if (self.f & 0x10) >> 4 == 1 {
            let (result, _) = self.pc.overflowing_add(signed_value);
            self.timer.add_cycles(12);
            self.pc = result;
        } else {
            self.timer.add_cycles(8);
            self.pc += 2;
        }
    }

    fn jp_hl(&mut self) {
        self.timer.add_cycles(4);
        self.pc = ((self.h as u16) << 8) | (self.l as u16);
    }

    fn jp_u16(&mut self) {
        self.timer.add_cycles(16);
        self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
    }
 
    fn jp_nz_u16(&mut self) {
        if (self.f & 0x80) >> 7 != 1 {
            self.timer.add_cycles(16);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
            self.pc += 3;
        }        
    }

    fn jp_z_u16(&mut self) {
        if (self.f & 0x80) >> 7 == 1 {
            self.timer.add_cycles(16);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
            self.pc += 3;
        }        
    }

    fn jp_nc_u16(&mut self) {
        if (self.f & 0x10) >> 4 != 1 {
            self.timer.add_cycles(16);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
            self.pc += 3;
        }        
    }

    fn jp_c_u16(&mut self) {
        if (self.f & 0x10) >> 4 == 1 {
            self.timer.add_cycles(16);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
            self.pc += 3;
        }        
    }

    fn pop_bc(&mut self) {
        self.b = self.mmu.fetch_u8(self.sp + 1);
        self.c = self.mmu.fetch_u8(self.sp);
        self.sp += 2;
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn pop_de(&mut self) {
        self.d = self.mmu.fetch_u8(self.sp + 1);
        self.e = self.mmu.fetch_u8(self.sp);
        self.sp += 2;
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn pop_hl(&mut self) {
        self.h = self.mmu.fetch_u8(self.sp + 1);
        self.l = self.mmu.fetch_u8(self.sp);
        self.sp += 2;
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn pop_af(&mut self) {
        self.a = self.mmu.fetch_u8(self.sp + 1);
        let f = self.mmu.fetch_u8(self.sp);
        self.update_flags(
            Some(f & 0x80 != 0),
            Some(f & 0x10 != 0),
            Some(f & 0x40 != 0),
            Some(f & 0x20 != 0),
        );
        self.sp += 2;
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn push_bc(&mut self) {
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, self.b);
        self.mmu.write_byte(self.sp, self.c);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn push_de(&mut self) {
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, self.d);
        self.mmu.write_byte(self.sp, self.e);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn push_hl(&mut self) {
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, self.h);
        self.mmu.write_byte(self.sp, self.l);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn push_af(&mut self) {
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, self.a);
        self.mmu.write_byte(self.sp, self.f);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn call_u16(&mut self) {
        self.pc += 3;
        let lower_byte = (self.pc & 0xFF) as u8;
        let upper_byte = (self.pc >> 8) as u8;
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, upper_byte);
        self.mmu.write_byte(self.sp, lower_byte);
        self.timer.add_cycles(24);
        self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
    }

    fn call_nz_u16(&mut self) {
        self.pc += 3;
        if (self.f & 0x80) >> 7 != 1 {
            let lower_byte = (self.pc & 0xFF) as u8;
            let upper_byte = (self.pc >> 8) as u8;
            self.sp -= 2;
            self.mmu.write_byte(self.sp + 1, upper_byte);
            self.mmu.write_byte(self.sp, lower_byte);
            self.timer.add_cycles(24);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
        }  
    }

    fn call_z_u16(&mut self) {
        self.pc += 3;
        if (self.f & 0x80) >> 7 == 1 {
            let lower_byte = (self.pc & 0xFF) as u8;
            let upper_byte = (self.pc >> 8) as u8;
            self.sp -= 2;
            self.mmu.write_byte(self.sp + 1, upper_byte);
            self.mmu.write_byte(self.sp, lower_byte);
            self.timer.add_cycles(24);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
        }     
    }

    fn call_nc_u16(&mut self) {
        self.pc += 3;
        if (self.f & 0x10) >> 4 != 1 {
            let lower_byte = (self.pc & 0xFF) as u8;
            let upper_byte = (self.pc >> 8) as u8;
            self.sp -= 2;
            self.mmu.write_byte(self.sp + 1, upper_byte);
            self.mmu.write_byte(self.sp, lower_byte);
            self.timer.add_cycles(24);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
        }            
    }

    fn call_c_u16(&mut self) {
        self.pc += 3;
        if (self.f & 0x10) >> 4 == 1 {
            let lower_byte = (self.pc & 0xFF) as u8;
            let upper_byte = (self.pc >> 8) as u8;
            self.sp -= 2;
            self.mmu.write_byte(self.sp + 1, upper_byte);
            self.mmu.write_byte(self.sp, lower_byte);
            self.timer.add_cycles(24);
            self.pc = ((self.mmu.fetch_u8(self.pc + 2) as u16) << 8) | (self.mmu.fetch_u8(self.pc + 1) as u16);
        } else {
            self.timer.add_cycles(12);
        }     
    }

    fn ret(&mut self) {
        self.timer.add_cycles(16);
        self.pc = ((self.mmu.fetch_u8(self.sp + 1) as u16) << 8) | (self.mmu.fetch_u8(self.sp) as u16); 
        self.sp += 2;
    }

    fn ret_nz(&mut self) {
        if (self.f & 0x80) >> 7 != 1 {
            self.timer.add_cycles(20);
            self.pc = ((self.mmu.fetch_u8(self.sp + 1) as u16) << 8) | (self.mmu.fetch_u8(self.sp) as u16); 
            self.sp += 2;
        } else {
            self.timer.add_cycles(8);
            self.pc += 1;
        }         
    }

    fn ret_z(&mut self) {
        if (self.f & 0x80) >> 7 == 1 {
            self.timer.add_cycles(20);
            self.pc = ((self.mmu.fetch_u8(self.sp + 1) as u16) << 8) | (self.mmu.fetch_u8(self.sp) as u16); 
            self.sp += 2;
        } else {
            self.timer.add_cycles(8);
            self.pc += 1;
        }        
    }

    fn ret_nc(&mut self) {
        if (self.f & 0x10) >> 4 != 1 {
            self.timer.add_cycles(20);
            self.pc = ((self.mmu.fetch_u8(self.sp + 1) as u16) << 8) | (self.mmu.fetch_u8(self.sp) as u16); 
            self.sp += 2;
        } else {
            self.timer.add_cycles(8);
            self.pc += 1;
        }         
    }

    fn ret_c(&mut self) {
        if (self.f & 0x10) >> 4 == 1 {
            self.timer.add_cycles(20);
            self.pc = ((self.mmu.fetch_u8(self.sp + 1) as u16) << 8) | (self.mmu.fetch_u8(self.sp) as u16); 
            self.sp += 2;
        } else {
            self.timer.add_cycles(8);
            self.pc += 1;
        }         
    }

    fn rst(&mut self, address: u16) {
        self.pc += 1;
        let lower_byte = (self.pc & 0xFF) as u8;
        let upper_byte = (self.pc >> 8) as u8;
        self.sp -= 2;
        self.mmu.write_byte(self.sp + 1, upper_byte);
        self.mmu.write_byte(self.sp, lower_byte);
        self.timer.add_cycles(16);
        self.pc = address;
    }

    fn rlca(&mut self) {
        let carry_flag = self.a >> 7;
        self.a = (self.a << 1) | carry_flag;
        let carry = carry_flag != 0;
        self.update_flags(Some(false), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn rrca(&mut self) {
        let carry_flag = self.a & 1;
        self.a = (self.a >> 1) | (carry_flag << 7);
        let carry = carry_flag != 0;
        self.update_flags(Some(false), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn rla(&mut self) {
        let carry_flag = self.a >> 7;
        self.a = (self.a << 1) | ((self.f & 0x10) >> 4);
        let carry = carry_flag != 0;
        self.update_flags(Some(false), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }   

    fn rra(&mut self) {
        let carry_flag = self.a & 1;
        self.a = (self.a >> 1) | ((self.f & 0x10) << 3);
        let carry = carry_flag != 0;
        self.update_flags(Some(false), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn daa(&mut self) {
        let mut correction = 0;
        let mut carry = false;
        if self.f & 0x20 != 0 || (self.a & 0x0F) > 0x09 {
            correction |= 0x06;
        }
        if self.f & 0x10 != 0 || (self.a >> 4) > 0x09 {
            correction |= 0x60;
            carry = true;
        }
        if self.f & 0x40 != 0 {
            self.a = self.a.wrapping_sub(correction);
        } else {
            self.a = self.a.wrapping_add(correction);
        }
        let zero = self.a == 0;
        self.update_flags(Some(zero), Some(carry), None, Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }
    
    fn cpl(&mut self) {
        self.a = !self.a;
        self.update_flags(None, None, Some(true), Some(true));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn scf(&mut self) {
        self.update_flags(None, Some(true), Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn ccf(&mut self) {
        self.f ^= 0x10;
        self.update_flags(None, None, Some(false), Some(false));
        self.timer.add_cycles(4);
        self.pc += 1;
    }

    fn rlc(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = (register << 1) | carry_flag;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn rlc_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value >> 7;
        let result = (value << 1) | carry_flag;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn rrc(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = (register >> 1) | (carry_flag << 7);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn rrc_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value & 1;
        let result = (value >> 1) | (carry_flag << 7);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn rl(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = (register << 1) | ((self.f & 0x10) >> 4);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn rl_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value >> 7;
        let result = (value << 1) | ((self.f & 0x10) >> 4);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }
    
    fn rr(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = (register >> 1) | ((self.f & 0x10) << 3);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn rr_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value & 1;
        let result = (value >> 1) | ((self.f & 0x10) << 3);
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn sla(&mut self, register: u8) -> u8 {
        let carry_flag = register >> 7;
        let result = register << 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn sla_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value >> 7;
        let result = value << 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn sra(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = register >> 1 | register & 0x80;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn sra_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value & 1;
        let result = value >> 1 | value & 0x80;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn swap(&mut self, register: u8) -> u8 {
        let result = register >> 4 | register << 4;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn swap_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let result = value >> 4 | value << 4;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(false), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn srl(&mut self, register: u8) -> u8 {
        let carry_flag = register & 1;
        let result = register >> 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.timer.add_cycles(8);
        self.pc += 1;
        result
    }

    fn srl_m_hl(&mut self) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let carry_flag = value & 1;
        let result = value >> 1;
        let carry = carry_flag != 0;
        let zero = result == 0;
        self.update_flags(Some(zero), Some(carry), Some(false), Some(false));
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn bit_n_r_u8(&mut self, bit: u8, register: u8) {
        let zero = (register & (1 << bit)) != 0;
        self.update_flags(Some(zero), None, Some(false), Some(true));
        self.timer.add_cycles(8);
        self.pc += 1;
    }

    fn bit_n_m_hl(&mut self, bit: u8) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let zero = (value & (1 << bit)) != 0;
        self.update_flags(Some(zero), None, Some(false), Some(true));
        self.timer.add_cycles(12);
        self.pc += 1;
    }

    fn res_n_r_u8(&mut self, bit: u8, register: u8) -> u8 {
        self.pc += 1;
        self.timer.add_cycles(8);
        register & !(1 << bit)
    }

    fn res_n_m_hl(&mut self, bit: u8) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);
        let result = value & !(1 << bit);
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
    }

    fn set_n_r_u8(&mut self, bit: u8, register: u8) -> u8 {
        self.pc += 1;
        self.timer.add_cycles(8);
        register | 1 << bit
    }

    fn set_n_m_hl(&mut self, bit: u8) {
        let address = ((self.h as u16) << 8) | (self.l as u16);
        let value = self.mmu.read_byte(address);     
        let result = value | 1 << bit;
        self.mmu.write_byte(address, result);
        self.timer.add_cycles(16);
        self.pc += 1;
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
                    0 => {
                        println!("NOP");
                    },
                    1 => {// LD (nn), SP
                        self.ld_m_u16_sp();
                    },
                    2 => {
                        println!("STOP");
                    },
                    3 => {// JR d
                        self.jr_i8();
                    },
                    4 => {// JR NZ, d
                        self.jr_nz_i8();
                    },
                    5 => {// JR Z, d
                        self.jr_z_i8();
                    },
                    6 => {// JR NC, d
                        self.jr_nc_i8();
                    },
                    7 => {// JR C, d
                        self.jr_c_i8();
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => {// LD BC, nn
                        self.ld_bc_u16();
                    },
                    1 => {// ADD HL, BC
                        let register_b = self.b;
                        let register_c = self.c;
                        self.add_hl_r_u16(register_b, register_c);
                    },
                    2 => {// LD DE, nn
                        self.ld_de_u16();
                    },
                    3 => {// ADD HL, DE
                        let register_d = self.d;
                        let register_e = self.e;
                        self.add_hl_r_u16(register_d, register_e);
                    },
                    4 => {// LD HL, nn
                        self.ld_hl_u16();
                    },
                    5 => {// ADD HL, HL
                        let register_h = self.h;
                        let register_l = self.l;
                        self.add_hl_r_u16(register_h, register_l);
                    },
                    6 => {// LD SP, nn
                        self.ld_sp_u16();
                    },
                    7 => {// ADD HL, SP
                        self.add_hl_sp();
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => {// LD (BC), A
                        let register_b = self.b;
                        let register_c = self.c;
                        self.ld_m_r_u16_a(register_b, register_c);
                    },
                    1 => {// LD A, (BC)
                        let register_b = self.b;
                        let register_c = self.c;
                        self.ld_a_m_r_u16(register_b, register_c);
                    },
                    2 => {// LD (DE), A
                        let register_d = self.d;
                        let register_e = self.e;
                        self.ld_m_r_u16_a(register_d, register_e);
                    },
                    3 => {// LD A, (DE)
                        let register_d = self.d;
                        let register_e = self.e;
                        self.ld_a_m_r_u16(register_d, register_e);
                    },
                    4 => {// LD (HL+), A
                        self.ld_hl_inc_a();
                    },
                    5 => {// LD A, (HL+)
                        self.ld_a_hl_inc();
                    },
                    6 => {// LD (HL-), A
                        self.ld_hl_dec_a();
                    },
                    7 => {// LD A, (HL-)
                        self.ld_a_hl_dec();
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => {// INC BC
                        let register_b = self.b;
                        let register_c = self.c;
                        (self.b, self.c) = self.inc_r_u16(register_b, register_c);
                    },
                    1 => {// DEC BC
                        let register_b = self.b;
                        let register_c = self.c;
                        (self.b, self.c) = self.dec_r_u16(register_b, register_c);
                    },
                    2 => {// INC DE
                        let register_d = self.d;
                        let register_e = self.e;
                        (self.d, self.e) = self.inc_r_u16(register_d, register_e);
                    },
                    3 => {// DEC DE
                        let register_d = self.d;
                        let register_e = self.e;
                        (self.d, self.e) = self.dec_r_u16(register_d, register_e);
                    },
                    4 => {// INC HL
                        let register_h = self.h;
                        let register_l = self.l;
                        (self.h, self.l) = self.inc_r_u16(register_h, register_l);
                    },
                    5 => {// DEC HL
                        let register_h = self.h;
                        let register_l = self.l;
                        (self.h, self.l) = self.dec_r_u16(register_h, register_l);
                    },
                    6 => {// INC SP
                        self.inc_sp();
                    },
                    7 => {// DEC SP
                        self.dec_sp();
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => {// INC B
                        let register_b = self.b;
                        self.b = self.inc_r_u8(register_b);
                    },
                    1 => {// INC C
                        let register_c = self.c;
                        self.c = self.inc_r_u8(register_c);
                    },
                    2 => {// INC D
                        let register_d = self.d;
                        self.d = self.inc_r_u8(register_d);
                    },
                    3 => {// INC E
                        let register_e = self.e;
                        self.e = self.inc_r_u8(register_e);
                    },
                    4 => {// INC H
                        let register_h = self.h;
                        self.h = self.inc_r_u8(register_h);
                    },
                    5 => {// INC L
                        let register_l = self.l;
                        self.l = self.inc_r_u8(register_l);
                    },
                    6 => {// INC (HL)
                        self.inc_m_hl();
                    },
                    7 => {// INC A
                        let register_a = self.a;
                        self.a = self.inc_r_u8(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => {// DEC B
                        let register_b = self.b;
                        self.b = self.dec_r_u8(register_b);
                    },
                    1 => {// DEC C
                        let register_c = self.c;
                        self.c = self.dec_r_u8(register_c);
                    },
                    2 => {// DEC D
                        let register_d = self.d;
                        self.d = self.dec_r_u8(register_d);
                    },
                    3 => {// DEC E
                        let register_e = self.e;
                        self.e = self.dec_r_u8(register_e);
                    },
                    4 => {// DEC H
                        let register_h = self.h;
                        self.h = self.dec_r_u8(register_h);
                    },
                    5 => {// DEC L
                        let register_l = self.l;
                        self.l = self.dec_r_u8(register_l);
                    },
                    6 => {// DEC (HL)
                        self.dec_m_hl();
                    },
                    7 => {// DEC A
                        let register_a = self.a;
                        self.a = self.dec_r_u8(register_a);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// LD B, n
                        self.ld_b_u8();
                    },
                    1 => {// LD C, n
                        self.ld_c_u8();
                    },
                    2 => {// LD D, n
                        self.ld_d_u8();
                    },
                    3 => {// LD E, n
                        self.ld_e_u8();
                    },
                    4 => {// LD H, n
                        self.ld_h_u8();
                    },
                    5 => {// LD L, n
                        self.ld_l_u8();
                    },
                    6 => {// LD (HL), n
                        self.ld_m_hl_u8();
                    },
                    7 => {// LD A, n
                        self.ld_a_u8();
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => {// RLCA
                        self.rlca();
                    },
                    1 => {// RRCA
                        self.rrca();
                    },
                    2 => {// RLA
                        self.rla();
                    },
                    3 => {// RRA
                        self.rra();
                    },
                    4 => {// DAA
                        self.daa();
                    },
                    5 => {// CPL
                        self.cpl();
                    },
                    6 => {// SCF
                        self.scf();
                    },
                    7 => {// CCF
                        self.ccf();
                    },
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
                    0 => {// LD B, B
                        self.ld_b_r_u8(register_b);
                    },
                    1 => {// LD C, B
                        self.ld_c_r_u8(register_b);
                    },
                    2 => {// LD D, B
                        self.ld_d_r_u8(register_b);
                    },
                    3 => {// LD E, B
                        self.ld_e_r_u8(register_b);
                    },
                    4 => {// LD H, B
                        self.ld_h_r_u8(register_b);
                    },
                    5 => {// LD L, B
                        self.ld_l_r_u8(register_b);
                    },
                    6 => {// LD (HL), B
                        self.ld_m_hl_r_u8(register_b);
                    },
                    7 => {// LD A, B
                        self.ld_a_r_u8(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// LD B, C
                        self.ld_b_r_u8(register_c);
                    },
                    1 => {// LD C, C
                        self.ld_c_r_u8(register_c);
                    },
                    2 => {// LD D, C
                        self.ld_d_r_u8(register_c);
                    },
                    3 => {// LD E, C
                        self.ld_e_r_u8(register_c);
                    },
                    4 => {// LD H, C
                        self.ld_h_r_u8(register_c);
                    },
                    5 => {// LD L, C
                        self.ld_l_r_u8(register_c);
                    },
                    6 => {// LD (HL), C
                        self.ld_m_hl_r_u8(register_c);
                    },
                    7 => {// LD A, C
                        self.ld_a_r_u8(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// LD B, D
                        self.ld_b_r_u8(register_d);
                    },
                    1 => {// LD C, D
                        self.ld_c_r_u8(register_d);
                    },
                    2 => {// LD D, D
                        self.ld_d_r_u8(register_d);
                    },
                    3 => {// LD E, D
                        self.ld_e_r_u8(register_d);
                    },
                    4 => {// LD H, D
                        self.ld_h_r_u8(register_d);
                    },
                    5 => {// LD L, D
                        self.ld_l_r_u8(register_d);
                    },
                    6 => {// LD (HL), D
                        self.ld_m_hl_r_u8(register_d);
                    },
                    7 => {// LD A, D
                        self.ld_a_r_u8(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// LD B, E
                        self.ld_b_r_u8(register_e);
                    },
                    1 => {// LD C, E
                        self.ld_c_r_u8(register_e);
                    },
                    2 => {// LD D, E
                        self.ld_d_r_u8(register_e);
                    },
                    3 => {// LD E, E
                        self.ld_e_r_u8(register_e);
                    },
                    4 => {// LD H, E
                        self.ld_h_r_u8(register_e);
                    },
                    5 => {// LD L, E
                        self.ld_l_r_u8(register_e);
                    },
                    6 => {// LD (HL), E
                        self.ld_m_hl_r_u8(register_e);
                    },
                    7 => {// LD A, E
                        self.ld_a_r_u8(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// LD B, H
                        self.ld_b_r_u8(register_h);
                    },
                    1 => {// LD C, H
                        self.ld_c_r_u8(register_h);
                    },
                    2 => {// LD D, H
                        self.ld_d_r_u8(register_h);
                    },
                    3 => {// LD E, H
                        self.ld_e_r_u8(register_h);
                    },
                    4 => {// LD H, H
                        self.ld_h_r_u8(register_h);
                    },
                    5 => {// LD L, H
                        self.ld_l_r_u8(register_h);
                    },
                    6 => {// LD (HL), H
                        self.ld_m_hl_r_u8(register_h);
                    },
                    7 => {// LD A, H
                        self.ld_a_r_u8(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// LD B, L
                        self.ld_b_r_u8(register_l);
                    },
                    1 => {// LD C, L
                        self.ld_c_r_u8(register_l);
                    },
                    2 => {// LD D, L
                        self.ld_d_r_u8(register_l);
                    },
                    3 => {// LD E, L
                        self.ld_e_r_u8(register_l);
                    },
                    4 => {// LD H, L
                        self.ld_h_r_u8(register_l);
                    },
                    5 => {// LD L, L
                        self.ld_l_r_u8(register_l);
                    },
                    6 => {// LD (HL), L
                        self.ld_m_hl_r_u8(register_l);
                    },
                    7 => {// LD A, L
                        self.ld_a_r_u8(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// LD B, (HL)
                        self.b = self.ld_r_u8_m_hl();
                    },
                    1 => {// LD C, (HL)
                        self.c = self.ld_r_u8_m_hl();
                    },
                    2 => {// LD D, (HL)
                        self.d = self.ld_r_u8_m_hl();
                    },
                    3 => {// LD E, (HL)
                        self.e = self.ld_r_u8_m_hl();
                    },
                    4 => {// LD H, (HL)
                        self.h = self.ld_r_u8_m_hl();
                    },
                    5 => {// LD L, (HL)
                        self.l = self.ld_r_u8_m_hl();
                    },
                    6 => {
                        println!("HALT");
                    },
                    7 => {// LD A, (HL)
                        self.a = self.ld_r_u8_m_hl();
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// LD B, A
                        self.ld_b_r_u8(register_a);
                    },
                    1 => {// LD C, A
                        self.ld_c_r_u8(register_a);
                    },
                    2 => {// LD D, A
                        self.ld_d_r_u8(register_a);
                    },
                    3 => {// LD E, A
                        self.ld_e_r_u8(register_a);
                    },
                    4 => {// LD H, A
                        self.ld_h_r_u8(register_a);
                    },
                    5 => {// LD L, A
                        self.ld_l_r_u8(register_a);
                    },
                    6 => {// LD (HL), A
                        self.ld_m_hl_r_u8(register_a);
                    },
                    7 => {// LD A, A
                        self.ld_a_r_u8(register_a);
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
                    0 => {// ADD A, B
                        self.add_a_r_u8(register_b);
                    },
                    1 => {// ADC A, B
                        self.adc_a_r_u8(register_b);
                    },
                    2 => {// SUB A, B
                        self.sub_a_r_u8(register_b);
                    },
                    3 => {// SBC A, B
                        self.sbc_a_r_u8(register_b);
                    },
                    4 => {// AND A, B
                        self.and_a_r_u8(register_b);
                    },
                    5 => {// XOR A, B
                        self.xor_a_r_u8(register_b);
                    },
                    6 => {// OR A, B
                        self.or_a_r_u8(register_b);
                    },
                    7 => {// CP A, B
                        self.cp_a_r_u8(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// ADD A, C
                        self.add_a_r_u8(register_c);
                    },
                    1 => {// ADC A, C                       
                        self.adc_a_r_u8(register_c);
                    },
                    2 => {// SUB A, C                      
                        self.sub_a_r_u8(register_c);
                    },
                    3 => {// SBC A, C                        
                        self.sbc_a_r_u8(register_c);
                    },
                    4 => {// AND A, C                  
                        self.and_a_r_u8(register_c);
                    },
                    5 => {// XOR A, C                        
                        self.xor_a_r_u8(register_c);
                    },
                    6 => {// OR A, C                      
                        self.or_a_r_u8(register_c);
                    },
                    7 => {// CP A, C                       
                        self.cp_a_r_u8(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// ADD A, D
                        self.add_a_r_u8(register_d);
                    },
                    1 => {// ADC A, D
                        self.adc_a_r_u8(register_d);
                    },
                    2 => {// SUB A, D
                        self.sub_a_r_u8(register_d);
                    },
                    3 => {// SBC A, D
                        self.sbc_a_r_u8(register_d);
                    },
                    4 => {// AND A, D
                        self.and_a_r_u8(register_d);
                    },
                    5 => {// XOR A, D
                        self.xor_a_r_u8(register_d);
                    },
                    6 => {// OR A, D
                        self.or_a_r_u8(register_d);
                    },
                    7 => {// CP A, D
                        self.cp_a_r_u8(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// ADD A, E
                        self.add_a_r_u8(register_e);
                    },
                    1 => {// ADC A, E
                        self.adc_a_r_u8(register_e);
                    },
                    2 => {// SUB A, E
                        self.sub_a_r_u8(register_e);
                    },
                    3 => {// SBC A, E
                        self.sbc_a_r_u8(register_e);
                    },
                    4 => {// AND A, E
                        self.and_a_r_u8(register_e);
                    },
                    5 => {// XOR A, E
                        self.xor_a_r_u8(register_e);
                    },
                    6 => {// OR A, E
                        self.or_a_r_u8(register_e);
                    },
                    7 => {// CP A, E
                        self.cp_a_r_u8(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// ADD A, H
                        self.add_a_r_u8(register_h);
                    },
                    1 => {// ADC A, H
                        self.adc_a_r_u8(register_h);
                    },
                    2 => {// SUB A, H
                        self.sub_a_r_u8(register_h);
                    },
                    3 => {// SBC A, H
                        self.sbc_a_r_u8(register_h);
                    },
                    4 => {// AND A, H
                        self.and_a_r_u8(register_h);
                    },
                    5 => {// XOR A, H
                        self.xor_a_r_u8(register_h);
                    },
                    6 => {// OR A, H
                        self.or_a_r_u8(register_h);
                    },
                    7 => {// CP A, H
                        self.cp_a_r_u8(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// ADD A, L
                        self.add_a_r_u8(register_l);
                    },
                    1 => {// ADC A, L
                        self.adc_a_r_u8(register_l);
                    },
                    2 => {// SUB A, L
                        self.sub_a_r_u8(register_l);
                    },
                    3 => {// SBC A, L
                        self.sbc_a_r_u8(register_l);
                    },
                    4 => {// AND A, L
                        self.and_a_r_u8(register_l);
                    },
                    5 => {// XOR A, L
                        self.xor_a_r_u8(register_l);
                    },
                    6 => {// OR A, L
                        self.or_a_r_u8(register_l);
                    },
                    7 => {// CP A, L
                        self.cp_a_r_u8(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// ADD A, (HL)
                        self.add_a_m_hl();
                    },
                    1 => {// ADC A, (HL)
                        self.adc_a_m_hl();
                    },
                    2 => {// SUB A, (HL)
                        self.sub_a_m_hl();
                    },
                    3 => {// SBC A, (HL)
                        self.sbc_a_m_hl();
                    },
                    4 => {// AND A, (HL)
                        self.and_a_m_hl();
                    },
                    5 => {// XOR A, (HL)
                        self.xor_a_m_hl();
                    },
                    6 => {// OR A, (HL)
                        self.or_a_m_hl();
                    },
                    7 => {// CP A, (HL)
                        self.cp_a_m_hl();
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// ADD A, A
                        self.add_a_r_u8(register_a);
                    },
                    1 => {// ADC A, A
                        self.adc_a_r_u8(register_a);
                    },
                    2 => {// SUB A, A
                        self.sub_a_r_u8(register_a);
                    },
                    3 => {// SBC A, A
                        self.sbc_a_r_u8(register_a);
                    },
                    4 => {// AND A, A
                        self.and_a_r_u8(register_a);
                    },
                    5 => {// XOR A, A
                        self.xor_a_r_u8(register_a);
                    },
                    6 => {// OR A, A
                        self.or_a_r_u8(register_a);
                    },
                    7 => {// CP A, A
                        self.cp_a_r_u8(register_a);
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
                    0 => {// RET NZ
                        self.ret_nz();
                    },
                    1 => {// RET Z
                        self.ret_z();
                    },
                    2 => {// RET NC
                        self.ret_nc();
                    },
                    3 => {// RET C
                        self.ret_c();
                    },
                    4 => {// LD (0xFF00+n), A
                        self.ld_ff00_plus_u8_a();
                    },
                    5 => {// ADD SP, d
                        self.add_sp_i8();
                    },
                    6 => {// LD A, (0xFF00+n)
                        self.ld_a_ff00_plus_u8();
                    },
                    7 => {// LD HL, SP+d
                        self.ld_hl_sp_plus_i8();
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                match bits_5_4_3 {
                    0 => {// POP BC
                        self.pop_bc();
                    },
                    1 => {// RET
                        self.ret();
                    },
                    2 => {// POP DE
                        self.pop_de();
                    },
                    3 => {// RETI
                    },
                    4 => {// POP HL
                        self.pop_hl();
                    },
                    5 => {// JP HL
                        self.jp_hl();
                    },
                    6 => {// POP AF
                        self.pop_af();
                    },
                    7 => {// LD SP, HL
                        self.ld_sp_hl();
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                match bits_5_4_3 {
                    0 => {// JP NZ, nn
                        self.jp_nz_u16();
                    },
                    1 => {// JP Z, nn
                        self.jp_z_u16();
                    },
                    2 => {// JP NC, nn
                        self.jp_nc_u16();
                    },
                    3 => {// JP C, nn
                        self.jp_c_u16();
                    },
                    4 => {// LD (0xFF00+C), A
                        self.ld_ff00_plus_c_a();
                    },
                    5 => {// LD (nn), A
                        self.ld_u16_a();
                    },
                    6 => {// LD A, (0xFF00+C)
                        self.ld_a_ff00_plus_c();
                    },
                    7 => {// LD A, (nn)
                        self.ld_a_u16();
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                match bits_5_4_3 {
                    0 => {// JP nn
                        self.jp_u16();
                    },
                    6 => {
                        println!("DI");
                    },
                    7 => {
                        println!("EI");
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                match bits_5_4_3 {
                    0 => {// CALL NZ, nn
                        self.call_nz_u16();
                    },
                    1 => {// CALL Z, nn
                        self.call_z_u16();
                    },
                    2 => {// CALL NC, nn
                        self.call_nc_u16();
                    },
                    3 => {// CALL C, nn
                        self.call_c_u16();
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                match bits_5_4_3 {
                    0 => {// PUSH BC
                        self.push_bc();
                    },
                    1 => {// CALL nn
                        self.call_u16();
                    },
                    2 => {// PUSH DE
                        self.push_de();
                    },
                    4 => {// PUSH HL
                        self.push_hl();
                    },
                    6 => {// PUSH AF
                        self.push_af();
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// ADD A, n
                        self.add_a_u8();
                    },
                    1 => {// ADC A, n
                        self.adc_a_u8();
                    },
                    2 => {// SUB A, n
                        self.sub_a_u8();
                    },
                    3 => {// SBC A, n
                        self.sbc_a_u8();
                    },
                    4 => {// AND A, n
                        self.and_a_u8();
                    },
                    5 => {// XOR A, n
                        self.xor_a_u8();
                    },
                    6 => {// OR A, n
                        self.or_a_u8();
                    },
                    7 => {// CP A, n
                        self.cp_a_u8();
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                match bits_5_4_3 {
                    0 => {// RST 00h
                        self.rst(0x0000);
                    },
                    1 => {// RST 08h
                        self.rst(0x0008);
                    },
                    2 => {// RST 10h
                        self.rst(0x0010);
                    },
                    3 => {// RST 18h
                        self.rst(0x0018);
                    },
                    4 => {// RST 20h
                        self.rst(0x0020);
                    },
                    5 => {// RST 28h
                        self.rst(0x0028);
                    },
                    6 => {// RST 30h
                        self.rst(0x0030);
                    },
                    7 => {// RST 38h
                        self.rst(0x0038);
                    },
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
                    0 => {// RLC B
                        self.b = self.rlc(register_b);
                    },
                    1 => {// RRC B
                        self.b = self.rrc(register_b);
                    },
                    2 => {// RL B
                        self.b = self.rl(register_b);
                    },
                    3 => {// RR B
                        self.b = self.rr(register_b);
                    },
                    4 => {// SLA B
                        self.b = self.sla(register_b);
                    },
                    5 => {// SRA B
                        self.b = self.sra(register_b);
                    },
                    6 => {// SWAP B
                        self.b = self.swap(register_b);
                    },
                    7 => {// SRL B
                        self.b = self.srl(register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// RLC C
                        self.c = self.rlc(register_c);
                    },
                    1 => {// RRC C
                        self.c = self.rrc(register_c);
                    },
                    2 => {// RL C
                        self.c = self.rl(register_c);
                    },
                    3 => {// RR C
                        self.c = self.rr(register_c);
                    },
                    4 => {// SLA C
                        self.c = self.sla(register_c);
                    },
                    5 => {// SRA C
                        self.c = self.sra(register_c);
                    },
                    6 => {// SWAP C
                        self.c = self.swap(register_c);
                    },
                    7 => {// SRL C
                        self.c = self.srl(register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// RLC D
                        self.d = self.rlc(register_d);
                    },
                    1 => {// RRC D
                        self.d = self.rrc(register_d);
                    },
                    2 => {// RL D
                        self.d = self.rl(register_d);
                    },
                    3 => {// RR D
                        self.d = self.rr(register_d);
                    },
                    4 => {// SLA D
                        self.d = self.sla(register_d);
                    },
                    5 => {// SRA D
                        self.d = self.sra(register_d);
                    },
                    6 => {// SWAP D
                        self.d = self.swap(register_d);
                    },
                    7 => {// SRL D
                        self.d = self.srl(register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// RLC E
                        self.e = self.rlc(register_e);
                    },
                    1 => {// RRC E
                        self.e = self.rrc(register_e);
                    },
                    2 => {// RL E
                        self.e = self.rl(register_e);
                    },
                    3 => {// RR E
                        self.e = self.rr(register_e);
                    },
                    4 => {// SLA E
                        self.e = self.sla(register_e);
                    },
                    5 => {// SRA E
                        self.e = self.sra(register_e);
                    },
                    6 => {// SWAP E
                        self.e = self.swap(register_e);
                    },
                    7 => {// SRL E
                        self.e = self.srl(register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// RLC H
                        self.h = self.rlc(register_h);
                    },
                    1 => {// RRC H
                        self.h = self.rrc(register_h);
                    },
                    2 => {// RL H
                        self.h = self.rl(register_h);
                    },
                    3 => {// RR H
                        self.h = self.rr(register_h);
                    },
                    4 => {// SLA H
                        self.h = self.sla(register_h);
                    },
                    5 => {// SRA H
                        self.h = self.sra(register_h);
                    },
                    6 => {// SWAP H
                        self.h = self.swap(register_h);
                    },
                    7 => {// SRL H
                        self.h = self.srl(register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// RLC L
                        self.l = self.rlc(register_l);
                    },
                    1 => {// RRC L
                        self.l = self.rrc(register_l);
                    },
                    2 => {// RL L
                        self.l = self.rl(register_l);
                    },
                    3 => {// RR L
                        self.l = self.rr(register_l);
                    },
                    4 => {// SLA L
                        self.l = self.sla(register_l);
                    },
                    5 => {// SRA L
                        self.l = self.sra(register_l);
                    },
                    6 => {// SWAP L
                        self.l = self.swap(register_l);
                    },
                    7 => {// SRL L
                        self.l = self.srl(register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// RLC (HL)
                        self.rlc_m_hl();
                    },
                    1 => {// RRC (HL)
                        self.rrc_m_hl();
                    },
                    2 => {// RL (HL)
                        self.rl_m_hl();
                    },
                    3 => {// RR (HL)
                        self.rr_m_hl();
                    },
                    4 => {// SLA (HL)
                        self.sla_m_hl();
                    },
                    5 => {// SRA (HL)
                        self.sra_m_hl();
                    },
                    6 => {// SWAP (HL)
                        self.swap_m_hl();
                    },
                    7 => {// SRL (HL)
                        self.srl_m_hl();
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// RLC A
                        self.a = self.rlc(register_a);
                    },
                    1 => {// RRC A
                        self.a = self.rrc(register_a);
                    },
                    2 => {// RL A
                        self.a = self.rl(register_a);
                    },
                    3 => {// RR A
                        self.a = self.rr(register_a);
                    },
                    4 => {// SLA A
                        self.a = self.sla(register_a);
                    },
                    5 => {// SRA A
                        self.a = self.sra(register_a);
                    },
                    6 => {// SWAP A
                        self.a = self.swap(register_a);
                    },
                    7 => {// SRL A
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
                    0 => {// BIT 0, B
                        self.bit_n_r_u8(0, register_b);
                    },
                    1 => {// BIT 1, B
                        self.bit_n_r_u8(1, register_b);
                    },
                    2 => {// BIT 2, B
                        self.bit_n_r_u8(2, register_b);
                    },
                    3 => {// BIT 3, B
                        self.bit_n_r_u8(3, register_b);
                    },
                    4 => {// BIT 4, B
                        self.bit_n_r_u8(4, register_b);
                    },
                    5 => {// BIT 5, B
                        self.bit_n_r_u8(5, register_b);
                    },
                    6 => {// BIT 6, B
                        self.bit_n_r_u8(6, register_b);
                    },
                    7 => {// BIT 7, B
                        self.bit_n_r_u8(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// BIT 0, C
                        self.bit_n_r_u8(0, register_c);
                    },
                    1 => {// BIT 1, C
                        self.bit_n_r_u8(1, register_c);
                    },
                    2 => {// BIT 2, C
                        self.bit_n_r_u8(2, register_c);
                    },
                    3 => {// BIT 3, C
                        self.bit_n_r_u8(3, register_c);
                    },
                    4 => {// BIT 4, C
                        self.bit_n_r_u8(4, register_c);
                    },
                    5 => {// BIT 5, C
                        self.bit_n_r_u8(5, register_c);
                    },
                    6 => {// BIT 6, C
                        self.bit_n_r_u8(6, register_c);
                    },
                    7 => {// BIT 7, C
                        self.bit_n_r_u8(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// BIT 0, D
                        self.bit_n_r_u8(0, register_d);
                    },
                    1 => {// BIT 1, D
                        self.bit_n_r_u8(1, register_d);
                    },
                    2 => {// BIT 2, D
                        self.bit_n_r_u8(2, register_d);
                    },
                    3 => {// BIT 3, D
                        self.bit_n_r_u8(3, register_d);
                    },
                    4 => {// BIT 4, D
                        self.bit_n_r_u8(4, register_d);
                    },
                    5 => {// BIT 5, D
                        self.bit_n_r_u8(5, register_d);
                    },
                    6 => {// BIT 6, D
                        self.bit_n_r_u8(6, register_d);
                    },
                    7 => {// BIT 7, D
                        self.bit_n_r_u8(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// BIT 0, E
                        self.bit_n_r_u8(0, register_e);
                    },
                    1 => {// BIT 1, E
                        self.bit_n_r_u8(1, register_e);
                    },
                    2 => {// BIT 2, E
                        self.bit_n_r_u8(2, register_e);
                    },
                    3 => {// BIT 3, E
                        self.bit_n_r_u8(3, register_e);
                    },
                    4 => {// BIT 4, E
                        self.bit_n_r_u8(4, register_e);
                    },
                    5 => {// BIT 5, E
                        self.bit_n_r_u8(5, register_e);
                    },
                    6 => {// BIT 6, E
                        self.bit_n_r_u8(6, register_e);
                    },
                    7 => {// BIT 7, E
                        self.bit_n_r_u8(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// BIT 0, H
                        self.bit_n_r_u8(0, register_h);
                    },
                    1 => {// BIT 1, H
                        self.bit_n_r_u8(1, register_h);
                    },
                    2 => {// BIT 2, H
                        self.bit_n_r_u8(2, register_h);
                    },
                    3 => {// BIT 3, H
                        self.bit_n_r_u8(3, register_h);
                    },
                    4 => {// BIT 4, H
                        self.bit_n_r_u8(4, register_h);
                    },
                    5 => {// BIT 5, H
                        self.bit_n_r_u8(5, register_h);
                    },
                    6 => {// BIT 6, H
                        self.bit_n_r_u8(6, register_h);
                    },
                    7 => {// BIT 7, H
                        self.bit_n_r_u8(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// BIT 0, L
                        self.bit_n_r_u8(0, register_l);
                    },
                    1 => {// BIT 1, L
                        self.bit_n_r_u8(1, register_l);
                    },
                    2 => {// BIT 2, L
                        self.bit_n_r_u8(2, register_l);
                    },
                    3 => {// BIT 3, L
                        self.bit_n_r_u8(3, register_l);
                    },
                    4 => {// BIT 4, L
                        self.bit_n_r_u8(4, register_l);
                    },
                    5 => {// BIT 5, L
                        self.bit_n_r_u8(5, register_l);
                    },
                    6 => {// BIT 6, L
                        self.bit_n_r_u8(6, register_l);
                    },
                    7 => {// BIT 7, L
                        self.bit_n_r_u8(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// BIT 0, (HL)
                        self.bit_n_m_hl(0);
                    },
                    1 => {// BIT 1, (HL)
                        self.bit_n_m_hl(1);
                    },
                    2 => {// BIT 2, (HL)
                        self.bit_n_m_hl(2);
                    },
                    3 => {// BIT 3, (HL)
                        self.bit_n_m_hl(3);
                    },
                    4 => {// BIT 4, (HL)
                        self.bit_n_m_hl(4);
                    },
                    5 => {// BIT 5, (HL)
                        self.bit_n_m_hl(5);
                    },
                    6 => {// BIT 6, (HL)
                        self.bit_n_m_hl(6);
                    },
                    7 => {// BIT 7, (HL)
                        self.bit_n_m_hl(7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// BIT 0, A
                        self.bit_n_r_u8(0, register_a);
                    },
                    1 => {// BIT 1, A
                        self.bit_n_r_u8(1, register_a);
                    },
                    2 => {// BIT 2, A
                        self.bit_n_r_u8(2, register_a);
                    },
                    3 => {// BIT 3, A
                        self.bit_n_r_u8(3, register_a);
                    },
                    4 => {// BIT 4, A
                        self.bit_n_r_u8(4, register_a);
                    },
                    5 => {// BIT 5, A
                        self.bit_n_r_u8(5, register_a);
                    },
                    6 => {// BIT 6, A
                        self.bit_n_r_u8(6, register_a);
                    },
                    7 => {// BIT 7, A
                        self.bit_n_r_u8(7, register_a);
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
                    0 => {// RES 0, B
                        self.b = self.res_n_r_u8(0, register_b);
                    },
                    1 => {// RES 1, B
                        self.b = self.res_n_r_u8(1, register_b);
                    },
                    2 => {// RES 2, B
                        self.b = self.res_n_r_u8(2, register_b);
                    },
                    3 => {// RES 3, B
                        self.b = self.res_n_r_u8(3, register_b);
                    },
                    4 => {// RES 4, B
                        self.b = self.res_n_r_u8(4, register_b);
                    },
                    5 => {// RES 5, B
                        self.b = self.res_n_r_u8(5, register_b);
                    },
                    6 => {// RES 6, B
                        self.b = self.res_n_r_u8(6, register_b);
                    },
                    7 => {// RES 7, B
                        self.b = self.res_n_r_u8(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// RES 0, C
                        self.c = self.res_n_r_u8(0, register_c);
                    },
                    1 => {// RES 1, C
                        self.c = self.res_n_r_u8(1, register_c);
                    },
                    2 => {// RES 2, C
                        self.c = self.res_n_r_u8(2, register_c);
                    },
                    3 => {// RES 3, C
                        self.c = self.res_n_r_u8(3, register_c);
                    },
                    4 => {// RES 4, C
                        self.c = self.res_n_r_u8(4, register_c);
                    },
                    5 => {// RES 5, C
                        self.c = self.res_n_r_u8(5, register_c);
                    },
                    6 => {// RES 6, C
                        self.c = self.res_n_r_u8(6, register_c);
                    },
                    7 => {// RES 7, C
                        self.c = self.res_n_r_u8(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// RES 0, D
                        self.d = self.res_n_r_u8(0, register_d);
                    },
                    1 => {// RES 1, D
                        self.d = self.res_n_r_u8(1, register_d);
                    },
                    2 => {// RES 2, D
                        self.d = self.res_n_r_u8(2, register_d);
                    },
                    3 => {// RES 3, D
                        self.d = self.res_n_r_u8(3, register_d);
                    },
                    4 => {// RES 4, D
                        self.d = self.res_n_r_u8(4, register_d);
                    },
                    5 => {// RES 5, D
                        self.d = self.res_n_r_u8(5, register_d);
                    },
                    6 => {// RES 6, D
                        self.d = self.res_n_r_u8(6, register_d);
                    },
                    7 => {// RES 7, D
                        self.d = self.res_n_r_u8(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// RES 0, E
                        self.e = self.res_n_r_u8(0, register_e);
                    },
                    1 => {// RES 1, E
                        self.e = self.res_n_r_u8(1, register_e);
                    },
                    2 => {// RES 2, E
                        self.e = self.res_n_r_u8(2, register_e);
                    },
                    3 => {// RES 3, E
                        self.e = self.res_n_r_u8(3, register_e);
                    },
                    4 => {// RES 4, E
                        self.e = self.res_n_r_u8(4, register_e);
                    },
                    5 => {// RES 5, E
                        self.e = self.res_n_r_u8(5, register_e);
                    },
                    6 => {// RES 6, E
                        self.e = self.res_n_r_u8(6, register_e);
                    },
                    7 => {// RES 7, E
                        self.e = self.res_n_r_u8(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// RES 0, H
                        self.h = self.res_n_r_u8(0, register_h);
                    },
                    1 => {// RES 1, H
                        self.h = self.res_n_r_u8(1, register_h);
                    },
                    2 => {// RES 2, H
                        self.h = self.res_n_r_u8(2, register_h);
                    },
                    3 => {// RES 3, H
                        self.h = self.res_n_r_u8(3, register_h);
                    },
                    4 => {// RES 4, H
                        self.h = self.res_n_r_u8(4, register_h);
                    },
                    5 => {// RES 5, H
                        self.h = self.res_n_r_u8(5, register_h);
                    },
                    6 => {// RES 6, H
                        self.h = self.res_n_r_u8(6, register_h);
                    },
                    7 => {// RES 7, H
                        self.h = self.res_n_r_u8(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// RES 0, L
                        self.l = self.res_n_r_u8(0, register_l);
                    },
                    1 => {// RES 1, L
                        self.l = self.res_n_r_u8(1, register_l);
                    },
                    2 => {// RES 2, L
                        self.l = self.res_n_r_u8(2, register_l);
                    },
                    3 => {// RES 3, L
                        self.l = self.res_n_r_u8(3, register_l);
                    },
                    4 => {// RES 4, L
                        self.l = self.res_n_r_u8(4, register_l);
                    },
                    5 => {// RES 5, L
                        self.l = self.res_n_r_u8(5, register_l);
                    },
                    6 => {// RES 6, L
                        self.l = self.res_n_r_u8(6, register_l);
                    },
                    7 => {// RES 7, L
                        self.l = self.res_n_r_u8(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// RES 0, (HL)
                        self.res_n_m_hl(0);
                    },
                    1 => {// RES 1, (HL)
                        self.res_n_m_hl(1);
                    },
                    2 => {// RES 2, (HL)
                        self.res_n_m_hl(2);
                    },
                    3 => {// RES 3, (HL)
                        self.res_n_m_hl(3);
                    },
                    4 => {// RES 4, (HL)
                        self.res_n_m_hl(4);
                    },
                    5 => {// RES 5, (HL)
                        self.res_n_m_hl(5);
                    },
                    6 => {// RES 6, (HL)
                        self.res_n_m_hl(6);
                    },
                    7 => {// RES 7, (HL)
                        self.res_n_m_hl(7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// RES 0, A
                        self.a = self.res_n_r_u8(0, register_a);
                    },
                    1 => {// RES 1, A
                        self.a = self.res_n_r_u8(1, register_a);
                    },
                    2 => {// RES 2, A
                        self.a = self.res_n_r_u8(2, register_a);
                    },
                    3 => {// RES 3, A
                        self.a = self.res_n_r_u8(3, register_a);
                    },
                    4 => {// RES 4, A
                        self.a = self.res_n_r_u8(4, register_a);
                    },
                    5 => {// RES 5, A
                        self.a = self.res_n_r_u8(5, register_a);
                    },
                    6 => {// RES 6, A
                        self.a = self.res_n_r_u8(6, register_a);
                    },
                    7 => {// RES 7, A
                        self.a = self.res_n_r_u8(7, register_a);
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
                    0 => {// SET 0, B
                        self.b = self.set_n_r_u8(0, register_b);
                    },
                    1 => {// SET 1, B
                        self.b = self.set_n_r_u8(1, register_b);
                    },
                    2 => {// SET 2, B
                        self.b = self.set_n_r_u8(2, register_b);
                    },
                    3 => {// SET 3, B
                        self.b = self.set_n_r_u8(3, register_b);
                    },
                    4 => {// SET 4, B
                        self.b = self.set_n_r_u8(4, register_b);
                    },
                    5 => {// SET 5, B
                        self.b = self.set_n_r_u8(5, register_b);
                    },
                    6 => {// SET 6, B
                        self.b = self.set_n_r_u8(6, register_b);
                    },
                    7 => {// SET 7, B
                        self.b = self.set_n_r_u8(7, register_b);
                    },
                    _ => unreachable!(),
                }
            },
            1 => {
                let register_c = self.c;
                match bits_5_4_3 {
                    0 => {// SET 0, C
                        self.c = self.set_n_r_u8(0, register_c);
                    },
                    1 => {// SET 1, C
                        self.c = self.set_n_r_u8(1, register_c);
                    },
                    2 => {// SET 2, C
                        self.c = self.set_n_r_u8(2, register_c);
                    },
                    3 => {// SET 3, C
                        self.c = self.set_n_r_u8(3, register_c);
                    },
                    4 => {// SET 4, C
                        self.c = self.set_n_r_u8(4, register_c);
                    },
                    5 => {// SET 5, C
                        self.c = self.set_n_r_u8(5, register_c);
                    },
                    6 => {// SET 6, C
                        self.c = self.set_n_r_u8(6, register_c);
                    },
                    7 => {// SET 7, C
                        self.c = self.set_n_r_u8(7, register_c);
                    },
                    _ => unreachable!(),
                }
            },
            2 => {
                let register_d = self.d;
                match bits_5_4_3 {
                    0 => {// SET 0, D
                        self.d = self.set_n_r_u8(0, register_d);
                    },
                    1 => {// SET 1, D
                        self.d = self.set_n_r_u8(1, register_d);
                    },
                    2 => {// SET 2, D
                        self.d = self.set_n_r_u8(2, register_d);
                    },
                    3 => {// SET 3, D
                        self.d = self.set_n_r_u8(3, register_d);
                    },
                    4 => {// SET 4, D
                        self.d = self.set_n_r_u8(4, register_d);
                    },
                    5 => {// SET 5, D
                        self.d = self.set_n_r_u8(5, register_d);
                    },
                    6 => {// SET 6, D
                        self.d = self.set_n_r_u8(6, register_d);
                    },
                    7 => {// SET 7, D
                        self.d = self.set_n_r_u8(7, register_d);
                    },
                    _ => unreachable!(),
                }
            },
            3 => {
                let register_e = self.e;
                match bits_5_4_3 {
                    0 => {// SET 0, E
                        self.e = self.set_n_r_u8(0, register_e);
                    },
                    1 => {// SET 1, E
                        self.e = self.set_n_r_u8(1, register_e);
                    },
                    2 => {// SET 2, E
                        self.e = self.set_n_r_u8(2, register_e);
                    },
                    3 => {// SET 3, E
                        self.e = self.set_n_r_u8(3, register_e);
                    },
                    4 => {// SET 4, E
                        self.e = self.set_n_r_u8(4, register_e);
                    },
                    5 => {// SET 5, E
                        self.e = self.set_n_r_u8(5, register_e);
                    },
                    6 => {// SET 6, E
                        self.e = self.set_n_r_u8(6, register_e);
                    },
                    7 => {// SET 7, E
                        self.e = self.set_n_r_u8(7, register_e);
                    },
                    _ => unreachable!(),
                }
            },
            4 => {
                let register_h = self.h;
                match bits_5_4_3 {
                    0 => {// SET 0, H
                        self.h = self.set_n_r_u8(0, register_h);
                    },
                    1 => {// SET 1, H
                        self.h = self.set_n_r_u8(1, register_h);
                    },
                    2 => {// SET 2, H
                        self.h = self.set_n_r_u8(2, register_h);
                    },
                    3 => {// SET 3, H
                        self.h = self.set_n_r_u8(3, register_h);
                    },
                    4 => {// SET 4, H
                        self.h = self.set_n_r_u8(4, register_h);
                    },
                    5 => {// SET 5, H
                        self.h = self.set_n_r_u8(5, register_h);
                    },
                    6 => {// SET 6, H
                        self.h = self.set_n_r_u8(6, register_h);
                    },
                    7 => {// SET 7, H
                        self.h = self.set_n_r_u8(7, register_h);
                    },
                    _ => unreachable!(),
                }
            },
            5 => {
                let register_l = self.l;
                match bits_5_4_3 {
                    0 => {// SET 0, L
                        self.l = self.set_n_r_u8(0, register_l);
                    },
                    1 => {// SET 1, L
                        self.l = self.set_n_r_u8(1, register_l);
                    },
                    2 => {// SET 2, L
                        self.l = self.set_n_r_u8(2, register_l);
                    },
                    3 => {// SET 3, L
                        self.l = self.set_n_r_u8(3, register_l);
                    },
                    4 => {// SET 4, L
                        self.l = self.set_n_r_u8(4, register_l);
                    },
                    5 => {// SET 5, L
                        self.l = self.set_n_r_u8(5, register_l);
                    },
                    6 => {// SET 6, L
                        self.l = self.set_n_r_u8(6, register_l);
                    },
                    7 => {// SET 7, L
                        self.l = self.set_n_r_u8(7, register_l);
                    },
                    _ => unreachable!(),
                }
            },
            6 => {
                match bits_5_4_3 {
                    0 => {// SET 0, (HL)
                        self.set_n_m_hl(0);
                    },
                    1 => {// SET 1, (HL)
                        self.set_n_m_hl(1);
                    },
                    2 => {// SET 2, (HL)
                        self.set_n_m_hl(2);
                    },
                    3 => {// SET 3, (HL)
                        self.set_n_m_hl(3);
                    },
                    4 => {// SET 4, (HL)
                        self.set_n_m_hl(4);
                    },
                    5 => {// SET 5, (HL)
                        self.set_n_m_hl(5);
                    },
                    6 => {// SET 6, (HL)
                        self.set_n_m_hl(6);
                    },
                    7 => {// SET 7, (HL)
                        self.set_n_m_hl(7);
                    },
                    _ => unreachable!(),
                }
            },
            7 => {
                let register_a = self.a;
                match bits_5_4_3 {
                    0 => {// SET 0, A
                        self.a = self.set_n_r_u8(0, register_a);
                    },
                    1 => {// SET 1, A
                        self.a = self.set_n_r_u8(1, register_a);
                    },
                    2 => {// SET 2, A
                        self.a = self.set_n_r_u8(2, register_a);
                    },
                    3 => {// SET 3, A
                        self.a = self.set_n_r_u8(3, register_a);
                    },
                    4 => {// SET 4, A
                        self.a = self.set_n_r_u8(4, register_a);
                    },
                    5 => {// SET 5, A
                        self.a = self.set_n_r_u8(5, register_a);
                    },
                    6 => {// SET 6, A
                        self.a = self.set_n_r_u8(6, register_a);
                    },
                    7 => {// SET 7, A
                        self.a = self.set_n_r_u8(7, register_a);
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
}
