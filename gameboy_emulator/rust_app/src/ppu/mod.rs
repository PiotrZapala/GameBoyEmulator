use std::cell::RefCell;
use std::rc::Rc;

use crate::mmu::MMU;

pub struct PPU {
    lcdc: u8,         // LCD Control (0xFF40)
    stat: u8,         // LCDC Status (0xFF41)
    scy: u8,          // Scroll Y (0xFF42)
    scx: u8,          // Scroll X (0xFF43)
    ly: u8,           // LY (0xFF44)
    lyc: u8,          // LYC (0xFF45)
    dma: u8,          // DMA Transfer and Start Address (0xFF46)
    bgp: u8,          // BG Palette Data (0xFF47)
    obp0: u8,         // Object Palette 0 Data (0xFF48)
    obp1: u8,         // Object Palette 1 Data (0xFF49)
    wy: u8,           // Window Y Position (0xFF4A)
    wx: u8,           // Window X Position minus 7 (0xFF4B)
    vram: [u8; 8192], // Video RAM (0x8000 - 0x9FFF)
    oam: [u8; 160],   // Object Attribute Memory (0xFE00 - 0xFE9F)
    mmu: Option<Rc<RefCell<MMU>>>,
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
            oam: [0; 160],
            mmu: None,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
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
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => {
                self.dma = value;
                self.dma_transfer(value);
            },
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }

    pub fn set_mmu(&mut self, mmu: Rc<RefCell<MMU>>) {
        self.mmu = Some(mmu);
    }

    fn dma_transfer(&mut self, value: u8) {
        let source_address = (value as u16) << 8;
        if let Some(mmu) = &self.mmu {
            for i in 0..0xA0 {
                let byte = mmu.borrow().read_byte(source_address + i as u16);
                self.oam[i] = byte;
            }
        } else {
            panic!("PPU has no reference to MMU.");
        }
    }

    pub fn render_line(&mut self, ly: u8) {
        self.render_background(ly);
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    
    use crate::mmu::MMU;
    
    pub struct PPU {
        lcdc: u8,         // LCD Control (0xFF40)
        stat: u8,         // LCDC Status (0xFF41)
        scy: u8,          // Scroll Y (0xFF42)
        scx: u8,          // Scroll X (0xFF43)
        ly: u8,           // LY (0xFF44)
        lyc: u8,          // LYC (0xFF45)
        dma: u8,          // DMA Transfer and Start Address (0xFF46)
        bgp: u8,          // BG Palette Data (0xFF47)
        obp0: u8,         // Object Palette 0 Data (0xFF48)
        obp1: u8,         // Object Palette 1 Data (0xFF49)
        wy: u8,           // Window Y Position (0xFF4A)
        wx: u8,           // Window X Position minus 7 (0xFF4B)
        vram: [u8; 8192], // Video RAM (0x8000 - 0x9FFF)
        oam: [u8; 160],   // Object Attribute Memory (0xFE00 - 0xFE9F)
        screen_buffer: [[u32; 160]; 144], // Screen buffer
        mmu: Option<Rc<RefCell<MMU>>>,
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
                oam: [0; 160],
                screen_buffer: [[0; 160]; 144],
                mmu: None,
            }
        }
    
        pub fn read_byte(&self, address: u16) -> u8 {
            match address {
                0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
                0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
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
                0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
                0xFF40 => self.lcdc = value,
                0xFF41 => self.stat = value,
                0xFF42 => self.scy = value,
                0xFF43 => self.scx = value,
                0xFF44 => self.ly = value,
                0xFF45 => self.lyc = value,
                0xFF46 => {
                    self.dma = value;
                    self.dma_transfer(value);
                },
                0xFF47 => self.bgp = value,
                0xFF48 => self.obp0 = value,
                0xFF49 => self.obp1 = value,
                0xFF4A => self.wy = value,
                0xFF4B => self.wx = value,
                _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
            }
        }
    
        pub fn set_mmu(&mut self, mmu: Rc<RefCell<MMU>>) {
            self.mmu = Some(mmu);
        }
    
        fn dma_transfer(&mut self, value: u8) {
            let source_address = (value as u16) << 8;
            if let Some(mmu) = &self.mmu {
                for i in 0..0xA0 {
                    let byte = mmu.borrow().read_byte(source_address + i as u16);
                    self.oam[i] = byte;
                }
            } else {
                panic!("PPU has no reference to MMU.");
            }
        }
    
        pub fn render_scanline(&mut self, ly: u8) {
            self.render_background(ly);
        }
    
        fn render_background(&mut self, ly: u8) {
            let tile_map_start: usize = if self.lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
            let tile_data_start: usize = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };
            
            let scy = self.scy;
            let scx = self.scx;
    
            let y_offset = ((ly as u16 + scy as u16) & 0xFF) as u8;
    
            for lx in 0..160 {
                let x_offset = ((lx as u16 + scx as u16) & 0xFF) as u8;
    
                let tile_col = x_offset / 8;
                let tile_row = y_offset / 8;
                let tile_index = self.vram[tile_map_start + (tile_row as usize * 32) + tile_col as usize];
    
                let tile_data_address = tile_data_start + (tile_index as usize * 16);
                let pixel_row_in_tile = y_offset % 8;
                let byte1 = self.vram[tile_data_address + pixel_row_in_tile as usize * 2];
                let byte2 = self.vram[tile_data_address + pixel_row_in_tile as usize * 2 + 1];
    
                let pixel_column_in_tile = 7 - (x_offset % 8);
                let low_bit = (byte1 >> pixel_column_in_tile) & 1;
                let high_bit = (byte2 >> pixel_column_in_tile) & 1;
                let color_index = (high_bit << 1) | low_bit;
    
                let color = self.get_bg_color(color_index);
    
                self.screen_buffer[ly as usize][lx as usize] = color;
            }
        }
    
        fn get_bg_color(&self, color_index: u8) -> u32 {
            let palette = self.bgp;
            let shade = (palette >> (color_index * 2)) & 0x03;
    
            match shade {
                0 => 0xFFFFFFFF, // White
                1 => 0xAAAAAAFF, // Light grey
                2 => 0x555555FF, // Dark grey
                3 => 0x000000FF, // Black
                _ => 0x000000FF, 
            }
        }
    }
}
