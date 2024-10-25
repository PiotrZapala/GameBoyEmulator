mod mbc;

use mbc::MBC;
use crate::ppu::PPU;
use crate::apu::APU;
use crate::timer::TIMER;
use crate::joypad::JOYPAD;
use crate::bootrom::BOOTROM;
use crate::cartridge::CARTRIDGE;

use std::sync::{Arc, Mutex};

pub struct MMU {
    mbc: Box<dyn MBC>,
    joypad: Arc<Mutex<JOYPAD>>,
    timer: Arc<Mutex<TIMER>>,
    apu: Arc<Mutex<APU>>,
    ppu: Arc<Mutex<PPU>>,
    bootrom: BOOTROM,
    wram: [u8; 8192],     // Work RAM (0xC000 - 0xDFFF)
    hram: [u8; 127],      // High RAM (0xFF80 - 0xFFFE)
    io: [u8; 71],         // IO
    unusable_area: [u8; 96],
    interrupt_flag: u8,   // Interrupt Flag
    interrupt_enable: u8, // Interrupt Enable Register
}

impl MMU {
    pub fn new(joypad: Arc<Mutex<JOYPAD>>, timer: Arc<Mutex<TIMER>>, apu: Arc<Mutex<APU>>, ppu: Arc<Mutex<PPU>>, cartridge: CARTRIDGE) -> Self {
        MMU {
            mbc: mbc::create_mbc(cartridge),
            wram: [0; 8192],
            hram: [0; 127],
            io: [0; 71],
            unusable_area: [0; 96],
            interrupt_enable: 0,
            interrupt_flag: 0,
            bootrom: BOOTROM::new(),
            joypad,
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
            0x0000..=0x7FFF => {
                if address <= 0x00FF && self.bootrom.boot_enabled {
                    self.bootrom.read_byte(address)
                } else {
                    self.mbc.read_byte(address)
                }
            }
            0x8000..=0x9FFF => self.ppu.lock().unwrap().read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_byte(address),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.ppu.lock().unwrap().read_byte(address),
            0xFEA0..=0xFEFF => self.unusable_area[address as usize - 0xFEA0],
            0xFF00 => self.joypad.lock().unwrap().read_byte(),
            0xFF01..=0xFF03 => self.io[address as usize - 0xFF01],
            0xFF04..=0xFF07 => self.timer.lock().unwrap().read_byte(address),
            0xFF08..=0xFF0E => self.io[address as usize - 0xFF08 + 3],
            0xFF0F => self.interrupt_flag,
            0xFF10..=0xFF26 => self.apu.lock().unwrap().read_byte(address),
            0xFF27..=0xFF2F => self.io[address as usize - 0xFF27 + 10],
            0xFF30..=0xFF3F => self.apu.lock().unwrap().read_byte(address),
            0xFF40..=0xFF4B => self.ppu.lock().unwrap().read_byte(address),
            0xFF4C..=0xFF7F => self.io[address as usize - 0xFF4C + 19],
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.interrupt_enable,
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_byte(address, value),
            0x8000..=0x9FFF => self.ppu.lock().unwrap().write_byte(address, value),
            0xA000..=0xBFFF => self.mbc.write_byte(address, value),
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value,
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000] = value,
            0xFE00..=0xFE9F => self.ppu.lock().unwrap().write_byte(address, value),
            0xFEA0..=0xFEFF => self.unusable_area[address as usize - 0xFEA0] = value,
            0xFF00 => self.joypad.lock().unwrap().write_byte(value),
            0xFF01..=0xFF03 => self.io[address as usize - 0xFF01] = value,
            0xFF04..=0xFF07 => self.timer.lock().unwrap().write_byte(address, value),
            0xFF08..=0xFF0E => self.io[address as usize - 0xFF08 + 3] = value,
            0xFF0F => self.interrupt_flag = value,
            0xFF10..=0xFF26 => self.apu.lock().unwrap().write_byte(address, value),
            0xFF27..=0xFF2F => self.io[address as usize - 0xFF27 + 10] = value,
            0xFF30..=0xFF3F => self.apu.lock().unwrap().write_byte(address, value),
            0xFF40..=0xFF4B => self.ppu.lock().unwrap().write_byte(address, value),
            0xFF50 => {
                if value == 0x01 {
                    self.bootrom.disable();
                }
            }
            0xFF4C..=0xFF7F => self.io[address as usize - 0xFF4C + 19] = value,
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}
