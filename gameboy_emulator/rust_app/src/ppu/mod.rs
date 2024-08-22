use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    pub lcdc: u8,         // LCD Control (0xFF40)
    pub stat: u8,         // LCDC Status (0xFF41)
    pub scy: u8,          // Scroll Y (0xFF42)
    pub scx: u8,          // Scroll X (0xFF43)
    pub ly: u8,           // LY (0xFF44)
    pub lyc: u8,          // LYC (0xFF45)
    pub dma: u8,          // DMA Transfer and Start Address (0xFF46)
    pub bgp: u8,          // BG Palette Data (0xFF47)
    pub obp0: u8,         // Object Palette 0 Data (0xFF48)
    pub obp1: u8,         // Object Palette 1 Data (0xFF49)
    pub wy: u8,           // Window Y Position (0xFF4A)
    pub wx: u8,           // Window X Position minus 7 (0xFF4B)
    pub vram: [u8; 8192], // Video RAM (0x8000 - 0x9FFF)
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            vram: [0; 8192],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("Attempted to read from an invalid memory address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }
}
