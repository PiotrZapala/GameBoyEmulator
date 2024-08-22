mod mbc;

use mbc::MBC;
use crate::cartridge::CARTRIDGE;

pub struct MMU {
    mbc: Box<dyn MBC>,
    pub wram: [u8; 8192],  // Internal RAM (0xC000 - 0xDFFF)
    pub vram: [u8; 8192],  // Video RAM (0x8000 - 0x9FFF)
    pub hram: [u8; 127],   // High RAM (0xFF80 - 0xFFFE)
    pub oam: [u8; 160],    // Object Attribute Memory (0xFE00 - 0xFE9F)
    pub io: [u8; 128],     // I/O Registers (0xFF00 - 0xFF7F)
    pub ie: u8,            // Interrupt Enable Register
}

impl MMU {
    pub fn new(cartridge: CARTRIDGE) -> Self {
        let mbc = mbc::create_mbc(cartridge);
        MMU {
            mbc,
            wram: [0; 8192],
            vram: [0; 8192],
            hram: [0; 127],
            oam: [0; 160],
            io: [0; 128],
            ie: 0,
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
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => self.mbc.read_byte(address),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00],
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.ie,
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
            0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00] = value,
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => self.ie = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}