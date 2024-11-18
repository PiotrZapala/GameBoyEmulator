use std::sync::{Arc, Mutex};

use crate::mmu::MMU;
use crate::cpu::CPU;

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
    dma_transfer_enabled: bool,
    screen_buffer: [[u32; 160]; 144], // Screen buffer
    mmu: Option<Arc<Mutex<MMU>>>,
    cpu: Option<Arc<Mutex<CPU>>>,
    mode: u8,         
    cycles: u32,
    frame_ready: bool,
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
            dma_transfer_enabled: false,
            screen_buffer: [[0; 160]; 144],
            mmu: None,
            cpu: None,
            mode: 2, 
            cycles: 0,
            frame_ready: false,
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
                self.dma_transfer_enabled = true;
            },
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => panic!("Attempted to write to an invalid memory address: {:04X}", address),
        }
    }

    pub fn tick(&mut self, cycles: u16) {
        self.cycles += cycles as u32;
    
        match self.mode {
            2 => { // OAM Search
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.stat |= 0b11;
                    self.mode = 3;
                }
            }
            3 => { // Access VRAM
                if self.cycles >= 170 {
                    self.cycles -= 170;

                    if self.stat & (1 << 3) != 0 {
                        if let Some(cpu) = &self.cpu {
                            cpu.lock().unwrap().request_interrupt(0b00000010);
                        }
                    }
                    let ly_coincidence = self.ly == self.lyc;
                    if ly_coincidence {
                        self.stat |= 1 << 2;
                        if self.stat & (1 << 6) != 0 {
                            if let Some(cpu) = &self.cpu {
                                cpu.lock().unwrap().request_interrupt(0b00000010);
                            }
                        }
                    } else {
                        self.stat &= !(1 << 2);
                    }
    
                    self.stat &= !0b11;
                    self.mode = 0;
                }
            }
            0 => { // H-Blank
                if self.cycles >= 206 {
                    self.cycles -= 206;
                    self.render_scanline();
                    self.ly += 1;

                    if self.ly == 144 {
                        self.stat |= 0b01;
                        self.stat &= !(1 << 1);
                        self.mode = 1;
                        if let Some(cpu) = &self.cpu {
                            cpu.lock().unwrap().request_interrupt(0b00000001);
                        }
                    } else {
                        self.stat |= 0b10;
                        self.stat &= !0b01;
                        self.mode = 2;
                    }
                }
            }
            1 => { // V-Blank
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.ly += 1;
    
                    if self.ly > 153 {
                        self.ly = 0;
                        self.stat |= 0b10;
                        self.stat &= !0b01;
                        self.mode = 2;
                        self.frame_ready = true;
                    }
                }
            }
            _ => panic!("Unknown mode!"),
        }
    }

    pub fn is_frame_ready(&self) -> bool {
        self.frame_ready
    }

    pub fn reset_frame_ready(&mut self) {
        self.frame_ready = false;
    }
    
    pub fn is_display_enabled(&self) -> bool {
        (self.lcdc & 0x80) != 0
    }

    fn is_background_enabled(&self) -> bool {
        self.lcdc & 0x01 != 0
    }

    pub fn set_mmu(&mut self, mmu: Arc<Mutex<MMU>>) {
        self.mmu = Some(mmu);
    }

    pub fn set_cpu(&mut self, cpu: Arc<Mutex<CPU>>) {
        self.cpu = Some(cpu);
    }

    pub fn get_screen_buffer(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = Vec::with_capacity(160 * 144);
        for y in 0..144 {
            for x in 0..160 {
                let pixel_color = self.screen_buffer[y][x];
                buffer.push(pixel_color);
            }
        }
        buffer
    }

    pub fn dma_transfer(&mut self) {
        if self.dma_transfer_enabled {
            let source_address = (self.dma as u16) << 8;
            if let Some(mmu) = &self.mmu {
                for i in 0..0xA0 {
                    let byte = mmu.lock().unwrap().read_byte(source_address + i as u16);
                    self.oam[i] = byte;
                }
            } else {
                panic!("PPU has no reference to MMU.");
            }
        }
        self.dma_transfer_enabled = false;
    }

    pub fn render_scanline(&mut self) {
        if !self.is_display_enabled() {
            return;
        }
        if self.is_background_enabled() {
            self.render_background();
        }
        self.render_window();
        self.render_sprites();
    }

    pub fn render_background(&mut self) {
        let tile_map_start: usize = if self.lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
        let tile_data_start: usize = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };
    
        let scy = self.scy;
        let scx = self.scx;
    
        let y_offset = ((self.ly as u16 + scy as u16) & 0xFF) as u8;
    
        for lx in 0..160 {
            let x_offset = ((lx as u16 + scx as u16) & 0xFF) as u8;
    
            let tile_col = x_offset / 8;
            let tile_row = y_offset / 8;
            let tile_index = self.read_byte((tile_map_start + (tile_row as usize * 32) + tile_col as usize) as u16);
    
            let tile_data_address = if tile_data_start == 0x8800 {
                tile_data_start + ((tile_index as i8 as i16 + 128) as usize * 16)
            } else {
                tile_data_start + (tile_index as usize * 16)
            };
    
            let pixel_row_in_tile = y_offset % 8;
            let byte1 = self.read_byte((tile_data_address + pixel_row_in_tile as usize * 2) as u16);
            let byte2 = self.read_byte((tile_data_address + pixel_row_in_tile as usize * 2 + 1) as u16);
    
            let pixel_column_in_tile = 7 - (x_offset % 8);
            let low_bit = (byte1 >> pixel_column_in_tile) & 1;
            let high_bit = (byte2 >> pixel_column_in_tile) & 1;
            let color_index = (high_bit << 1) | low_bit;
    
            let color = self.get_bg_color(color_index);
    
            self.screen_buffer[self.ly as usize][lx as usize] = color;
        }
    }

    fn render_window(&mut self) {
        if self.lcdc & 0x20 != 0 && self.ly >= self.wy {
            let tile_map_start: usize = if self.lcdc & 0x40 != 0 { 0x9C00 } else { 0x9800 };
            let tile_data_start: usize = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };
    
            let window_y = self.ly - self.wy;
    
            for lx in 0..160 {
                if lx + 7 >= self.wx {
                    let window_x = lx + 7 - self.wx;
    
                    let tile_col = window_x / 8;
                    let tile_row = window_y / 8;
    
                    let tile_index_address = tile_map_start + (tile_row as usize * 32) + tile_col as usize;
                    let tile_index = self.read_byte(tile_index_address as u16);
    
                    let tile_data_address = if tile_data_start == 0x8800 {
                        tile_data_start + ((tile_index as i8 as i16 + 128) as usize * 16)
                    } else {
                        tile_data_start + (tile_index as usize * 16)
                    };
    
                    let pixel_row_in_tile = window_y % 8;
                    let byte1 = self.read_byte((tile_data_address + pixel_row_in_tile as usize * 2) as u16);
                    let byte2 = self.read_byte((tile_data_address + pixel_row_in_tile as usize * 2 + 1) as u16);
    
                    let pixel_column_in_tile = 7 - (window_x % 8);
                    let low_bit = (byte1 >> pixel_column_in_tile) & 1;
                    let high_bit = (byte2 >> pixel_column_in_tile) & 1;
                    let color_index = (high_bit << 1) | low_bit;
    
                    let color = self.get_bg_color(color_index);
    
                    self.screen_buffer[self.ly as usize][lx as usize] = color;
                }
            }
        }
    }

    fn render_sprites(&mut self) {
        let mut sprite_count = 0;
    
        for sprite_index in 0..40 {
            if sprite_count >= 10 {
                break;
            }
    
            let sprite_y = self.oam[sprite_index * 4] as i16 - 16;
            let sprite_x = self.oam[sprite_index * 4 + 1] as i16 - 8;
            let tile_index = self.oam[sprite_index * 4 + 2];
            let attributes = self.oam[sprite_index * 4 + 3];
    
            let sprite_size = if self.lcdc & 0x04 != 0 { 16 } else { 8 };
    
            if sprite_y <= self.ly as i16 && (sprite_y + sprite_size as i16) > self.ly as i16 {
                let y_flip = attributes & 0x40 != 0;
                let x_flip = attributes & 0x20 != 0;
                let palette_index = if attributes & 0x10 != 0 { 1 } else { 0 };
    
                let sprite_row = if y_flip {
                    sprite_size - 1 - (self.ly as i16 - sprite_y) as u8
                } else {
                    (self.ly as i16 - sprite_y) as u8
                };
    
                for lx in 0..8 {
                    let sprite_col = if x_flip { 7 - lx } else { lx };
    
                    let tile_data_address: u16 = if self.lcdc & 0x10 == 0 && tile_index >= 0x80 {
                        0x8800u16.wrapping_add((tile_index as i8 as i16 + 128) as u16 * 16)
                    } else {
                        0x8000 + (tile_index as u16 * 16)
                    };
    
                    let tile_line = self.get_tile_data(tile_data_address, sprite_row, sprite_col);
    
                    if tile_line != 0 {
                        let color = self.get_sprite_color(tile_line, palette_index);
    
                        let pixel_x = sprite_x + lx as i16;
                        if pixel_x >= 0 && pixel_x < 160 {
                            let priority = attributes & 0x80 == 0;
                            let bg_pixel = self.screen_buffer[self.ly as usize][pixel_x as usize];
                            if priority || bg_pixel == 0x00FFFFFF {
                                self.screen_buffer[self.ly as usize][pixel_x as usize] = color;
                            }
                        }
                    }
                }
                sprite_count += 1;
            }
        }
    }
    

    fn get_tile_data(&self, tile_data_address: u16, row: u8, col: u8) -> u8 {
        let byte1 = self.read_byte(tile_data_address + row as u16 * 2);
        let byte2 = self.read_byte(tile_data_address + row as u16 * 2 + 1);
    
        let bit = 7 - col;
        let low_bit = (byte1 >> bit) & 1;
        let high_bit = (byte2 >> bit) & 1;
        let result = (high_bit << 1) | low_bit;
    
        result
    }
    

    fn get_bg_color(&self, color_index: u8) -> u32 {
        let palette = self.bgp;
        let shade = (palette >> (color_index * 2)) & 0x03;

        let color = match shade {
            0 => 0x00FFFFFF, // White (0RGB: 00FF FF FF)
            1 => 0x00AAAAAA, // Light grey (0RGB: 00AA AA AA)
            2 => 0x00555555, // Dark grey (0RGB: 0055 55 55)
            3 => 0x00000000, // Black (0RGB: 0000 00 00)
            _ => 0x00000000, // Default to black
        };
        color
    }

    fn get_sprite_color(&self, color_index: u8, palette_index: u8) -> u32 {
        let palette = if palette_index == 0 { self.obp0 } else { self.obp1 };
        let shade = (palette >> (color_index * 2)) & 0x03;
    
        let color = match shade {
            0 => 0x00FFFFFF, // Transparent (0RGB: 00FF FF FF)
            1 => 0x00AAAAAA, // Light grey (0RGB: 00AA AA AA)
            2 => 0x00555555, // Dark grey (0RGB: 0055 55 55)
            3 => 0x00000000, // Black (0RGB: 0000 00 00)
            _ => 0x00000000, // Default to black
        };
        color
    }
}
