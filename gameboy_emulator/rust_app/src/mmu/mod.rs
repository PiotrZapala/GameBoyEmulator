mod mbc;

use mbc::MBC;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::timer::TIMER;
use crate::cartridge::CARTRIDGE;

use std::cell::RefCell;
use std::rc::Rc;

pub struct MMU {
    mbc: Box<dyn MBC>,
    timer: Rc<RefCell<TIMER>>,
    apu: Rc<RefCell<APU>>,
    ppu: Rc<RefCell<PPU>>,
    wram: [u8; 8192],     // Work RAM (0xC000 - 0xDFFF)
    hram: [u8; 127],      // High RAM (0xFF80 - 0xFFFE)
    oam: [u8; 160],       // Object Attribute Memory (0xFE00 - 0xFE9F)
    interrupt_flag: u8,   // Interrupt Flag
    interrupt_enable: u8, // Interrupt Enable Register
}

impl MMU {
    pub fn new(timer: Rc<RefCell<TIMER>>, apu: Rc<RefCell<APU>>, ppu: Rc<RefCell<PPU>>, cartridge: CARTRIDGE) -> Self {
        MMU {
            mbc: mbc::create_mbc(cartridge),
            wram: [0; 8192],
            hram: [0; 127],
            oam: [0; 160],
            interrupt_enable: 0,
            interrupt_flag: 0,
            timer,
            apu,
            ppu,
        }
    }

    pub fn fetch_instruction(&self, pc: u16) -> u8 {
        self.read_byte(pc)
    }

    pub fn fetch_u8(&self, pc: u16) -> u8 {
        self.read_byte(pc)
    }

    pub fn fetch_i8(&self, pc: u16) -> i8 {
        self.read_byte(pc) as i8
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_byte(address),
            0x8000..=0x9FFF => self.ppu.borrow().read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_byte(address),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF04..=0xFF07 => self.timer.borrow().read_byte(address),
            0xFF0F => self.interrupt_flag,
            0xFF10..=0xFF3F => self.apu.borrow().read_byte(address),
            0xFF40..=0xFF4B => self.ppu.borrow().read_byte(address),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.interrupt_enable,
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.ppu.borrow_mut().write_byte(address, value),
            0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
            0xFF04..=0xFF07 => self.timer.borrow_mut().write_byte(address, value),
            0xFF0F => self.interrupt_flag = value,
            0xFF10..=0xFF3F => self.apu.borrow_mut().write_byte(address, value),
            0xFF40..=0xFF4B => self.ppu.borrow_mut().write_byte(address, value),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}