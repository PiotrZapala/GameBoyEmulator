use crate::cpu::CPU;

pub fn nop(cpu: &mut CPU) {
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn stop(cpu: &mut CPU) {
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn halt(cpu: &mut CPU) {
    cpu.set_cycles(4);
    cpu.halted = true;
}

pub fn inc_r_u8(cpu: &mut CPU, register: u8) -> u8 {
    let half_carry = (register & 0x0F) == 0x0F;
    let result = register.wrapping_add(1);
    let zero = result == 0;
    cpu.update_flags(Some(zero), None, Some(false), Some(half_carry));
    cpu.set_cycles(4);
    cpu.pc += 1;
    result
}

pub fn dec_r_u8(cpu: &mut CPU, register: u8) -> u8 {
    let half_carry = (register & 0x0F) == 0x00;
    let result = register.wrapping_sub(1);
    let zero = result == 0;
    cpu.update_flags(Some(zero), None, Some(true), Some(half_carry));
    cpu.set_cycles(4);
    cpu.pc += 1;
    result
}

pub fn inc_r_u16(cpu: &mut CPU, register1: u8, register2: u8) -> (u8, u8) {
    let combined = ((register1 as u16) << 8) | (register2 as u16);
    let result = combined.wrapping_add(1);
    let new_register1 = (result >> 8) as u8;
    let new_register2 = (result & 0xFF) as u8;
    cpu.pc += 1;
    cpu.set_cycles(8);
    (new_register1, new_register2)
}

pub fn dec_r_u16(cpu: &mut CPU, register1: u8, register2: u8) -> (u8, u8) {
    let combined = ((register1 as u16) << 8) | (register2 as u16);
    let result = combined.wrapping_sub(1);
    let new_register1 = (result >> 8) as u8;
    let new_register2 = (result & 0xFF) as u8;
    cpu.pc += 1;
    cpu.set_cycles(8);
    (new_register1, new_register2)
}

pub fn inc_sp(cpu: &mut CPU) {
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn dec_sp(cpu: &mut CPU) {
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn inc_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (half_carry, zero) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let half_carry = (value & 0x0F) == 0x0F;
        let result = value.wrapping_add(1);
        let zero = result == 0;

        mmu.write_byte(address, result);
        (half_carry, zero)
    };

    cpu.update_flags(Some(zero), None, Some(false), Some(half_carry));

    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn dec_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (half_carry, zero) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let half_carry = (value & 0x0F) == 0x00;
        let result = value.wrapping_sub(1);
        let zero = result == 0;

        mmu.write_byte(address, result);
        (half_carry, zero)
    };

    cpu.update_flags(Some(zero), None, Some(true), Some(half_carry));

    cpu.set_cycles(12);
    cpu.pc += 1;
}


pub fn add_a_r_u8(cpu: &mut CPU, register: u8) {
    let (result, carry) = cpu.a.overflowing_add(register);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) + (register & 0xF) > 0xF;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(4);
    cpu.a = result;
    cpu.pc += 1;
}

pub fn add_hl_r_u16(cpu: &mut CPU, register1: u8, register2: u8) {
    let combined1 = ((cpu.h as u16) << 8) | (cpu.l as u16);
    let combined2 = ((register1 as u16) << 8) | (register2 as u16);
    let (result, carry) = combined1.overflowing_add(combined2);
    let half_carry = (combined1 & 0x0FFF) + (combined2 & 0x0FFF) > 0x0FFF;
    cpu.update_flags(None, Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.h = (result >> 8) as u8;
    cpu.l = (result & 0xFF) as u8;
    cpu.pc += 1;
}

pub fn add_hl_sp(cpu: &mut CPU) {
    let combined = ((cpu.h as u16) << 8) | (cpu.l as u16);
    let (result, carry) = combined.overflowing_add(cpu.sp);
    let half_carry = (combined & 0x0FFF) + (cpu.sp & 0x0FFF) > 0x0FFF;
    cpu.update_flags(None, Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.h = (result >> 8) as u8;
    cpu.l = (result & 0xFF) as u8;
    cpu.pc += 1;
}

pub fn add_a_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);
    
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };
    
    let (result, carry) = cpu.a.overflowing_add(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) + (value & 0xF) > 0xF;
    
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result;
    cpu.pc += 1;
}

pub fn add_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };
    
    let (result, carry) = cpu.a.overflowing_add(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) + (value & 0xF) > 0xF;
    
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result;
    cpu.pc += 2;
}

pub fn add_sp_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1) as i16 as u16
    };

    let result = cpu.sp.wrapping_add(offset);
    let half_carry = (cpu.sp & 0x000F) + (offset & 0x000F) > 0x000F;
    let carry = (cpu.sp & 0x00FF) + (offset & 0x00FF) > 0x00FF;

    cpu.update_flags(Some(false), Some(carry), Some(false), Some(half_carry));

    cpu.sp = result;
    cpu.pc += 2;
    cpu.set_cycles(16);
}


pub fn adc_a_r_u8(cpu: &mut CPU, register: u8) {
    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_add(register);
    let (result2, carry2) = result1.overflowing_add(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) + (register & 0xF) + carry_flag > 0xF;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(4);
    cpu.a = result2;
    cpu.pc += 1;
}

pub fn adc_a_m_hl(cpu: &mut CPU) {
    let value = {
        let address = ((cpu.h as u16) << 8) | (cpu.l as u16);
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };

    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_add(value);
    let (result2, carry2) = result1.overflowing_add(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) + (value & 0xF) + carry_flag > 0xF;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result2;
    cpu.pc += 1;
}

pub fn adc_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_add(value);
    let (result2, carry2) = result1.overflowing_add(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) + (value & 0xF) + carry_flag > 0xF;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result2;
    cpu.pc += 2;
}

pub fn sub_a_r_u8(cpu: &mut CPU, register: u8) {
    let (result, carry) = cpu.a.overflowing_sub(register);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (register & 0xF);
    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(4);
    cpu.a = result;
    cpu.pc += 1;
}

pub fn sub_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    let (result, carry) = cpu.a.overflowing_sub(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (value & 0xF);

    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result;
    cpu.pc += 1;
}

pub fn sub_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    let (result, carry) = cpu.a.overflowing_sub(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (value & 0xF);

    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result;
    cpu.pc += 2;
}

pub fn sbc_a_r_u8(cpu: &mut CPU, register: u8) {
    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_sub(register);
    let (result2, carry2) = result1.overflowing_sub(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) < ((register & 0xF) + carry_flag);
    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(4);
    cpu.a = result2;
    cpu.pc += 1;
}

pub fn sbc_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_sub(value);
    let (result2, carry2) = result1.overflowing_sub(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) < ((value & 0xF) + carry_flag);

    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result2;
    cpu.pc += 1;
}

pub fn sbc_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    let carry_flag = if cpu.f & 0x10 != 0 { 1 } else { 0 };
    let (result1, carry1) = cpu.a.overflowing_sub(value);
    let (result2, carry2) = result1.overflowing_sub(carry_flag);
    let carry = carry1 || carry2;
    let zero = result2 == 0;
    let half_carry = (cpu.a & 0xF) < ((value & 0xF) + carry_flag);

    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.a = result2;
    cpu.pc += 2;
}

pub fn and_a_r_u8(cpu: &mut CPU, register: u8) {
    cpu.a &= register;
    let zero = cpu.a == 0;
    let half_carry = true;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn and_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    cpu.a &= value;
    let zero = cpu.a == 0;
    let half_carry = true;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn and_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.a &= value;
    let zero = cpu.a == 0;
    let half_carry = true;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(half_carry));
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn xor_a_r_u8(cpu: &mut CPU, register: u8) {
    cpu.a ^= register;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn xor_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    cpu.a ^= value;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn xor_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.a ^= value;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn or_a_r_u8(cpu: &mut CPU, register: u8) {
    cpu.a |= register;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn or_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    cpu.a |= value;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn or_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.a |= value;
    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn cp_a_r_u8(cpu: &mut CPU, register: u8) {
    let (result, carry) = cpu.a.overflowing_sub(register);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (register & 0xF);
    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn cp_a_m_hl(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(((cpu.h as u16) << 8) | (cpu.l as u16))
    };

    let (result, carry) = cpu.a.overflowing_sub(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (value & 0xF);
    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn cp_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    let (result, carry) = cpu.a.overflowing_sub(value);
    let zero = result == 0;
    let half_carry = (cpu.a & 0xF) < (value & 0xF);
    cpu.update_flags(Some(zero), Some(carry), Some(true), Some(half_carry));
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_b_r_u8(cpu: &mut CPU, register: u8) {
    cpu.b = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_c_r_u8(cpu: &mut CPU, register: u8) {
    cpu.c = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_d_r_u8(cpu: &mut CPU, register: u8) {
    cpu.d = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_e_r_u8(cpu: &mut CPU, register: u8) {
    cpu.e = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_h_r_u8(cpu: &mut CPU, register: u8) {
    cpu.h = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_l_r_u8(cpu: &mut CPU, register: u8) {
    cpu.l = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_a_r_u8(cpu: &mut CPU, register: u8) {
    cpu.a = register;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ld_b_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };
    cpu.b = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_c_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.c = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_d_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.d = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_e_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.e = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_h_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.h = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_l_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.l = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_a_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    cpu.a = value;
    cpu.set_cycles(8);
    cpu.pc += 2;
}

pub fn ld_m_hl_u8(cpu: &mut CPU) {
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_u8(cpu.pc + 1)
    };

    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, value);
    }

    cpu.set_cycles(12);
    cpu.pc += 2;
}

pub fn ld_bc_u16(cpu: &mut CPU) {
    let (c, b) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.pc + 1), mmu.fetch_u8(cpu.pc + 2))
    };

    cpu.c = c;
    cpu.b = b;
    cpu.set_cycles(12);
    cpu.pc += 3;
}

pub fn ld_de_u16(cpu: &mut CPU) {
    let (e, d) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.pc + 1), mmu.fetch_u8(cpu.pc + 2))
    };

    cpu.e = e;
    cpu.d = d;
    cpu.set_cycles(12);
    cpu.pc += 3;
}

pub fn ld_hl_u16(cpu: &mut CPU) {
    let (l, h) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.pc + 1), mmu.fetch_u8(cpu.pc + 2))
    };

    cpu.l = l;
    cpu.h = h;
    cpu.set_cycles(12);
    cpu.pc += 3;
}

pub fn ld_sp_u16(cpu: &mut CPU) {
    let sp = {
        let mmu = cpu.mmu.lock().unwrap();
        let high_byte = mmu.fetch_u8(cpu.pc + 2) as u16;
        let low_byte = mmu.fetch_u8(cpu.pc + 1) as u16;
        (high_byte << 8) | low_byte
    };

    cpu.sp = sp;
    cpu.set_cycles(12);
    cpu.pc += 3;
}

pub fn ld_m_r_u16_a(cpu: &mut CPU, register1: u8, register2: u8) {
    let address = ((register1 as u16) << 8) | (register2 as u16);
    
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, cpu.a);
    }

    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_a_m_r_u16(cpu: &mut CPU, register1: u8, register2: u8) {
    let address = ((register1 as u16) << 8) | (register2 as u16);
    
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };

    cpu.a = value;
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_a_hl_inc(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);
    
    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };
    
    let incremented_address = address.wrapping_add(1);
    cpu.a = value;
    cpu.h = (incremented_address >> 8) as u8;
    cpu.l = (incremented_address & 0xFF) as u8;
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_hl_inc_a(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, cpu.a);
    }

    let incremented_address = address.wrapping_add(1);
    cpu.h = (incremented_address >> 8) as u8;
    cpu.l = (incremented_address & 0xFF) as u8;
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_a_hl_dec(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };
    
    let decremented_address = address.wrapping_sub(1);
    cpu.a = value;
    cpu.h = (decremented_address >> 8) as u8;
    cpu.l = (decremented_address & 0xFF) as u8;
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_hl_dec_a(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, cpu.a);
    }

    let decremented_address = address.wrapping_sub(1);
    cpu.h = (decremented_address >> 8) as u8;
    cpu.l = (decremented_address & 0xFF) as u8;
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_m_hl_r_u8(cpu: &mut CPU, register: u8) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, register);
    }

    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_r_u8_m_hl(cpu: &mut CPU) -> u8 {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let value = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };

    cpu.set_cycles(8);
    cpu.pc += 1;
    
    value
}


pub fn ld_m_u16_sp(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
    };

    let lower_byte = (cpu.sp & 0xFF) as u8;
    let upper_byte = (cpu.sp >> 8) as u8;

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, lower_byte);
        mmu.write_byte(address + 1, upper_byte);
    }

    cpu.set_cycles(20);
    cpu.pc += 3;
}

pub fn ld_sp_hl(cpu: &mut CPU) {
    cpu.sp = ((cpu.h as u16) << 8) | (cpu.l as u16);
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_ff00_plus_u8_a(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        0xFF00 + mmu.fetch_u8(cpu.pc + 1) as u16
    };
    
    cpu.mmu.lock().unwrap().write_byte(address, cpu.a);
    cpu.set_cycles(12);
    cpu.pc += 2;
}

pub fn ld_a_ff00_plus_u8(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        0xFF00 + mmu.fetch_u8(cpu.pc + 1) as u16
    };
    cpu.a = cpu.mmu.lock().unwrap().read_byte(address);
    cpu.set_cycles(12);
    cpu.pc += 2;
}

pub fn ld_ff00_plus_c_a(cpu: &mut CPU) {
    let address = 0xFF00 + cpu.c as u16;
    
    cpu.mmu.lock().unwrap().write_byte(address, cpu.a);
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn ld_a_ff00_plus_c(cpu: &mut CPU) {
    let address = 0xFF00 + cpu.c as u16;
    
    cpu.a = cpu.mmu.lock().unwrap().read_byte(address);
    cpu.set_cycles(8);
    cpu.pc += 1;
}
  

pub fn ld_a_u16(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
    };

    cpu.a = cpu.mmu.lock().unwrap().read_byte(address);
    cpu.set_cycles(16);
    cpu.pc += 3;
}

pub fn ld_u16_a(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
    };

    cpu.mmu.lock().unwrap().write_byte(address, cpu.a);
    cpu.set_cycles(16);
    cpu.pc += 3;
}

pub fn ld_hl_sp_plus_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1) as i16 as u16
    };
    
    let result = cpu.sp.wrapping_add(offset);
    let half_carry = (cpu.sp & 0x000F) + (offset & 0x000F) > 0x000F;
    let carry = (cpu.sp & 0x00FF) + (offset & 0x00FF) > 0x00FF;

    cpu.update_flags(Some(false), Some(carry), Some(false), Some(half_carry));

    cpu.h = (result >> 8) as u8;
    cpu.l = (result & 0xFF) as u8;
    cpu.set_cycles(12);
    cpu.pc += 2;
}

pub fn jr_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1)
    };
    let signed_value = offset as i16 as u16;
    cpu.pc = cpu.pc.wrapping_add(2).wrapping_add(signed_value);
    cpu.set_cycles(12);
}

pub fn jr_nz_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1)
    };
    let signed_value = offset as i16 as u16;
    if (cpu.f & 0x80) >> 7 != 1 {
        cpu.pc = cpu.pc.wrapping_add(2).wrapping_add(signed_value);
        cpu.set_cycles(12);
    } else {
        cpu.set_cycles(8);
        cpu.pc += 2;
    }
}

pub fn jr_z_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1)
    };
    let signed_value = offset as i16 as u16;
    if (cpu.f & 0x80) >> 7 == 1 {
        cpu.pc = cpu.pc.wrapping_add(2).wrapping_add(signed_value);
        cpu.set_cycles(12);
    } else {
        cpu.set_cycles(8);
        cpu.pc += 2;
    }
}

pub fn jr_nc_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1)
    };
    let signed_value = offset as i16 as u16;
    if (cpu.f & 0x10) >> 4 != 1 {
        cpu.pc = cpu.pc.wrapping_add(2).wrapping_add(signed_value);
        cpu.set_cycles(12);
    } else {
        cpu.set_cycles(8);
        cpu.pc += 2;
    }
}

pub fn jr_c_i8(cpu: &mut CPU) {
    let offset = {
        let mmu = cpu.mmu.lock().unwrap();
        mmu.fetch_i8(cpu.pc + 1)
    };
    let signed_value = offset as i16 as u16;
    if (cpu.f & 0x10) >> 4 == 1 {
        cpu.pc = cpu.pc.wrapping_add(2).wrapping_add(signed_value);
        cpu.set_cycles(12);
    } else {
        cpu.set_cycles(8);
        cpu.pc += 2;
    }
}

pub fn jp_hl(cpu: &mut CPU) {
    cpu.set_cycles(4);
    cpu.pc = ((cpu.h as u16) << 8) | (cpu.l as u16);
}

pub fn jp_u16(cpu: &mut CPU) {
    let address = {
        let mmu = cpu.mmu.lock().unwrap();
        ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
    };
    cpu.set_cycles(16);
    cpu.pc = address;
}

pub fn jp_nz_u16(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 != 1 {
        let address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.set_cycles(16);
        cpu.pc = address;
    } else {
        cpu.set_cycles(12);
        cpu.pc += 3;
    }
}

pub fn jp_z_u16(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 == 1 {
        let address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.set_cycles(16);
        cpu.pc = address;
    } else {
        cpu.set_cycles(12);
        cpu.pc += 3;
    }
}

pub fn jp_nc_u16(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 != 1 {
        let address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.set_cycles(16);
        cpu.pc = address;
    } else {
        cpu.set_cycles(12);
        cpu.pc += 3;
    }
}

pub fn jp_c_u16(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 == 1 {
        let address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.set_cycles(16);
        cpu.pc = address;
    } else {
        cpu.set_cycles(12);
        cpu.pc += 3;
    }
}

pub fn pop_bc(cpu: &mut CPU) {
    let (b, c) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.sp + 1), mmu.fetch_u8(cpu.sp))
    };

    cpu.b = b;
    cpu.c = c;
    cpu.sp += 2;
    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn pop_de(cpu: &mut CPU) {
    let (d, e) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.sp + 1), mmu.fetch_u8(cpu.sp))
    };

    cpu.d = d;
    cpu.e = e;
    cpu.sp += 2;
    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn pop_hl(cpu: &mut CPU) {
    let (h, l) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.sp + 1), mmu.fetch_u8(cpu.sp))
    };

    cpu.h = h;
    cpu.l = l;
    cpu.sp += 2;
    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn pop_af(cpu: &mut CPU) {
    let (a, f) = {
        let mmu = cpu.mmu.lock().unwrap();
        (mmu.fetch_u8(cpu.sp + 1), mmu.fetch_u8(cpu.sp))
    };

    cpu.a = a;
    cpu.update_flags(
        Some(f & 0x80 != 0),
        Some(f & 0x10 != 0),
        Some(f & 0x40 != 0),
        Some(f & 0x20 != 0),
    );

    cpu.sp += 2;
    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn push_bc(cpu: &mut CPU) {
    cpu.sp -= 2;
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(cpu.sp + 1, cpu.b);
        mmu.write_byte(cpu.sp, cpu.c);
    }
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn push_de(cpu: &mut CPU) {
    cpu.sp -= 2;
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(cpu.sp + 1, cpu.d);
        mmu.write_byte(cpu.sp, cpu.e);
    }
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn push_hl(cpu: &mut CPU) {
    cpu.sp -= 2;
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(cpu.sp + 1, cpu.h);
        mmu.write_byte(cpu.sp, cpu.l);
    }
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn push_af(cpu: &mut CPU) {
    cpu.sp -= 2;
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(cpu.sp + 1, cpu.a);
        mmu.write_byte(cpu.sp, cpu.f);
    }
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn call_u16(cpu: &mut CPU) {
    let target_address = {
        let mmu = cpu.mmu.lock().unwrap();
        ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
    };
    cpu.pc += 3;
    let lower_byte = (cpu.pc & 0xFF) as u8;
    let upper_byte = (cpu.pc >> 8) as u8;
    cpu.sp -= 2;
    
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(cpu.sp + 1, upper_byte);
        mmu.write_byte(cpu.sp, lower_byte);
    }

    cpu.pc = target_address;
    cpu.set_cycles(24);
}

pub fn call_nz_u16(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 != 1 {
        let target_address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.pc += 3;
        let lower_byte = (cpu.pc & 0xFF) as u8;
        let upper_byte = (cpu.pc >> 8) as u8;
        cpu.sp -= 2;

        {
            let mut mmu = cpu.mmu.lock().unwrap();
            mmu.write_byte(cpu.sp + 1, upper_byte);
            mmu.write_byte(cpu.sp, lower_byte);
        }

        cpu.pc = target_address;
        cpu.set_cycles(24);
    } else {
        cpu.pc += 3;
        cpu.set_cycles(12);
    }
}

pub fn call_z_u16(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 == 1 {
        let target_address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.pc += 3;
        let lower_byte = (cpu.pc & 0xFF) as u8;
        let upper_byte = (cpu.pc >> 8) as u8;
        cpu.sp -= 2;

        {
            let mut mmu = cpu.mmu.lock().unwrap();
            mmu.write_byte(cpu.sp + 1, upper_byte);
            mmu.write_byte(cpu.sp, lower_byte);
        }

        cpu.pc = target_address;
        cpu.set_cycles(24);
    } else {
        cpu.pc += 3;
        cpu.set_cycles(12);
    }
}

pub fn call_nc_u16(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 != 1 {
        let target_address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.pc += 3;
        let lower_byte = (cpu.pc & 0xFF) as u8;
        let upper_byte = (cpu.pc >> 8) as u8;
        cpu.sp -= 2;

        {
            let mut mmu = cpu.mmu.lock().unwrap();
            mmu.write_byte(cpu.sp + 1, upper_byte);
            mmu.write_byte(cpu.sp, lower_byte);
        }

        cpu.pc = target_address;
        cpu.set_cycles(24);
    } else {
        cpu.pc += 3;
        cpu.set_cycles(12);
    }
}

pub fn call_c_u16(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 == 1 {
        let target_address = {
            let mmu = cpu.mmu.lock().unwrap();
            ((mmu.fetch_u8(cpu.pc + 2) as u16) << 8) | (mmu.fetch_u8(cpu.pc + 1) as u16)
        };
        cpu.pc += 3;
        let lower_byte = (cpu.pc & 0xFF) as u8;
        let upper_byte = (cpu.pc >> 8) as u8;
        cpu.sp -= 2;

        {
            let mut mmu = cpu.mmu.lock().unwrap();
            mmu.write_byte(cpu.sp + 1, upper_byte);
            mmu.write_byte(cpu.sp, lower_byte);
        }

        cpu.pc = target_address;
        cpu.set_cycles(24);
    } else {
        cpu.pc += 3;
        cpu.set_cycles(12);
    }
}

pub fn ret(cpu: &mut CPU) {
    cpu.set_cycles(16);
    let (new_pc, new_sp) = {
        let mmu = cpu.mmu.lock().unwrap();
        let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
        (pc, cpu.sp + 2)
    };
    cpu.pc = new_pc;
    cpu.sp = new_sp;
}

pub fn reti(cpu: &mut CPU) {
    cpu.set_cycles(16);
    let (new_pc, new_sp) = {
        let mmu = cpu.mmu.lock().unwrap();
        let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
        (pc, cpu.sp + 2)
    };
    cpu.pc = new_pc;
    cpu.sp = new_sp;
    cpu.ime = true;
}

pub fn ret_nz(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 != 1 {
        cpu.set_cycles(20);
        let (new_pc, new_sp) = {
            let mmu = cpu.mmu.lock().unwrap();
            let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
            (pc, cpu.sp + 2)
        };
        cpu.pc = new_pc;
        cpu.sp = new_sp;
    } else {
        cpu.set_cycles(8);
        cpu.pc += 1;
    }
}

pub fn ret_z(cpu: &mut CPU) {
    if (cpu.f & 0x80) >> 7 == 1 {
        cpu.set_cycles(20);
        let (new_pc, new_sp) = {
            let mmu = cpu.mmu.lock().unwrap();
            let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
            (pc, cpu.sp + 2)
        };
        cpu.pc = new_pc;
        cpu.sp = new_sp;
    } else {
        cpu.set_cycles(8);
        cpu.pc += 1;
    }
}

pub fn ret_nc(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 != 1 {
        cpu.set_cycles(20);
        let (new_pc, new_sp) = {
            let mmu = cpu.mmu.lock().unwrap();
            let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
            (pc, cpu.sp + 2)
        };
        cpu.pc = new_pc;
        cpu.sp = new_sp;
    } else {
        cpu.set_cycles(8);
        cpu.pc += 1;
    }
}

pub fn ret_c(cpu: &mut CPU) {
    if (cpu.f & 0x10) >> 4 == 1 {
        cpu.set_cycles(20);
        let (new_pc, new_sp) = {
            let mmu = cpu.mmu.lock().unwrap();
            let pc = ((mmu.fetch_u8(cpu.sp + 1) as u16) << 8) | (mmu.fetch_u8(cpu.sp) as u16); 
            (pc, cpu.sp + 2)
        };
        cpu.pc = new_pc;
        cpu.sp = new_sp;
    } else {
        cpu.set_cycles(8);
        cpu.pc += 1;
    }
}

pub fn rst(cpu: &mut CPU, address: u16) {
    cpu.pc += 1;
    
    let (lower_byte, upper_byte, new_sp) = {
        let lower_byte = (cpu.pc & 0xFF) as u8;
        let upper_byte = (cpu.pc >> 8) as u8;
        (lower_byte, upper_byte, cpu.sp.wrapping_sub(2))
    };
    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(new_sp.wrapping_add(1), upper_byte);
        mmu.write_byte(new_sp, lower_byte);
    }
    cpu.sp = new_sp;
    cpu.set_cycles(16);
    cpu.pc = address;
}

pub fn rlca(cpu: &mut CPU) {
    let carry_flag = cpu.a >> 7;
    cpu.a = (cpu.a << 1) | carry_flag;
    let carry = carry_flag != 0;
    cpu.update_flags(Some(false), Some(carry), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn rrca(cpu: &mut CPU) {
    let carry_flag = cpu.a & 1;
    cpu.a = (cpu.a >> 1) | (carry_flag << 7);
    let carry = carry_flag != 0;
    cpu.update_flags(Some(false), Some(carry), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn rla(cpu: &mut CPU) {
    let carry_flag = cpu.a >> 7;
    cpu.a = (cpu.a << 1) | ((cpu.f & 0x10) >> 4);
    let carry = carry_flag != 0;
    cpu.update_flags(Some(false), Some(carry), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}   

pub fn rra(cpu: &mut CPU) {
    let carry_flag = cpu.a & 1;
    cpu.a = (cpu.a >> 1) | ((cpu.f & 0x10) << 3);
    let carry = carry_flag != 0;
    cpu.update_flags(Some(false), Some(carry), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn daa(cpu: &mut CPU) {
    let mut correction = 0;
    let mut carry = cpu.f & 0x10 != 0;

    if cpu.f & 0x40 == 0 {
        if carry || cpu.a > 0x99 {
            correction |= 0x60;
            carry = true;
        }
        if cpu.f & 0x20 != 0 || (cpu.a & 0x0F) > 0x09 {
            correction |= 0x06;
        }
        cpu.a = cpu.a.wrapping_add(correction);
    } else {
        if carry {
            correction |= 0x60;
        }
        if cpu.f & 0x20 != 0 {
            correction |= 0x06;
        }
        cpu.a = cpu.a.wrapping_sub(correction);
    }

    let zero = cpu.a == 0;
    cpu.update_flags(Some(zero), Some(carry), None, Some(false));
    
    cpu.set_cycles(4);
    cpu.pc += 1;
}



pub fn cpl(cpu: &mut CPU) {
    cpu.a = !cpu.a;
    cpu.update_flags(None, None, Some(true), Some(true));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn scf(cpu: &mut CPU) {
    cpu.update_flags(None, Some(true), Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ccf(cpu: &mut CPU) {
    cpu.f ^= 0x10;
    cpu.update_flags(None, None, Some(false), Some(false));
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn ei(cpu: &mut CPU) {
    cpu.ime = true;
    cpu.set_cycles(4);
    cpu.pc += 1;
}

pub fn di(cpu: &mut CPU) {
    cpu.ime = false;
    cpu.set_cycles(4);
    cpu.pc += 1;        
}

pub fn rlc(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register >> 7;
    let result = (register << 1) | carry_flag;
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn rlc_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let carry_flag = value >> 7;
        let result = (value << 1) | carry_flag;
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value >> 7) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn rrc(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register & 1;
    let result = (register >> 1) | (carry_flag << 7);
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn rrc_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let carry_flag = value & 1;
        let result = (value >> 1) | (carry_flag << 7);
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value & 1) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn rl(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register >> 7;
    let result = (register << 1) | ((cpu.f & 0x10) >> 4);
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn rl_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let result = (value << 1) | ((cpu.f & 0x10) >> 4);
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value >> 7) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn rr(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register & 1;
    let result = (register >> 1) | ((cpu.f & 0x10) << 3);
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn rr_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let result = (value >> 1) | ((cpu.f & 0x10) << 3);
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value & 1) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn sla(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register >> 7;
    let result = register << 1;
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn sla_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let result = value << 1;
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value >> 7) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn sra(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register & 1;
    let result = register >> 1 | register & 0x80;
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn sra_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let (value, result) = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let result = (value >> 1) | (value & 0x80);
        mmu.write_byte(address, result);
        (value, result)
    };

    let carry = (value & 1) != 0;
    let zero = result == 0;

    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn swap(cpu: &mut CPU, register: u8) -> u8 {
    let result = register >> 4 | register << 4;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn swap_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let result = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let result = (value >> 4) | (value << 4);
        mmu.write_byte(address, result);
        result
    };

    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(false), Some(false), Some(false));
    cpu.set_cycles(16);
    cpu.pc += 1;
}


pub fn srl(cpu: &mut CPU, register: u8) -> u8 {
    let carry_flag = register & 1;
    let result = register >> 1;
    let carry = carry_flag != 0;
    let zero = result == 0;
    cpu.update_flags(Some(zero), Some(carry), Some(false), Some(false));
    cpu.set_cycles(8);
    cpu.pc += 1;
    result
}

pub fn srl_m_hl(cpu: &mut CPU) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let result = {
        let mut mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        let carry_flag = value & 0x01 != 0;
        let result = value >> 1;
        mmu.write_byte(address, result);
        (result, carry_flag)
    };

    let zero = result.0 == 0;

    cpu.update_flags(Some(zero), Some(result.1), Some(false), Some(false));

    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn bit_n_r_u8(cpu: &mut CPU, bit: u8, register: u8) {
    let zero = (register & (1 << bit)) == 0;
    cpu.update_flags(Some(zero), None, Some(false), Some(true));
    cpu.set_cycles(8);
    cpu.pc += 1;
}

pub fn bit_n_m_hl(cpu: &mut CPU, bit: u8) {
    let value = {
        let address = ((cpu.h as u16) << 8) | (cpu.l as u16);
        let mmu = cpu.mmu.lock().unwrap();
        mmu.read_byte(address)
    };

    let zero = (value & (1 << bit)) == 0;
    cpu.update_flags(Some(zero), None, Some(false), Some(true));
    cpu.set_cycles(12);
    cpu.pc += 1;
}

pub fn res_n_r_u8(cpu: &mut CPU, bit: u8, register: u8) -> u8 {
    cpu.pc += 1;
    cpu.set_cycles(8);
    register & !(1 << bit)
}

pub fn res_n_m_hl(cpu: &mut CPU, bit: u8) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let result = {
        let mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        value & !(1 << bit)
    };

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, result);
    }

    cpu.set_cycles(16);
    cpu.pc += 1;
}

pub fn set_n_r_u8(cpu: &mut CPU, bit: u8, register: u8) -> u8 {
    cpu.pc += 1;
    cpu.set_cycles(8);
    register | 1 << bit
}

pub fn set_n_m_hl(cpu: &mut CPU, bit: u8) {
    let address = ((cpu.h as u16) << 8) | (cpu.l as u16);

    let result = {
        let mmu = cpu.mmu.lock().unwrap();
        let value = mmu.read_byte(address);
        value | (1 << bit)
    };

    {
        let mut mmu = cpu.mmu.lock().unwrap();
        mmu.write_byte(address, result);
    }

    cpu.set_cycles(16);
    cpu.pc += 1;
}